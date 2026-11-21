macro_rules! warning {
    () => {{}};
    ($fmt:literal) => {{
        ::std::println!(::std::concat!("cargo:warning=", $fmt));
    }};
    ($fmt:literal, $($args:tt)*) => {{
        ::std::println!(::std::concat!("cargo:warning=", $fmt), $($args)*);
    }};
}

macro_rules! set_cfg {
    () => {{}};
    ($fmt:literal) => {{
        ::std::println!(::std::concat!("cargo:rustc-cfg=", $fmt));
    }};
    ($fmt:literal, $($args:tt)*) => {{
        ::std::println!(::std::concat!("cargo:rustc-cfg=", $fmt), $($args)*);
    }};
}

macro_rules! set_env {
    () => {{}};
    ($fmt:literal) => {{
        ::std::println!(::std::concat!("cargo:rustc-env=", $fmt));
    }};
    ($fmt:literal, $($args:tt)*) => {{
        ::std::println!(::std::concat!("cargo:rustc-env=", $fmt), $($args)*);
    }};
}

macro_rules! link_arg {
    () => {{}};
    ($fmt:literal) => {{
        ::std::println!(::std::concat!("cargo:rustc-link-arg=", $fmt));
    }};
    ($fmt:literal, $($args:tt)*) => {{
        ::std::println!(::std::concat!("cargo:rustc-link-arg=", $fmt), $($args)*);
    }};
}

macro_rules! link_lib {
    () => {{}};
    ($fmt:literal) => {{
        ::std::println!(::std::concat!("cargo:rustc-link-lib=", $fmt));
    }};
    ($fmt:literal, $($args:tt)*) => {{
        ::std::println!(::std::concat!("cargo:rustc-link-lib=", $fmt), $($args)*);
    }};
}

macro_rules! link_search {
    () => {{}};
    ($fmt:literal) => {{
        ::std::println!(::std::concat!("cargo:rustc-link-search=", $fmt));
    }};
    ($fmt:literal, $($args:tt)*) => {{
        ::std::println!(::std::concat!("cargo:rustc-link-search=", $fmt), $($args)*);
    }};
}
