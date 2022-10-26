/// <summary>
/// The set of options that can be specified when creating a <see cref="ServiceBusSender"/>
/// to configure its behavior.
/// </summary>
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceBusSenderOptions {
    /// <summary>
    /// A property used to set the <see cref="ServiceBusSender"/> ID to identify the client. This can be used to correlate logs
    /// and exceptions. If <c>null</c> or empty, a random unique value will be used.
    /// </summary>
    pub identifier: String,
}
