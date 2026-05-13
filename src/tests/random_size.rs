#![cfg(feature = "stats")]

use crate::tests::*;
use crate::LocalBuf;
use rand::{thread_rng, Rng};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::Barrier;

/// 随机请求 32~4096B 的 buffer，模拟真实负载下不同大小的请求模式
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_cache_hit_rate_random_size() {
    let (start_a, start_h) = LocalBuf::stats();
    let total_ops = Arc::new(AtomicUsize::new(0));
    let barrier = Arc::new(Barrier::new(TASK_COUNT + 1));
    let handles: Vec<_> = (0..TASK_COUNT)
        .map(|_| {
            let total_ops = total_ops.clone();
            let barrier = barrier.clone();
            tokio::spawn(async move {
                barrier.wait().await;
                let mut rng = thread_rng();
                for _ in 0..OPS_PER_TASK {
                    let cap = rng.gen_range(32..4096);
                    let _buf = LocalBuf::with_capacity(cap);
                    total_ops.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    barrier.wait().await;
    for h in handles {
        h.await.unwrap();
    }
    let (end_a, end_h) = LocalBuf::stats();
    report(
        "随机大小",
        end_a - start_a,
        end_h - start_h,
        total_ops.load(Ordering::Relaxed),
    );
}
