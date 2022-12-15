use serde_amqp::{
    described::Described, descriptor::Descriptor, DeserializeComposite, SerializeComposite, Value,
};

// <type name="com.microsoft:session-filter" class="restricted" source="string" provides="filter">
//     <descriptor name="com.microsoft:session-filter" code="0x00000137:000000C"/>
// </type>
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeComposite, DeserializeComposite,
)]
#[amqp_contract(
    name = "com.microsoft:session-filter",
    code = "0x0000_0013_7000_000c",
    encoding = "basic"
)]
pub struct SessionFilter(pub String);

impl From<SessionFilter> for Described<String> {
    fn from(filter: SessionFilter) -> Self {
        Self {
            descriptor: Descriptor::Code(0x0000_0013_7000_000c), // FIXME: descriptor code doesn't work yet
            // descriptor: Descriptor::Name(Symbol::from("com.microsoft:session-filter")),
            value: filter.0,
        }
    }
}

impl From<SessionFilter> for Described<Value> {
    fn from(filter: SessionFilter) -> Self {
        let described: Described<String> = filter.into();
        Self {
            descriptor: described.descriptor,
            value: described.value.into(),
        }
    }
}

impl From<SessionFilter> for Option<Described<Value>> {
    fn from(filter: SessionFilter) -> Self {
        Some(filter.into())
    }
}
