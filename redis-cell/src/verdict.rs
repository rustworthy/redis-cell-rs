use crate::Error;
use redis::Value as RedisValue;

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
    pub retry_after: u64,
    pub reset_after: u64,
}

#[derive(Debug, Clone)]
pub enum Verdict {
    Allowed(AllowedDetails),
    Blocked(BlockedDetails),
}

impl TryFrom<RedisValue> for Verdict {
    type Error = Error;
    fn try_from(value: RedisValue) -> Result<Self, Self::Error> {
        let value = value.into_sequence().map_err(|value| {
            Error::from(format!(
                "failed to decode Redis Cell response: exapected sequence, but got {:?}",
                value
            ))
        })?;

        if value.len() != 5 {
            return Err(format!(
                "failed to decode Redis Cell response: exapected sequence of 5 elements, but got {:?}",
                value
            ).into());
        }

        let (throttled, total, remaining, retry_after, reset_after) =
            (&value[0], &value[1], &value[2], &value[3], &value[4]);

        let verdict = if parse_throttled(throttled)? {
            Verdict::Blocked(BlockedDetails {
                total: try_to_usize("total", total)?,
                remaining: try_to_usize("remaining", remaining)?,
                retry_after: try_to_u64("retry_after", retry_after)?,
                reset_after: try_to_u64("reset_after", reset_after)?,
            })
        } else {
            Verdict::Allowed(AllowedDetails {
                total: try_to_usize("total", total)?,
                remaining: try_to_usize("remaining", remaining)?,
                reset_after: try_to_u64("reset_after", reset_after)?,
            })
        };
        Ok(verdict)
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
