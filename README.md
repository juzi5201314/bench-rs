# bench-rs

![Rust](https://github.com/juzi5201314/bench-rs/workflows/Rust/badge.svg)
[![docs.rs](https://docs.rs/bench-rs/badge.svg)](https://docs.rs/bench-rs)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/juzi5201314/bench-rs)
![Downloads](https://img.shields.io/crates/d/bench-rs)
![GitHub](https://img.shields.io/github/license/juzi5201314/bench-rs)
[![Crates.io](https://img.shields.io/crates/v/bench-rs)](https://crates.io/crates/bench-rs)

A benchmark library.

- [x] Stable rust (no black_box)
- [x] Beautiful output
- [x] Async support
- [x] Custom async runtime
- [x] Memory usage
- [x] Custom formatting
- [ ] Intuitive numerical units
- [x] Support custom memory allocator

### Examples
```
$ cargo test --release --color=always -q --package bench-rs --test bench --no-fail-fast -- --test-threads=1 --nocapture
```
Look `./tests/bench.rs`

![image.png](https://i.loli.net/2020/09/23/RsCfvr4OIVyj9Lc.png)

### black_box
I don't know how to implement the black box.

Please use [core::hint::black_box](https://doc.rust-lang.org/core/hint/fn.black_box.html). (unstable)

If you have a better idea, welcome to submit a pull request or open an issue

### global_allocator
In order to detect Memory usage, `bench-rs` modified global_allocator.
This will make it impossible to use other allocators.
If you need to use other allocator: 


Change your allocator: (if your allocator is `std::alloc::System`)
```
use std::alloc::System;
use bench_rs::{TrackAllocator, new_allocator};

#[global_allocator]
pub static GLOBAL: TrackAllocator<System> = new_allocator!(System);
```

Turn off default features:
```
[dependencies]
bench-rs = { version = "*", default-features = false }
```

Import the allocator named `GLOBAL`:
```
use bench_rs::bench;
use yourlib::GLOBAL;

#[bench]
fn ...
```
If your allocator name is not `GLOBAL`, please set its alias to `GLOBAL`:

`use yourlib::OtherAllocator as GLOBAL`

Because the bencher-macro will look for `GLOBAL.counter()` and `GLOBAL.peak()` 
at the same level of the bench function.

---

> I am a rust beginner, please correct me if the code is bad. Thank you
>
> Contributions welcome