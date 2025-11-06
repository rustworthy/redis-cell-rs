# redis-cell-rs

## Description

This crate provides Rust bindings for the [Redis Cell](https://github.com/brandur/redis-cell)
module.

You can define a rate-limiting `Policy` in quite a few ways:

```rust
use redis_cell_rs::Policy;
use std::time::Duration;

const POLICY1: Policy = Policy::from_tokens_per_second(1);

const POLICY2: Policy = Policy::from_tokens_per_minute(100);

const POLICY3: Policy = Policy::from_tokens_per_hour(1_000);

const POLICY4: Policy = Policy::from_tokens_per_day(5_000);

const POLICY5: Policy = Policy::from_tokens_per_period(100, Duration::from_secs(100))
 .max_burst(100)
 .apply_tokens(2)
 .name("general_policy");

const POLICY6: Policy = Policy::new(
 /* burst */  10,
 /* tokens */ 100,
 /* period */ Duration::from_secs(100),
 /* apply */  1
);
```

A policy can now be used to crate rate-limiting request (command), which - in
its turn - can be turned into a Redis command and sent over to the server using
a Redis client. The response can then be converted into `Verdict`.

```rust
use redis_cell_rs::Policy;
use redis::{Cmd as RedisCmd, Client};
use redis_cell_rs::{Cmd, Verdict, AllowedDetails, BlockedDetails};

const POLICY1: Policy = Policy::from_tokens_per_second(1);

let cmd: RedisCmd = Cmd::new("user123", &POLICY1).into();

let client = Client::open("redis://127.0.0.1/").unwrap();
let mut con = client.get_connection().unwrap();

let verdict: Verdict = cmd.query(&mut con).unwrap();
match verdict {
 Verdict::Allowed(details) => {
     let AllowedDetails {total, remaining, reset_after, .. } = details;
     println!("total={}, remaining={}, reset_after={}", total, remaining, reset_after);
 },
 Verdict::Blocked(details) => {
     let BlockedDetails {total, remaining, reset_after, retry_after, .. } = details;
     println!(
         "total={}, remaining={}, reset_after={}, retry_after={}",
         total,
         remaining,
         reset_after,
         retry_after,
     );
 }
}
```

## Development & Contributing

Please find utility commands in [`Makefile`](./Makefile).
