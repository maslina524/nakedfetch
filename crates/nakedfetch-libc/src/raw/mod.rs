#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[macro_export]
macro_rules! syscall {
    ($nr:expr) => {{
        #[allow(
            trivial_numeric_casts,
            reason = "When passing usize, a useless warning occurs"
        )]
        $crate::raw::syscall0($nr as usize)
    }};
    ($nr:expr, $a1:expr) => {{
        #[allow(
            trivial_numeric_casts,
            reason = "When passing usize, a useless warning occurs"
        )]
        $crate::raw::syscall1($nr as usize, $a1 as usize)
    }};
    ($nr:expr, $a1:expr, $a2:expr) => {{
        #[allow(
            trivial_numeric_casts,
            reason = "When passing usize, a useless warning occurs"
        )]
        $crate::raw::syscall2($nr as usize, $a1 as usize, $a2 as usize)
    }};
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => {{
        #[allow(
            trivial_numeric_casts,
            reason = "When passing usize, a useless warning occurs"
        )]
        $crate::raw::syscall3($nr as usize, $a1 as usize, $a2 as usize, $a3 as usize)
    }};
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {{
        #[allow(
            trivial_numeric_casts,
            reason = "When passing usize, a useless warning occurs"
        )]
        $crate::raw::syscall4(
            $nr as usize,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
        )
    }};
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {{
        #[allow(
            trivial_numeric_casts,
            reason = "When passing usize, a useless warning occurs"
        )]
        $crate::raw::syscall5(
            $nr as usize,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
            $a5 as usize,
        )
    }};
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        #[allow(
            trivial_numeric_casts,
            reason = "When passing usize, a useless warning occurs"
        )]
        $crate::raw::syscall6(
            $nr as usize,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
            $a5 as usize,
            $a6 as usize,
        )
    }};
}
