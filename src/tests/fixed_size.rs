#![cfg(feature = "stats")]

use crate::tests::*;
use crate::LocalBuf;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::Barrier;

/// 所有任务请求相同的 capacity，由于每个线程的缓存会被重复命中，命中率应接近 100%
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_cache_hit_rate_fixed_size() {
    let (start_a, start_h) = LocalBuf::stats();
    let total_ops = Arc::new(AtomicUsize::new(0));
    let barrier = Arc::new(Barrier::new(TASK_COUNT + 1));
    let handles: Vec<_> = (0..TASK_COUNT)
        .map(|_| {
            let total_ops = total_ops.clone();
            let barrier = barrier.clone();
            tokio::spawn(async move {
                barrier.wait().await;
                for _ in 0..OPS_PER_TASK {
                    let _buf = LocalBuf::with_capacity(1024);
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
        "固定大小",
        end_a - start_a,
        end_h - start_h,
        total_ops.load(Ordering::Relaxed),
    );
}
