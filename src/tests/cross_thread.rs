#![cfg(feature = "stats")]

use crate::tests::*;
use crate::LocalBuf;
use std::thread;

/// 跨线程场景：通过 `std::thread::spawn` + `Handle::block_on` 让每次操作
/// 都运行在全新的 OS 线程上，thread-local 缓存完全失效 → 命中率 0%
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_cache_hit_rate_cross_thread() {
    const TOTAL_OPS: usize = 500;
    let (start_a, start_h) = LocalBuf::stats();
    let handle = tokio::runtime::Handle::current();
    for _ in 0..TOTAL_OPS {
        let handle = handle.clone();
        thread::spawn(move || {
            handle.block_on(async {
                let _buf = LocalBuf::with_capacity(1024);
            });
        })
        .join()
        .unwrap();
    }
    let (end_a, end_h) = LocalBuf::stats();
    report(
        "跨线程(每操作新线程×500)",
        end_a - start_a,
        end_h - start_h,
        TOTAL_OPS,
    );
}
