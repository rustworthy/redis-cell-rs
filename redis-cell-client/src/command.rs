use derive_builder::Builder;
use redis::Cmd as RedisCmd;
use std::time::Duration;

#[derive(Clone, Debug, Builder)]
#[builder(
    pattern = "mutable",
    derive(Debug),
    setter(into),
    build_fn(name = "try_build", private)
)]
pub struct Policy {
    #[builder(default = "15")]
    pub burst: usize,

    #[builder(default = "30")]
    pub tokens: usize,

    #[builder(default = "Duration::from_secs(60)")]
    pub period: Duration,

    #[builder(default = "1")]
    pub apply: usize,
}

impl Policy {
    pub fn builder() -> PolicyBuilder {
        PolicyBuilder::create_empty()
    }
}

impl PolicyBuilder {
    pub fn build(&mut self) -> Policy {
        self.try_build().expect("all set")
    }
}

pub struct Cmd<'a> {
    key: &'a str,
    policy: &'a Policy,
}

impl<'a> Cmd<'a> {
    pub fn new(key: &'a str, policy: &'a Policy) -> Self {
        Cmd { key, policy }
    }
}

impl<'a> From<Cmd<'a>> for RedisCmd {
    fn from(Cmd { key, policy }: Cmd<'a>) -> Self {
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
