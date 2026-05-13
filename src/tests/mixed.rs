#![cfg(feature = "stats")]

use crate::tests::*;
use crate::LocalBuf;
use rand::{thread_rng, Rng};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::Barrier;

/// 综合以上所有模式，再加一些极端值（超大 buffer、0 容量），
/// 全面评估缓存在各场景下的表现
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_cache_hit_rate_mixed() {
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
                let wave: Vec<usize> = (32..4096)
                    .step_by(32)
                    .chain((32..4096).step_by(32).rev())
                    .collect();
                for (i, _) in (0..OPS_PER_TASK).enumerate() {
                    let cap = match i % 5 {
                        0 => 1024,
                        1 => rng.gen_range(32..4096),
                        2 => wave[i % wave.len()],
                        3 => rng.gen_range(4096..65536),
                        4 => 0,
                        _ => unreachable!(),
                    };
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
        "混合场景",
        end_a - start_a,
        end_h - start_h,
        total_ops.load(Ordering::Relaxed),
    );
}
