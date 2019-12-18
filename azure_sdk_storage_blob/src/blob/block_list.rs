use crate::blob::{BlobBlockType, BlockWithSizeList};
use base64;
use std::borrow::Borrow;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BlockList<T>
where
    T: Borrow<[u8]>,
{
    pub blocks: Vec<BlobBlockType<T>>,
}

impl<'a> BlockList<&'a [u8]> {
    pub fn to_owned(&self) -> BlockList<Vec<u8>> {
        let mut bl: BlockList<Vec<u8>> = BlockList {
            blocks: Vec::with_capacity(self.blocks.len()),
        };

        for entry in &self.blocks {
            bl.blocks.push(match entry {
                BlobBlockType::Committed(id) => BlobBlockType::Committed(id.to_vec()),
                BlobBlockType::Uncommitted(id) => BlobBlockType::Uncommitted(id.to_vec()),
                BlobBlockType::Latest(id) => BlobBlockType::Latest(id.to_vec()),
            });
        }

        bl
    }
}

impl<T> From<BlockWithSizeList<T>> for BlockList<T>
where
    T: Borrow<[u8]> + Default,
{
    fn from(b: BlockWithSizeList<T>) -> BlockList<T> {
        let mut bl = BlockList::default();
        for block in b.blocks {
            bl.blocks.push(block.block_list_type);
        }
        bl
    }
}

impl<T> BlockList<T>
where
    T: Borrow<[u8]>,
{
    pub fn to_xml(&self) -> String {
        let mut s = String::new();
        s.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<BlockList>\n");
        for bl in &self.blocks {
            let node = match bl {
                BlobBlockType::Committed(content) => format!(
                    "\t<Committed>{}</Committed>\n",
                    base64::encode(content.borrow())
                ),
                BlobBlockType::Uncommitted(content) => format!(
                    "\t<Uncommitted>{}</Uncommitted>\n",
                    base64::encode(content.borrow())
                ),
                BlobBlockType::Latest(content) => {
                    format!("\t<Latest>{}</Latest>\n", base64::encode(content.borrow()))
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

    #[test]
    fn to_xml() {
        let mut blocks = BlockList { blocks: Vec::new() };
        blocks
            .blocks
            .push(BlobBlockType::Committed(Vec::from(b"numero1" as &[u8])));
        blocks
            .blocks
            .push(BlobBlockType::Uncommitted(Vec::from(b"numero2" as &[u8])));
        blocks
            .blocks
            .push(BlobBlockType::Uncommitted(Vec::from(b"numero3" as &[u8])));
        blocks
            .blocks
            .push(BlobBlockType::Latest(Vec::from(b"numero4" as &[u8])));

        let _retu: &str = &blocks.to_xml();

        // to assert with handcrafted XML
    }
}
