use std::convert::TryFrom;
use azure::storage::blob::BlobBlockType;
use azure::core::errors::BlockListParseError;

#[derive(Debug, Clone)]
pub struct BlockList<T> {
    pub bls: Vec<BlobBlockType<T>>,
}

impl<'a> TryFrom<&'a str> for BlockList<&'a str> {
    type Error = BlockListParseError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        BlockList::from_xml(value)
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


   pub fn from_xml(xml: &'a str) -> Result<BlockList<&'a str>, BlockListParseError> {
        // this is terrible XML parsing but will do temporarily.
        // at least we are not copying strings around.
        // we assume here the XML is composed by
        // single byte chars. It should (base64 encoding should
        // comply) but if we get unpleasant errors
        // this can be a place to start looking
        trace!("BlockList::from_xml called with xml == \"{}\"", xml);

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

