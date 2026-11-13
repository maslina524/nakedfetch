#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(unused, reason = "All unused will be removed closer to 1.0.0.")]

mod base64;
mod color;
mod config;
mod kitty;
mod logo;
mod lua;
mod macros;
mod nvidia;
mod parser;
mod sync;
mod url;

mod detect;
mod field;
mod formats;
mod modules;
mod str;

cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
        use windows as imp;
    } else if #[cfg(target_family = "unix")] {
        mod unix;
        use unix as imp;
    } else {
        compile_error!("Unsupported OS");
    }
}

extern crate alloc;

use alloc::{borrow::ToOwned, boxed::Box, string::String, vec::Vec};
use core::{slice::Iter, str::FromStr as _};

use common::{
    fs::{self, FileCreateError as FCErr, FileCreateReadError as FCRErr},
    prelude::*,
    sync::OnceLock,
};
use image::{Image, Png};
use json::Map;

use crate::{
    config::{Config, ConfigModule},
    detect::os,
    field::cached,
    formats::{MemorySize, SplittedAnsiIter},
    imp::{env, http::Request},
    logo::{LogoInfo, UILogo},
    modules::{Commit, DocsVtable, FormatValue, Module, Version},
    nvidia::NvidiaLib,
    url::Url,
};

#[global_allocator]
static ALLOCATOR: allocator::Allocator = allocator::Allocator;
static HELP: &str = concat!(
    "nakedfetch is a neofetch-like tool for beautiful system information display with flexible output customization\n",
    "\n",
    "\x1b[1mUsage: nakedfetch\x1b[22;3m <?options>\x1b[0m\n",
    "\n",
    "\x1b[1mCommands:\x1b[0m\n",
    "  -h, --help <?options>     \tPrint this message\n",
    "  -v, --version <?options>  \tPrint nakedfetch version\n",
    "  -l, --logo                \tCustom logo (name or file)\n",
    "  -c, --config              \tCustom preset (http url or file)",
);

const MIN_OFFSET: usize = 36;
const IMAGE_SIZE: usize = 40;
const CELL_ASPECT: f64 = 2.0;
const ALLOC_REP_BAR_SIZE: u128 = 64;

#[cfg(not(test))]
mod panic_impl {
    use core::panic::PanicInfo;

    use crate::exit;

    #[panic_handler]
    #[cfg(no_alloc_in_panic)]
    fn panic(info: &PanicInfo) -> ! {
        use crate::imp::io;

        fn push_to_buf(s: &str, buf: &mut [u8], pos: &mut usize) {
            let bytes = s.as_bytes();
            let end = *pos + bytes.len();
            buf[*pos..end].copy_from_slice(bytes);
            *pos += bytes.len();
        }

        fn u32_as_str(n: u32, buf: &mut [u8]) -> &str {
            let mut i = buf.len();
            let mut n = n;
            loop {
                i -= 1;
                buf[i] = b'0' + (n % 10) as u8;
                n /= 10;
                if n == 0 {
                    break;
                }
            }
            core::str::from_utf8(&buf[i..]).unwrap()
        }

        let mut buf = [0u8; 4 * 1024];
        let mut pos = 0;

        if let Some(loc) = info.location() {
            let mut u32_buf = [0u8; 10];

            push_to_buf(loc.file(), &mut buf, &mut pos);
            push_to_buf(":", &mut buf, &mut pos);
            push_to_buf(u32_as_str(loc.line(), &mut u32_buf), &mut buf, &mut pos);
            push_to_buf(":", &mut buf, &mut pos);
            push_to_buf(u32_as_str(loc.column(), &mut u32_buf), &mut buf, &mut pos);
            push_to_buf(": ", &mut buf, &mut pos);
        }

        let message = info.message().as_str().unwrap_or("No panic message!");

        push_to_buf(message, &mut buf, &mut pos);
        push_to_buf("\n", &mut buf, &mut pos);

        io::write(io::stderr(), &buf[..pos]);

        exit(101)
    }

    #[panic_handler]
    #[cfg(not(no_alloc_in_panic))]
    fn panic(info: &PanicInfo<'_>) -> ! {
        use common::{feprintln, formatf};

        let msg = formatf!("{}", info.message());

        let location = info
            .location()
            .map(|l| formatf!("{}:{}:{}", l.file(), l.line(), l.column()));

        if let Some(ref loc) = location {
            feprintln!("\x1b[1m{loc}: {msg}\x1b[0m");
        } else {
            feprintln!("\x1b[1m{msg}\x1b[0m");
        }

        let link = formatf!(
            "https://github.com/maslina524/nakedfetch/issues/new?template=panic.yaml&version={}&title={}&target={}&location={}",
            env!("CARGO_PKG_VERSION"),
            msg,
            env!("TARGET"),
            location.unwrap_or_default()
        );
        feprintln!("Leave an Issue at this [\x1b]8;;{link}\x1b\\link\x1b]8;;\x1b\\]");

        exit(101)
    }
}

#[cfg(not(test))]
#[unsafe(no_mangle)]
pub const extern "C" fn rust_eh_personality() {}

fn max_line_len(lines: &[(String, usize)]) -> usize {
    let mut ret = 0;
    for (_, len) in lines {
        if *len > ret {
            ret = *len;
        }
    }
    ret
}

fn build_logo_buf(lines: &[(String, usize)], max_len: usize) -> Vec<String> {
    let padding = Config::get().get_logo_padding();
    let max_len_padding = max_len + padding.left + padding.right;
    let mut ret = Vec::with_capacity(lines.len() + padding.top + padding.bottom);

    let (w, _) = env::terminal_size();
    if w < max_len_padding {
        return Vec::new();
    }

    for _ in 0..padding.top {
        ret.push(" ".repeat(max_len_padding));
    }
    for (line, len) in lines {
        let string = formatf!(
            "{}{}{}",
            " ".repeat(padding.left),
            line,
            " ".repeat(padding.right + max_len - len)
        );
        ret.push(string);
    }
    for _ in 0..padding.bottom {
        ret.push(" ".repeat(max_len_padding));
    }

    ret
}

fn get_module_lines(preset_module: &ConfigModule, max_len_line: usize) -> Option<SplittedAnsiIter> {
    modules::from_preset_module(&preset_module.typ).map(|module| {
        let string = module.format(
            FormatValue {
                format: preset_module.key.as_deref(),
                color: preset_module.key_color.as_deref(),
            },
            FormatValue {
                format: preset_module.format.as_deref(),
                color: None,
            },
            Some(&preset_module.map),
        );
        string.map_or_else(SplittedAnsiIter::empty, |s| {
            SplittedAnsiIter::new(&s, max_len_line)
        })
    })
}

fn build_info_buf(max_len: usize) -> Vec<&'static str> {
    let padding = Config::get().get_logo_padding();
    let (w, _) = env::terminal_size();
    let max_len_line = if max_len + padding.left + padding.right < w {
        w - max_len - padding.left - padding.right
    } else {
        w
    };
    let mut ret = Vec::with_capacity(40);
    let preset = Config::get();

    for module in preset.modules() {
        if let Some(m) = get_module_lines(module, max_len_line) {
            ret.extend(m);
        }
    }

    ret
}

pub fn exit(code: u32) -> ! {
    cfg_if! {
        if #[cfg(target_family = "unix")] {
            unixlib::process::exit(code)
        } else if #[cfg(target_os = "windows")] {
            // SAFETY: Run in binary, safe
            unsafe { windows::link::ExitProcess(code) }
        }
    }
}

fn get_config(args: &mut Iter<'_, &'static str>) -> Config {
    let Some(_) = args.position(|a| *a == "--config" || *a == "-c") else {
        return Config::default();
    };
    let Some(path) = args.next() else {
        print_help(None);
    };

    // URL path
    if let Some(url) = Url::new(path) {
        let response = match Request::from_url(url).get() {
            Ok(r) => r,
            Err(e) => {
                warning!("Failed to connect to server (config): {}", e.code());
                return Config::default();
            },
        };
        if !response.is_success() {
            warning!("Failed to get preset from URL, Code: {}", response.code());
            return Config::default();
        }
        let Ok(t) = response.as_text() else {
            warning!("Failed to parse the response");
            return Config::default();
        };
        return match Map::from_str(&t) {
            Ok(c) => Config::from_json(&c),
            Err(e) => {
                warning!("Failed to parse the json config: {e}");
                Config::default()
            },
        };
    }

    // FS path
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            warning!("Failed to read json config: {e}");
            return Config::default();
        },
    };
    match Map::from_str(path) {
        Ok(c) => Config::from_json(&c),
        Err(e) => {
            warning!("Failed to parse the json config: {e}");
            Config::default()
        },
    }
}

fn get_logo_name_and_custom(val: &str) -> (Box<str>, UILogo) {
    let id = os::get_id().as_boxed_str();
    match fs::read(val) {
        Ok(b) => {
            if Png::is_png(&b) {
                match Png::decode(&b) {
                    Ok(p) => (id, UILogo::Image(p.into_image())),
                    Err(e) => {
                        warning!("Failed to parse png: {e}");
                        (id, UILogo::Preset)
                    },
                }
            } else if let Ok(s) = String::from_utf8(b) {
                (id, UILogo::Ascii(s))
            } else {
                warning!("Failed to represent data as utf8");
                (id, UILogo::Preset)
            }
        },
        Err(e) => {
            if e == FCRErr::Create(FCErr::NameNotFound) {
                let ready_id = val.to_lowercase().replace('_', " ").into_boxed_str();
                (ready_id, UILogo::Preset)
            } else {
                warning!("Failed to use logo from fs: {e}");
                (id, UILogo::Preset)
            }
        },
    }
}

#[cold]
fn print_help(theme: Option<&str>) -> ! {
    if let Some(t) = theme {
        let idx = t.find('-').unwrap_or_else(|| {
            fprintln!("Specify the module name!");
            exit(0)
        });
        let ident = &t[..idx];
        let action = &t[idx + 1..];

        let vtable = DocsVtable::from_str(ident).unwrap_or_else(|| {
            fprintln!("Module with this name not found!");
            exit(0)
        });
        match action {
            "format" => {
                if let Some(doc) = (vtable.format)() {
                    fprintln!(
                        "# In config file: {{ \"type\": \"{ident}\", \"format\": \"{{{}}} or {{1}}\" }}",
                        doc[0].name
                    );
                    fprintln!("The following variables are passed:");
                    for i in doc {
                        fprintln!(
                            "{:>24} : {:<4} : {}",
                            i.name,
                            i.second,
                            i.desc.unwrap_or("Empty")
                        );
                    }
                } else {
                    fprintln!("Module `{ident}` doesn't support output formatting");
                }
            },
            "lua" => {
                if let Some(doc) = (vtable.lua)() {
                    fprintln!(
                        "# In config file: {{ \"type\": \"{ident}\", \"format\": \"lua: return (...).{}\" }}",
                        doc[0].name
                    );
                    fprintln!("The following variables are passed:");
                    for i in doc {
                        fprintln!(
                            "{:>24} : {:<6} : {}",
                            i.name,
                            i.second,
                            i.desc.unwrap_or("Empty")
                        );
                    }
                } else {
                    fprintln!("Module `{ident}` doesn't support lua");
                }
            },
            "example" => {
                let id = os::get_id().as_boxed_str();
                LogoInfo::new(&id);
                Config::get_or_init(Config::default());

                if let Some(doc) = (vtable.example)() {
                    // println!(
                    //     "# In config file: {{ \"type\": \"{ident}\",
                    // \"format\": \"lua: return (...).{}\" }}",     doc[0].name
                    // );
                    fprintln!("The following variables are passed:");
                    for (name, value) in doc {
                        fprintln!("{name:>24} : {value}");
                    }
                } else {
                    fprintln!("Module `{ident}` doesn't have variables");
                }
            },
            _ => {
                fprintln!("Incorrect action");
                exit(1);
            },
        }

        exit(0);
    }

    fprintln!("{HELP}");

    exit(0)
}

#[cold]
fn print_version(method: Option<&str>) -> ! {
    let ver = Version::new();
    match method {
        None => fprintln!("{} {} ({})", ver.project_name, ver.version, ver.arch),
        Some("raw") => fprintln!("{}", ver.version),
        Some("dbg") => fprintln!("{ver:#?}"),
        Some("extended") => {
            let typ = match ver.build_type {
                "release" => "\x1b[32mrelease\x1b[0m",
                "debug" => "\x1b[33mdebug\x1b[0m",
                _ => unreachable!(),
            };
            let com = Commit::new();
            fprintln!(
                "\x1b[1m{} {} ({}) {typ} [\x1b]8;;{}\x1b\\link\x1b]8;;\x1b\\]",
                ver.project_name,
                ver.version,
                ver.arch,
                ver.release_link
            );

            if ver.libc.is_empty() {
                fprintln!("    {}", ver.compiler);
            } else {
                fprintln!("    {} (libc {})", ver.compiler, ver.libc);
            }

            fprintln!(
                "    {} <{}> ({})",
                com.message,
                com.date_small,
                com.sha_short
            );
        },
        _ => feprintln!("Unknown method for version, supported: raw, dbg, extended"),
    }
    exit(0)
}

fn print_none() {
    let id = os::get_id().as_boxed_str();
    LogoInfo::new(&id);

    let (w, _) = env::terminal_size();
    let info_lines = build_info_buf(w);

    for line in info_lines {
        fprintln!("{line}\x1b[0m");
    }
}

#[allow(
    clippy::cast_precision_loss,
    reason = "It should not lead to a problem"
)]
fn print_image(image: &Image) {
    let id = os::get_id().as_boxed_str();
    LogoInfo::new(&id);

    let padding = Config::get().get_logo_padding();
    let (w, term_h) = env::terminal_size();

    let (img_w, img_h) = image.size();

    let mut cols = IMAGE_SIZE;
    let mut height = (img_h as f64 / img_w as f64 * cols as f64 / CELL_ASPECT) as usize;

    let max_h = term_h.saturating_sub(padding.top + padding.bottom).max(1);
    if height > max_h {
        let scale = max_h as f64 / height as f64;
        cols = ((cols as f64 * scale) as usize).max(1);
        height = max_h;
    }

    let max_logo_len_padding = cols + padding.left + padding.right;
    let empty_logo_line = " ".repeat(max_logo_len_padding);
    let info_buf = build_info_buf(w);
    let lines_printed;

    if max_logo_len_padding + MIN_OFFSET < w {
        lines_printed = height.max(info_buf.len());
        for i in 0..lines_printed {
            let info_line = info_buf.get(i).map_or("", |s| *s);
            fprintln!("{empty_logo_line}{info_line}\x1b[0m");
        }
    } else {
        lines_printed = height + info_buf.len();
        for _ in 0..height {
            fprintln!("{empty_logo_line}");
        }
        for line in info_buf {
            fprintln!("{line}\x1b[0m");
        }
    }

    fprint!("\x1b7\x1b[{lines_printed}A\x1b[{}C", padding.left);

    let _ = kitty::print_image(image, Some(cols), Some(height), 0);
    fprint!("\x1b8");
}

fn print_base(logo_name: &str) {
    let logo = LogoInfo::new(logo_name);
    just_print_logo_and_info(&logo.get_ready_logo_lines(UILogo::Preset));
}

fn print_ascii(ascii: String) {
    let id = os::get_id().as_boxed_str();
    let logo = LogoInfo::new(&id);
    just_print_logo_and_info(&logo.get_ready_logo_lines(UILogo::Ascii(ascii)));
}

fn just_print_logo_and_info(logo_lines: &[(String, usize)]) {
    let max_logo_len = max_line_len(logo_lines);
    let padding = Config::get().get_logo_padding();
    let max_logo_len_padding = max_logo_len + padding.left + padding.right;
    let (w, _) = env::terminal_size();
    let split_len = if max_logo_len_padding + MIN_OFFSET < w {
        max_logo_len
    } else {
        w
    };
    let logo_buf = build_logo_buf(logo_lines, max_logo_len);
    let info_buf = build_info_buf(split_len);
    let max_lines = logo_buf.len().max(info_buf.len());

    // Print buffers
    if max_logo_len_padding + MIN_OFFSET < w {
        let empty_logo_line = " ".repeat(max_logo_len_padding);
        for i in 0..max_lines {
            let logo_line = logo_buf
                .get(i)
                .map_or(empty_logo_line.as_str(), String::as_str);

            let info_line = info_buf.get(i).map_or("", |s| *s);
            fprintln!("{logo_line}{info_line}\x1b[0m");
        }
    } else {
        for line in logo_buf {
            fprintln!("{line}\x1b[0m");
        }
        for line in info_buf {
            fprintln!("{line}\x1b[0m");
        }
    }
}

static ARGS: OnceLock<Vec<String>> = OnceLock::new();

// #[cfg(not(test))]
#[allow(
    clippy::similar_names,
    reason = "that's what they're called in C, i don't give a fuck about clippy"
)]
#[cfg(target_family = "unix")]
#[unsafe(no_mangle)]
extern "C" fn main(
    argc: ffi::c_int,
    argv: *const *const ffi::c_char,
    envp: *const *const ffi::c_char,
) -> ffi::c_int {
    use core::sync::atomic::Ordering;

    use unixlib::api::alloc::init;

    init::falloc_init();
    libc::unistd::ENVIRON.store(envp.cast_mut(), Ordering::Relaxed);
    let _ = ARGS.set(imp::env::args_init(argc as usize, argv.cast()));
    let ret = nakedfetch_main() as ffi::c_int;
    init::falloc_destroy();
    ret
}

#[cfg(not(test))]
#[cfg(target_os = "windows")]
#[unsafe(no_mangle)]
extern "C" fn main() -> i32 {
    let _ = ARGS.set(env::args_init());
    let ret = nakedfetch_main();

    if env::args().iter().any(|a| a == "--alloc-snapshot") {
        fprintln!("{}", allocator::Snapshot::create());
    }

    ret
}

fn nakedfetch_main() -> i32 {
    let mut args: Vec<&'static str> = common::env::args().collect();

    if let Some(pos) = args.iter().position(|a| *a == "--help" || *a == "-h") {
        print_help(args.get(pos + 1).copied());
    }

    if let Some(pos) = args.iter().position(|a| *a == "--version" || *a == "-v") {
        print_version(args.get(pos + 1).copied());
    }

    let config = get_config(&mut args.iter());
    Config::get_or_init(config);

    // Logo init
    let name_raw = args
        .iter()
        .position(|a| *a == "--logo" || *a == "-l")
        .and_then(|pos| args.get(pos + 1).copied())
        .map(|v| v.to_owned())
        .or_else(|| Config::get().get_logo_name());

    let (logo_name, custom) = name_raw.map_or_else(
        || {
            let id = os::get_id().as_boxed_str();
            (id, UILogo::Preset)
        },
        |name| {
            if name == "null" {
                let id = os::get_id().as_boxed_str();
                (id, UILogo::None)
            } else {
                get_logo_name_and_custom(&name)
            }
        },
    );

    // Print logo and info
    match custom {
        UILogo::None => print_none(),
        UILogo::Preset => print_base(&logo_name),
        UILogo::Ascii(s) => print_ascii(s),
        UILogo::Image(i) => print_image(&i),
    }

    // The handle is created not with `GetStdHandle`,
    // but with `CreateFile`, which requires manual freeing
    #[cfg(target_os = "windows")]
    let _ = env::close_terminal_handle();
    NvidiaLib::drop_nvidia();
    cached::flush_to_file();

    0
}
