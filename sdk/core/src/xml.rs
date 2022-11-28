use crate::error::{ErrorKind, ResultExt};

/// The UTF8 [byte order marker](https://en.wikipedia.org/wiki/Byte_order_mark)
const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

/// Reads the XML from bytes.
pub fn read_xml<'de, T: serde::de::Deserialize<'de>>(body: &[u8]) -> crate::Result<T> {
    serde_xml_rs::from_reader(slice_bom(body)).with_context(ErrorKind::DataConversion, || {
        let t = core::any::type_name::<T>();
        let xml = std::str::from_utf8(body).unwrap_or("<XML IS NOT UTF-8>");
        format!("failed to deserialize the following xml into a {t}\n{xml}")
    })
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
    fn reading_xml() {
        #[derive(Deserialize, PartialEq, Debug)]
        #[serde(rename = "Foo")]
        struct Test {
            x: String,
        }
        let test = Test {
            x: "Hello, world!".into(),
        };
        let xml = br#"<?xml version="1.0" encoding="utf-8"?><Foo><x>Hello, world!</x></Foo>"#;
        assert_eq!(test, read_xml(xml).unwrap());

        let error = read_xml::<Test>(&xml[..xml.len() - 2]).unwrap_err();
        assert!(format!("{error}").contains("reading_xml::Test"));
    }
}
