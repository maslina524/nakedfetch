use core::mem::MaybeUninit;

use path::Path;

use crate::fs::{File, FileByLine, FileByLineError, FileCreateError, Read};

pub struct MapFile {
    lines: FileByLine<Read>,
    sep: char,
}

impl MapFile {
    pub fn new<T: AsRef<Path>>(path: T, sep: char) -> Result<Self, FileCreateError> {
        let file = File::open(path)?;
        Ok(Self {
            lines: FileByLine::new(file),
            sep,
        })
    }

    fn next<'a>(
        &mut self,
        buf: &'a mut [MaybeUninit<u8>],
    ) -> Result<Option<(&'a str, &'a str)>, FileByLineError> {
        let mut s;
        let sep_idx;
        loop {
            s = if let Some(string) = self.lines.next(buf)? {
                string
            } else {
                return Ok(None);
            };

            if let Some(idx) = s.find(self.sep) {
                sep_idx = idx;
                break;
            }
        }

        let k = s[..sep_idx].trim();
        let mut v = s[sep_idx + 1..].trim();
        if v.starts_with('"') && v.ends_with('"') {
            v = &v[1..v.len() - 1];
        }

        Ok(Some((k, v)))
    }

    pub fn get<'a>(
        &mut self,
        buf: &'a mut [MaybeUninit<u8>],
        key: &str,
    ) -> Result<Option<&'a str>, FileByLineError> {
        *self.lines.pos_mut() = 0;
        while let Some((k, v)) = self.next(buf)? {
            if k == key {
                return Ok(Some(v));
            }
        }
        Ok(None)
    }

    pub fn get_or<'a>(
        &mut self,
        buf: &'a mut [MaybeUninit<u8>],
        key: &str,
        default: &'a str,
    ) -> Result<&'a str, FileByLineError> {
        self.get(buf, key).map(|v| v.unwrap_or(default))
    }
}

#[cfg(test)]
mod tests {
    use core::mem::MaybeUninit;

    use crate::unix::MapFile;

    const CONTENT: &str = "
        Name: \"Rustling\"\n
        Age: 18\n
        IsStudent: True\n
    ";

    #[test]
    fn parse_map_file() {
        let path = "Common-Unix-MapFile-1";
        std::fs::write(path, CONTENT).expect("std::fs::write failed");

        let mut buf = [MaybeUninit::uninit(); 512];
        let mut map = MapFile::new(path, ':').expect("Failed to open file");

        assert_eq!(
            map.get(&mut buf, "Name").expect("Failed to read file"),
            Some("Rustling")
        );
        assert_eq!(
            map.get(&mut buf, "Name").expect("Failed to read file"),
            Some("Rustling")
        );
        assert_eq!(
            map.get(&mut buf, "Name").expect("Failed to read file"),
            Some("Rustling")
        );

        assert_eq!(
            map.get(&mut buf, "IsStudent").expect("Failed to read file"),
            Some("True")
        );
        assert_eq!(
            map.get(&mut buf, "Age").expect("Failed to read file"),
            Some("18")
        );
        assert_eq!(
            map.get(&mut buf, "NotExists").expect("Failed to read file"),
            None
        );
    }
}
