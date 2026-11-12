use alloc::{
    borrow::ToOwned,
    boxed::Box,
    string::{String, ToString},
};
use core::{
    fmt::{Debug, Display, Write as _},
    marker::PhantomData,
    sync::atomic::{AtomicBool, Ordering},
};

use common::{
    env, formatf,
    fs::{self, File, FileCreateError as FCErr, FileReadToStringError as FRTSErr, Write},
    sync::Mutex,
    warning,
};
use path::Path;

use crate::{
    detect::uptime::UptimeInfo,
    formats::Frequency,
    lua::{AsLua, LuaType},
    parser::LineBased,
};

static LOADED: AtomicBool = AtomicBool::new(false);
static EDITED: AtomicBool = AtomicBool::new(false);

static ENTRIES: Mutex<LineBased<'_>> = Mutex::new(LineBased::new());

pub fn flush_to_file() {
    if !EDITED.load(Ordering::Acquire) {
        return;
    }

    let path_dir = env::cache_dir();
    let _ = fs::create_dirs(&path_dir);
    let path = path_dir.join("session");
    let mut file = match File::<Write>::create_new_or_rewrite(path) {
        Ok(f) => f,
        Err(e) => {
            warning!("Failed to create cache/session: {e}");
            return;
        },
    };

    let entries = ENTRIES.lock();
    if let Err(e) = write!(file, "{}", *entries) {
        warning!("Failed to write to cache/session: {e}");
    }
}

fn load_entries() {
    if LOADED.load(Ordering::Acquire) {
        return;
    }

    let mut entries = ENTRIES.lock();
    if LOADED.load(Ordering::Acquire) {
        return;
    }

    let actual = UptimeInfo::new().boot_timestamp;
    let path = env::cache_dir().join("session");

    let fallback = || LineBased::parse(formatf!("boot_timestamp={actual}"), '=');

    let mut parsed = match LineBased::parse_file(path, '=') {
        Ok(p) => p,
        Err(FRTSErr::Create(FCErr::NameNotFound)) => fallback(),
        Err(e) => {
            warning!("Failed to read cache session: {e}");
            fallback()
        },
    };

    match parsed.get("boot_timestamp").and_then(u64::from_cached) {
        Some(bt) if bt == actual => {},
        _ => {
            parsed = LineBased::parse(formatf!("boot_timestamp={actual}"), '=');
        },
    }

    *entries = parsed;
    LOADED.store(true, Ordering::Release);
}

pub struct SessionCached<T: AsCached> {
    name: Box<str>,
    func: Box<dyn Fn() -> T + Send + Sync>,
    _marker: PhantomData<fn() -> T>,
}

impl<T: AsCached + Clone + Send + Sync + 'static> SessionCached<T> {
    pub fn new<F>(name: &str, func: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            func: Box::new(func),
            _marker: PhantomData,
        }
    }

    pub fn load(&self) -> T {
        self.get_value().unwrap_or_else(|| {
            let v = (self.func)();
            self.save_value(&v);
            v
        })
    }

    fn get_value(&self) -> Option<T> {
        load_entries();
        ENTRIES
            .lock()
            .get(&self.name)
            .and_then(AsCached::from_cached)
    }

    fn save_value(&self, val: &T) {
        let cached = val.as_cached();
        (*ENTRIES.lock()).insert(&self.name, &cached);
        EDITED.store(true, Ordering::Release);
    }
}

impl<T: Display + AsCached + Clone + Send + Sync + 'static> Display for SessionCached<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.load())
    }
}

impl<T: Debug + AsCached + Clone + Send + Sync + 'static> Debug for SessionCached<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.load())
    }
}

impl<T: Display + AsCached + AsLua + Clone + Send + Sync + 'static> AsLua for SessionCached<T> {
    const LUA_TYPE: &'static str = T::LUA_TYPE;

    fn as_lua(&self) -> LuaType {
        LuaType::String(formatf!("{self}"))
    }
}

pub trait AsCached: Sized {
    fn as_cached(&self) -> String;
    fn from_cached(cache: &str) -> Option<Self>;
}

impl AsCached for String {
    fn as_cached(&self) -> String {
        self.clone()
    }

    fn from_cached(cache: &str) -> Option<Self> {
        Some(cache.to_owned())
    }
}

impl AsCached for u64 {
    fn as_cached(&self) -> String {
        self.to_string()
    }

    fn from_cached(cache: &str) -> Option<Self> {
        cache.parse::<Self>().map_or_else(
            |_| {
                warning!("Incorrect u64 in cache");
                None
            },
            Some,
        )
    }
}

impl AsCached for u32 {
    fn as_cached(&self) -> String {
        self.to_string()
    }

    fn from_cached(cache: &str) -> Option<Self> {
        cache.parse::<Self>().map_or_else(
            |_| {
                warning!("Incorrect u32 in cache");
                None
            },
            Some,
        )
    }
}

impl AsCached for Frequency {
    fn as_cached(&self) -> String {
        self.as_hz().to_string()
    }

    fn from_cached(cache: &str) -> Option<Self> {
        cache.parse::<u64>().map_or_else(
            |_| {
                warning!("Incorrect u64 in cache");
                None
            },
            |v| Some(Self::from_hz(v)),
        )
    }
}
