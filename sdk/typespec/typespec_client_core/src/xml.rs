// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! XML serialization functions.
use bytes::Bytes;
pub use quick_xml::serde_helpers::text_content as content;
use quick_xml::{
    de::{from_reader, from_str},
    se::{to_string, to_string_with_root},
};
use serde::de::DeserializeOwned;
use typespec::error::{ErrorKind, Result, ResultExt};

/// The UTF8 [byte order marker](https://en.wikipedia.org/wiki/Byte_order_mark).
const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

/// The XML declaration used when serializing.
const DECLARATION: &str = r#"<?xml version="1.0" encoding="utf-8"?>\n"#;

/// Reads XML from bytes.
pub fn read_xml_str<T>(body: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    from_str(body).with_context(ErrorKind::DataConversion, || {
        let t = core::any::type_name::<T>();
        format!("failed to deserialize the following xml into a {t}\n{body}")
    })
}

/// Reads XML from bytes.
pub fn read_xml<T>(body: &[u8]) -> Result<T>
where
    T: DeserializeOwned,
{
    from_reader(slice_bom(body)).with_context(ErrorKind::DataConversion, || {
        let t = core::any::type_name::<T>();
        let xml = std::str::from_utf8(body).unwrap_or("(XML is not UTF8-encoded)");
        format!("failed to deserialize the following xml into a {t}\n{xml}")
    })
}

/// Serializes a type to bytes.
/// Automaticallyl includes the XML declaration.
pub fn to_xml<T>(value: &T) -> Result<Bytes>
where
    T: serde::Serialize,
{
    let value = to_string(value).with_context(ErrorKind::DataConversion, || {
        let t = core::any::type_name::<T>();
        format!("failed to serialize {t} into xml")
    })?;
    Ok(Bytes::from(format!("{DECLARATION}{value}")))
}

/// Serializes a type to bytes with a specified root tag.
/// Automaticallyl includes the XML declaration.
pub fn to_xml_with_root<T>(root_tag: &str, value: &T) -> Result<Bytes>
where
    T: serde::Serialize,
{
    let value =
        to_string_with_root(root_tag, value).with_context(ErrorKind::DataConversion, || {
            let t = core::any::type_name::<T>();
            format!("failed to serialize {t} into xml")
        })?;
    Ok(Bytes::from(format!("{DECLARATION}{value}")))
}

/// Returns bytes without the UTF-8 BOM.
fn slice_bom(bytes: &[u8]) -> &[u8] {
    if bytes.len() > 3 && bytes[0..3] == UTF8_BOM {
        &bytes[3..]
    } else {
        bytes
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_slice_bom() {
        let bytes = &[0xEF, 0xBB, 0xBF, 7];
        assert_eq!(&[7], slice_bom(bytes));

        let bytes = &[8];
        assert_eq!(&[8], slice_bom(bytes));
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    #[serde(rename = "Foo")]
    struct Test {
        x: String,
    }

    #[test]
    fn reading_xml() -> Result<()> {
        let test = Test {
            x: "Hello, world!".into(),
        };
        let xml = br#"<?xml version="1.0" encoding="utf-8"?><Foo><x>Hello, world!</x></Foo>"#;
        assert_eq!(test, read_xml(xml)?);

        let error = read_xml::<Test>(&xml[..xml.len() - 2]).unwrap_err();
        assert!(format!("{error}").contains("typespec_client_core::xml::test::Test"));

        let xml = r#"<?xml version="1.0" encoding="utf-8"?><Foo><x>Hello, world!</x></Foo>"#;
        assert_eq!(test, read_xml_str(xml)?);

        let error = read_xml_str::<Test>(&xml[..xml.len() - 2]).unwrap_err();
        assert!(format!("{error}").contains("typespec_client_core::xml::test::Test"));
        Ok(())
    }

    #[test]
    fn writing_xml() -> Result<()> {
        assert_eq!(
            br#"<?xml version="1.0" encoding="utf-8"?>\n<Foo><x>Hello, world!</x></Foo>"#,
            to_xml(&Test {
                x: "Hello, world!".to_string()
            })?
            .to_vec()
            .as_slice()
        );

        assert_eq!(
            br#"<?xml version="1.0" encoding="utf-8"?>\n<Bob><x>Hello, world!</x></Bob>"#,
            to_xml_with_root(
                "Bob",
                &Test {
                    x: "Hello, world!".to_string()
                }
            )?
            .to_vec()
            .as_slice()
        );
        Ok(())
    }
}
