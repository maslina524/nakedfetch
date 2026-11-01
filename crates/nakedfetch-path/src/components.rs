use core::ops::Range;

use wtf8::CodePoint;

use crate::Path;
pub use crate::os::*;

#[derive(Clone)]
pub struct Components<'path> {
    source: &'path PathInner,
    pos: usize,
}

impl<'path> Components<'path> {
    #[must_use]
    pub fn new<T: AsRef<PathInner> + ?Sized>(source: &'path T) -> Self {
        Self {
            source: source.as_ref(),
            pos: 0,
        }
    }

    fn build_path(&self, range: Range<usize>) -> &'path Path {
        let s = &self.source[range];
        s.as_ref()
    }
}

impl<'path> Iterator for Components<'path> {
    type Item = &'path Path;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.source.len() {
            return None;
        }
        let start = self.pos;

        for (off, &b) in self.source.as_bytes()[start..].iter().enumerate() {
            if SEPARATOR.contains(&char::from(b)) {
                self.pos = start + off + 1;
                return Some(self.build_path(start..start + off));
            }
        }

        self.pos = self.source.len();
        Some(self.build_path(start..self.pos))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sum = self
            .source
            .codepoints()
            .filter(|ch| match ch {
                CodePoint::Char(c) => SEPARATOR.contains(c),
                CodePoint::Surrogate(_) => false,
            })
            .count();
        (0, Some(sum))
    }
}

#[cfg(test)]
mod tests {
    use crate::Components;

    macro_rules! next_is {
        ($comps:ident, $item:expr) => {
            assert_eq!($comps.next(), Some($item.as_ref()));
        };
    }

    #[test]
    fn components_test() {
        let mut comps = Components::new(crate::Path::from_str(r"C:\User\rustling/Проекты/Rust/"));
        next_is!(comps, "C:");
        next_is!(comps, "User");
        next_is!(comps, "rustling");
        next_is!(comps, "Проекты");
        next_is!(comps, "Rust");
        assert_eq!(comps.next(), None);
    }
}
