use azure::core::errors::BlockListParseError;
use azure::storage::blob::BlobBlockType;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockList<T> {
    pub bls: Vec<BlobBlockType<T>>,
}

impl<'a> TryFrom<&'a str> for BlockList<&'a str> {
    type Error = BlockListParseError;

    fn try_from(xml: &'a str) -> Result<Self, Self::Error> {
        // this is terrible XML parsing but will do temporarily.
        // at least we are not copying strings around.
        // we assume here the XML is composed by
        // single byte chars. It should (base64 encoding should
        // comply) but if we get unpleasant errors
        // this can be a place to start looking
        trace!("BlockList::try_from called with xml == \"{}\"", xml);

        let mut bl = BlockList { bls: Vec::new() };

        let begin = xml[..].find("<BlockList>")? + "<BlockList>".len();
        let end = xml[begin..].find("</BlockList>")? + begin;

        debug!("begin == {}, end == {}", begin, end);

        let mut cur = begin;

        while cur < end {
            debug!("cur == {}", cur);

            let tagbegin = xml[cur..].find("<")? + cur + 1;
            let tagend = xml[cur..].find(">")? + cur;

            debug!("tagbegin == {}, tagend == {}", tagbegin, tagend);
            let node_type = &xml[tagbegin..tagend];

            debug!("{}", node_type);
            if node_type == "/BlockList" {
                break;
            }

            cur = tagend + 1;

            let close_tag = format!("</{}>", node_type);
            let close_pos = xml[cur..].find(&close_tag)? + cur;

            let id = &xml[cur..close_pos];
            debug!("id == {}", id);

            cur = close_pos + close_tag.len() + 1;

            bl.bls.push(match node_type {
                "Committed" => BlobBlockType::Committed(id),
                "Uncommitted" => BlobBlockType::Uncommitted(id),
                "Latest" => BlobBlockType::Latest(id),
                _ => {
                    return Err(BlockListParseError::InvalidBlockType {
                        name: node_type.to_owned(),
                    })
                }
            });
        }

        Ok(bl)
    }
}

impl<'a> BlockList<&'a str> {
    pub fn to_owned(&self) -> BlockList<String> {
        let mut bl: BlockList<String> = BlockList {
            bls: Vec::with_capacity(self.bls.len()),
        };

        for entry in self.bls.iter() {
            bl.bls.push(match entry {
                BlobBlockType::Committed(id) => BlobBlockType::Committed(id.to_string()),
                BlobBlockType::Uncommitted(id) => BlobBlockType::Uncommitted(id.to_string()),
                BlobBlockType::Latest(id) => BlobBlockType::Latest(id.to_string()),
            });
        }

        bl
    }

    pub fn to_xml(&self) -> String {
        let mut s = String::new();
        s.extend("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<BlockList>\n".chars());
        for bl in self.bls.iter() {
            let node = match bl {
                BlobBlockType::Committed(ref content) => {
                    format!("\t<Committed>{}</Committed>\n", content)
                }
                BlobBlockType::Uncommitted(ref content) => {
                    format!("\t<Uncommitted>{}</Uncommitted>\n", content)
                }
                BlobBlockType::Latest(ref content) => format!("\t<Latest>{}</Latest>\n", content),
            };

            s.extend(node.chars());
        }

        s.extend("</BlockList>".chars());
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_parse() {
        let range = "<?xml version=\"1.0\" encoding=\"utf-8\"?>
        <BlockList>
                <Committed>numero1</Committed>
                <Uncommitted>numero2</Uncommitted>
                <Uncommitted>numero3</Uncommitted>
                <Latest>numero4</Latest>
        </BlockList>";

        let bl = BlockList::try_from(range).unwrap();
        assert!(bl.bls.len() == 4);
        assert!(bl.bls[0] == BlobBlockType::Committed("numero1"));
        assert!(bl.bls[1] == BlobBlockType::Uncommitted("numero2"));
        assert!(bl.bls[2] == BlobBlockType::Uncommitted("numero3"));
        assert!(bl.bls[3] == BlobBlockType::Latest("numero4"));
    }

    #[test]
    fn to_xml_and_then_parse() {
        let mut bls = BlockList { bls: Vec::new() };
        bls.bls.push(BlobBlockType::Committed("numero1"));
        bls.bls.push(BlobBlockType::Uncommitted("numero2"));
        bls.bls.push(BlobBlockType::Uncommitted("numero3"));
        bls.bls.push(BlobBlockType::Latest("numero4"));

        let retu: &str = &bls.to_xml();

        let bl2 = BlockList::try_from(retu).unwrap();
        assert!(bl2.bls.len() == 4);
        assert!(bls == bl2);

        let bl_owned = bl2.to_owned();
        assert!(bl_owned.bls[0] == BlobBlockType::Committed(String::from("numero1")));
        assert!(bl_owned.bls[1] == BlobBlockType::Uncommitted(String::from("numero2")));
        assert!(bl_owned.bls[2] == BlobBlockType::Uncommitted(String::from("numero3")));
        assert!(bl_owned.bls[3] == BlobBlockType::Latest(String::from("numero4")));
    }

}
