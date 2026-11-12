use core::ops::Range;

use crate::Path;
pub use crate::os::*;

#[derive(Clone)]
pub struct Components<'path> {
    source: &'path OsStr,
    pos: usize,
    end: usize,
}

impl<'path> Components<'path> {
    #[must_use]
    pub fn new<T: AsRef<OsStr> + ?Sized>(source: &'path T) -> Self {
        let source = source.as_ref();
        Self {
            source,
            pos: 0,
            end: source.len(),
        }
    }

    fn build_path(&self, range: Range<usize>) -> &'path Path {
        let s = &self.source[range];
        s.as_ref()
    }

    #[cfg(target_os = "windows")]
    const fn source_as_bytes(&self) -> &[u8] {
        self.source.as_bytes()
    }

    #[cfg(target_family = "unix")]
    const fn source_as_bytes(&self) -> &[u8] {
        self.source
    }
}

impl<'path> Iterator for Components<'path> {
    type Item = &'path Path;

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.end {
            let start = self.pos;
            if let Some(off) = self.source_as_bytes()[start..self.end]
                .iter()
                .position(|&b| SEPARATOR.contains(&char::from(b)))
            {
                self.pos = start + off + 1;
                if off > 0 {
                    return Some(self.build_path(start..start + off));
                }
            } else {
                self.pos = self.end;
                return Some(self.build_path(start..self.end));
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let seps = self.source_as_bytes()[self.pos..self.end]
            .iter()
            .filter(|&&b| SEPARATOR.contains(&char::from(b)))
            .count();
        (0, Some(seps + 1))
    }
}

impl DoubleEndedIterator for Components<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while self.pos < self.end {
            let end = self.end;
            if let Some(off) = self.source_as_bytes()[self.pos..end]
                .iter()
                .rposition(|&b| SEPARATOR.contains(&char::from(b)))
            {
                let sep_pos = self.pos + off;
                self.end = sep_pos;
                if sep_pos + 1 < end {
                    return Some(self.build_path(sep_pos + 1..end));
                }
            } else {
                self.end = self.pos;
                return Some(self.build_path(self.pos..end));
            }
        }
        None
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

    macro_rules! next_back_is {
        ($comps:ident, $item:expr) => {
            assert_eq!($comps.next_back(), Some($item.as_ref()));
        };
    }

    #[test]
    fn components_test() {
        let mut comps = Components::new(r"C:/User/rustling/Проекты/Rust/");
        next_is!(comps, "C:");
        next_is!(comps, "User");
        next_is!(comps, "rustling");
        next_is!(comps, "Проекты");
        next_is!(comps, "Rust");
        assert_eq!(comps.next(), None);
    }

    #[test]
    fn next_back_components_test() {
        let mut comps = Components::new(r"C:/User/rustling/Проекты/Rust/");
        next_back_is!(comps, "Rust");
        next_back_is!(comps, "Проекты");
        next_back_is!(comps, "rustling");
        next_back_is!(comps, "User");
        next_back_is!(comps, "C:");

        assert_eq!(comps.next_back(), None);
    }
}
