use fe2o3_amqp_management::response::Response;
use serde_amqp::{DeserializeComposite, SerializeComposite};

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "com.microsoft:enumerate-rules:list",
    code = "0x0000_0137:0x0000_0004",
    encoding = "list",
    rename_all = "kebab-case"
)]
pub struct RuleDescription {}

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "com.microsoft:empty-rule-action:list",
    code = "0x0000_0137:0x0000_0005",
    encoding = "list",
    rename_all = "kebab-case"
)]
pub struct EmptyRuleAction {}

#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "com.microsoft:sql-rule-action:list",
    code = "0x0000_0137:0x0000_0006",
    encoding = "list",
    rename_all = "kebab-case"
)]
pub struct SqlRuleAction {
    pub expression: String,
}

pub(crate) struct EnumerateRulesResponse {}

impl Response for EnumerateRulesResponse {
    const STATUS_CODE: u16 = 200;

    type Body = ();

    type Error = super::ManagementError;

    fn decode_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}
