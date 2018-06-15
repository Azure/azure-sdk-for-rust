extern crate hyper;
extern crate ring;
extern crate url;

#[macro_use]
pub mod errors;
pub mod parsing;
#[macro_use]
pub mod enumerations;
pub mod incompletevector;
pub mod lease;

pub mod ba512_range;
pub mod range;

use url::percent_encoding;
define_encode_set! {
    pub COMPLETE_ENCODE_SET = [percent_encoding::USERINFO_ENCODE_SET] | {
        '+', '-', '&'
    }
}

use uuid::Uuid;
pub type RequestId = Uuid;

pub(crate) mod util;