use azure_core::error::{Error, ErrorKind, ResultExt};

const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

/// Reads the XML from the Bytes.
pub fn read_xml<'de, T: serde::de::Deserialize<'de>>(body: &[u8]) -> Result<T, Error> {
    serde_xml_rs::from_reader(slice_bom(body).as_ref())
        .context(ErrorKind::DataConversion, "failed to deserialize xml")
}

/// Returns Bytes without the UTF-8 BOM.
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

    #[test]
    fn test_slice_bom() {
        let bytes = &[0xEF, 0xBB, 0xBF, 7];
        assert_eq!(&[7], slice_bom(bytes));

        let bytes = &[8];
        assert_eq!(&[8], slice_bom(bytes));
    }
}
