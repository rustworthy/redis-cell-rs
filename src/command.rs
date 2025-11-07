use crate::key::Key;
use crate::policy::Policy;
use redis::Cmd as RedisCmd;

pub struct Cmd<'a> {
    key: &'a Key<'a>,
    policy: &'a Policy,
}

impl<'a> Cmd<'a> {
    pub fn new(key: &'a Key<'a>, policy: &'a Policy) -> Self {
        Cmd { key, policy }
    }
}

impl From<&Cmd<'_>> for RedisCmd {
    fn from(cmd: &Cmd<'_>) -> Self {
        let mut redis_cmd = RedisCmd::new();
        redis_cmd
            .arg("CL.THROTTLE")
            .arg(cmd.key)
            .arg(cmd.policy.burst)
            .arg(cmd.policy.tokens)
            .arg(cmd.policy.period.as_secs())
            .arg(cmd.policy.apply);
        redis_cmd
    }
}

impl From<Cmd<'_>> for RedisCmd {
    fn from(cmd: Cmd<'_>) -> Self {
        RedisCmd::from(&cmd)
    }
}
