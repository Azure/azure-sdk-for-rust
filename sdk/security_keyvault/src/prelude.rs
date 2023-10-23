pub use crate::clients::*;
pub use crate::{account::*, certificates::*, keys::*, secrets::*};
use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy)]
pub enum JsonWebKeyType {
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EC-HSM")]
    EcHsm,
    #[serde(rename = "RSA")]
    Rsa,
    #[serde(rename = "RSA-HSM")]
    RsaHsm,
    #[serde(rename = "oct")]
    Oct,
    #[serde(rename = "oct-HSM")]
    OctHsm,
}
