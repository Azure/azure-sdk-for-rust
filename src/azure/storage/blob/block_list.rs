use azure::core::errors::BlockListParseError;
use azure::storage::blob::BlobBlockType;
use std::borrow::Borrow;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockList<T>
where
    T: Borrow<str>,
{
    pub blocks: Vec<BlobBlockType<T>>,
}

impl<T> BlockList<T>
where
    T: Borrow<str>,
{
    pub fn new() -> BlockList<T> {
        BlockList { blocks: Vec::new() }
    }
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

        let mut bl = BlockList { blocks: Vec::new() };

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

            bl.blocks.push(match node_type {
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
            blocks: Vec::with_capacity(self.blocks.len()),
        };

        for entry in self.blocks.iter() {
            bl.blocks.push(match entry {
                BlobBlockType::Committed(id) => BlobBlockType::Committed(id.to_string()),
                BlobBlockType::Uncommitted(id) => BlobBlockType::Uncommitted(id.to_string()),
                BlobBlockType::Latest(id) => BlobBlockType::Latest(id.to_string()),
            });
        }

        bl
    }
}

impl<T> BlockList<T>
where
    T: Borrow<str>,
{
    pub fn to_xml(&self) -> String {
        let mut s = String::new();
        s.extend("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<BlockList>\n".chars());
        for bl in self.blocks.iter() {
            let node = match bl {
                BlobBlockType::Committed(content) => {
                    format!("\t<Committed>{}</Committed>\n", content.borrow())
                }
                BlobBlockType::Uncommitted(content) => {
                    format!("\t<Uncommitted>{}</Uncommitted>\n", content.borrow())
                }
                BlobBlockType::Latest(content) => {
                    format!("\t<Latest>{}</Latest>\n", content.borrow())
                }
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
        assert!(bl.blocks.len() == 4);
        assert!(bl.blocks[0] == BlobBlockType::Committed("numero1"));
        assert!(bl.blocks[1] == BlobBlockType::Uncommitted("numero2"));
        assert!(bl.blocks[2] == BlobBlockType::Uncommitted("numero3"));
        assert!(bl.blocks[3] == BlobBlockType::Latest("numero4"));
    }

    #[test]
    fn to_xml_and_then_parse() {
        let mut blocks = BlockList { blocks: Vec::new() };
        blocks.blocks.push(BlobBlockType::Committed("numero1"));
        blocks.blocks.push(BlobBlockType::Uncommitted("numero2"));
        blocks.blocks.push(BlobBlockType::Uncommitted("numero3"));
        blocks.blocks.push(BlobBlockType::Latest("numero4"));

        let retu: &str = &blocks.to_xml();

        let bl2 = BlockList::try_from(retu).unwrap();
        assert!(bl2.blocks.len() == 4);
        assert!(blocks == bl2);

        let bl_owned = bl2.to_owned();
        assert!(bl_owned.blocks[0] == BlobBlockType::Committed(String::from("numero1")));
        assert!(bl_owned.blocks[1] == BlobBlockType::Uncommitted(String::from("numero2")));
        assert!(bl_owned.blocks[2] == BlobBlockType::Uncommitted(String::from("numero3")));
        assert!(bl_owned.blocks[3] == BlobBlockType::Latest(String::from("numero4")));
    }

}
