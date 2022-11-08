use serde_amqp::{DeserializeComposite, SerializeComposite};

// <type name="com.microsoft:session-filter" class="restricted" source="string" provides="filter">
//     <descriptor name="com.microsoft:session-filter" code="0x00000137:000000C"/>
// </type>
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeComposite, DeserializeComposite,
)]
#[amqp_contract(
    name = "com.microsoft:session-filter",
    code = "0x0000_0137:0x0000_000c",
    encoding = "basic"
)]
pub struct SessionFilter(pub String);
