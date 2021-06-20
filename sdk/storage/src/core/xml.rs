use crate::Error;
use azure_core::util::slice_bom;
use bytes::Bytes;

/// Reads the XML from the Bytes.
pub fn read_xml<'de, T: serde::de::Deserialize<'de>>(body: &Bytes) -> Result<T, Error> {
    serde_xml_rs::from_reader(slice_bom(body).as_ref()).map_err(Error::XmlError)
}
