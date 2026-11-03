#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! get_fn {
    ($handle:tt, $name:expr, $typ:ident) => {{
        // SAFETY: Completely safe
        let addr = $crate::windows::link::GetProcAddress($handle, $name.as_ptr().cast())
            .unwrap_or_else(|| {
                unload($handle);
                $crate::abort!(concat!(stringify!($name), " not found in dll"));
            });
        core::mem::transmute::<ApiBaseFn, $typ>(addr)
    }};
}

#[cfg(target_family = "unix")]
#[macro_export]
macro_rules! get_fn {
    ($handle:tt, $name:expr, $typ:ident) => {{
        // SAFETY: Completely safe
        let addr = $crate::unix::libc::dlsym($handle, $name.as_ptr().cast());
        if addr.is_null() {
            unload($handle);
            $crate::abort!(concat!(stringify!($name), " not found in library"));
        }
        core::mem::transmute::<ApiBaseFn, $typ>(addr)
    }};
}

#[macro_export]
/// Copied from `cfg_if`
///
/// Source link: <https://docs.rs/cfg-if/latest/src/cfg_if/lib.rs.html#1-212>
macro_rules! cfg_if {
    (
        if #[cfg( $($i_meta:tt)+ )] { $( $i_tokens:tt )* }
        $(
            else if #[cfg( $($ei_meta:tt)+ )] { $( $ei_tokens:tt )* }
        )*
        $(
            else { $( $e_tokens:tt )* }
        )?
    ) => {
        $crate::cfg_if! {
            @__items () ;
            (( $($i_meta)+ ) ( $( $i_tokens )* )),
            $(
                (( $($ei_meta)+ ) ( $( $ei_tokens )* )),
            )*
            $(
                (() ( $( $e_tokens )* )),
            )?
        }
    };

    (@__items ( $( ($($_:tt)*) , )* ) ; ) => {};
    (
        @__items ( $( ($($no:tt)+) , )* ) ;
        (( $( $($yes:tt)+ )? ) ( $( $tokens:tt )* )),
        $( $rest:tt , )*
    ) => {
        #[cfg(all(
            $( $($yes)+ , )?
            not(any( $( $($no)+ ),* ))
        ))]

        $crate::cfg_if! { @__temp_group $( $tokens )* }

        $crate::cfg_if! {
            @__items ( $( ($($no)+) , )* $( ($($yes)+) , )? ) ;
            $( $rest , )*
        }
    };

    (@__temp_group $( $tokens:tt )* ) => {
        $( $tokens )*
    };
}

#[macro_export]
macro_rules! w {
    ($s:literal) => {{
        const fn to_utf16<const N: usize>(s: &str) -> [u16; N] {
            let bytes = s.as_bytes();
            let mut out = [0u16; N];
            let mut i = 0;
            let mut o = 0;
            while i < bytes.len() {
                let b = bytes[i];
                if b < 0x80 {
                    out[o] = b as u16;
                    i += 1;
                }
                o += 1;
            }
            out[o] = 0;
            out
        }
        const fn count(s: &str) -> usize {
            s.as_bytes().len() + 1
        }
        to_utf16::<{ count($s) }>($s)
    }};
}
