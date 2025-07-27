use anyhow::Result;
use axum::{
    body::Body,
    http::{header, Response},
};
use bytes::Bytes;
use futures_util::Stream;

pub struct StreamResponse {
    pub stream: Box<dyn Stream<Item = Result<Bytes>> + Send + Unpin>,
    pub content_type: String,
    pub content_length: Option<u64>,
}

/// Creates a streaming response with proper headers
pub fn stream_response(response: StreamResponse) -> Result<Response<Body>> {
    let StreamResponse {
        stream,
        content_type,
        content_length,
    } = response;
    let mut builder = Response::builder();

    builder = builder
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CACHE_CONTROL, "no-cache")
        .header(header::X_CONTENT_TYPE_OPTIONS, "nosniff");

    if let Some(length) = content_length {
        builder = builder.header(header::CONTENT_LENGTH, length);
    } else {
        builder = builder.header(header::TRANSFER_ENCODING, "chunked");
    }

    Ok(builder.body(Body::from_stream(stream))?)
}
