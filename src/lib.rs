//!This crate provides Rust bindings for the [Redis Cell](https://github.com/brandur/redis-cell) module.
//!
//!You can define a rate-limiting [`Policy`] in quite a few ways:
//!```
//!use redis_cell_rs::Policy;
//!use std::time::Duration;
//!
//!const POLICY1: Policy = Policy::from_tokens_per_second(1);
//!
//!const POLICY2: Policy = Policy::from_tokens_per_minute(100);
//!
//!const POLICY3: Policy = Policy::from_tokens_per_hour(1_000);
//!
//!const POLICY4: Policy = Policy::from_tokens_per_day(5_000);
//!
//!const POLICY5: Policy = Policy::from_tokens_per_period(100, Duration::from_secs(100))
//! .max_burst(100)
//! .apply_tokens(2)
//! .name("general_policy");
//!
//!const POLICY6: Policy = Policy::new(
//! /* burst */  10,
//! /* tokens */ 100,
//! /* period */ Duration::from_secs(100),
//! /* apply */  1
//!);
//!```
//!
//!A policy (accompanied by [`Key`]) can now be used to crate rate-limiting
//!request ([`Cmd`]), which - in its turn - can be turned into a Redis command
//!and sent over to the server using a Redis client. The response can then be
//!converted into [`Verdict`].
//!
//!```no_run
//!# use redis_cell_rs::Policy;
//!# const POLICY1: Policy = Policy::from_tokens_per_second(1);
//!
//!use redis::{Cmd as RedisCmd, Client};
//!use redis_cell_rs::{Cmd, Key, Verdict, AllowedDetails, BlockedDetails};
//!
//!let key = Key::pair("user123", "/api/infer");
//!let cmd: RedisCmd = Cmd::new(&key, &POLICY1).into();
//!
//!let client = Client::open("redis://127.0.0.1/").unwrap();
//!let mut con = client.get_connection().unwrap();
//!
//!let verdict: Verdict = cmd.query(&mut con).unwrap();
//!match verdict {
//! Verdict::Allowed(details) => {
//!     let AllowedDetails {total, remaining, reset_after, .. } = details;
//!     println!("total={}, remaining={}, reset_after={}", total, remaining, reset_after);
//! },
//! Verdict::Blocked(details) => {
//!     let BlockedDetails {total, remaining, reset_after, retry_after, .. } = details;
//!     println!(
//!         "total={}, remaining={}, reset_after={}, retry_after={}",
//!         total,
//!         remaining,
//!         reset_after,
//!         retry_after,
//!     );
//! }
//!}
//!```
//!

// #![deny(missing_docs)]

mod command;
mod key;
mod policy;
mod verdict;

pub use command::Cmd;
pub use key::Key;
pub use policy::Policy;
pub use verdict::{AllowedDetails, BlockedDetails, Verdict};

#[cfg(test)]
mod tests {
    use crate::{Cmd, Policy, Verdict};
    use redis::Cmd as RedisCmd;
    use std::time::Duration;
    use testcontainers::core::IntoContainerPort as _;
    use testcontainers::runners::AsyncRunner;
    use testcontainers::{core::WaitFor, GenericImage};

    async fn it_works_with(image: &str) {
        let container = GenericImage::new(image, "latest")
            .with_exposed_port(6379.tcp())
            .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
            .start()
            .await
            .unwrap();
        let port = container.get_host_port_ipv4(6379).await.unwrap();
        let client = redis::Client::open(("localhost", port)).unwrap();
        let config = redis::aio::ConnectionManagerConfig::new().set_number_of_retries(1);
        let mut client = redis::aio::ConnectionManager::new_with_config(client, config)
            .await
            .unwrap();
        let policy = Policy::new(1, 10, Duration::from_secs(60), 1);
        let key = "user123".into();
        let cmd: RedisCmd = Cmd::new(&key, &policy).into();
        let verdict: Verdict = cmd.query_async(&mut client).await.unwrap();
        dbg!(verdict);
    }

    #[tokio::test]
    async fn it_works_with_redis() {
        it_works_with("redis-cell").await
    }

    #[tokio::test]
    async fn it_works_with_valkey() {
        it_works_with("valkey-cell").await
    }
}
