/// An error caused by a range not being 512-byte aligned.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Not512ByteAlignedError {
    #[error("start range not 512-byte aligned: {0}")]
    StartRange(u64),
    #[error("end range not 512-byte aligned: {0}")]
    EndRange(u64),
}

/// An error caused by a range not being 512-byte aligned or by a parse failure.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Parse512AlignedError {
    #[error("split not found")]
    SplitNotFound,
    #[error("parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("not 512 byte aligned error: {0}")]
    Not512ByteAlignedError(#[from] Not512ByteAlignedError),
}
