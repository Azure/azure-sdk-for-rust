pub const APPLICATION_JSON: &str = "application/json";
pub const APPLICATION_XML: &str = "application/xml";
pub const APPLICATION_OCTET_STREAM: &str = "application/octet-stream";

/// Pick `application/json` if it is an option, else the first one in the list
pub fn pick_consumes(consumes: Vec<&str>) -> Option<&str> {
    if consumes.contains(&APPLICATION_JSON) {
        return Some(APPLICATION_JSON);
    }
    consumes.into_iter().next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn test_consumes_xml() -> Result<()> {
        let consumes = vec![APPLICATION_XML];
        assert_eq!(Some(APPLICATION_XML), pick_consumes(consumes));
        Ok(())
    }

    #[test]
    fn test_consumes_none() -> Result<()> {
        let consumes = vec![];
        assert_eq!(None, pick_consumes(consumes));
        Ok(())
    }

    #[test]
    fn test_consumes_json() -> Result<()> {
        let consumes = vec![APPLICATION_XML, APPLICATION_JSON];
        assert_eq!(Some(APPLICATION_JSON), pick_consumes(consumes));
        Ok(())
    }
}
