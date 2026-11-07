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
        writeln!(f, "\n\x1b[1mAllocation Ops\x1b[0m")?;

        let alloc_ops = self.alloc_ops;
        let dealloc_ops = self.dealloc_ops;
        let realloc_ops = self.realloc_ops;
        let leaked_ops = self.leaked_ops;
        writeln!(
            f,
            "Alloc: {alloc_ops}   DeAlloc: {dealloc_ops}   ReAlloc: {realloc_ops}   Leaked: {leaked_ops}\n"
        )?;

        writeln!(f, "\x1b[1mIn Heap\x1b[0m")?;
        let allocated_bytes = self.allocated_bytes;
        let deallocated_bytes = self.deallocated_bytes;
        let peak_bytes = self.peak_bytes;
        let leaked_bytes = self.leaked_bytes;
        writeln!(
            f,
            "Allocated: {allocated_bytes}   DeAllocated: {deallocated_bytes}   Peak: {peak_bytes}   Leaked: {leaked_bytes}"
        )?;

        Ok(())
    }
}
