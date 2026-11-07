use core::{fmt, sync::atomic::Ordering::Relaxed};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snapshot {
    alloc_ops: usize,
    dealloc_ops: usize,
    realloc_ops: usize,
    leaked_ops: usize,

    allocated_bytes: usize,
    deallocated_bytes: usize,
    peak_bytes: usize,
    leaked_bytes: usize,
}

impl Snapshot {
    pub fn create() -> Self {
        let alloc_ops = crate::ALLOC_COUNTER.load(Relaxed);
        let dealloc_ops = crate::DEALLOC_COUNTER.load(Relaxed);
        let realloc_ops = crate::REALLOC_COUNTER.load(Relaxed);
        let leaked_ops = alloc_ops - dealloc_ops;

        let allocated_bytes = crate::ALLOCATED_TOTAL.load(Relaxed);
        let deallocated_bytes = crate::DEALLOCATED_TOTAL.load(Relaxed);
        let peak_bytes = crate::MAX_IN_RUNTIME.load(Relaxed);
        let leaked_bytes = crate::CURRENT_ALLOCATED.load(Relaxed);

        Self {
            alloc_ops,
            dealloc_ops,
            realloc_ops,
            leaked_ops,
            allocated_bytes,
            deallocated_bytes,
            peak_bytes,
            leaked_bytes,
        }
    }
}

impl fmt::Display for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
