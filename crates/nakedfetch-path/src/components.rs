pub use crate::os::*;

pub struct Components<'path> {
    source: &'path str,
    pos: usize,
}

impl<'path> Components<'path> {
    #[must_use]
    pub const fn new(source: &'path str) -> Self {
        Self { source, pos: 0 }
    }
}

impl<'path> Iterator for Components<'path> {
    type Item = &'path str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.source.len() {
            return None;
        }
        let start = self.pos;

        for (off, &b) in self.source.as_bytes()[start..].iter().enumerate() {
            if SEPARATOR.contains(&b) {
                self.pos = start + off + 1;
                return Some(&self.source[start..start + off]);
            }
        }

        self.pos = self.source.len();
        Some(&self.source[start..])
    }
}

#[cfg(test)]
mod tests {
    use crate::Components;

    macro_rules! next_is {
        ($comps:ident, $item:literal) => {
            assert_eq!($comps.next(), Some($item));
        };
    }

    #[test]
    fn components_test() {
        let mut comps = Components::new(r"C:\User\rustling/Проекты/Rust/");
        next_is!(comps, "C:");
        next_is!(comps, "User");
        next_is!(comps, "rustling");
        next_is!(comps, "Проекты");
        next_is!(comps, "Rust");
        assert_eq!(comps.next(), None);
    }
}
