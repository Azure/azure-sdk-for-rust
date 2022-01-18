use crate::headers::*;
use crate::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SequenceNumberCondition {
    Less(u64),
    LessOrEqual(u64),
    Equal(u64),
}

impl AddAsHeader for SequenceNumberCondition {
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            SequenceNumberCondition::Equal(val) => {
                builder.header(IF_SEQUENCE_NUMBER_EQ, &val.to_string() as &str)
            }
            SequenceNumberCondition::LessOrEqual(val) => {
                builder.header(IF_SEQUENCE_NUMBER_LE, &val.to_string() as &str)
            }
            SequenceNumberCondition::Less(val) => {
                builder.header(IF_SEQUENCE_NUMBER_LT, &val.to_string() as &str)
            }
        }
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HttpHeaderError> {
        let (header_name, val) = match self {
            SequenceNumberCondition::Equal(val) => (IF_SEQUENCE_NUMBER_EQ, val),
            SequenceNumberCondition::LessOrEqual(val) => (IF_SEQUENCE_NUMBER_LE, val),
            SequenceNumberCondition::Less(val) => (IF_SEQUENCE_NUMBER_LT, val),
        };

        request
            .headers_mut()
            .append(header_name, http::HeaderValue::from_str(&val.to_string())?);

        Ok(())
    }
}
