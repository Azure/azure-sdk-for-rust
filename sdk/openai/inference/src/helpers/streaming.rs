// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
use azure_core::{Error, Result};
use futures::{Stream, StreamExt};

/// A trait used to designate a type into which the streams will be deserialized.
pub(crate) trait EventStreamer<T>
where
    T: serde::de::DeserializeOwned,
{
    fn event_stream(response_body: azure_core::ResponseBody) -> impl Stream<Item = Result<T>>;
}

/// A helper function to be used in streaming scenarios. The `response_body`, the input stream
/// is buffered until a `stream_event_delimiter` is found. This constitutes a single event.
/// These series of events are then returned as a stream.
///
/// # Arguments
/// * `response_body` - The response body stream of an HTTP request.
/// * `stream_event_delimiter` - The delimiter that separates events in the stream. In some cases `\n\n`, in other cases can be `\n\r\n\n`.
/// # Returns
/// The `response_body` stream segmented and streamed into String events demarcated by `stream_event_delimiter`.
pub(crate) fn string_chunks<'a>(
    response_body: (impl Stream<Item = Result<bytes::Bytes>> + Unpin + 'a),
    stream_event_delimiter: &'a str,
) -> impl Stream<Item = Result<String>> + 'a {
    let chunk_buffer = Vec::new();
    let stream = futures::stream::unfold(
        (response_body, chunk_buffer),
        move |(mut response_body, mut chunk_buffer)| async move {
            let delimiter = stream_event_delimiter.as_bytes();
            let delimiter_len = delimiter.len();

            if let Some(Ok(bytes)) = response_body.next().await {
                chunk_buffer.extend_from_slice(&bytes);
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
                if !chunk_buffer.is_empty() {
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

    // We specifically allow the Error::with_messagge(ErrorKind::DataConversion, || "Incomplete chunk")
    // So that we are able to continue pushing bytes to the buffer until we find the next delimiter
    return stream.filter(|it| {
        std::future::ready(
            it.is_ok()
                || it.as_ref().unwrap_err().to_string()
                    != Error::with_message(azure_core::error::ErrorKind::DataConversion, || {
                        "Incomplete chunk"
                    })
                    .to_string(),
        )
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::pin_mut;

    #[tokio::test]
    async fn clean_chunks() {
        let mut source_stream = futures::stream::iter(vec![
            Ok(bytes::Bytes::from_static(b"data: piece 1\n\n")),
            Ok(bytes::Bytes::from_static(b"data: piece 2\n\n")),
            Ok(bytes::Bytes::from_static(b"data: [DONE]\n\n")),
        ]);

        let actual = string_chunks(&mut source_stream, "\n\n");
        pin_mut!(actual);
        let actual: Vec<Result<String>> = actual.collect().await;

        let expected: Vec<Result<String>> =
            vec![Ok("piece 1".to_string()), Ok("piece 2".to_string())];
        assert_result_vectors(expected, actual);
    }

    #[tokio::test]
    async fn multiple_message_in_one_chunk() {
        let mut source_stream = futures::stream::iter(vec![
            Ok(bytes::Bytes::from_static(
                b"data: piece 1\n\ndata: piece 2\n\n",
            )),
            Ok(bytes::Bytes::from_static(
                b"data: piece 3\n\ndata: [DONE]\n\n",
            )),
        ]);

        let mut actual = Vec::new();

        let actual_stream = string_chunks(&mut source_stream, "\n\n");
        pin_mut!(actual_stream);

        while let Some(event) = actual_stream.next().await {
            actual.push(event);
        }

        let expected: Vec<Result<String>> = vec![
            Ok("piece 1".to_string()),
            Ok("piece 2".to_string()),
            Ok("piece 3".to_string()),
        ];
        assert_result_vectors(expected, actual);
    }

    #[tokio::test]
    async fn data_marker_in_previous_chunk() {
        let mut source_stream = futures::stream::iter(vec![
            Ok(bytes::Bytes::from_static(
                b"data: piece 1\n\ndata: piece 2\n\ndata:",
            )),
            Ok(bytes::Bytes::from_static(b" piece 3\n\ndata: [DONE]\n\n")),
        ]);

        let mut actual = Vec::new();

        let actual_stream = string_chunks(&mut source_stream, "\n\n");
        pin_mut!(actual_stream);

        while let Some(event) = actual_stream.next().await {
            actual.push(event);
        }

        let expected: Vec<Result<String>> = vec![
            Ok("piece 1".to_string()),
            Ok("piece 2".to_string()),
            Ok("piece 3".to_string()),
        ];
        assert_result_vectors(expected, actual);
    }

    #[tokio::test]
    async fn event_delimeter_split_across_chunks() {
        let mut source_stream = futures::stream::iter(vec![
            Ok(bytes::Bytes::from_static(b"data: piece 1\n")),
            Ok(bytes::Bytes::from_static(b"\ndata: [DONE]")),
        ]);

        let actual = string_chunks(&mut source_stream, "\n\n");
        pin_mut!(actual);
        let actual: Vec<Result<String>> = actual.collect().await;

        let expected: Vec<Result<String>> = vec![Ok("piece 1".to_string())];
        assert_result_vectors(expected, actual);
    }

    #[tokio::test]
    async fn event_delimiter_at_start_of_next_chunk() {
        let mut source_stream = futures::stream::iter(vec![
            Ok(bytes::Bytes::from_static(b"data: piece 1")),
            Ok(bytes::Bytes::from_static(b"\n\ndata: [DONE]")),
        ]);

        let actual = string_chunks(&mut source_stream, "\n\n");
        pin_mut!(actual);
        let actual: Vec<Result<String>> = actual.collect().await;

        let expected: Vec<Result<String>> = vec![Ok("piece 1".to_string())];
        assert_result_vectors(expected, actual);
    }

    // This is an over simplification, reasonable for an MVP. We should:
    //   1. propagate error upwards
    //   2. handle an unexpected "data:" marker (this will simply send the string as is, which will fail deserialization in an upper mapping layer)
    #[tokio::test]
    async fn error_in_response_ends_stream() {
        let mut source_stream = futures::stream::iter(vec![
            Ok(bytes::Bytes::from_static(b"data: piece 1\n\n")),
            Err(Error::with_message(
                azure_core::error::ErrorKind::Other,
                || "Incomplete chunk",
            )),
        ]);

        let actual = string_chunks(&mut source_stream, "\n\n");
        pin_mut!(actual);
        let actual: Vec<Result<String>> = actual.collect().await;

        let expected: Vec<Result<String>> = vec![Ok("piece 1".to_string())];
        assert_result_vectors(expected, actual);
    }

    fn assert_result_vectors<T>(expected: Vec<Result<T>>, actual: Vec<Result<T>>)
    where
        T: std::fmt::Debug + PartialEq,
    {
        assert_eq!(expected.len(), actual.len());
        for (expected, actual) in expected.iter().zip(actual.iter()) {
            if let Ok(actual) = actual {
                assert_eq!(actual, expected.as_ref().unwrap());
            } else {
                let actual_err = actual.as_ref().unwrap_err();
                let expected_err = expected.as_ref().unwrap_err();
                assert_eq!(actual_err.kind(), expected_err.kind());
                assert_eq!(actual_err.to_string(), expected_err.to_string());
            }
        }
    }
}
