use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LuaType {
    String,
    Number,
    Boolean,
}

impl From<LuaType> for &'static str {
    fn from(val: LuaType) -> Self {
        match val {
            LuaType::String => "string",
            LuaType::Number => "number",
            LuaType::Boolean => "boolean",
        }
    }
}

#[derive(Clone)]
pub enum LuaValue<'a> {
    Display(&'a dyn fmt::Display),
    Debug(&'a dyn fmt::Debug),
    Number(f64),
    Boolean(bool),
}

impl fmt::Display for LuaValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LuaValue::Display(d) => fmt::Display::fmt(d, f),
            LuaValue::Debug(d) => fmt::Debug::fmt(d, f),
            LuaValue::Number(n) => fmt::Display::fmt(n, f),
            LuaValue::Boolean(b) => fmt::Display::fmt(b, f),
        }
    }
}

pub trait AsLua {
    const INTO_LUA_TYPE: LuaType;
    fn as_lua_value(&self) -> LuaValue<'_>;
}

macro_rules! impl_as_lua {
    {$($way:ident { $($Type:ty),* $(,)? })*} => {
        $(
            $(
                impl $crate::aslua::AsLua for $Type {
                    impl_as_lua!(@$way $Type);
                }
            )*
        )*
    };
    (@Debug $Type:ty) => {
        const INTO_LUA_TYPE: $crate::aslua::LuaType = $crate::aslua::LuaType::String;
        fn as_lua_value(&self) -> $crate::aslua::LuaValue<'_> {
            $crate::aslua::LuaValue::Debug(self)
        }
    };
    (@Display $Type:ty) => {
        const INTO_LUA_TYPE: $crate::aslua::LuaType = $crate::aslua::LuaType::String;
        fn as_lua_value(&self) -> $crate::aslua::LuaValue<'_> {
            $crate::aslua::LuaValue::Display(self)
        }
    };
    (@AsF64 $Type:ty) => {
        const INTO_LUA_TYPE: $crate::aslua::LuaType = $crate::aslua::LuaType::Number;
        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_lossless,
            trivial_numeric_casts,
            reason = "f64 to f64. Cast lossless should not cause problems"
        )]
        fn as_lua_value(&self) -> $crate::aslua::LuaValue<'_> {
            $crate::aslua::LuaValue::Number(*self as f64)
        }
    };
    (@AsBool $Type:ty) => {
        const INTO_LUA_TYPE: $crate::aslua::LuaType = $crate::aslua::LuaType::Boolean;
        #[allow(
            trivial_casts,
            reason = "bool to bool"
        )]
        fn as_lua_value(&self) -> $crate::aslua::LuaValue<'_> {
            $crate::aslua::LuaValue::Boolean(*self as bool)
        }
    };
}

use alloc::{borrow::Cow, string::String};

impl_as_lua! {
    Debug {
        String,
        &str,
        char,
        Cow<'static, str>
    }
    AsF64 {
        usize,u8, u16, u32, u64, u128,
        isize, i8, i16, i32, i64, i128,
        f32, f64,
    }
    AsBool {
        bool
    }
}
