use alloc::{vec, vec::Vec};

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
    pub const fn get_size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    #[must_use]
    pub fn as_rgba_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.w * self.h * 4);
        for p in &self.data {
            out.extend_from_slice(&[p.0, p.1, p.2, p.3]);
        }
        out
    }
}
