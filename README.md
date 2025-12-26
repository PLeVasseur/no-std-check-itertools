# no-std-check-itertools

Check that we can build itertools with default-features set to false in a `#![no_std]` context.

## Build

```shell
rustup target add thumbv7m-none-eabi
cargo build --target thumbv7m-none-eabi
```
