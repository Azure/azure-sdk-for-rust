use azure_core::{
    error::{ErrorKind, ResultExt},
    prelude::Range,
};

#[derive(Debug, Deserialize)]
struct Start {
    #[serde(rename = "$value")]
    pub value: u64,
}

#[derive(Debug, Deserialize)]
struct End {
    #[serde(rename = "$value")]
    pub value: u64,
}

#[derive(Debug, Deserialize)]
struct PageRange {
    #[serde(rename = "Start")]
    pub start: Start,
    #[serde(rename = "End")]
    pub end: End,
}

#[derive(Debug, Deserialize)]
struct PageList {
    #[serde(rename = "PageRange")]
    pub page_list: Option<Vec<PageRange>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct PageRangeList {
    pub ranges: Vec<Range>,
}

impl PageRangeList {
    pub fn try_from_xml(xml: &str) -> azure_core::Result<Self> {
        let pl: PageList =
            serde_xml_rs::de::from_reader(xml.as_bytes()).map_kind(ErrorKind::DataConversion)?;

        let mut prl = PageRangeList { ranges: Vec::new() };

        if let Some(range_list) = pl.page_list {
            for range in range_list {
                prl.ranges
                    .push(Range::new(range.start.value, range.end.value));
            }
        }

        Ok(prl)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_parse() {
        let page_list = "<?xml version=\"1.0\" encoding=\"utf-8\"?>
            <PageList>
              <PageRange>
                <Start>0</Start>
                <End>511</End>
              </PageRange>
              <PageRange>
                <Start>1024</Start>
                <End>1535</End>
              </PageRange>
            </PageList>  ";

        let prl = PageRangeList::try_from_xml(page_list).unwrap();
        assert!(prl.ranges.len() == 2);
        assert!(prl.ranges[0].start == 0);
        assert!(prl.ranges[0].end == 511);
        assert!(prl.ranges[1].start == 1024);
        assert!(prl.ranges[1].end == 1535);
    }
}
