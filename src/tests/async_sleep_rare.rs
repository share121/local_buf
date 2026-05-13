#![cfg(feature = "stats")]

use crate::tests::*;
use crate::LocalBuf;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;
use tokio::sync::Barrier;

/// 异步场景：低频 sleep（每 20 次操作），线程迁移较少，命中率应较高
/// buffer 在 sleep 期间保持存活，迁移时随任务一起带走
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_cache_hit_rate_async_sleep_rare() {
    let (start_a, start_h) = LocalBuf::stats();
    let total_ops = Arc::new(AtomicUsize::new(0));
    let barrier = Arc::new(Barrier::new(ASYNC_TASK_COUNT + 1));
    let handles: Vec<_> = (0..ASYNC_TASK_COUNT)
        .map(|_| {
            let total_ops = total_ops.clone();
            let barrier = barrier.clone();
            tokio::spawn(async move {
                barrier.wait().await;
                for i in 0..ASYNC_OPS_PER_TASK {
                    let _buf = LocalBuf::with_capacity(1024);
                    if i % 20 == 0 {
                        for _ in 0..3 {
                            tokio::time::sleep(Duration::from_micros(1)).await;
                        }
                    }
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
        "异步sleep(低频=每20次)",
        end_a - start_a,
        end_h - start_h,
        total_ops.load(Ordering::Relaxed),
    );
}
