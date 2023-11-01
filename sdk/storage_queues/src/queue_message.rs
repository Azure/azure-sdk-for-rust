use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename = "QueueMessage")]
pub(crate) struct QueueMessageSubmit {
    #[serde(rename = "MessageText")]
    pub(crate) message_text: String,
}

#[cfg(test)]
mod test {
    use super::QueueMessageSubmit;
    use azure_core::xml::to_xml;

    #[test]
    fn test_serialize() -> azure_core::Result<()> {
        let serialized = to_xml(&QueueMessageSubmit {
            message_text: "<hello> \" there &".to_string(),
        })?;
        let expected = "<QueueMessage><MessageText>&lt;hello&gt; &quot; there &amp;</MessageText></QueueMessage>";
        assert_eq!(expected, serialized);
        Ok(())
    }
}
