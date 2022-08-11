use crate::blob::{BlobBlockType, BlobBlockWithSize};
use azure_core::error::{ErrorKind, ResultExt};

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
    pub committed_blocks: Option<OuterBlock>,
    #[serde(rename = "UncommittedBlocks")]
    pub uncommitted_blocks: Option<OuterBlock>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct BlockWithSizeList {
    pub blocks: Vec<BlobBlockWithSize>,
}

impl BlockWithSizeList {
    pub fn try_from_xml(xml: &str) -> azure_core::Result<Self> {
        let bl: BlockList =
            serde_xml_rs::de::from_reader(xml.as_bytes()).map_kind(ErrorKind::DataConversion)?;

        let mut lbs = BlockWithSizeList { blocks: Vec::new() };

        if let Some(b) = bl.committed_blocks.and_then(|c| c.block) {
            for b_val in b {
                lbs.blocks.push(BlobBlockWithSize {
                    block_list_type: BlobBlockType::Committed(
                        base64::decode(&b_val.name.value)
                            .map_kind(ErrorKind::DataConversion)?
                            .into(),
                    ),
                    size_in_bytes: b_val.size.value,
                });
            }
        }

        if let Some(b) = bl.uncommitted_blocks.and_then(|c| c.block) {
            for b_val in b {
                lbs.blocks.push(BlobBlockWithSize {
                    block_list_type: BlobBlockType::Uncommitted(
                        base64::decode(&b_val.name.value)
                            .map_kind(ErrorKind::DataConversion)?
                            .into(),
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

        let bl = BlockWithSizeList::try_from_xml(range).unwrap();
        assert_eq!(bl.blocks.len(), 2);
        assert_eq!(bl.blocks[0].size_in_bytes, 200);
        assert_eq!(bl.blocks[1].size_in_bytes, 4096);

        assert_eq!(
            bl.blocks[0].block_list_type,
            BlobBlockType::new_committed("base64-encoded-block-id")
        );
        let b2 = BlobBlockType::new_uncommitted("base64-encoded-block-id-number2");
        assert_eq!(
            bl.blocks[1].block_list_type, b2,
            "bl.blocks[1].block_list_type == {:?}, b2 == {:?}",
            bl.blocks[1].block_list_type, b2
        );
    }

    #[test]
    fn try_parse2() {
        let range = "<?xml version=\"1.0\" encoding=\"utf-8\"?><BlockList><CommittedBlocks /><UncommittedBlocks><Block><Name>YmxvY2sx</Name><Size>62</Size></Block><Block><Name>YmxvY2sy</Name><Size>62</Size></Block><Block><Name>YmxvY2sz</Name><Size>62</Size></Block></UncommittedBlocks></BlockList>";

        let bl = BlockWithSizeList::try_from_xml(range).unwrap();
        assert_eq!(bl.blocks.len(), 3);
        assert_eq!(bl.blocks[0].size_in_bytes, 62);
        assert_eq!(bl.blocks[1].size_in_bytes, 62);
        assert_eq!(bl.blocks[2].size_in_bytes, 62);
    }

    /// Tests that we can explicitly deserialize the response even if not all
    /// block types are present in the response
    #[test]
    fn test_incomplete_response() {
        let range = "<?xml version=\"1.0\" encoding=\"utf-8\"?>
        <BlockList>
          <CommittedBlocks>
               <Block>
                   <Name>YmFzZTY0LWVuY29kZWQtYmxvY2staWQ=</Name>
                   <Size>200</Size>
                </Block>
           </CommittedBlocks>
        </BlockList>  ";

        let bl = BlockWithSizeList::try_from_xml(range).unwrap();
        assert_eq!(bl.blocks.len(), 1);
        assert_eq!(bl.blocks[0].size_in_bytes, 200);

        let range = "<?xml version=\"1.0\" encoding=\"utf-8\"?>
        <BlockList>
          <UncommittedBlocks>
                <Block>
                    <Name>YmFzZTY0LWVuY29kZWQtYmxvY2staWQtbnVtYmVyMg==</Name>
                    <Size>4096</Size>
                </Block>
          </UncommittedBlocks>
        </BlockList>  ";

        let bl = BlockWithSizeList::try_from_xml(range).unwrap();
        assert_eq!(bl.blocks.len(), 1);
        assert_eq!(bl.blocks[0].size_in_bytes, 4096);

        let range = "<?xml version=\"1.0\" encoding=\"utf-8\"?>
        <BlockList>
          <UncommittedBlocks>
                <Block>
                    <Name>YmFzZTY0LWVuY29kZWQtYmxvY2staWQtbnVtYmVyMg==</Name>
                    <Size>4096</Size>
                </Block>
                <Block>
                    <Name>YmFzZTY0LWVuY29kZWQtYmxvY2sfaWQtbnVtYmVyMg==</Name>
                    <Size>200</Size>
                </Block>
          </UncommittedBlocks>
        </BlockList>  ";

        let bl = BlockWithSizeList::try_from_xml(range).unwrap();
        assert_eq!(bl.blocks.len(), 2);
        assert_eq!(bl.blocks[0].size_in_bytes, 4096);
        assert_eq!(bl.blocks[1].size_in_bytes, 200);

        let range = "<?xml version=\"1.0\" encoding=\"utf-8\"?>
        <BlockList>
        </BlockList>  ";
        let bl = BlockWithSizeList::try_from_xml(range).unwrap();
        assert!(bl.blocks.is_empty());
    }
}
