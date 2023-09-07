use azure_core::{prelude::Range, xml::read_xml_str};

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
    #[serde(rename = "PageRange", default)]
    pub page_list: Vec<PageRange>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct PageRangeList {
    pub ranges: Vec<Range>,
}

impl PageRangeList {
    pub fn try_from_xml(xml: &str) -> azure_core::Result<Self> {
        let pl: PageList = read_xml_str(xml)?;

        let mut prl = PageRangeList { ranges: Vec::new() };

        for range in pl.page_list {
            prl.ranges
                .push(Range::new(range.start.value, range.end.value));
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

        let page_list = "<?xml version=\"1.0\" encoding=\"utf-8\"?><PageList></PageList>";
        let prl = PageRangeList::try_from_xml(page_list).unwrap();
        assert!(prl.ranges.is_empty());

        let page_list = "<?xml version=\"1.0\" encoding=\"utf-8\"?><PageList />";
        let prl = PageRangeList::try_from_xml(page_list).unwrap();
        assert!(prl.ranges.is_empty());
    }
}
