use redis::aio::ConnectionManager;
use redis_cell_client::{Cmd, Policy};
use std::borrow::Cow;

pub trait ExtractKey {
    type Error;
    type Request;

    fn extract<'a>(&self, req: &'a Self::Request) -> Result<Cow<'a, str>, Self::Error>;
}

#[derive(Clone)]
pub struct RateLimitConfig<Ex> {
    extractor: Ex,
    policy: Policy,
}

#[derive(Clone)]
pub struct RateLimit<S, Ex> {
    inner: S,
    config: RateLimitConfig<Ex>,
    connection: ConnectionManager,
}

impl<S, Ex> RateLimit<S, Ex> {
    pub fn new(inner: S, config: RateLimitConfig<Ex>, connection: ConnectionManager) -> Self {
        RateLimit {
            inner,
            config,
            connection,
        }
    }
}

impl<S, Ex, ReqTy> tower::Service<ReqTy> for RateLimit<S, Ex>
where
    S: tower::Service<ReqTy>,
    Ex: ExtractKey<Request = ReqTy>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: ReqTy) -> Self::Future {
        // XXX: into response please
        let Ok(key) = self.config.extractor.extract(&req) else {
            // not using unwrap to postpone imposing bounds (Debug in this case);
            todo!();
        };
        let cmd = Cmd::new(&key, &self.config.policy);
        self.connection.send_packed_command(&cmd.into());
        self.inner.call(req)
    }
}
