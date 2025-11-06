use redis::{ErrorKind, FromRedisValue, RedisError, RedisResult, Value as RedisValue};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct AllowedDetails {
    pub total: usize,
    pub remaining: usize,
    pub reset_after: u64,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct BlockedDetails {
    pub total: usize,
    pub remaining: usize,
    pub reset_after: u64,
    pub retry_after: u64,
}

#[derive(Debug, Clone)]
pub enum Verdict {
    Allowed(AllowedDetails),
    Blocked(BlockedDetails),
}

impl Verdict {
    pub fn try_from_redis_value(value: &RedisValue) -> RedisResult<Self> {
        let value = value.as_sequence().ok_or_else(|| {
            let detail = format!(
                "failed to decode Redis Cell response: exapected sequence, but got {:?}",
                value
            );
            (
                ErrorKind::ResponseError,
                "invalid Redis Cell response",
                detail,
            )
        })?;

        if value.len() != 5 {
            let detail = format!(
                "failed to decode Redis Cell response: exapected sequence of 5 elements, but got {:?}",
                value
            );
            let error = (
                ErrorKind::ResponseError,
                "invalid Redis Cell response",
                detail,
            )
                .into();
            return Err(error);
        }

        let (throttled, total, remaining, retry_after, reset_after) =
            (&value[0], &value[1], &value[2], &value[3], &value[4]);

        let verdict = if parse_throttled(throttled).map_to_redis_err()? {
            Verdict::Blocked(BlockedDetails {
                total: try_to_usize("total", total).map_to_redis_err()?,
                remaining: try_to_usize("remaining", remaining).map_to_redis_err()?,
                retry_after: try_to_u64("retry_after", retry_after).map_to_redis_err()?,
                reset_after: try_to_u64("reset_after", reset_after).map_to_redis_err()?,
            })
        } else {
            Verdict::Allowed(AllowedDetails {
                total: try_to_usize("total", total).map_to_redis_err()?,
                remaining: try_to_usize("remaining", remaining).map_to_redis_err()?,
                reset_after: try_to_u64("reset_after", reset_after).map_to_redis_err()?,
            })
        };
        Ok(verdict)
    }
}

impl FromRedisValue for Verdict {
    fn from_redis_value(v: &RedisValue) -> redis::RedisResult<Self> {
        Verdict::try_from_redis_value(v)
    }
}

fn parse_throttled(value: &RedisValue) -> Result<bool, String> {
    let value = try_to_int("throttled", value)?;
    match value {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(format!(
            "failed to parse value for throttled (blocked), expected 0 or 1, but got {}",
            other
        )),
    }
}

fn try_to_usize(field: &str, value: &RedisValue) -> Result<usize, String> {
    let value = try_to_int(field, value)?;
    usize::try_from(value).map_err(|_| {
        format!(
            "failed to parse {} as usize: tried to convert {}",
            field, value
        )
    })
}

fn try_to_u64(field: &str, value: &RedisValue) -> Result<u64, String> {
    let value = try_to_int(field, value)?;
    u64::try_from(value).map_err(|_| {
        format!(
            "failed to parse {} as u64: tried to convert {}",
            field, value
        )
    })
}

fn try_to_int(field: &str, value: &RedisValue) -> Result<i64, String> {
    match value {
        RedisValue::Int(value) => Ok(*value),
        _ => Err(format!(
            "failed to parse {}: expected integer, but got {:?}",
            field, value
        )),
    }
}

trait MapToRedisError<T> {
    fn map_to_redis_err(self) -> Result<T, RedisError>;
}

impl<T> MapToRedisError<T> for Result<T, String> {
    fn map_to_redis_err(self) -> Result<T, RedisError> {
        self.map_err(|detail| {
            (
                ErrorKind::ResponseError,
                "invalid Redis Cell response",
                detail,
            )
                .into()
        })
    }
}
