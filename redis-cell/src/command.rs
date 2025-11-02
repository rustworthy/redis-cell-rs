use redis::{Cmd as RedisCmd, ToRedisArgs};
use std::time::Duration;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Policy {
    pub burst: usize,

    pub tokens: usize,

    pub period: Duration,

    pub apply: usize,
}

impl Policy {
    pub const fn new(burst: usize, tokens: usize, period: Duration, apply: usize) -> Policy {
        Self {
            burst,
            tokens,
            period,
            apply,
        }
    }
}

pub struct Cmd<'a, K> {
    key: K,
    policy: &'a Policy,
}

impl<'a, K> Cmd<'a, K> {
    pub fn new(key: K, policy: &'a Policy) -> Self {
        Cmd { key, policy }
    }
}

impl<'a, K> From<Cmd<'a, K>> for RedisCmd
where
    K: ToRedisArgs,
{
    fn from(Cmd { key, policy }: Cmd<'a, K>) -> Self {
        let mut cmd = RedisCmd::new();
        cmd.arg("CL.THROTTLE")
            .arg(key)
            .arg(policy.burst)
            .arg(policy.tokens)
            .arg(policy.period.as_secs())
            .arg(policy.apply);
        cmd
    }
}
