use core::fmt::{self, Debug, Display, Formatter};

use crate::{
    format,
    lua::{AsLua, LuaType},
    warning,
};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Level {
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
}

impl TryFrom<u8> for Level {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            1 => Ok(Self::V1),
            2 => Ok(Self::V2),
            3 => Ok(Self::V3),
            4 => Ok(Self::V4),
            _ => {
                warning!("Incorrect march level value: {value}");
                Err(())
            },
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct MicroArch {
    level: Option<Level>,
}

impl MicroArch {
    pub fn new(level: u8) -> Self {
        Self {
            level: Level::try_from(level).ok(),
        }
    }
}

impl Display for MicroArch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(l) = self.level {
            write!(f, "{}-v{}", env!("TARGET_ARCH"), l as u8)?;
        } else {
            write!(f, "{}", env!("TARGET_ARCH"))?;
        }
        Ok(())
    }
}

impl Debug for MicroArch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self, f)
    }
}

impl AsLua for MicroArch {
    const LUA_TYPE: &'static str = "string";

    fn as_lua(&self) -> LuaType {
        LuaType::String(format!("{self:?}"))
    }
}
