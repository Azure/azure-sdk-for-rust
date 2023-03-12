use serde_amqp::{SerializeComposite, DeserializeComposite};

#[derive(Debug, PartialEq, Eq, Hash, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "apache.org:selector-filter:string",
    code = "0x0000_0013_7000_000A",
    encoding = "basic",
)]
pub struct ConsumerFilter(pub String);
