# `local_buf`

[![GitHub last commit](https://img.shields.io/github/last-commit/share121/local_buf/master)](https://github.com/share121/local_buf/commits/master)
[![Test](https://github.com/share121/local_buf/workflows/Test/badge.svg)](https://github.com/share121/local_buf/actions)
[![Latest version](https://img.shields.io/crates/v/local_buf.svg)](https://crates.io/crates/local_buf)
[![Documentation](https://docs.rs/local_buf/badge.svg)](https://docs.rs/local_buf)
[![License](https://img.shields.io/crates/l/local_buf.svg)](https://github.com/share121/local_buf/blob/master/LICENSE)

`local_buf` 是一个支持多线程异步的线程缓冲区。

## 示例

```rust
use local_buf::LocalBuf;

fn main() {
    let buf = LocalBuf::with_capacity(10);
}
```
