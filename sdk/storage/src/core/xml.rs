use crate::Error;
use bytes::Bytes;

/// Reads the XML from the Bytes.
pub fn read_xml<'de, T: serde::de::Deserialize<'de>>(body: &Bytes) -> Result<T, Error> {
    serde_xml_rs::from_reader(slice_bom(body).as_ref()).map_err(Error::XmlError)
}

const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

/// Returns Bytes without the UTF-8 BOM.
fn slice_bom(bytes: &Bytes) -> Bytes {
    if bytes.len() > 3 && bytes.slice(0..3).as_ref() == UTF8_BOM {
        bytes.slice(3..)
    } else {
        bytes.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slice_bom() {
        let bytes = Bytes::from_static(&[0xEF, 0xBB, 0xBF, 7]);
        assert_eq!(Bytes::from_static(&[7]), slice_bom(&bytes));

        let bytes = Bytes::from_static(&[8]);
        assert_eq!(Bytes::from_static(&[8]), slice_bom(&bytes));
    }
}
