extern crate crypto;
extern crate hyper;
extern crate url;

#[macro_use]
pub mod errors;
pub mod parsing;
#[macro_use]
pub mod enumerations;
pub mod incompletevector;
pub mod lease;

pub mod range;
pub mod ba512_range;

use url::percent_encoding;
define_encode_set! {
    pub COMPLETE_ENCODE_SET = [percent_encoding::USERINFO_ENCODE_SET] | {
        '+', '-', '&'
    }
}
