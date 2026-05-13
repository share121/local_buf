#![cfg(feature = "stats")]

use crate::tests::*;
use crate::LocalBuf;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::Barrier;

/// 逐步增大再减小 capacity，测试"保留最大容量"策略：先增到最大再降下来，
/// 始终有足够大的缓存可用，命中率应较高
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_cache_hit_rate_wave() {
    let (start_a, start_h) = LocalBuf::stats();
    let total_ops = Arc::new(AtomicUsize::new(0));
    let barrier = Arc::new(Barrier::new(TASK_COUNT + 1));
    let handles: Vec<_> = (0..TASK_COUNT)
        .map(|_| {
            let total_ops = total_ops.clone();
            let barrier = barrier.clone();
            tokio::spawn(async move {
                barrier.wait().await;
                let wave: Vec<usize> = (32..4096)
                    .step_by(32)
                    .chain((32..4096).step_by(32).rev())
                    .cycle()
                    .take(OPS_PER_TASK)
                    .collect();
                for &cap in &wave {
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
        "波浪大小",
        end_a - start_a,
        end_h - start_h,
        total_ops.load(Ordering::Relaxed),
    );
}
