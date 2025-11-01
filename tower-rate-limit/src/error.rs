use redis::RedisError;
use redis_cell_rs::BlockedDetails;
use std::{borrow::Cow, sync::Arc};

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ExtractKeyError {
    pub detail: Option<Cow<'static, str>>,
}

impl ExtractKeyError {
    pub fn with_detail(detail: Cow<'static, str>) -> Self {
        ExtractKeyError {
            detail: Some(detail),
        }
    }
}

impl From<String> for ExtractKeyError {
    fn from(value: String) -> Self {
        ExtractKeyError::with_detail(value.into())
    }
}

impl From<&'static str> for ExtractKeyError {
    fn from(value: &'static str) -> Self {
        ExtractKeyError::with_detail(value.into())
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Error {
    Extract(ExtractKeyError),
    Throttle(BlockedDetails),
    Redis(Arc<RedisError>),
    Protocol(String),
}
