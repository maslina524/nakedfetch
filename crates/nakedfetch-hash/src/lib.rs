// SOURCE:
// https://github.com/python/cpython/blob/main/Modules/zlibmodule.c

static CRC32_TABLE: [u32; 256] = generate_table();

const fn generate_table() -> [u32; 256] {
    let mut table = [0u32; 256];

    let mut i = 0u32;
    while i < 256 {
        let mut c = i;
        let mut j = 0;
        while j < 8 {
            c = if c & 1 != 0 {
                0xED_B8_83_20 ^ (c >> 1)
            } else {
                c >> 1
            };

            j += 1;
        }

        table[i as usize] = c;
        i += 1;
    }

    table
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crc32;

impl Crc32 {
    #[must_use]
    pub const fn hash_with_init(buf: &[u8], init: u32) -> u32 {
        let mut crc = init ^ 0xFF_FF_FF_FF;
        let mut pos = 0usize;

        while pos < buf.len() {
            let index = (crc ^ buf[pos] as u32) & 0xFF;
            crc = CRC32_TABLE[index as usize] ^ (crc >> 8);
            pos += 1;
        }

        crc ^ 0xFF_FF_FF_FF
    }

    #[must_use]
    pub const fn hash_png(buf: &[u8]) -> u32 {
        Self::hash_with_init(buf, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::Crc32;

    #[test]
    fn table_gen_test() {
        for i in &super::CRC32_TABLE {
            print!("{i:08x} ");
        }
    }

    #[test]
    fn encrypt_png_test() {
        let source = "Hello World";
        let correct_hash = 0x4A_17_B1_56u32;
        let my_hash = Crc32::hash_png(source.as_bytes());
        assert_eq!(correct_hash, my_hash);
    }
}
