use crate::error::{ErrorKind, ResultExt};
use bytes::Bytes;
pub use quick_xml::serde_helpers::text_content;
use quick_xml::{
    de::{from_reader, from_str},
    se::{to_string, to_string_with_root},
};
use serde::de::DeserializeOwned;

/// The UTF8 [byte order marker](https://en.wikipedia.org/wiki/Byte_order_mark)
const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

/// Reads the XML from bytes.
pub fn read_xml_str<T>(body: &str) -> crate::Result<T>
where
    T: DeserializeOwned,
{
    from_str(body).with_context(ErrorKind::DataConversion, || {
        let t = core::any::type_name::<T>();
        format!("failed to deserialize the following xml into a {t}\n{body}")
    })
}

/// Reads the XML from bytes.
pub fn read_xml<T>(body: &[u8]) -> crate::Result<T>
where
    T: DeserializeOwned,
{
    from_reader(slice_bom(body)).with_context(ErrorKind::DataConversion, || {
        let t = core::any::type_name::<T>();
        let xml = std::str::from_utf8(body).unwrap_or("<XML IS NOT UTF-8>");
        format!("failed to deserialize the following xml into a {t}\n{xml}")
    })
}

pub fn to_xml<T>(value: &T) -> crate::Result<Bytes>
where
    T: serde::Serialize,
{
    let value = to_string(value).with_context(ErrorKind::DataConversion, || {
        let t = core::any::type_name::<T>();
        format!("failed to serialize {t} into xml")
    })?;
    Ok(Bytes::from(value))
}

pub fn to_xml_with_root<T>(root_tag: &str, value: &T) -> crate::Result<Bytes>
where
    T: serde::Serialize,
{
    let value =
        to_string_with_root(root_tag, value).with_context(ErrorKind::DataConversion, || {
            let t = core::any::type_name::<T>();
            format!("failed to serialize {t} into xml")
        })?;
    Ok(Bytes::from(value))
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
    use serde::Deserialize;

    #[test]
    fn test_slice_bom() {
        let bytes = &[0xEF, 0xBB, 0xBF, 7];
        assert_eq!(&[7], slice_bom(bytes));

        let bytes = &[8];
        assert_eq!(&[8], slice_bom(bytes));
    }

    #[test]
    fn reading_xml() -> crate::Result<()> {
        #[derive(Deserialize, PartialEq, Debug)]
        #[serde(rename = "Foo")]
        struct Test {
            x: String,
        }
        let test = Test {
            x: "Hello, world!".into(),
        };
        let xml = br#"<?xml version="1.0" encoding="utf-8"?><Foo><x>Hello, world!</x></Foo>"#;
        assert_eq!(test, read_xml(xml)?);

        let error = read_xml::<Test>(&xml[..xml.len() - 2]).unwrap_err();
        assert!(format!("{error}").contains("reading_xml::Test"));

        let xml = r#"<?xml version="1.0" encoding="utf-8"?><Foo><x>Hello, world!</x></Foo>"#;
        assert_eq!(test, read_xml_str(xml)?);

        let error = read_xml_str::<Test>(&xml[..xml.len() - 2]).unwrap_err();
        assert!(format!("{error}").contains("reading_xml::Test"));
        Ok(())
    }
}
