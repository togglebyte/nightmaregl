use std::io::Error as IoErr;
use std::string::FromUtf8Error;

use thiserror::Error;
use rusttype::gpu_cache::CacheWriteErr;
use png::{EncodingError, DecodingError};
use glutin::ContextError;

pub type Result<T> = std::result::Result<T, NightmareError>;

#[derive(Error, Debug)]
pub enum NightmareError {
    #[error(transparent)]
    Io(#[from] IoErr),

    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),

    #[error(transparent)]
    Decode(#[from] DecodingError),

    #[error(transparent)]
    Encode(#[from] EncodingError),

    #[error("Invalid colour type")]
    InvalidColorType,

    #[error(transparent)]
    FontCacheError(#[from] CacheWriteErr),

    #[error("Failed to load font")]
    FailedToLoadFont,

    #[error(transparent)]
    ContextError(#[from] ContextError),

    #[error("Shader failure")]
    Shader(String),

    #[error("Shader program failure")]
    ShaderProgram(String),
}
