# redis-cell-rs

## Description

This crate provides Rust bindings for the [Redis Cell](https://github.com/brandur/redis-cell)
module.

Yon can defined a rate-limiting policy:

```rust
const BASIC_POLICY: Policy = Policy::from_tokens_per_second(1);

```

## Development & Contributing

Please find utility commands in [`Makefile`](./Makefile).
