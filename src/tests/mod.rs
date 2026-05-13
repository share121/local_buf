pub const TASK_COUNT: usize = 500;
pub const OPS_PER_TASK: usize = 200;
pub const ASYNC_TASK_COUNT: usize = 2000;
pub const ASYNC_OPS_PER_TASK: usize = 200;

#[allow(clippy::cast_precision_loss)]
pub fn report(test_name: &str, allocs: usize, hits: usize, total_ops: usize) {
    let total = allocs + hits;
    let hit_rate = if total > 0 {
        hits as f64 / total as f64 * 100.0
    } else {
        0.0
    };
    println!("[{test_name}] 总操作={total_ops}  分配={allocs}  命中={hits}  命中率={hit_rate:.2}%");
}

mod async_random;
mod async_sleep_frequent;
mod async_sleep_rare;
mod cross_thread;
mod fixed_size;
mod mixed;
mod random_size;
mod wave;
