use core::{error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
    Rgba,
    Rgb,
    GrayscaleAlpha,
    Grayscale,
}

impl ColorType {
    #[must_use]
    pub const fn get_bytes_count(self) -> usize {
        match self {
            Self::Rgba => 4,
            Self::Rgb => 3,
            Self::GrayscaleAlpha => 2,
            Self::Grayscale => 1,
        }
    }
}

impl TryFrom<u8> for ColorType {
    type Error = ColorTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Grayscale),
            2 => Ok(Self::Rgb),
            4 => Ok(Self::GrayscaleAlpha),
            6 => Ok(Self::Rgba),
            _ => Err(ColorTypeError(value)),
        }
    }
}

#[derive(Debug)]
pub struct ColorTypeError(u8);

impl fmt::Display for ColorTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to get ColorType from index ({}); file is corrupted or nakedfetch does not support the type, try converting the file to another type",
            self.0
        )
    }
}

impl Error for ColorTypeError {}

#[derive(Debug)]
pub enum RgbaError {
    Type(UnknownType),
    BufTooSmall,
    InvalidByte,
}

impl fmt::Display for RgbaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Type(t) => write!(f, "{t}"),
            Self::BufTooSmall => write!(f, "Buf too small"),
            Self::InvalidByte => write!(f, "Invalid byte"),
        }
    }
}

impl Error for RgbaError {}

#[derive(Debug)]
pub struct UnknownType {
    pub typ: ColorType,
    pub bits: u8,
}

impl fmt::Display for UnknownType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "nakedfetch does not support {:?} with {}-bit color depth, try converting the image to another format",
            self.typ, self.bits
        )
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Rgba {
    /// # Errors
    ///
    /// `Err` on an invalid byte sequence for ``ColorType``.
    pub const fn from(data: &[u8], typ: ColorType, bits: u8) -> Result<Self, RgbaError> {
        match (typ, bits) {
            (ColorType::Rgba, 8) => {
                if data.len() < 4 {
                    return Err(RgbaError::BufTooSmall);
                }
                Ok(Self(data[0], data[1], data[2], data[3]))
            },
            (ColorType::Rgba, 16) => {
                if data.len() < 8 {
                    return Err(RgbaError::BufTooSmall);
                }
                let r = data[0];
                let g = data[2];
                let b = data[4];
                let a = data[6];
                Ok(Self(r, g, b, a))
            },
            (ColorType::Rgb, 8) => {
                if data.len() < 3 {
                    return Err(RgbaError::BufTooSmall);
                }
                Ok(Self(data[0], data[1], data[2], 255))
            },
            (ColorType::Rgb, 16) => {
                if data.len() < 6 {
                    return Err(RgbaError::BufTooSmall);
                }
                let r = data[0];
                let g = data[2];
                let b = data[4];
                Ok(Self(r, g, b, 255))
            },
            (ColorType::Grayscale, 8) => {
                if data.is_empty() {
                    return Err(RgbaError::BufTooSmall);
                }
                let g = data[0];
                Ok(Self(g, g, g, 255))
            },
            (ColorType::Grayscale, 16) => {
                if data.len() < 2 {
                    return Err(RgbaError::BufTooSmall);
                }
                let g = data[0];
                Ok(Self(g, g, g, 255))
            },
            (ColorType::Grayscale, 1 | 2 | 4) => {
                if data.is_empty() {
                    return Err(RgbaError::BufTooSmall);
                }
                let val = data[0];
                let (max_val, multiplier) = match bits {
                    1 => (1, 255),
                    2 => (3, 85),
                    4 => (15, 17),
                    _ => return Err(RgbaError::InvalidByte),
                };
                if val > max_val {
                    return Err(RgbaError::InvalidByte);
                }
                let g = val * multiplier;
                Ok(Self(g, g, g, 255))
            },
            (ColorType::GrayscaleAlpha, 8) => {
                if data.len() < 2 {
                    return Err(RgbaError::BufTooSmall);
                }
                let g = data[0];
                let a = data[1];
                Ok(Self(g, g, g, a))
            },
            (ColorType::GrayscaleAlpha, 16) => {
                if data.len() < 4 {
                    return Err(RgbaError::BufTooSmall);
                }
                let g = data[0];
                let a = data[2];
                Ok(Self(g, g, g, a))
            },
            _ => Err(RgbaError::Type(UnknownType { typ, bits })),
        }
    }
}

#[macro_export]
macro_rules! rgba {
    ($v1:expr, $v2:expr, $v3:expr, $v4:expr) => {
        $crate::pixel::Rgba($v1, $v2, $v3, $v4)
    };
    ($v1:expr, $v2:expr, $v3:expr) => {
        $crate::pixel::Rgba($v1, $v2, $v3, 255)
    };
    ($v1:expr) => {
        $crate::pixel::Rgba($v1, $v1, $v1, 255)
    };
}
