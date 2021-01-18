use crate::blob::blob::{BlobBlockType, BlockWithSizeList};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BlockList {
    pub blocks: Vec<BlobBlockType>,
}

impl BlockList {
    pub fn to_owned(&self) -> Self {
        let mut bl: BlockList = BlockList {
            blocks: Vec::with_capacity(self.blocks.len()),
        };

        for entry in &self.blocks {
            bl.blocks.push(match entry {
                BlobBlockType::Committed(id) => BlobBlockType::Committed(id.clone()),
                BlobBlockType::Uncommitted(id) => BlobBlockType::Uncommitted(id.clone()),
                BlobBlockType::Latest(id) => BlobBlockType::Latest(id.clone()),
            });
        }

        bl
    }
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
                BlobBlockType::Committed(content) => format!(
                    "\t<Committed>{}</Committed>\n",
                    base64::encode(content.as_ref())
                ),
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
            .push(BlobBlockType::Committed(Bytes::from_static(b"numero1")));
        blocks
            .blocks
            .push(BlobBlockType::Uncommitted(Bytes::from_static(
                b"numero2" as &[u8],
            )));
        blocks
            .blocks
            .push(BlobBlockType::Uncommitted(Bytes::from_static(
                b"numero3" as &[u8],
            )));
        blocks.blocks.push(BlobBlockType::Latest(Bytes::from_static(
            b"numero4" as &[u8],
        )));

        let _retu: &str = &blocks.to_xml();

        // to assert with handcrafted XML
    }
}
