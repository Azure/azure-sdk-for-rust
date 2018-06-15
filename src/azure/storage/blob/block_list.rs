use azure::storage::blob::{BlobBlockType, BlockWithSizeList};
use std::borrow::Borrow;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BlockList<T>
where
    T: Borrow<str>,
{
    pub blocks: Vec<BlobBlockType<T>>,
}

impl<'a> BlockList<&'a str> {
    pub fn to_owned(&self) -> BlockList<String> {
        let mut bl: BlockList<String> = BlockList {
            blocks: Vec::with_capacity(self.blocks.len()),
        };

        for entry in &self.blocks {
            bl.blocks.push(match entry {
                BlobBlockType::Committed(id) => BlobBlockType::Committed(id.to_string()),
                BlobBlockType::Uncommitted(id) => BlobBlockType::Uncommitted(id.to_string()),
                BlobBlockType::Latest(id) => BlobBlockType::Latest(id.to_string()),
            });
        }

        bl
    }
}

impl<T> From<BlockWithSizeList<T>> for BlockList<T>
where
    T: Borrow<str> + Default,
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
    T: Borrow<str>,
{
    pub fn to_xml(&self) -> String {
        let mut s = String::new();
        s.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<BlockList>\n");
        for bl in &self.blocks {
            let node = match bl {
                BlobBlockType::Committed(content) => format!("\t<Committed>{}</Committed>\n", content.borrow()),
                BlobBlockType::Uncommitted(content) => format!("\t<Uncommitted>{}</Uncommitted>\n", content.borrow()),
                BlobBlockType::Latest(content) => format!("\t<Latest>{}</Latest>\n", content.borrow()),
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
        blocks.blocks.push(BlobBlockType::Committed("numero1"));
        blocks.blocks.push(BlobBlockType::Uncommitted("numero2"));
        blocks.blocks.push(BlobBlockType::Uncommitted("numero3"));
        blocks.blocks.push(BlobBlockType::Latest("numero4"));

        let _retu: &str = &blocks.to_xml();

        // to assert with handcrafted XML
    }

}
