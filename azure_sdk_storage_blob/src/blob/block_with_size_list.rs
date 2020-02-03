use crate::blob::BlobBlockType;
use crate::blob::BlobBlockWithSize;
use azure_sdk_core::errors::AzureError;
use base64;
use std::borrow::Borrow;

#[derive(Debug, Deserialize)]
struct Name {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
struct Size {
    #[serde(rename = "$value")]
    pub value: u64,
}

#[derive(Debug, Deserialize)]
struct InnerBlock {
    #[serde(rename = "Name")]
    pub name: Name,
    #[serde(rename = "Size")]
    pub size: Size,
}

#[derive(Debug, Deserialize)]
struct OuterBlock {
    #[serde(rename = "Block")]
    pub block: Option<Vec<InnerBlock>>,
}

#[derive(Debug, Deserialize)]
struct BlockList {
    #[serde(rename = "CommittedBlocks")]
    pub committed_blocks: OuterBlock,
    #[serde(rename = "UncommittedBlocks")]
    pub uncommitted_blocks: OuterBlock,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BlockWithSizeList<T>
where
    T: Borrow<[u8]>,
{
    pub blocks: Vec<BlobBlockWithSize<T>>,
}

impl BlockWithSizeList<Vec<u8>> {
    pub fn try_from(xml: &str) -> Result<BlockWithSizeList<Vec<u8>>, AzureError> {
        let bl: BlockList = serde_xml_rs::de::from_reader(xml.as_bytes())?;
        debug!("bl == {:?}", bl);

        let mut lbs = BlockWithSizeList { blocks: Vec::new() };

        if let Some(b) = bl.committed_blocks.block {
            for b_val in b {
                lbs.blocks.push(BlobBlockWithSize {
                    block_list_type: BlobBlockType::Committed(
                        base64::decode(&b_val.name.value)?.to_owned(),
                    ),
                    size_in_bytes: b_val.size.value,
                });
            }
        }

        if let Some(b) = bl.uncommitted_blocks.block {
            for b_val in b {
                lbs.blocks.push(BlobBlockWithSize {
                    block_list_type: BlobBlockType::Uncommitted(
                        base64::decode(&b_val.name.value)?.to_owned(),
                    ),
                    size_in_bytes: b_val.size.value,
                });
            }
        }

        Ok(lbs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_parse() {
        let range = "<?xml version=\"1.0\" encoding=\"utf-8\"?>  
            <BlockList>  
              <CommittedBlocks>  
                   <Block>  
                       <Name>YmFzZTY0LWVuY29kZWQtYmxvY2staWQ=</Name>  
                       <Size>200</Size>  
                    </Block>  
               </CommittedBlocks>  
               <UncommittedBlocks>  
                    <Block>  
                        <Name>YmFzZTY0LWVuY29kZWQtYmxvY2staWQtbnVtYmVyMg==</Name>  
                        <Size>4096</Size>  
                    </Block>  
               </UncommittedBlocks>  
            </BlockList>  ";

        let bl = BlockWithSizeList::try_from(range).unwrap();
        assert!(bl.blocks.len() == 2);
        assert!(bl.blocks[0].size_in_bytes == 200);
        assert!(bl.blocks[1].size_in_bytes == 4096);

        assert!(
            bl.blocks[0].block_list_type
                == BlobBlockType::Committed(Vec::from(b"base64-encoded-block-id" as &[u8]))
        );
        let b2 = BlobBlockType::Uncommitted(Vec::from(b"base64-encoded-block-id-number2" as &[u8]));
        assert!(
            bl.blocks[1].block_list_type == b2,
            "bl.blocks[1].block_list_type == {:?}, b2 == {:?}",
            bl.blocks[1].block_list_type,
            b2
        );
    }

    #[test]
    fn try_parse2() {
        let range = "<?xml version=\"1.0\" encoding=\"utf-8\"?><BlockList><CommittedBlocks /><UncommittedBlocks><Block><Name>YmxvY2sx</Name><Size>62</Size></Block><Block><Name>YmxvY2sy</Name><Size>62</Size></Block><Block><Name>YmxvY2sz</Name><Size>62</Size></Block></UncommittedBlocks></BlockList>";

        let bl = BlockWithSizeList::try_from(range).unwrap();
        assert!(bl.blocks.len() == 3);
        assert!(bl.blocks[0].size_in_bytes == 62);
        assert!(bl.blocks[1].size_in_bytes == 62);
        assert!(bl.blocks[2].size_in_bytes == 62);
    }
}
