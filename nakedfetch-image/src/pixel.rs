use core::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
    Rgba,
    Rgb,
    GrayscaleAlpha,
    Grayscale,
}

impl ColorType {
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

impl core::fmt::Display for ColorTypeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Failed to get ColorType from index ({}); file is corrupted or nakedfetch does not support the type, try converting the file to another type",
            self.0
        )
    }
}

impl core::error::Error for ColorTypeError {}

#[derive(Debug)]
pub struct RgbaConvertError {
    pub typ: ColorType,
    pub bits: u8,
}

impl core::fmt::Display for RgbaConvertError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "nakedfetch does not support {:?} with {}-bit color depth, try converting the image to another format",
            self.typ, self.bits
        )
    }
}

impl Error for RgbaConvertError {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

impl Rgba {
    pub const fn from(data: &[u8], typ: ColorType, bits: u8) -> Result<Self, RgbaConvertError> {
        match (typ, bits) {
            (ColorType::Rgba, 8) => Ok(Self(data[0], data[1], data[2], data[3])),
            (ColorType::Rgba, 16) => {
                let r = data[0];
                let g = data[2];
                let b = data[4];
                let a = data[6];
                Ok(Self(r, g, b, a))
            },
            (ColorType::Rgb, 8) => Ok(Self(data[0], data[1], data[2], 255)),
            (ColorType::Rgb, 16) => {
                let r = data[0];
                let g = data[2];
                let b = data[4];
                Ok(Self(r, g, b, 255))
            },
            (ColorType::Grayscale, 8 | 16) => {
                let g = data[0];
                Ok(Self(g, g, g, 255))
            },
            (ColorType::Grayscale, 1 | 2 | 4) => {
                let val = data[0];
                let g = match bits {
                    1 => {
                        if val == 0 {
                            0
                        } else {
                            255
                        }
                    },
                    2 => val * 85,
                    4 => val * 17,
                    _ => unreachable!(),
                };
                Ok(Self(g, g, g, 255))
            },
            (ColorType::GrayscaleAlpha, 8) => {
                let g = data[0];
                let a = data[1];
                Ok(Self(g, g, g, a))
            },
            (ColorType::GrayscaleAlpha, 16) => {
                let g = data[0];
                let a = data[2];
                Ok(Self(g, g, g, a))
            },
            _ => Err(RgbaConvertError { typ, bits }),
        }
    }
}
