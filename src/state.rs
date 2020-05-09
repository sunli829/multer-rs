use crate::buffer::StreamBuffer;
use futures::{lock::Mutex, stream::Stream};
use std::sync::Arc;
use std::task::Waker;

pub(crate) struct MultipartState<S> {
    pub(crate) buffer: StreamBuffer<S>,
    pub(crate) boundary: String,
    pub(crate) stage: StreamingStage,
    pub(crate) is_prev_field_consumed: bool,
    pub(crate) next_field_waker: Option<Waker>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StreamingStage {
    ReadingBoundary,
    ReadingFieldHeaders,
    ReadingFieldData,
    Eof,
}