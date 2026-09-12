# azure_core_amqp

- **Description**: Rust client library for the AMQP protocol
- **Edition**: 2021

## Features

- `default`
  - `fe2o3_amqp`
  - `fe2o3_amqp_rustls`
  - `fe2o3_amqp_ws`
  - `fe2o3_amqp_ws_rustls`
- `fe2o3_amqp`
- `fe2o3_amqp_rustls`
- `fe2o3_amqp_ws`
- `fe2o3_amqp_ws_rustls`

```rust
#![cfg(feature = "fe2o3_amqp")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
pub use azure_core_amqp::error::*;
pub struct AmqpClaimsBasedSecurity {
}
impl AmqpClaimsBasedSecurity {
    pub fn new(session: AmqpSession) -> Result<Self>;
}
impl AmqpClaimsBasedSecurityApis for AmqpClaimsBasedSecurity {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn attach(&self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn authorize_path(&self, path: String, token_type: Option<String>, secret: &Secret, expires_on: OffsetDateTime) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn detach(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
}
#[cfg(feature = "ffi")]
#[derive(Clone, Debug, PartialEq)]
pub struct AmqpComposite {
}
#[cfg(feature = "ffi")]
impl AmqpComposite {
    pub fn descriptor(&self) -> &AmqpDescriptor;
    pub fn new<impl Into<AmqpDescriptor>: Into<AmqpDescriptor>, impl Into<AmqpList>: Into<AmqpList>>(descriptor: impl Into<AmqpDescriptor>, value: impl Into<AmqpList>) -> Self;
    pub fn value(&self) -> &AmqpList;
    pub fn value_mut(&mut self) -> &mut AmqpList;
}
#[derive(Default)]
pub struct AmqpConnection {
}
impl AmqpConnection {
    pub fn new() -> Self;
}
impl AmqpConnectionApis for AmqpConnection {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn close(&self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn close_with_error(&self, condition: AmqpSymbol, description: Option<String>, info: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn open(&self, name: String, url: Url, options: Option<AmqpConnectionOptions>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
}
#[derive(Clone, Debug, Default)]
pub struct AmqpConnectionOptions {
    pub max_frame_size: Option<u32>,
    pub channel_max: Option<u16>,
    pub idle_timeout: Option<azure_core::time::Duration>,
    pub outgoing_locales: Option<Vec<String>>,
    pub incoming_locales: Option<Vec<String>>,
    pub offered_capabilities: Option<Vec<crate::value::AmqpSymbol>>,
    pub desired_capabilities: Option<Vec<crate::value::AmqpSymbol>>,
    pub properties: Option<crate::value::AmqpOrderedMap<crate::value::AmqpSymbol, crate::value::AmqpValue>>,
    pub buffer_size: Option<usize>,
    pub custom_endpoint: Option<azure_core::http::Url>,
    pub transport: Option<AmqpTransport>,
}
#[derive(Debug)]
pub struct AmqpDelivery(/* private fields */);
impl AmqpDeliveryApis for AmqpDelivery {
    fn delivery_id(&self) -> u32;
    fn delivery_tag(&self) -> &Vec<u8>;
    fn into_message(self) -> AmqpMessage;
    fn message(&self) -> &AmqpMessage;
    fn message_format(&self) -> &Option<u32>;
}
impl From<Delivery<Body<Value>>> for crate::messaging::AmqpDelivery {
    fn from(delivery: fe2o3_amqp::link::delivery::Delivery<fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>>) -> Self;
}
#[derive(Clone, Debug, PartialEq)]
pub struct AmqpDescribed {
    pub descriptor: AmqpDescriptor,
    pub value: AmqpValue,
}
impl AmqpDescribed {
    pub fn new<impl Into<AmqpDescriptor>: Into<AmqpDescriptor>, impl Into<AmqpValue>: Into<AmqpValue>>(descriptor: impl Into<AmqpDescriptor>, value: impl Into<AmqpValue>) -> Self;
}
impl From<&AmqpValue> for Box<AmqpDescribed> {
    fn from(v: &AmqpValue) -> Self;
}
impl From<AmqpValue> for Box<AmqpDescribed> {
    fn from(v: AmqpValue) -> Self;
}
impl From<Box<AmqpDescribed>> for AmqpDescribed {
    fn from(b: Box<AmqpDescribed>) -> Self;
}
impl PartialEq<AmqpDescribed> for serde_amqp::described::Described<fe2o3_amqp_types::primitives::Value> {
    fn eq(&self, other: &AmqpDescribed) -> bool;
}
impl PartialEq<AmqpValue> for Box<AmqpDescribed> {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<Described<Value>> for crate::value::AmqpDescribed {
    fn eq(&self, other: &serde_amqp::described::Described<fe2o3_amqp_types::primitives::Value>) -> bool;
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AmqpList(pub Vec<AmqpValue>);
impl AmqpList {
    pub fn is_empty(&self) -> bool;
    pub fn iter(&self) -> impl Iterator<Item = &AmqpValue>;
    pub fn len(&self) -> usize;
    pub fn new() -> Self;
    pub fn push(&mut self, value: AmqpValue);
    pub fn with_capacity(size: usize) -> Self;
}
impl From<&AmqpList> for fe2o3_amqp_types::primitives::Value {
    fn from(value: &AmqpList) -> Self;
}
impl From<&AmqpValue> for AmqpList {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&Value> for crate::value::AmqpList {
    fn from(value: &fe2o3_amqp_types::primitives::Value) -> Self;
}
impl From<AmqpList> for AmqpMessageBody {
    fn from(list: AmqpList) -> Self;
}
#[cfg(feature = "ffi")]
impl From<AmqpList> for AmqpMessageHeader {
    fn from(list: AmqpList) -> Self;
}
#[cfg(feature = "ffi")]
impl From<AmqpList> for AmqpMessageProperties {
    fn from(list: AmqpList) -> Self;
}
impl From<AmqpList> for AmqpValue {
    fn from(v: AmqpList) -> Self;
}
#[cfg(feature = "ffi")]
impl From<AmqpMessageHeader> for crate::value::AmqpList {
    fn from(header: AmqpMessageHeader) -> AmqpList;
}
#[cfg(feature = "ffi")]
impl From<AmqpMessageProperties> for crate::value::AmqpList {
    fn from(properties: AmqpMessageProperties) -> AmqpList;
}
impl From<AmqpValue> for AmqpList {
    fn from(v: AmqpValue) -> Self;
}
impl From<Vec<AmqpValue>> for AmqpList {
    fn from(v: Vec<AmqpValue>) -> Self;
}
impl PartialEq<AmqpList> for AmqpValue {
    fn eq(&self, other: &AmqpList) -> bool;
}
impl PartialEq<AmqpValue> for AmqpList {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl<V> FromIterator<V> for AmqpList where V: Into<AmqpValue> {
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self;
}
pub struct AmqpManagement {
}
impl AmqpManagement {
    pub fn new(session: AmqpSession, client_node_name: String, access_token: AccessToken) -> Result<Self>;
}
impl AmqpManagementApis for AmqpManagement {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn attach(&self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn call(&self, operation_type: String, application_properties: AmqpOrderedMap<String, AmqpSimpleValue>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<AmqpOrderedMap<String, AmqpValue>>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn detach(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AmqpMessage {
    pub body: AmqpMessageBody,
    pub header: Option<AmqpMessageHeader>,
    pub application_properties: Option<AmqpApplicationProperties>,
    pub message_annotations: Option<AmqpAnnotations>,
    pub delivery_annotations: Option<AmqpAnnotations>,
    pub properties: Option<AmqpMessageProperties>,
    pub footer: Option<AmqpAnnotations>,
}
impl AmqpMessage {
    pub fn add_message_annotation<impl Into<AmqpValue>: Into<AmqpValue>>(&mut self, name: AmqpSymbol, value: impl Into<AmqpValue>);
    pub fn builder() -> builders::AmqpMessageBuilder;
    pub fn serialize(message: &AmqpMessage) -> Result<Vec<u8>>;
    pub fn set_message_body<impl Into<AmqpMessageBody>: Into<AmqpMessageBody>>(&mut self, body: impl Into<AmqpMessageBody>);
    pub fn set_message_id<impl Into<AmqpMessageId>: Into<AmqpMessageId>>(&mut self, message_id: impl Into<AmqpMessageId>);
}
impl AsRef<AmqpMessage> for AmqpMessage {
    fn as_ref(&self) -> &AmqpMessage;
}
#[cfg(feature = "ffi")]
impl Deserializable<AmqpMessage> for AmqpMessage {
    fn decode(data: &[u8]) -> Result<AmqpMessage>;
}
impl From<&AmqpMessage> for fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>> {
    fn from(message: &AmqpMessage) -> Self;
}
impl From<&Message<Body<EmptyBody>>> for crate::messaging::AmqpMessage {
    fn from(message: &fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::messaging::message::EmptyBody>>) -> Self;
}
impl From<&Message<Body<TransparentVec<Data>>>> for crate::messaging::AmqpMessage {
    fn from(message: &fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<TransparentVec<fe2o3_amqp_types::messaging::Data>>>) -> Self;
}
impl From<&Message<Body<Value>>> for crate::messaging::AmqpMessage {
    fn from(message: &fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<Value>>) -> Self;
}
impl From<AmqpList> for AmqpMessage {
    fn from(list: AmqpList) -> Self;
}
impl From<AmqpMessage> for fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<fe2o3_amqp_types::primitives::Value>> {
    fn from(message: AmqpMessage) -> Self;
}
impl From<AmqpMessage> for fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::message::EmptyBody> {
    fn from(message: AmqpMessage) -> Self;
}
impl From<AmqpMessage> for fe2o3_amqp_types::messaging::Message<serde_amqp::extensions::TransparentVec<fe2o3_amqp_types::messaging::AmqpSequence<fe2o3_amqp_types::primitives::Value>>> {
    fn from(message: AmqpMessage) -> Self;
}
impl From<AmqpMessage> for fe2o3_amqp_types::messaging::Message<serde_amqp::extensions::TransparentVec<fe2o3_amqp_types::messaging::Data>> {
    fn from(message: AmqpMessage) -> Self;
}
impl From<AmqpValue> for AmqpMessage {
    fn from(value: AmqpValue) -> Self;
}
impl From<Message<Body<Value>>> for crate::messaging::AmqpMessage {
    fn from(message: fe2o3_amqp_types::messaging::Message<fe2o3_amqp_types::messaging::Body<Value>>) -> Self;
}
impl From<Vec<u8>> for AmqpMessage {
    fn from(body: Vec<u8>) -> Self;
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AmqpOrderedMap<K, V> where K: PartialEq, V: Clone {
}
impl<K, V> AmqpOrderedMap<K, V> where K: PartialEq + Clone, V: Clone {
    pub fn contains_key<Q>(&self, key: &Q) -> bool where K: Borrow<Q>, Q: PartialEq<K> + ?Sized;
    pub fn get<Q>(&self, key: &Q) -> Option<&V> where K: Borrow<Q>, Q: PartialEq<K> + ?Sized;
    pub fn insert(&mut self, key: K, value: V);
    pub fn is_empty(&self) -> bool;
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> + '_;
    pub fn len(&self) -> usize;
    pub fn new() -> Self;
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V> where K: Borrow<Q>, Q: PartialEq<K> + ?Sized;
}
impl From<&AmqpValue> for AmqpOrderedMap<AmqpValue, AmqpValue> {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&OrderedMap<String, Value>> for crate::value::AmqpOrderedMap<std::string::String, crate::value::AmqpValue> {
    fn from(value: &fe2o3_amqp_types::primitives::OrderedMap<std::string::String, fe2o3_amqp_types::primitives::Value>) -> Self;
}
impl From<&OrderedMap<Symbol, Value>> for crate::value::AmqpOrderedMap<crate::value::AmqpSymbol, crate::value::AmqpValue> {
    fn from(fields: &fe2o3_amqp_types::definitions::Fields) -> Self;
}
impl From<AmqpOrderedMap<AmqpSymbol, AmqpValue>> for fe2o3_amqp_types::primitives::OrderedMap<fe2o3_amqp_types::primitives::Symbol, fe2o3_amqp_types::primitives::Value> {
    fn from(value: AmqpOrderedMap<AmqpSymbol, AmqpValue>) -> Self;
}
impl From<AmqpOrderedMap<AmqpValue, AmqpValue>> for AmqpValue {
    fn from(v: AmqpOrderedMap<AmqpValue, AmqpValue>) -> Self;
}
impl From<AmqpOrderedMap<AmqpValue, AmqpValue>> for fe2o3_amqp_types::primitives::OrderedMap<fe2o3_amqp_types::primitives::Value, fe2o3_amqp_types::primitives::Value> {
    fn from(value: AmqpOrderedMap<AmqpValue, AmqpValue>) -> Self;
}
impl From<AmqpValue> for AmqpOrderedMap<AmqpValue, AmqpValue> {
    fn from(v: AmqpValue) -> Self;
}
impl PartialEq<AmqpOrderedMap<AmqpValue, AmqpValue>> for AmqpValue {
    fn eq(&self, other: &AmqpOrderedMap<AmqpValue, AmqpValue>) -> bool;
}
impl PartialEq<AmqpValue> for AmqpOrderedMap<AmqpValue, AmqpValue> {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl<K, V> From<Vec<(K, V)>> for AmqpOrderedMap<K, V> where K: PartialEq, V: Clone {
    fn from(v: Vec<(K, V)>) -> Self;
}
impl<K, V> FromIterator<(K, V)> for AmqpOrderedMap<K, V> where K: PartialEq, V: Clone {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self;
}
impl<K, V> IntoIterator for AmqpOrderedMap<K, V> where K: PartialEq, V: Clone {
    type IntoIter = <Vec<(K, V)> as IntoIterator>::IntoIter;
    type Item = (K, V);
    fn into_iter(self) -> <Self as >::IntoIter;
}
#[derive(Default)]
pub struct AmqpReceiver {
}
impl AmqpReceiver {
    pub fn new() -> Self;
}
impl AmqpReceiverApis for AmqpReceiver {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn accept_delivery(&self, delivery: &AmqpDelivery) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn attach<impl 'async_trait + Into<AmqpSource> + Send: Into<AmqpSource> + Send>(&self, session: &AmqpSession, source: impl Into<AmqpSource> + Send, options: Option<AmqpReceiverOptions>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn credit_mode(&self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<ReceiverCreditMode>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn detach(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn receive_delivery(&self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<AmqpDelivery>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn reject_delivery(&self, delivery: &AmqpDelivery) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn release_delivery(&self, delivery: &AmqpDelivery) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn set_credit_mode(&self, credit_mode: ReceiverCreditMode) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn settle_delivery(&self, delivery: &AmqpDelivery, outcome: AmqpDeliveryOutcome) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
}
#[derive(Clone, Debug, Default)]
pub struct AmqpReceiverOptions {
    pub receiver_settle_mode: Option<crate::ReceiverSettleMode>,
    pub target: Option<crate::messaging::AmqpTarget>,
    pub name: Option<String>,
    pub credit_mode: Option<ReceiverCreditMode>,
    pub auto_accept: bool,
    pub properties: Option<crate::value::AmqpOrderedMap<crate::value::AmqpSymbol, crate::value::AmqpValue>>,
}
#[derive(Clone, Debug, Default)]
pub struct AmqpSendOptions {
    pub message_format: Option<u32>,
    pub settled: Option<bool>,
}
pub struct AmqpSender {
}
impl AmqpSender {
    pub fn new() -> Self;
}
impl AmqpSenderApis for AmqpSender {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn attach<impl 'async_trait + Into<AmqpTarget> + Send: Into<AmqpTarget> + Send>(&self, session: &AmqpSession, name: String, target: impl Into<AmqpTarget> + Send, options: Option<AmqpSenderOptions>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn detach(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn max_message_size(&self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Option<u64>>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn send<M>(&self, message: M, options: Option<AmqpSendOptions>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<AmqpSendOutcome>> + ::core::marker::Send>> where M: Into<AmqpMessage> + std::fmt::Debug + Send;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn send_ref<M>(&self, message: M, options: Option<AmqpSendOptions>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<AmqpSendOutcome>> + ::core::marker::Send>> where M: AsRef<AmqpMessage> + std::fmt::Debug + Send;
}
impl Default for AmqpSender {
    fn default() -> Self;
}
#[derive(Clone, Debug, Default)]
pub struct AmqpSenderOptions {
    pub sender_settle_mode: Option<crate::SenderSettleMode>,
    pub receiver_settle_mode: Option<crate::ReceiverSettleMode>,
    pub source: Option<crate::messaging::AmqpSource>,
    pub offered_capabilities: Option<Vec<crate::value::AmqpSymbol>>,
    pub desired_capabilities: Option<Vec<crate::value::AmqpSymbol>>,
    pub properties: Option<crate::value::AmqpOrderedMap<crate::value::AmqpSymbol, crate::value::AmqpValue>>,
    pub initial_delivery_count: Option<u32>,
    pub max_message_size: Option<u64>,
}
#[derive(Clone, Default)]
pub struct AmqpSession {
}
impl AmqpSession {
    pub fn new() -> Self;
}
impl AmqpSessionApis for AmqpSession {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn begin(&self, connection: &AmqpConnection, options: Option<AmqpSessionOptions>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn end(&self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
}
#[derive(Clone, Debug, Default)]
pub struct AmqpSessionOptions {
    pub next_outgoing_id: Option<u32>,
    pub incoming_window: Option<u32>,
    pub outgoing_window: Option<u32>,
    pub handle_max: Option<u32>,
    pub offered_capabilities: Option<Vec<super::value::AmqpSymbol>>,
    pub desired_capabilities: Option<Vec<super::value::AmqpSymbol>>,
    pub properties: Option<super::value::AmqpOrderedMap<super::value::AmqpSymbol, super::value::AmqpValue>>,
    pub buffer_size: Option<usize>,
}
impl AmqpSessionOptions {
    pub fn with_unbounded_windows() -> Self;
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AmqpSource {
    pub address: Option<String>,
    pub durable: Option<TerminusDurability>,
    pub expiry_policy: Option<TerminusExpiryPolicy>,
    pub timeout: Option<u32>,
    pub dynamic: Option<bool>,
    pub dynamic_node_properties: Option<crate::value::AmqpOrderedMap<crate::value::AmqpSymbol, crate::value::AmqpValue>>,
    pub distribution_mode: Option<DistributionMode>,
    pub filter: Option<crate::value::AmqpOrderedMap<crate::value::AmqpSymbol, crate::value::AmqpValue>>,
    pub default_outcome: Option<AmqpOutcome>,
    pub outcomes: Option<Vec<crate::value::AmqpSymbol>>,
    pub capabilities: Option<Vec<crate::value::AmqpSymbol>>,
}
impl AmqpSource {
    pub fn builder() -> builders::AmqpSourceBuilder;
}
#[cfg(feature = "ffi")]
impl From<AmqpList> for AmqpSource {
    fn from(list: AmqpList) -> Self;
}
#[cfg(feature = "ffi")]
impl From<AmqpSource> for crate::value::AmqpList {
    fn from(source: AmqpSource) -> Self;
}
impl From<AmqpSource> for fe2o3_amqp_types::messaging::Source {
    fn from(source: AmqpSource) -> Self;
}
impl From<Source> for crate::messaging::AmqpSource {
    fn from(source: fe2o3_amqp_types::messaging::Source) -> Self;
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AmqpSymbol(pub String);
impl Borrow<str> for AmqpSymbol {
    fn borrow(&self) -> &str;
}
impl From<&AmqpSymbol> for AmqpOutcome {
    fn from(symbol: &AmqpSymbol) -> Self;
}
impl From<&AmqpSymbol> for DistributionMode {
    fn from(symbol: &AmqpSymbol) -> Self;
}
impl From<&AmqpSymbol> for String {
    fn from(s: &AmqpSymbol) -> Self;
}
impl From<&AmqpSymbol> for TerminusExpiryPolicy {
    fn from(symbol: &AmqpSymbol) -> Self;
}
impl From<&AmqpSymbol> for fe2o3_amqp_types::primitives::Symbol {
    fn from(s: &AmqpSymbol) -> fe2o3_amqp_types::primitives::Symbol;
}
impl From<&AmqpValue> for AmqpSymbol {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&Symbol> for crate::value::AmqpSymbol {
    fn from(s: &fe2o3_amqp_types::primitives::Symbol) -> AmqpSymbol;
}
impl From<&str> for AmqpSymbol {
    fn from(s: &str) -> Self;
}
impl From<AmqpOutcome> for crate::value::AmqpSymbol {
    fn from(outcome: AmqpOutcome) -> Self;
}
impl From<AmqpSymbol> for AmqpAnnotationKey {
    fn from(symbol: AmqpSymbol) -> Self;
}
impl From<AmqpSymbol> for AmqpErrorCondition {
    fn from(condition: AmqpSymbol) -> Self;
}
impl From<AmqpSymbol> for AmqpValue {
    fn from(v: AmqpSymbol) -> Self;
}
impl From<AmqpSymbol> for String {
    fn from(s: AmqpSymbol) -> Self;
}
impl From<AmqpSymbol> for fe2o3_amqp_types::primitives::Symbol {
    fn from(s: AmqpSymbol) -> fe2o3_amqp_types::primitives::Symbol;
}
impl From<AmqpValue> for AmqpSymbol {
    fn from(v: AmqpValue) -> Self;
}
impl From<DistributionMode> for crate::value::AmqpSymbol {
    fn from(mode: DistributionMode) -> Self;
}
impl From<String> for AmqpSymbol {
    fn from(s: String) -> Self;
}
impl From<Symbol> for crate::value::AmqpSymbol {
    fn from(s: fe2o3_amqp_types::primitives::Symbol) -> AmqpSymbol;
}
impl From<TerminusDurability> for crate::value::AmqpSymbol {
    fn from(durability: TerminusDurability) -> Self;
}
impl From<TerminusExpiryPolicy> for crate::value::AmqpSymbol {
    fn from(policy: TerminusExpiryPolicy) -> Self;
}
impl PartialEq<&AmqpSymbol> for AmqpSymbol {
    fn eq(&self, other: &&AmqpSymbol) -> bool;
}
impl PartialEq<&str> for AmqpSymbol {
    fn eq(&self, other: &&str) -> bool;
}
impl PartialEq<AmqpSymbol> for &AmqpSymbol {
    fn eq(&self, other: &AmqpSymbol) -> bool;
}
impl PartialEq<AmqpSymbol> for &str {
    fn eq(&self, other: &AmqpSymbol) -> bool;
}
impl PartialEq<AmqpSymbol> for AmqpValue {
    fn eq(&self, other: &AmqpSymbol) -> bool;
}
impl PartialEq<AmqpSymbol> for fe2o3_amqp_types::primitives::Symbol {
    fn eq(&self, other: &AmqpSymbol) -> bool;
}
impl PartialEq<AmqpSymbol> for str {
    fn eq(&self, other: &AmqpSymbol) -> bool;
}
impl PartialEq<AmqpValue> for AmqpSymbol {
    fn eq(&self, other: &AmqpValue) -> bool;
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AmqpTarget {
    pub address: Option<String>,
    pub durable: Option<TerminusDurability>,
    pub expiry_policy: Option<TerminusExpiryPolicy>,
    pub timeout: Option<u32>,
    pub dynamic: Option<bool>,
    pub dynamic_node_properties: Option<crate::value::AmqpOrderedMap<String, crate::value::AmqpValue>>,
    pub capabilities: Option<Vec<crate::value::AmqpValue>>,
}
impl AmqpTarget {
    pub fn builder() -> builders::AmqpTargetBuilder;
}
#[cfg(feature = "ffi")]
impl From<AmqpList> for AmqpTarget {
    fn from(list: AmqpList) -> Self;
}
impl From<AmqpTarget> for String {
    fn from(target: AmqpTarget) -> String;
}
#[cfg(feature = "ffi")]
impl From<AmqpTarget> for crate::value::AmqpList {
    fn from(target: AmqpTarget) -> Self;
}
impl From<String> for AmqpTarget {
    fn from(address: String) -> AmqpTarget;
}
#[derive(Clone, Debug, PartialEq)]
pub struct AmqpTimestamp(pub Option<std::time::SystemTime>);
impl From<&AmqpTimestamp> for fe2o3_amqp_types::primitives::Timestamp {
    fn from(timestamp: &AmqpTimestamp) -> Self;
}
impl From<&AmqpValue> for AmqpTimestamp {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&Timestamp> for crate::value::AmqpTimestamp {
    fn from(timestamp: &fe2o3_amqp_types::primitives::Timestamp) -> Self;
}
impl From<AmqpTimestamp> for AmqpValue {
    fn from(v: AmqpTimestamp) -> Self;
}
impl From<AmqpTimestamp> for fe2o3_amqp_types::primitives::Timestamp {
    fn from(timestamp: AmqpTimestamp) -> Self;
}
impl From<AmqpValue> for AmqpTimestamp {
    fn from(v: AmqpValue) -> Self;
}
impl From<SystemTime> for AmqpTimestamp {
    fn from(v: SystemTime) -> Self;
}
impl From<Timestamp> for crate::value::AmqpTimestamp {
    fn from(timestamp: fe2o3_amqp_types::primitives::Timestamp) -> Self;
}
impl PartialEq<AmqpTimestamp> for AmqpValue {
    fn eq(&self, other: &AmqpTimestamp) -> bool;
}
impl PartialEq<AmqpValue> for AmqpTimestamp {
    fn eq(&self, other: &AmqpValue) -> bool;
}
#[derive(Clone, Debug, PartialEq)]
pub enum AmqpDeliveryOutcome {
    Accepted,
    Rejected(Option<crate::error::AmqpDescribedError>),
    Released,
    Modified { delivery_failed: Option<bool>, undeliverable_here: Option<bool>, message_annotations: Option<crate::value::AmqpOrderedMap<crate::value::AmqpSymbol, crate::value::AmqpValue>> },
}
#[cfg(feature = "ffi")]
#[derive(Clone, Debug, PartialEq)]
pub enum AmqpDescriptor {
    Code(u64),
    Name(AmqpSymbol),
}
#[cfg(feature = "ffi")]
impl From<&AmqpDescriptor> for serde_amqp::descriptor::Descriptor {
    fn from(descriptor: &AmqpDescriptor) -> Self;
}
#[cfg(feature = "ffi")]
impl From<&Descriptor> for crate::value::AmqpDescriptor {
    fn from(descriptor: &serde_amqp::descriptor::Descriptor) -> Self;
}
#[cfg(feature = "ffi")]
impl From<u64> for AmqpDescriptor {
    fn from(v: u64) -> Self;
}
#[cfg(feature = "ffi")]
impl PartialEq<AmqpDescriptor> for serde_amqp::descriptor::Descriptor {
    fn eq(&self, other: &AmqpDescriptor) -> bool;
}
#[cfg(feature = "ffi")]
impl PartialEq<Descriptor> for crate::value::AmqpDescriptor {
    fn eq(&self, other: &serde_amqp::descriptor::Descriptor) -> bool;
}
#[cfg(feature = "ffi")]
impl<T> From<T> for AmqpDescriptor where T: Into<AmqpSymbol> {
    fn from(v: T) -> Self;
}
pub enum AmqpSendOutcome {
    Accepted,
    Rejected(Option<crate::error::AmqpDescribedError>),
    Released,
    Modified(SendModification),
}
impl Send for AmqpSendOutcome {
}
#[derive(Clone, Debug, Default, PartialEq)]
pub enum AmqpSimpleValue {
    #[default]
    Null,
    Boolean(bool),
    UByte(u8),
    Byte(i8),
    Char(char),
    UShort(u16),
    Short(i16),
    UInt(u32),
    Int(i32),
    ULong(u64),
    Long(i64),
    Float(f32),
    Double(f64),
    Decimal128([u8; 16]),
    Decimal64([u8; 8]),
    Decimal32([u8; 4]),
    TimeStamp(crate::value::AmqpTimestamp),
    Uuid(azure_core::Uuid),
    String(String),
    Symbol(crate::value::AmqpSymbol),
    Binary(Vec<u8>),
    Described(Box<crate::value::AmqpDescribed>),
}
impl From<&AmqpSimpleValue> for Vec<u8> {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for [u8; 16] {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for [u8; 4] {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for [u8; 8] {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for azure_core::Uuid {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for bool {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for char {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for crate::value::AmqpSymbol {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for crate::value::AmqpTimestamp {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for f32 {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for f64 {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for fe2o3_amqp_types::primitives::SimpleValue {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for i16 {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for i32 {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for i64 {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for i8 {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for std::string::String {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for u16 {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for u32 {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for u64 {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&AmqpSimpleValue> for u8 {
    fn from(v: &AmqpSimpleValue) -> Self;
}
impl From<&SimpleValue> for crate::simple_value::AmqpSimpleValue {
    fn from(v: &fe2o3_amqp_types::primitives::SimpleValue) -> Self;
}
impl From<&str> for AmqpSimpleValue {
    fn from(value: &str) -> Self;
}
impl From<AmqpSimpleValue> for Vec<u8> {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for [u8; 16] {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for [u8; 4] {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for [u8; 8] {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for azure_core::Uuid {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for bool {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for char {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for crate::value::AmqpSymbol {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for crate::value::AmqpTimestamp {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for f32 {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for f64 {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for fe2o3_amqp_types::primitives::SimpleValue {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for i16 {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for i32 {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for i64 {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for i8 {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for std::string::String {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for u16 {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for u32 {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for u64 {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSimpleValue> for u8 {
    fn from(v: AmqpSimpleValue) -> Self;
}
impl From<AmqpSymbol> for AmqpSimpleValue {
    fn from(v: AmqpSymbol) -> Self;
}
impl From<AmqpTimestamp> for AmqpSimpleValue {
    fn from(v: AmqpTimestamp) -> Self;
}
impl From<SimpleValue> for crate::simple_value::AmqpSimpleValue {
    fn from(v: fe2o3_amqp_types::primitives::SimpleValue) -> Self;
}
impl From<String> for AmqpSimpleValue {
    fn from(v: std::string::String) -> Self;
}
impl From<Uuid> for AmqpSimpleValue {
    fn from(v: Uuid) -> Self;
}
impl From<Vec<u8>> for AmqpSimpleValue {
    fn from(v: Vec<u8>) -> Self;
}
impl From<[u8; 16]> for AmqpSimpleValue {
    fn from(v: [u8; 16]) -> Self;
}
impl From<[u8; 4]> for AmqpSimpleValue {
    fn from(v: [u8; 4]) -> Self;
}
impl From<[u8; 8]> for AmqpSimpleValue {
    fn from(v: [u8; 8]) -> Self;
}
impl From<bool> for AmqpSimpleValue {
    fn from(v: bool) -> Self;
}
impl From<char> for AmqpSimpleValue {
    fn from(v: char) -> Self;
}
impl From<f32> for AmqpSimpleValue {
    fn from(v: f32) -> Self;
}
impl From<f64> for AmqpSimpleValue {
    fn from(v: f64) -> Self;
}
impl From<i16> for AmqpSimpleValue {
    fn from(v: i16) -> Self;
}
impl From<i32> for AmqpSimpleValue {
    fn from(v: i32) -> Self;
}
impl From<i64> for AmqpSimpleValue {
    fn from(v: i64) -> Self;
}
impl From<i8> for AmqpSimpleValue {
    fn from(v: i8) -> Self;
}
impl From<u16> for AmqpSimpleValue {
    fn from(v: u16) -> Self;
}
impl From<u32> for AmqpSimpleValue {
    fn from(v: u32) -> Self;
}
impl From<u64> for AmqpSimpleValue {
    fn from(v: u64) -> Self;
}
impl From<u8> for AmqpSimpleValue {
    fn from(v: u8) -> Self;
}
impl PartialEq<AmqpSimpleValue> for Vec<u8> {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for [u8; 16] {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for [u8; 4] {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for [u8; 8] {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for azure_core::Uuid {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for bool {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for char {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for crate::value::AmqpSymbol {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for crate::value::AmqpTimestamp {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for f32 {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for f64 {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for i16 {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for i32 {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for i64 {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for i8 {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for std::string::String {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for u16 {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for u32 {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for u64 {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSimpleValue> for u8 {
    fn eq(&self, other: &AmqpSimpleValue) -> bool;
}
impl PartialEq<AmqpSymbol> for AmqpSimpleValue {
    fn eq(&self, other: &AmqpSymbol) -> bool;
}
impl PartialEq<AmqpTimestamp> for AmqpSimpleValue {
    fn eq(&self, other: &AmqpTimestamp) -> bool;
}
impl PartialEq<String> for AmqpSimpleValue {
    fn eq(&self, other: &std::string::String) -> bool;
}
impl PartialEq<Uuid> for AmqpSimpleValue {
    fn eq(&self, other: &Uuid) -> bool;
}
impl PartialEq<Vec<u8>> for AmqpSimpleValue {
    fn eq(&self, other: &Vec<u8>) -> bool;
}
impl PartialEq<[u8; 16]> for AmqpSimpleValue {
    fn eq(&self, other: &[u8; 16]) -> bool;
}
impl PartialEq<[u8; 4]> for AmqpSimpleValue {
    fn eq(&self, other: &[u8; 4]) -> bool;
}
impl PartialEq<[u8; 8]> for AmqpSimpleValue {
    fn eq(&self, other: &[u8; 8]) -> bool;
}
impl PartialEq<bool> for AmqpSimpleValue {
    fn eq(&self, other: &bool) -> bool;
}
impl PartialEq<char> for AmqpSimpleValue {
    fn eq(&self, other: &char) -> bool;
}
impl PartialEq<f32> for AmqpSimpleValue {
    fn eq(&self, other: &f32) -> bool;
}
impl PartialEq<f64> for AmqpSimpleValue {
    fn eq(&self, other: &f64) -> bool;
}
impl PartialEq<i16> for AmqpSimpleValue {
    fn eq(&self, other: &i16) -> bool;
}
impl PartialEq<i32> for AmqpSimpleValue {
    fn eq(&self, other: &i32) -> bool;
}
impl PartialEq<i64> for AmqpSimpleValue {
    fn eq(&self, other: &i64) -> bool;
}
impl PartialEq<i8> for AmqpSimpleValue {
    fn eq(&self, other: &i8) -> bool;
}
impl PartialEq<u16> for AmqpSimpleValue {
    fn eq(&self, other: &u16) -> bool;
}
impl PartialEq<u32> for AmqpSimpleValue {
    fn eq(&self, other: &u32) -> bool;
}
impl PartialEq<u64> for AmqpSimpleValue {
    fn eq(&self, other: &u64) -> bool;
}
impl PartialEq<u8> for AmqpSimpleValue {
    fn eq(&self, other: &u8) -> bool;
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AmqpTransport {
    #[default]
    Tcp,
    WebSocket,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub enum AmqpValue {
    #[default]
    Null,
    Boolean(bool),
    UByte(u8),
    UShort(u16),
    UInt(u32),
    ULong(u64),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Char(char),
    TimeStamp(AmqpTimestamp),
    Uuid(azure_core::Uuid),
    Binary(Vec<u8>),
    String(String),
    Symbol(AmqpSymbol),
    Decimal128([u8; 16]),
    Decimal64([u8; 8]),
    Decimal32([u8; 4]),
    List(AmqpList),
    Map(AmqpOrderedMap<AmqpValue, AmqpValue>),
    Array(Vec<AmqpValue>),
    Described(Box<AmqpDescribed>),
    #[cfg(feature = "ffi")]
    Composite(Box<AmqpComposite>),
}
#[cfg(feature = "ffi")]
impl Deserializable<AmqpValue> for AmqpValue {
    #[allow(unused_variables)]
    fn decode(data: &[u8]) -> Result<AmqpValue>;
}
impl From<&AmqpValue> for Vec<AmqpValue> {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for Vec<u8> {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for azure_core::Uuid {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for bool {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for char {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for f32 {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for f64 {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for fe2o3_amqp_types::primitives::Value {
    fn from(value: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for i16 {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for i32 {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for i64 {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for i8 {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for std::string::String {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for u16 {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for u32 {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for u64 {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&AmqpValue> for u8 {
    fn from(v: &AmqpValue) -> Self;
}
impl From<&Value> for crate::value::AmqpValue {
    fn from(value: &fe2o3_amqp_types::primitives::Value) -> Self;
}
impl From<&str> for AmqpValue {
    fn from(b: &str) -> Self;
}
impl From<()> for AmqpValue {
    fn from(_: ()) -> Self;
}
impl From<AmqpAnnotationKey> for crate::value::AmqpValue {
    fn from(key: AmqpAnnotationKey) -> Self;
}
impl From<AmqpMessageId> for crate::value::AmqpValue {
    fn from(message_id: AmqpMessageId) -> Self;
}
impl From<AmqpValue> for () {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for AmqpAnnotationKey {
    fn from(value: AmqpValue) -> Self;
}
impl From<AmqpValue> for AmqpMessageBody {
    fn from(value: AmqpValue) -> Self;
}
impl From<AmqpValue> for Vec<AmqpValue> {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for Vec<u8> {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for azure_core::Uuid {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for bool {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for char {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for f32 {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for f64 {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for fe2o3_amqp_types::primitives::Value {
    fn from(value: AmqpValue) -> Self;
}
impl From<AmqpValue> for i16 {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for i32 {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for i64 {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for i8 {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for std::string::String {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for u16 {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for u32 {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for u64 {
    fn from(v: AmqpValue) -> Self;
}
impl From<AmqpValue> for u8 {
    fn from(v: AmqpValue) -> Self;
}
impl From<Box<AmqpDescribed>> for AmqpValue {
    fn from(v: Box<AmqpDescribed>) -> Self;
}
impl From<Box<AmqpValue>> for AmqpValue {
    fn from(b: Box<AmqpValue>) -> Self;
}
impl From<String> for AmqpValue {
    fn from(v: std::string::String) -> Self;
}
impl From<Uuid> for AmqpValue {
    fn from(v: Uuid) -> Self;
}
impl From<Value> for crate::value::AmqpValue {
    fn from(value: fe2o3_amqp_types::primitives::Value) -> Self;
}
impl From<Vec<AmqpValue>> for AmqpValue {
    fn from(v: Vec<AmqpValue>) -> Self;
}
impl From<Vec<u8>> for AmqpValue {
    fn from(v: Vec<u8>) -> Self;
}
impl From<bool> for AmqpValue {
    fn from(v: bool) -> Self;
}
impl From<char> for AmqpValue {
    fn from(v: char) -> Self;
}
impl From<f32> for AmqpValue {
    fn from(v: f32) -> Self;
}
impl From<f64> for AmqpValue {
    fn from(v: f64) -> Self;
}
impl From<i16> for AmqpValue {
    fn from(v: i16) -> Self;
}
impl From<i32> for AmqpValue {
    fn from(v: i32) -> Self;
}
impl From<i64> for AmqpValue {
    fn from(v: i64) -> Self;
}
impl From<i8> for AmqpValue {
    fn from(v: i8) -> Self;
}
impl From<u16> for AmqpValue {
    fn from(v: u16) -> Self;
}
impl From<u32> for AmqpValue {
    fn from(v: u32) -> Self;
}
impl From<u64> for AmqpValue {
    fn from(v: u64) -> Self;
}
impl From<u8> for AmqpValue {
    fn from(v: u8) -> Self;
}
impl PartialEq<()> for AmqpValue {
    fn eq(&self, _: &()) -> bool;
}
impl PartialEq<AmqpValue> for () {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for Vec<AmqpValue> {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for Vec<u8> {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for azure_core::Uuid {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for bool {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for char {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for f32 {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for f64 {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for fe2o3_amqp_types::primitives::Value {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for i16 {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for i32 {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for i64 {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for i8 {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for std::string::String {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for u16 {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for u32 {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for u64 {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<AmqpValue> for u8 {
    fn eq(&self, other: &AmqpValue) -> bool;
}
impl PartialEq<Box<AmqpDescribed>> for AmqpValue {
    fn eq(&self, other: &Box<AmqpDescribed>) -> bool;
}
impl PartialEq<String> for AmqpValue {
    fn eq(&self, other: &std::string::String) -> bool;
}
impl PartialEq<Uuid> for AmqpValue {
    fn eq(&self, other: &Uuid) -> bool;
}
impl PartialEq<Value> for crate::value::AmqpValue {
    fn eq(&self, other: &fe2o3_amqp_types::primitives::Value) -> bool;
}
impl PartialEq<Vec<AmqpValue>> for AmqpValue {
    fn eq(&self, other: &Vec<AmqpValue>) -> bool;
}
impl PartialEq<Vec<u8>> for AmqpValue {
    fn eq(&self, other: &Vec<u8>) -> bool;
}
impl PartialEq<bool> for AmqpValue {
    fn eq(&self, other: &bool) -> bool;
}
impl PartialEq<char> for AmqpValue {
    fn eq(&self, other: &char) -> bool;
}
impl PartialEq<f32> for AmqpValue {
    fn eq(&self, other: &f32) -> bool;
}
impl PartialEq<f64> for AmqpValue {
    fn eq(&self, other: &f64) -> bool;
}
impl PartialEq<i16> for AmqpValue {
    fn eq(&self, other: &i16) -> bool;
}
impl PartialEq<i32> for AmqpValue {
    fn eq(&self, other: &i32) -> bool;
}
impl PartialEq<i64> for AmqpValue {
    fn eq(&self, other: &i64) -> bool;
}
impl PartialEq<i8> for AmqpValue {
    fn eq(&self, other: &i8) -> bool;
}
impl PartialEq<u16> for AmqpValue {
    fn eq(&self, other: &u16) -> bool;
}
impl PartialEq<u32> for AmqpValue {
    fn eq(&self, other: &u32) -> bool;
}
impl PartialEq<u64> for AmqpValue {
    fn eq(&self, other: &u64) -> bool;
}
impl PartialEq<u8> for AmqpValue {
    fn eq(&self, other: &u8) -> bool;
}
#[cfg(feature = "ffi")]
impl Serializable for AmqpValue {
    fn encoded_size(&self) -> Result<usize>;
    #[allow(unused_variables)]
    fn serialize(&self, buffer: &mut [u8]) -> Result<()>;
}
impl TryInto<AmqpValue> for Vec<Vec<serde_amqp::Value>> {
    type Error = Error;
    fn try_into(self) -> Result<AmqpValue, <Self as >::Error>;
}
impl TryInto<AmqpValue> for fe2o3_amqp_types::messaging::Data {
    type Error = Error;
    fn try_into(self) -> Result<AmqpValue, <Self as >::Error>;
}
impl TryInto<AmqpValue> for fe2o3_amqp_types::messaging::message::EmptyBody {
    type Error = Error;
    fn try_into(self) -> Result<AmqpValue, <Self as >::Error>;
}
impl TryInto<AmqpValue> for serde_amqp::extensions::TransparentVec<fe2o3_amqp_types::messaging::Data> {
    type Error = Error;
    fn try_into(self) -> Result<AmqpValue, <Self as >::Error>;
}
#[derive(Clone, Debug, PartialEq)]
pub enum ReceiverCreditMode {
    Auto(u32),
    Manual,
}
impl Default for ReceiverCreditMode {
    fn default() -> Self;
}
impl From<&CreditMode> for crate::receiver::ReceiverCreditMode {
    fn from(credit_mode: &fe2o3_amqp::link::receiver::CreditMode) -> Self;
}
impl From<ReceiverCreditMode> for fe2o3_amqp::link::receiver::CreditMode {
    fn from(credit_mode: ReceiverCreditMode) -> Self;
}
#[derive(Clone, Debug, PartialEq)]
pub enum ReceiverSettleMode {
    First = AMQP_RECEIVER_SETTLE_MODE_FIRST,
    Second = AMQP_RECEIVER_SETTLE_MODE_SECOND,
}
impl From<&ReceiverSettleMode> for crate::ReceiverSettleMode {
    fn from(mode: &fe2o3_amqp_types::definitions::ReceiverSettleMode) -> crate::ReceiverSettleMode;
}
impl From<ReceiverSettleMode> for fe2o3_amqp_types::definitions::ReceiverSettleMode {
    fn from(mode: crate::ReceiverSettleMode) -> fe2o3_amqp_types::definitions::ReceiverSettleMode;
}
#[derive(Clone, Debug, PartialEq)]
pub enum SenderSettleMode {
    Unsettled = AMQP_SENDER_SETTLE_MODE_UNSETTLED,
    Settled = AMQP_SENDER_SETTLE_MODE_SETTLED,
    Mixed = AMQP_SENDER_SETTLE_MODE_MIXED,
}
impl From<&SenderSettleMode> for crate::SenderSettleMode {
    fn from(mode: &fe2o3_amqp_types::definitions::SenderSettleMode) -> crate::SenderSettleMode;
}
impl From<SenderSettleMode> for fe2o3_amqp_types::definitions::SenderSettleMode {
    fn from(mode: crate::SenderSettleMode) -> fe2o3_amqp_types::definitions::SenderSettleMode;
}
#[async_trait]
pub trait AmqpClaimsBasedSecurityApis {
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn attach(&self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn authorize_path(&self, path: String, token_type: Option<String>, secret: &Secret, expires_on: OffsetDateTime) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn detach(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
}
#[async_trait]
pub trait AmqpConnectionApis {
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn close(&self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn close_with_error(&self, condition: AmqpSymbol, description: Option<String>, info: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn open(&self, name: String, url: Url, options: Option<AmqpConnectionOptions>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
}
pub trait AmqpDeliveryApis {
    fn delivery_id(&self) -> u32;
    fn delivery_tag(&self) -> &Vec<u8>;
    fn into_message(self) -> AmqpMessage;
    fn message(&self) -> &AmqpMessage;
    fn message_format(&self) -> &Option<u32>;
}
#[async_trait]
pub trait AmqpManagementApis {
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn attach(&self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn call(&self, operation_type: String, application_properties: AmqpOrderedMap<String, AmqpSimpleValue>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<AmqpOrderedMap<String, AmqpValue>>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn detach(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
}
#[async_trait]
pub trait AmqpReceiverApis {
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn accept_delivery(&self, delivery: &AmqpDelivery) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn attach<impl 'async_trait + Into<AmqpSource> + Send: Into<AmqpSource> + Send>(&self, session: &AmqpSession, source: impl Into<AmqpSource> + Send, options: Option<AmqpReceiverOptions>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn credit_mode(&self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<ReceiverCreditMode>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn detach(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn receive_delivery(&self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<AmqpDelivery>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn reject_delivery(&self, delivery: &AmqpDelivery) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn release_delivery(&self, delivery: &AmqpDelivery) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn set_credit_mode(&self, credit_mode: ReceiverCreditMode) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn settle_delivery(&self, delivery: &AmqpDelivery, outcome: AmqpDeliveryOutcome) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>> where Self: ::core::marker::Sync;
}
#[async_trait]
pub trait AmqpSenderApis {
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn attach<impl 'async_trait + Into<AmqpTarget> + Send: Into<AmqpTarget> + Send>(&self, session: &AmqpSession, name: String, target: impl Into<AmqpTarget> + Send, options: Option<AmqpSenderOptions>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn detach(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn max_message_size(&self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Option<u64>>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn send<M>(&self, message: M, options: Option<AmqpSendOptions>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<AmqpSendOutcome>> + ::core::marker::Send>> where M: Into<AmqpMessage> + std::fmt::Debug + Send;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn send_ref<M>(&self, message: M, options: Option<AmqpSendOptions>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<AmqpSendOutcome>> + ::core::marker::Send>> where M: AsRef<AmqpMessage> + std::fmt::Debug + Send;
}
#[async_trait]
pub trait AmqpSessionApis {
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn begin(&self, connection: &AmqpConnection, options: Option<AmqpSessionOptions>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn end(&self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
}
#[cfg(feature = "ffi")]
pub trait Deserializable<T> {
    fn decode(data: &[u8]) -> crate::error::Result<T>;
}
#[cfg(feature = "ffi")]
pub trait Serializable {
    fn encoded_size(&self) -> crate::error::Result<usize>;
    fn serialize(&self, buffer: &mut [u8]) -> crate::error::Result<()>;
}
pub mod builder {
    pub struct AmqpMessageBuilder {
    }
    impl AmqpMessageBuilder {
        pub fn add_application_property<impl Into<AmqpSimpleValue>: Into<AmqpSimpleValue>>(self, key: String, value: impl Into<AmqpSimpleValue>) -> Self;
        pub fn add_message_body_binary(self, body: Vec<u8>) -> Self;
        pub fn add_message_body_sequence(self, body: AmqpList) -> Self;
        pub fn build(&self) -> AmqpMessage;
        pub fn with_application_properties(self, application_properties: AmqpApplicationProperties) -> Self;
        pub fn with_body<impl Into<AmqpMessageBody>: Into<AmqpMessageBody>>(self, body: impl Into<AmqpMessageBody>) -> Self;
        pub fn with_delivery_annotations(self, delivery_annotations: AmqpAnnotations) -> Self;
        pub fn with_footer(self, footer: AmqpAnnotations) -> Self;
        pub fn with_header(self, header: AmqpMessageHeader) -> Self;
        pub fn with_message_annotations(self, message_annotations: AmqpAnnotations) -> Self;
        pub fn with_properties<T>(self, properties: T) -> Self where T: Into<AmqpMessageProperties>;
    }
    pub struct AmqpSourceBuilder {
    }
    impl AmqpSourceBuilder {
        pub fn add_to_filter<impl Into<AmqpValue>: Into<AmqpValue>>(self, key: AmqpSymbol, value: impl Into<AmqpValue>) -> Self;
        pub fn build(&self) -> AmqpSource;
        pub fn with_address(self, address: String) -> Self;
        pub fn with_capabilities(self, capabilities: Vec<AmqpSymbol>) -> Self;
        pub fn with_default_outcome(self, default_outcome: AmqpOutcome) -> Self;
        pub fn with_distribution_mode(self, distribution_mode: DistributionMode) -> Self;
        pub fn with_durable(self, durable: TerminusDurability) -> Self;
        pub fn with_dynamic(self, dynamic: bool) -> Self;
        pub fn with_dynamic_node_properties<impl Into<AmqpOrderedMap<AmqpSymbol, AmqpValue>>: Into<AmqpOrderedMap<AmqpSymbol, AmqpValue>>>(self, dynamic_node_properties: impl Into<AmqpOrderedMap<AmqpSymbol, AmqpValue>>) -> Self;
        pub fn with_expiry_policy(self, expiry_policy: TerminusExpiryPolicy) -> Self;
        pub fn with_filter<impl Into<AmqpOrderedMap<AmqpSymbol, AmqpValue>>: Into<AmqpOrderedMap<AmqpSymbol, AmqpValue>>>(self, filter: impl Into<AmqpOrderedMap<AmqpSymbol, AmqpValue>>) -> Self;
        pub fn with_outcomes(self, outcomes: Vec<AmqpSymbol>) -> Self;
        pub fn with_timeout(self, timeout: u32) -> Self;
    }
    pub struct AmqpTargetBuilder {
    }
    impl AmqpTargetBuilder {
        pub fn build(&self) -> AmqpTarget;
        pub fn with_address(self, address: String) -> Self;
        pub fn with_capabilities(self, capabilities: Vec<AmqpValue>) -> Self;
        pub fn with_durable(self, durable: TerminusDurability) -> Self;
        pub fn with_dynamic(self, dynamic: bool) -> Self;
        pub fn with_dynamic_node_properties<impl Into<AmqpOrderedMap<String, AmqpValue>>: Into<AmqpOrderedMap<String, AmqpValue>>>(self, dynamic_node_properties: impl Into<AmqpOrderedMap<String, AmqpValue>>) -> Self;
        pub fn with_expiry_policy(self, expiry_policy: TerminusExpiryPolicy) -> Self;
        pub fn with_timeout(self, timeout: u32) -> Self;
    }
}
pub mod error {
    #[derive(Clone, Debug, PartialEq)]
    pub struct AmqpDescribedError {
        pub condition: AmqpErrorCondition,
        pub description: Option<String>,
        pub info: crate::AmqpOrderedMap<crate::AmqpSymbol, crate::AmqpValue>,
    }
    impl AmqpDescribedError {
        pub fn new(condition: AmqpErrorCondition, description: Option<String>, info: AmqpOrderedMap<AmqpSymbol, AmqpValue>) -> Self;
    }
    impl From<AmqpDescribedError> for fe2o3_amqp_types::definitions::Error {
        fn from(e: AmqpDescribedError) -> Self;
    }
    impl From<Error> for crate::error::AmqpDescribedError {
        fn from(e: fe2o3_amqp_types::definitions::Error) -> Self;
    }
    pub struct AmqpError {
    }
    impl AmqpError {
        pub fn kind(&self) -> &AmqpErrorKind;
        #[cfg(feature = "test")]
        pub fn new_described_error(condition: AmqpErrorCondition, description: Option<String>, info: AmqpOrderedMap<AmqpSymbol, AmqpValue>) -> Self;
        #[cfg(feature = "test")]
        pub fn new_management_error(status_code: azure_core::http::StatusCode, description: Option<String>) -> Self;
        pub fn with_message<C>(message: C) -> AmqpError where C: Into<Cow<'static, str>>;
    }
    impl Debug for AmqpError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl Display for AmqpError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl Error for AmqpError {
        fn source(&self) -> Option<&dyn std::error::Error + 'static>;
    }
    impl From<AmqpError> for azure_core::Error {
        fn from(value: AmqpError) -> Self;
    }
    impl From<AttachError> for crate::AmqpError {
        fn from(e: fe2o3_amqp_management::error::AttachError) -> Self;
    }
    impl From<BeginError> for crate::AmqpError {
        fn from(e: fe2o3_amqp::session::BeginError) -> Self;
    }
    impl From<DetachError> for crate::AmqpError {
        fn from(e: fe2o3_amqp::link::DetachError) -> Self;
    }
    impl From<Error> for AmqpError {
        fn from(error: azure_core::Error) -> Self;
    }
    impl From<Error> for crate::AmqpError {
        fn from(e: fe2o3_amqp_management::error::Error) -> Self;
    }
    impl From<IllegalLinkStateError> for crate::AmqpError {
        fn from(e: fe2o3_amqp::link::IllegalLinkStateError) -> Self;
    }
    impl From<LinkStateError> for crate::AmqpError {
        fn from(e: fe2o3_amqp::link::LinkStateError) -> Self;
    }
    impl From<ReceiverAttachError> for crate::AmqpError {
        fn from(e: fe2o3_amqp::link::ReceiverAttachError) -> Self;
    }
    impl From<RecvError> for crate::AmqpError {
        fn from(e: fe2o3_amqp::link::RecvError) -> Self;
    }
    impl From<SendError> for crate::error::AmqpError {
        fn from(e: fe2o3_amqp::link::SendError) -> Self;
    }
    impl From<SenderAttachError> for crate::error::AmqpError {
        fn from(e: fe2o3_amqp::link::SenderAttachError) -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum AmqpErrorCondition {
        DecodeError,
        FrameSizeTooSmall,
        IllegalState,
        InternalError,
        InvalidField,
        NotAllowed,
        NotFound,
        NotImplemented,
        PreconditionFailed,
        ResourceDeleted,
        ResourceLimitExceeded,
        ResourceLocked,
        UnauthorizedAccess,
        LinkStolen,
        LinkPayloadSizeExceeded,
        LinkDetachForced,
        ConnectionForced,
        DeadLetter,
        ServerBusyError,
        ArgumentError,
        ArgumentOutOfRangeError,
        EntityDisabledError,
        PartitionNotOwnedError,
        StoreLockLostError,
        PublisherRevokedError,
        TimeoutError,
        TrackingIdProperty,
        ProtonIo,
        ConnectionFramingError,
        OperationCancelled,
        MessageLockLost,
        SessionLockLost,
        SessionCannotBeLocked,
        EntityUpdated,
        MessageNotFound,
        SessionNotFound,
        EntityAlreadyExists,
        ConnectionRedirect,
        LinkRedirect,
        TransferLimitExceeded,
        SessionWindowViolation,
        SessionErrantLink,
        SessionHandleInUse,
        SessionUnattachedHandle,
        UnknownValue(String),
    }
    impl AsRef<str> for AmqpErrorCondition {
        fn as_ref(&self) -> &str;
    }
    impl Display for AmqpErrorCondition {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result;
    }
    impl From<&AmqpError> for crate::error::AmqpErrorCondition {
        fn from(e: &fe2o3_amqp_types::definitions::AmqpError) -> Self;
    }
    impl From<&ConnectionError> for crate::error::AmqpErrorCondition {
        fn from(e: &fe2o3_amqp_types::definitions::ConnectionError) -> Self;
    }
    impl From<&ErrorCondition> for crate::error::AmqpErrorCondition {
        fn from(e: &fe2o3_amqp_types::definitions::ErrorCondition) -> Self;
    }
    impl From<&LinkError> for crate::error::AmqpErrorCondition {
        fn from(e: &fe2o3_amqp_types::definitions::LinkError) -> Self;
    }
    impl From<&SessionError> for crate::error::AmqpErrorCondition {
        fn from(e: &fe2o3_amqp_types::definitions::SessionError) -> Self;
    }
    impl From<AmqpSymbol> for AmqpErrorCondition {
        fn from(condition: AmqpSymbol) -> Self;
    }
    impl FromStr for AmqpErrorCondition {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as ::std::str::FromStr>::Err>;
    }
    impl Serialize for AmqpErrorCondition {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: serde::Serializer;
    }
    impl<'a> From<&'a AmqpErrorCondition> for &'a str {
        fn from(e: &'a AmqpErrorCondition) -> Self;
    }
    impl<'de> Deserialize<'de> for AmqpErrorCondition {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: serde::Deserializer<'de>;
    }
    pub enum AmqpErrorKind {
        SimpleMessage(std::borrow::Cow<'static, str>),
        AzureCore(azure_core::Error),
        AmqpDescribedError(AmqpDescribedError),
        LinkClosedByRemote(Box<dyn std::error::Error + Send + Sync>),
        SessionClosedByRemote(Box<dyn std::error::Error + Send + Sync>),
        ConnectionClosedByRemote(Box<dyn std::error::Error + Send + Sync>),
        LinkDetachedByRemote(Box<dyn std::error::Error + Send + Sync>),
        SessionDetachedByRemote(Box<dyn std::error::Error + Send + Sync>),
        ConnectionDetachedByRemote(Box<dyn std::error::Error + Send + Sync>),
        NonTerminalDeliveryState,
        IllegalDeliveryState,
        ConnectionDropped(Box<dyn std::error::Error + Send + Sync>),
        LinkStateError(Box<dyn std::error::Error + Send + Sync>),
        FramingError(Box<dyn std::error::Error + Send + Sync>),
        IdleTimeoutElapsed(Box<dyn std::error::Error + Send + Sync>),
        TransferLimitExceeded(Box<dyn std::error::Error + Send + Sync>),
        ManagementStatusCode(azure_core::http::StatusCode, Option<String>),
        DetachError(Box<dyn std::error::Error + Send + Sync>),
        TransportImplementationError(Box<dyn std::error::Error + Send + Sync>),
        SendRejected,
    }
    impl From<AmqpErrorKind> for AmqpError {
        fn from(kind: AmqpErrorKind) -> Self;
    }
    pub type Result<T> = std::result::Result<T, AmqpError>;
}
pub mod message {
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct AmqpAnnotations(pub crate::value::AmqpOrderedMap<AmqpAnnotationKey, crate::value::AmqpValue>);
    impl AmqpAnnotations {
        pub fn insert<impl Into<AmqpAnnotationKey>: Into<AmqpAnnotationKey>, impl Into<AmqpValue>: Into<AmqpValue>>(&mut self, key: impl Into<AmqpAnnotationKey>, value: impl Into<AmqpValue>);
        pub fn new() -> Self;
    }
    impl From<&AmqpAnnotations> for fe2o3_amqp_types::messaging::Annotations {
        fn from(annotations: &AmqpAnnotations) -> Self;
    }
    impl From<&AmqpAnnotations> for fe2o3_amqp_types::messaging::DeliveryAnnotations {
        fn from(annotations: &AmqpAnnotations) -> Self;
    }
    impl From<&AmqpAnnotations> for fe2o3_amqp_types::messaging::Footer {
        fn from(annotations: &AmqpAnnotations) -> Self;
    }
    impl From<&AmqpAnnotations> for fe2o3_amqp_types::messaging::MessageAnnotations {
        fn from(annotations: &AmqpAnnotations) -> Self;
    }
    impl From<&OrderedMap<OwnedKey, Value>> for crate::messaging::AmqpAnnotations {
        fn from(annotations: &fe2o3_amqp_types::messaging::Annotations) -> Self;
    }
    impl From<AmqpAnnotations> for fe2o3_amqp_types::messaging::Annotations {
        fn from(annotations: AmqpAnnotations) -> Self;
    }
    impl From<AmqpAnnotations> for fe2o3_amqp_types::messaging::DeliveryAnnotations {
        fn from(annotations: AmqpAnnotations) -> Self;
    }
    impl From<AmqpAnnotations> for fe2o3_amqp_types::messaging::Footer {
        fn from(annotations: AmqpAnnotations) -> Self;
    }
    impl From<AmqpAnnotations> for fe2o3_amqp_types::messaging::MessageAnnotations {
        fn from(annotations: AmqpAnnotations) -> Self;
    }
    impl From<OrderedMap<OwnedKey, Value>> for crate::messaging::AmqpAnnotations {
        fn from(annotations: fe2o3_amqp_types::messaging::Annotations) -> Self;
    }
    impl<K, V> From<Vec<(K, V)>> for AmqpAnnotations where K: Into<AmqpAnnotationKey>, V: Into<crate::value::AmqpValue> {
        fn from(vec: Vec<(K, V)>) -> Self;
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct AmqpApplicationProperties(pub crate::value::AmqpOrderedMap<String, crate::simple_value::AmqpSimpleValue>);
    impl AmqpApplicationProperties {
        pub fn insert<impl Into<AmqpSimpleValue>: Into<AmqpSimpleValue>>(&mut self, key: String, value: impl Into<AmqpSimpleValue>);
        pub fn new() -> Self;
    }
    impl From<&AmqpApplicationProperties> for fe2o3_amqp_types::messaging::ApplicationProperties {
        fn from(application_properties: &AmqpApplicationProperties) -> Self;
    }
    impl From<&ApplicationProperties> for crate::messaging::AmqpApplicationProperties {
        fn from(application_properties: &fe2o3_amqp_types::messaging::ApplicationProperties) -> Self;
    }
    impl From<AmqpApplicationProperties> for fe2o3_amqp_types::messaging::ApplicationProperties {
        fn from(application_properties: AmqpApplicationProperties) -> Self;
    }
    impl From<ApplicationProperties> for crate::messaging::AmqpApplicationProperties {
        fn from(application_properties: fe2o3_amqp_types::messaging::ApplicationProperties) -> Self;
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct AmqpMessageHeader {
        pub durable: bool,
        pub priority: u8,
        pub time_to_live: Option<azure_core::time::Duration>,
        pub first_acquirer: bool,
        pub delivery_count: u32,
    }
    impl Default for AmqpMessageHeader {
        fn default() -> Self;
    }
    impl From<&AmqpMessageHeader> for fe2o3_amqp_types::messaging::Header {
        fn from(header: &AmqpMessageHeader) -> Self;
    }
    impl From<&Header> for crate::messaging::AmqpMessageHeader {
        fn from(header: &fe2o3_amqp_types::messaging::Header) -> Self;
    }
    #[cfg(feature = "ffi")]
    impl From<AmqpList> for AmqpMessageHeader {
        fn from(list: AmqpList) -> Self;
    }
    #[cfg(feature = "ffi")]
    impl From<AmqpMessageHeader> for crate::value::AmqpList {
        fn from(header: AmqpMessageHeader) -> AmqpList;
    }
    impl From<AmqpMessageHeader> for fe2o3_amqp_types::messaging::Header {
        fn from(header: AmqpMessageHeader) -> Self;
    }
    impl From<Header> for crate::messaging::AmqpMessageHeader {
        fn from(header: fe2o3_amqp_types::messaging::Header) -> Self;
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct AmqpMessageProperties {
        pub message_id: Option<AmqpMessageId>,
        pub user_id: Option<Vec<u8>>,
        pub to: Option<String>,
        pub subject: Option<String>,
        pub reply_to: Option<String>,
        pub correlation_id: Option<AmqpMessageId>,
        pub content_type: Option<crate::value::AmqpSymbol>,
        pub content_encoding: Option<crate::value::AmqpSymbol>,
        pub absolute_expiry_time: Option<crate::value::AmqpTimestamp>,
        pub creation_time: Option<crate::value::AmqpTimestamp>,
        pub group_id: Option<String>,
        pub group_sequence: Option<u32>,
        pub reply_to_group_id: Option<String>,
    }
    impl From<&AmqpMessageProperties> for fe2o3_amqp_types::messaging::Properties {
        fn from(properties: &AmqpMessageProperties) -> Self;
    }
    impl From<&Properties> for crate::messaging::AmqpMessageProperties {
        fn from(properties: &fe2o3_amqp_types::messaging::Properties) -> Self;
    }
    #[cfg(feature = "ffi")]
    impl From<AmqpList> for AmqpMessageProperties {
        fn from(list: AmqpList) -> Self;
    }
    #[cfg(feature = "ffi")]
    impl From<AmqpMessageProperties> for crate::value::AmqpList {
        fn from(properties: AmqpMessageProperties) -> AmqpList;
    }
    impl From<AmqpMessageProperties> for fe2o3_amqp_types::messaging::Properties {
        fn from(properties: AmqpMessageProperties) -> Self;
    }
    impl From<Properties> for crate::messaging::AmqpMessageProperties {
        fn from(properties: fe2o3_amqp_types::messaging::Properties) -> Self;
    }
    #[derive(Debug, Default)]
    pub struct AmqpSourceFilter {
    }
    impl AmqpSourceFilter {
        pub fn code(&self) -> u64;
        pub fn description(&self) -> &'static str;
        pub fn selector_filter() -> AmqpSourceFilter;
    }
    #[derive(Clone, Debug, PartialEq)]
    pub enum AmqpAnnotationKey {
        Symbol(crate::value::AmqpSymbol),
        Ulong(u64),
    }
    impl AsRef<AmqpAnnotationKey> for AmqpAnnotationKey {
        fn as_ref(&self) -> &AmqpAnnotationKey;
    }
    impl Default for AmqpAnnotationKey {
        fn default() -> Self;
    }
    impl From<&AmqpAnnotationKey> for fe2o3_amqp_types::messaging::annotations::OwnedKey {
        fn from(key: &AmqpAnnotationKey) -> Self;
    }
    impl From<&OwnedKey> for crate::messaging::AmqpAnnotationKey {
        fn from(key: &fe2o3_amqp_types::messaging::annotations::OwnedKey) -> Self;
    }
    impl From<AmqpAnnotationKey> for crate::value::AmqpValue {
        fn from(key: AmqpAnnotationKey) -> Self;
    }
    impl From<AmqpAnnotationKey> for fe2o3_amqp_types::messaging::annotations::OwnedKey {
        fn from(key: AmqpAnnotationKey) -> Self;
    }
    impl From<AmqpSymbol> for AmqpAnnotationKey {
        fn from(symbol: AmqpSymbol) -> Self;
    }
    impl From<AmqpValue> for AmqpAnnotationKey {
        fn from(value: AmqpValue) -> Self;
    }
    impl From<u64> for AmqpAnnotationKey {
        fn from(ulong: u64) -> Self;
    }
    impl PartialEq<&str> for AmqpAnnotationKey {
        fn eq(&self, other: &&str) -> bool;
    }
    impl PartialEq<String> for AmqpAnnotationKey {
        fn eq(&self, other: &String) -> bool;
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub enum AmqpMessageBody {
        Binary(Vec<Vec<u8>>),
        Sequence(Vec<crate::value::AmqpList>),
        Value(crate::value::AmqpValue),
        #[default]
        Empty,
    }
    impl From<AmqpList> for AmqpMessageBody {
        fn from(list: AmqpList) -> Self;
    }
    impl From<AmqpValue> for AmqpMessageBody {
        fn from(value: AmqpValue) -> Self;
    }
    impl From<Vec<AmqpList>> for AmqpMessageBody {
        fn from(lists: Vec<AmqpList>) -> Self;
    }
    impl From<Vec<AmqpValue>> for AmqpMessageBody {
        fn from(values: Vec<AmqpValue>) -> Self;
    }
    impl From<Vec<Vec<u8>>> for AmqpMessageBody {
        fn from(body: Vec<Vec<u8>>) -> Self;
    }
    impl From<Vec<u8>> for AmqpMessageBody {
        fn from(body: Vec<u8>) -> Self;
    }
    #[derive(Clone, Debug, PartialEq)]
    pub enum AmqpMessageId {
        String(String),
        Uuid(azure_core::Uuid),
        Binary(Vec<u8>),
        Ulong(u64),
    }
    impl AsRef<AmqpMessageId> for AmqpMessageId {
        fn as_ref(&self) -> &AmqpMessageId;
    }
    impl From<&AmqpMessageId> for fe2o3_amqp_types::messaging::MessageId {
        fn from(message_id: &AmqpMessageId) -> Self;
    }
    impl From<&MessageId> for crate::messaging::AmqpMessageId {
        fn from(message_id: &fe2o3_amqp_types::messaging::MessageId) -> Self;
    }
    impl From<&str> for AmqpMessageId {
        fn from(string: &str) -> Self;
    }
    impl From<AmqpMessageId> for crate::value::AmqpValue {
        fn from(message_id: AmqpMessageId) -> Self;
    }
    impl From<AmqpMessageId> for fe2o3_amqp_types::messaging::MessageId {
        fn from(message_id: AmqpMessageId) -> Self;
    }
    impl From<MessageId> for crate::messaging::AmqpMessageId {
        fn from(message_id: fe2o3_amqp_types::messaging::MessageId) -> Self;
    }
    impl From<String> for AmqpMessageId {
        fn from(string: String) -> Self;
    }
    impl From<Uuid> for AmqpMessageId {
        fn from(uuid: azure_core::Uuid) -> Self;
    }
    impl From<Vec<u8>> for AmqpMessageId {
        fn from(binary: Vec<u8>) -> Self;
    }
    impl From<u64> for AmqpMessageId {
        fn from(ulong: u64) -> Self;
    }
    #[derive(Clone, Debug, PartialEq)]
    pub enum AmqpOutcome {
        Accepted,
        Rejected,
        Released,
        Modified,
    }
    impl From<&AmqpSymbol> for AmqpOutcome {
        fn from(symbol: &AmqpSymbol) -> Self;
    }
    impl From<AmqpOutcome> for crate::value::AmqpSymbol {
        fn from(outcome: AmqpOutcome) -> Self;
    }
    impl From<AmqpOutcome> for fe2o3_amqp_types::messaging::Outcome {
        fn from(outcome: AmqpOutcome) -> Self;
    }
    impl From<Outcome> for crate::messaging::AmqpOutcome {
        fn from(outcome: fe2o3_amqp_types::messaging::Outcome) -> Self;
    }
    #[derive(Clone, Debug, PartialEq)]
    pub enum DistributionMode {
        Move,
        Copy,
    }
    impl From<&AmqpSymbol> for DistributionMode {
        fn from(symbol: &AmqpSymbol) -> Self;
    }
    impl From<DistributionMode> for crate::messaging::DistributionMode {
        fn from(distribution_mode: fe2o3_amqp_types::messaging::DistributionMode) -> Self;
    }
    impl From<DistributionMode> for crate::value::AmqpSymbol {
        fn from(mode: DistributionMode) -> Self;
    }
    impl From<DistributionMode> for fe2o3_amqp_types::messaging::DistributionMode {
        fn from(distribution_mode: crate::messaging::DistributionMode) -> Self;
    }
    #[derive(Clone, Debug, PartialEq)]
    pub enum TerminusDurability {
        None,
        Configuration,
        UnsettledState,
    }
    impl From<TerminusDurability> for crate::messaging::TerminusDurability {
        fn from(durability: fe2o3_amqp_types::messaging::TerminusDurability) -> Self;
    }
    impl From<TerminusDurability> for crate::value::AmqpSymbol {
        fn from(durability: TerminusDurability) -> Self;
    }
    impl From<TerminusDurability> for fe2o3_amqp_types::messaging::TerminusDurability {
        fn from(durability: crate::messaging::TerminusDurability) -> Self;
    }
    #[derive(Clone, Debug, PartialEq)]
    pub enum TerminusExpiryPolicy {
        LinkDetach,
        SessionEnd,
        ConnectionClose,
        Never,
    }
    impl From<&AmqpSymbol> for TerminusExpiryPolicy {
        fn from(symbol: &AmqpSymbol) -> Self;
    }
    impl From<TerminusExpiryPolicy> for crate::messaging::TerminusExpiryPolicy {
        fn from(expiry_policy: fe2o3_amqp_types::messaging::TerminusExpiryPolicy) -> Self;
    }
    impl From<TerminusExpiryPolicy> for crate::value::AmqpSymbol {
        fn from(policy: TerminusExpiryPolicy) -> Self;
    }
    impl From<TerminusExpiryPolicy> for fe2o3_amqp_types::messaging::TerminusExpiryPolicy {
        fn from(expiry_policy: TerminusExpiryPolicy) -> Self;
    }
}
```
