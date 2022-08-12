use crate::blob::{BlobBlockType, BlockWithSizeList};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct BlockList {
    pub blocks: Vec<BlobBlockType>,
}

impl From<BlockWithSizeList> for BlockList {
    fn from(b: BlockWithSizeList) -> BlockList {
        let mut bl = BlockList::default();
        for block in b.blocks {
            bl.blocks.push(block.block_list_type);
        }
        bl
    }
}

impl BlockList {
    pub fn to_xml(&self) -> String {
        let mut s = String::new();
        s.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<BlockList>\n");
        for bl in &self.blocks {
            let node = match bl {
                BlobBlockType::Committed(content) => {
                    format!(
                        "\t<Committed>{}</Committed>\n",
                        base64::encode(content.as_ref())
                    )
                }
                BlobBlockType::Uncommitted(content) => format!(
                    "\t<Uncommitted>{}</Uncommitted>\n",
                    base64::encode(content.as_ref())
                ),
                BlobBlockType::Latest(content) => {
                    format!("\t<Latest>{}</Latest>\n", base64::encode(content.as_ref()))
                }
            };

            s.push_str(&node);
        }

        s.push_str("</BlockList>");
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn to_xml() {
        let mut blocks = BlockList { blocks: Vec::new() };
        blocks
            .blocks
            .push(BlobBlockType::new_committed(Bytes::from_static(b"numero1")));
        blocks
            .blocks
            .push(BlobBlockType::new_uncommitted("numero2"));
        blocks
            .blocks
            .push(BlobBlockType::new_uncommitted("numero3"));
        blocks.blocks.push(BlobBlockType::new_latest("numero4"));

        let _retu: &str = &blocks.to_xml();

        // to assert with handcrafted XML
    }
}
