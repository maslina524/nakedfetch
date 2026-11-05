use alloc::{vec, vec::Vec};
use core::slice;

use crate::Rgba;

#[derive(Debug)]
pub struct Image {
    w: usize,
    h: usize,
    data: Vec<Rgba>,
}

impl Image {
    #[must_use]
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            data: vec![Rgba::default(); w * h],
        }
    }

    #[must_use]
    pub const fn from_data(w: usize, h: usize, data: Vec<Rgba>) -> Self {
        Self { w, h, data }
    }

    #[must_use]
    pub const fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, v: Rgba) -> Option<()> {
        if x < self.w && y < self.h {
            self.data[y * self.w + x] = v;
            Some(())
        } else {
            None
        }
    }

    #[must_use]
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Rgba> {
        self.data.get(y * self.w + x).copied()
    }

    #[must_use]
    pub fn get_pixel_wrapped(&self, x: isize, y: isize) -> Option<Rgba> {
        if x < 0 || y < 0 || x >= self.w as isize || y >= self.h as isize {
            return None;
        }
        self.data.get(y as usize * self.w + x as usize).copied()
    }

    #[must_use]
    pub fn get_pixel_wrapped_or_default(&self, x: isize, y: isize) -> Rgba {
        self.get_pixel_wrapped(x, y).unwrap_or_default()
    }

    #[must_use]
    pub const fn as_rgba_bytes(&self) -> &[u8] {
        const _: () = assert!(size_of::<Rgba>() == 4);

        let data = self.data.as_ptr().cast::<u8>();
        let len = self.data.len() * 4;
        // SAFETY: In memory all Rgba are stored contiguously, each one holds 4
        // bytes, the size of the output array is `data.len() * 4`
        unsafe { slice::from_raw_parts(data, len) }
    }

    #[must_use]
    pub fn as_rgba_slice(&self) -> &[Rgba] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use crate::{Image, Rgba};

    #[test]
    fn as_rgba_bytes() {
        let vec = vec![
            Rgba(255, 0, 0, 255),
            Rgba(0, 255, 0, 255),
            Rgba(0, 0, 255, 255),
            Rgba(255, 255, 255, 255),
        ];
        let image = Image::from_data(2, 2, vec);

        let bytes = image.as_rgba_bytes();
        assert_eq!(
            bytes,
            [
                255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255
            ]
        );
    }
}
