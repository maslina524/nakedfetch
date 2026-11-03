#[macro_export]
macro_rules! fprint {
    ($($tt:tt)+) => {{
        let mut std = $crate::io::Stdout::new();
        let _ = ::core::fmt::Write::write_fmt(&mut std, ::core::format_args!($($tt)*));
    }};
    ($lit:literal) => {{
        let mut std = $crate::io::Stdout::new();
        let _ = ::core::fmt::Write::write_str(&mut std, $lit);
    }};
}

#[macro_export]
macro_rules! fprintln {
    ($($tt:tt)+) => {{
        let mut std = $crate::io::Stdout::new();
        let _ = ::core::fmt::Write::write_fmt(&mut std, ::core::format_args!($($tt)*));
        $crate::fprint!("\n");
    }};
    ($lit:literal) => {{
        let mut std = $crate::io::Stdout::new();
        let _ = ::core::fmt::Write::write_str(&mut std, ::core::concat!($lit, "\n"));
    }};
    () => {{
        $crate::fprint!("\n");
    }};
}

#[macro_export]
macro_rules! feprint {
    ($($tt:tt)+) => {{
        let mut std = $crate::io::Stderr::new();
        let _ = ::core::fmt::Write::write_fmt(&mut std, ::core::format_args!($($tt)*));
    }};
    ($lit:literal) => {{
        let mut std = $crate::io::Stderr::new();
        let _ = ::core::fmt::Write::write_str(&mut std, $lit);
    }};
}

#[macro_export]
macro_rules! feprintln {
    ($($tt:tt)+) => {{
        let mut std = $crate::io::Stderr::new();
        let _ = ::core::fmt::Write::write_fmt(&mut std, ::core::format_args!($($tt)*));
        $crate::fprint!("\n");
    }};
    ($lit:literal) => {{
        let mut std = $crate::io::Stderr::new();
        let _ = ::core::fmt::Write::write_str(&mut std, ::core::concat!($lit, "\n"));
    }};
    () => {{
        $crate::feprint!("\n");
    }};
}

#[macro_export]
macro_rules! formatf {
    ($($tt:tt)*) => {{
        let mut string = ::alloc::string::String::with_capacity(32);
        let _ = ::core::fmt::Write::write_fmt(&mut string, ::core::format_args!($($tt)*));
        string
    }};
}

#[macro_export]
macro_rules! warning {
    ($($args:tt)*) => {{
        #[cfg(debug_assertions)]
        $crate::feprint!("\x1b[1;33mWarning\x1b[0m:{}:{}: ", file!(), line!());

        #[cfg(not(debug_assertions))]
        $crate::feprint!("\x1b[1;33mWarning\x1b[0m: ");

        $crate::feprintln!($($args)*);
    }};
}

#[macro_export]
macro_rules! abort {
    ($($args:tt)*) => {{
        $crate::feprintln!("\x1b[1;31mProgram Aborted\x1b[0m");
        panic!($($args)*);
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn print_macro_test() {
        crate::fprint!("Hello Worlddd\n");
        crate::fprint!("Hello {}", "Rust\n");
    }

    #[test]
    fn println_macro_test() {
        crate::fprintln!("Hello Worlddd");
        crate::fprintln!();
        crate::fprintln!("Hello {}", "Golang");
    }

    #[test]
    fn eprint_macros_test() {
        crate::feprint!("Hello ");
        crate::feprintln!("{} & {}", "Python", "Haskell");
    }

    #[test]
    fn format_test() {
        let std = format!("Hello, {}. 2 + 2 = {}", "World", 2 + 2);
        let my = crate::formatf!("Hello, {}. 2 + 2 = {}", "World", 2 + 2);
        assert_eq!(std, my);
    }
}
