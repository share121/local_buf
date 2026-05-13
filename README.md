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

## 性能

```bash
$ cargo test --features stats -- --test-threads=1 --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src\lib.rs (target\debug\deps\local_buf-b2d03a868ad6fe4c.exe)

running 8 tests
test tests::async_random::test_cache_hit_rate_async_random ... [异步随机(大小+频率)] 总操作=400000  分配=10689  命中=389311  命中率=97.33%
ok
test tests::async_sleep_frequent::test_cache_hit_rate_async_sleep_frequent ... [异步sleep(高频=每2次)] 总操作=400000  分配=2000  命中=398000  命中率=99.50%
ok
test tests::async_sleep_rare::test_cache_hit_rate_async_sleep_rare ... [异步sleep(低频=每20次)] 总操作=400000  分配=2000  命中=398000  命中率=99.50%
ok
test tests::cross_thread::test_cache_hit_rate_cross_thread ... [跨线程(每操作新线程×500)] 总操作=500  分配=500  命中=0  命中率=0.00%
ok
test tests::fixed_size::test_cache_hit_rate_fixed_size ... [固定大小] 总操作=100000  分配=4  命中=99996  命中率=100.00%
ok
test tests::mixed::test_cache_hit_rate_mixed ... [混合场景] 总操作=100000  分配=35  命中=99965  命中率=99.97%
ok
test tests::random_size::test_cache_hit_rate_random_size ... [随机大小] 总操作=100000  分配=37  命中=99963  命中率=99.96%
ok
test tests::wave::test_cache_hit_rate_wave ... [波浪大小] 总操作=100000  分配=508  命中=99492  命中率=99.49%
ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.95s

   Doc-tests local_buf

running 1 test
test src\lib.rs - (line 13) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s
```
