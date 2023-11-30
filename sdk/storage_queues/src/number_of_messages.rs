use azure_core::{AppendToUrlQuery, Url};

#[derive(Debug, Clone)]
pub struct NumberOfMessages(u8);

impl NumberOfMessages {
    pub fn new(number_of_messages: impl Into<u8>) -> Self {
        Self(number_of_messages.into())
    }
}

impl AppendToUrlQuery for NumberOfMessages {
    fn append_to_url_query(&self, url: &mut Url) {
        url.query_pairs_mut()
            .append_pair("numofmessages", &self.0.to_string());
    }
}

impl From<u8> for NumberOfMessages {
    fn from(number_of_messages: u8) -> Self {
        Self(number_of_messages)
    }
}
