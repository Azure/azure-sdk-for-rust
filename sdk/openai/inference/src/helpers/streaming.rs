use azure_core::{Error, Result};
use futures::{Stream, StreamExt};

pub trait EventStreamer<T>
where
    T: serde::de::DeserializeOwned,
{
    fn delimiter(&self) -> impl AsRef<str>;

    fn event_stream(
        &self,
        response_body: azure_core::ResponseBody,
    ) -> impl Stream<Item = Result<T>>;
}

pub(crate) fn string_chunks<'a>(
    response_body: (impl Stream<Item = Result<bytes::Bytes>> + Unpin + 'a),
    stream_event_delimiter: &'a str, // figure out how to use it in the move
) -> impl Stream<Item = Result<String>> + 'a {
    let chunk_buffer = Vec::new();
    let stream = futures::stream::unfold(
        (response_body, chunk_buffer),
        move |(mut response_body, mut chunk_buffer)| async move {
            let delimiter = stream_event_delimiter.as_bytes();
            let delimiter_len = delimiter.len();

            if let Some(Ok(bytes)) = response_body.next().await {
                chunk_buffer.extend_from_slice(&bytes);
                // Looking for the next occurence of the event delimiter
                // it's + 4 because the \n\n are escaped and represented as [92, 110, 92, 110]
                if let Some(pos) = chunk_buffer
                    .windows(delimiter_len)
                    .position(|window| window == delimiter)
                {
                    // the range must include the delimiter bytes
                    let mut bytes = chunk_buffer
                        .drain(..pos + delimiter_len)
                        .collect::<Vec<_>>();
                    bytes.truncate(bytes.len() - delimiter_len);

                    return if let Ok(yielded_value) = std::str::from_utf8(&bytes) {
                        // We strip the "data: " portion of the event. The rest is always JSON and will be deserialized
                        // by a subsquent mapping function for this stream
                        let yielded_value = yielded_value.trim_start_matches("data:").trim();
                        if yielded_value == "[DONE]" {
                            return None;
                        } else {
                            Some((Ok(yielded_value.to_string()), (response_body, chunk_buffer)))
                        }
                    } else {
                        None
                    };
                }
                if chunk_buffer.len() > 0 {
                    return Some((
                        Err(Error::with_message(
                            azure_core::error::ErrorKind::DataConversion,
                            || "Incomplete chunk",
                        )),
                        (response_body, chunk_buffer),
                    ));
                }
            // We drain the buffer of any messages that may be left over.
            // The block above will be skipped, since response_body.next() will be None every time
            } else if !chunk_buffer.is_empty() {
                // we need to verify if there are any event left in the buffer and emit them individually
                // it's + 4 because the \n\n are escaped and represented as [92, 110, 92, 110]
                if let Some(pos) = chunk_buffer
                    .windows(delimiter_len)
                    .position(|window| window == delimiter)
                {
                    // the range must include the delimiter bytes
                    let mut bytes = chunk_buffer
                        .drain(..pos + delimiter_len)
                        .collect::<Vec<_>>();
                    bytes.truncate(bytes.len() - delimiter_len);

                    return if let Ok(yielded_value) = std::str::from_utf8(&bytes) {
                        let yielded_value = yielded_value.trim_start_matches("data:").trim();
                        if yielded_value == "[DONE]" {
                            return None;
                        } else {
                            Some((Ok(yielded_value.to_string()), (response_body, chunk_buffer)))
                        }
                    } else {
                        None
                    };
                }
                // if we get to this point, it means we have drained the buffer of all events, meaning that we haven't been able to find the next delimiter
            }
            None
        },
    );

    // We filter errors, we should specifically target the error type yielded when we are not able to find an event in a chunk
    // Specifically the Error::with_messagge(ErrorKind::DataConversion, || "Incomplete chunk")
    return stream.filter(|it| std::future::ready(it.is_ok()));
}
