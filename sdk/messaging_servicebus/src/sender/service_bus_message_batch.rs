/// A set of <see cref="ServiceBusMessage" /> with size constraints known up-front, intended to be
/// sent to the Queue/Topic as a single batch. A <see cref="ServiceBusMessageBatch"/> can be
/// created using <see
/// cref="ServiceBusSender.CreateMessageBatchAsync(System.Threading.CancellationToken)"/>.
/// Messages can be added to the batch using the <see cref="TryAddMessage"/> method on the batch.
pub struct ServiceBusMessageBatch {}
