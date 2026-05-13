#![doc = include_str!("../README.md")]

#[cfg(test)]
mod tests;

use bytes::BytesMut;
use std::cell::Cell;

mod local_buf {
    use super::{BytesMut, Cell};

    thread_local! {
        static POOL: Cell<Option<BytesMut>> = const { Cell::new(None) };
    }

    #[cfg(feature = "stats")]
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[cfg(feature = "stats")]
    static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[cfg(feature = "stats")]
    static HIT_COUNT: AtomicUsize = AtomicUsize::new(0);

    pub fn clear_buffer() -> Option<BytesMut> {
        POOL.with(Cell::take)
    }

    pub fn take_buffer(capacity: usize) -> BytesMut {
        POOL.with(|cell| {
            if let Some(mut buf) = cell.take() {
                if buf.capacity() >= capacity {
                    buf.clear();
                    #[cfg(feature = "stats")]
                    HIT_COUNT.fetch_add(1, Ordering::Relaxed);
                    return buf;
                }
                cell.set(Some(buf));
            }
            #[cfg(feature = "stats")]
            ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
            #[cfg(feature = "tracing")]
            tracing::debug!("current thread LocalBuf is None, allocate {capacity}B");
            BytesMut::with_capacity(capacity)
        })
    }

    pub fn return_buffer(buf: BytesMut) {
        POOL.with(|cell| {
            let mut slot = cell.take();
            if slot.as_ref().map_or(0, BytesMut::capacity) < buf.capacity() {
                slot = Some(buf);
            }
            cell.set(slot);
        });
    }

    #[cfg(feature = "stats")]
    pub fn stats() -> (usize, usize) {
        (
            ALLOC_COUNT.load(Ordering::Relaxed),
            HIT_COUNT.load(Ordering::Relaxed),
        )
    }

    #[cfg(feature = "stats")]
    pub fn reset_stats() {
        ALLOC_COUNT.store(0, Ordering::Relaxed);
        HIT_COUNT.store(0, Ordering::Relaxed);
    }
}

/// 支持多线程异步的线程本地缓冲区
pub struct LocalBuf {
    inner: Option<BytesMut>,
}

impl LocalBuf {
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let buf = local_buf::take_buffer(capacity);
        Self { inner: Some(buf) }
    }

    #[must_use]
    pub const fn from_bytes(buf: BytesMut) -> Self {
        Self { inner: Some(buf) }
    }

    /// 调用 `into_bytes` 后，你需要调用 `from_bytes` 手动放回缓冲区，否则缓冲区就会被正常释放，无法复用
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn into_bytes(mut self) -> BytesMut {
        self.inner.take().expect("LocalBuf 已被意外消费")
    }

    /// 清理当前线程的缓冲区
    #[allow(clippy::must_use_candidate)]
    pub fn clear_buffer() -> Option<BytesMut> {
        local_buf::clear_buffer()
    }

    /// 返回全局分配统计：`(分配次数, 缓存命中次数)`
    #[cfg(feature = "stats")]
    #[must_use]
    pub fn stats() -> (usize, usize) {
        local_buf::stats()
    }

    /// 重置全局分配统计
    #[cfg(feature = "stats")]
    pub fn reset_stats() {
        local_buf::reset_stats();
    }
}

impl std::ops::Deref for LocalBuf {
    type Target = BytesMut;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().expect("LocalBuf 已被意外消费")
    }
}

impl std::ops::DerefMut for LocalBuf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().expect("LocalBuf 已被意外消费")
    }
}

impl Drop for LocalBuf {
    fn drop(&mut self) {
        if let Some(buf) = self.inner.take() {
            local_buf::return_buffer(buf);
        }
    }
}

impl From<BytesMut> for LocalBuf {
    fn from(value: BytesMut) -> Self {
        Self::from_bytes(value)
    }
}
