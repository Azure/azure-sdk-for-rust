# Examples

## Authenticate the client

- [Authenticate client using connection string](./auth_with_connection_string.rs)
- [Authenticate the client using Azure named key credential](auth_with_named_key.rs)

## Send and receive

- [Send and receive using a queue](./send_and_receive_using_queue.rs)
- [Send and receive using a topic and subscription](./send_and_receive_using_topic_and_subscription.rs)
- [Send and receive batch of messages](./send_and_receive_message_batch.rs)
- [Peeking a message](./peek_a_message.rs)
- [Schedule a message](./schedule_a_message.rs)
- [Cancel a scheduled message](./cancel_a_scheduled_message.rs)
- [Setting time to live on a message](./setting_ttl.rs)

## Message settlement

- [Complete a message](./complete_a_message.rs)
- [Abandon a message](./abandon_a_message.rs)
- [Defer a message](./defer_a_message.rs)
- [Deadletter a message](./deadletter_a_message.rs)

## Send and receive using sessions

- [Send and receive session messages](./send_and_receive_session_messages.rs)
- [Receive from next available session](./receive_from_next_available_session.rs)

## Managing rules

- [Managing rules](./manage_rules.rs)

## Configuring the transport

- [AMQP over WebSocket](./amqp_over_websocket.rs)
