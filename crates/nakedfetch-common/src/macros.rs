#[macro_export]
macro_rules! print {
    ($($tt:tt)+) => {
        let mut std = $crate::io::Stdout::new();
        let _ = ::core::fmt::Write::write_fmt(&mut std, ::core::format_args!($($tt)*));
    };
    ($lit:literal) => {
        let mut std = $crate::io::Stdout::new();
        let _ = ::core::fmt::Write::write_str(&mut std, $lit);
    };
}

#[macro_export]
macro_rules! println {
    ($($tt:tt)+) => {
        let mut std = $crate::io::Stdout::new();
        let _ = ::core::fmt::Write::write_fmt(&mut std, ::core::format_args!($($tt)*));
        $crate::print!("\n");
    };
    ($lit:literal) => {
        let mut std = $crate::io::Stdout::new();
        let _ = ::core::fmt::Write::write_str(&mut std, ::core::concat!($lit, "\n"));
    };
    () => {
        $crate::print!("\n");
    };
}

#[macro_export]
macro_rules! eprint {
    ($($tt:tt)+) => {
        let mut std = $crate::io::Stderr::new();
        let _ = ::core::fmt::Write::write_fmt(&mut std, ::core::format_args!($($tt)*));
    };
    ($lit:literal) => {
        let mut std = $crate::io::Stderr::new();
        let _ = ::core::fmt::Write::write_str(&mut std, $lit);
    };
}

#[macro_export]
macro_rules! eprintln {
    ($($tt:tt)+) => {
        let mut std = $crate::io::Stderr::new();
        let _ = ::core::fmt::Write::write_fmt(&mut std, ::core::format_args!($($tt)*));
        $crate::print!("\n");
    };
    ($lit:literal) => {
        let mut std = $crate::io::Stderr::new();
        let _ = ::core::fmt::Write::write_str(&mut std, ::core::concat!($lit, "\n"));
    };
    () => {
        $crate::eprint!("\n");
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn print_macro_test() {
        crate::print!("Hello Worlddd\n");
        crate::print!("Hello {}", "Rust\n");
    }

    #[test]
    fn println_macro_test() {
        crate::println!("Hello Worlddd");
        crate::println!();
        crate::println!("Hello {}", "Golang");
    }

    #[test]
    fn eprint_macros_test() {
        crate::eprint!("Hello ");
        crate::eprintln!("{} & {}", "Python", "Haskell");
    }
}
