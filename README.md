# bench-rs

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

### Examples
```
$ cargo test --release --color=always -q --package bench-rs --test bench --no-fail-fast
```
Look `./tests/bench.rs`

![image.png](https://i.loli.net/2020/09/22/qlgLCjnESe7Q839.png)

### black_box
I don't know how to implement the black box.

Please use [core::hint::black_box](https://doc.rust-lang.org/core/hint/fn.black_box.html). (unstable)

If you have a better idea, welcome to submit a pull request or open an issue

> I am a rust beginner, please correct me if the code is bad. Thank you