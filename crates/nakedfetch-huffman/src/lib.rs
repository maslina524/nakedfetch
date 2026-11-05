#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod stream;
mod tree;

pub use stream::Stream;
pub use tree::{HuffmanTree, decode_symb};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_read_byte() {
        let data = [0x12, 0x34, 0x56];
        let mut s = Stream::new(&data);
        assert_eq!(s.read_byte(), 0x12);
        assert_eq!(s.read_byte(), 0x34);
        assert_eq!(s.read_byte(), 0x56);
    }

    #[test]
    fn stream_read_bytes_le() {
        let data = [0x34, 0x12, 0x78, 0x56];
        let mut s = Stream::new(&data);
        assert_eq!(s.read_bytes(2), 0x1234);
        assert_eq!(s.read_bytes(2), 0x5678);
    }

    #[test]
    fn stream_read_bytes_zero() {
        let data = [0xAA];
        let mut s = Stream::new(&data);
        assert_eq!(s.read_bytes(0), 0);
        assert_eq!(s.read_byte(), 0xAA);
    }

    #[test]
    fn stream_read_bit_lsb_first() {
        let data = [0b1011_0101, 0xAA];
        let mut s = Stream::new(&data);
        assert_eq!(s.read_bit(), 1);
        assert_eq!(s.read_bit(), 0);
        assert_eq!(s.read_bit(), 1);
        assert_eq!(s.read_bit(), 0);
        assert_eq!(s.read_bit(), 1);
        assert_eq!(s.read_bit(), 1);
        assert_eq!(s.read_bit(), 0);
        assert_eq!(s.read_bit(), 1);
        assert_eq!(s.read_byte(), 0xAA);
    }

    #[test]
    fn stream_read_bit_across_bytes() {
        let data = [0xFF, 0x00];
        let mut s = Stream::new(&data);
        for _ in 0..8 {
            assert_eq!(s.read_bit(), 1);
        }
        assert_eq!(s.read_bit(), 0);
    }

    #[test]
    fn stream_read_bits_lsb_first() {
        let data = [0b1011_0101];
        let mut s = Stream::new(&data);
        assert_eq!(s.read_bits(4), 5);
        assert_eq!(s.read_bits(4), 11);
    }

    #[test]
    fn stream_read_bits_across_bytes() {
        let data = [0xFF, 0x01];
        let mut s = Stream::new(&data);
        assert_eq!(s.read_bits(9), 0x1FF);
    }

    #[test]
    fn stream_read_bits_zero() {
        let data = [0xAA];
        let mut s = Stream::new(&data);
        assert_eq!(s.read_bits(0), 0);
        assert_eq!(s.read_byte(), 0xAA);
    }

    #[test]
    fn stream_align_to_byte() {
        let data = [0b1011_0101, 0x42];
        let mut s = Stream::new(&data);
        let _ = s.read_bit();
        let _ = s.read_bit();
        let _ = s.read_bit();
        s.align_to_byte();
        assert_eq!(s.read_byte(), 0x42);
    }

    #[test]
    fn stream_align_to_byte_when_aligned() {
        let data = [0x12u8, 0x34];
        let mut s = Stream::new(&data);
        s.align_to_byte();
        assert_eq!(s.read_byte(), 0x12);
    }

    #[test]
    fn stream_read_byte_skips_partial() {
        let data = [0x12, 0x34, 0x56];
        let mut s = Stream::new(&data);
        let _ = s.read_bit();
        assert_eq!(s.read_byte(), 0x34);
        assert_eq!(s.read_byte(), 0x56);
    }

    #[test]
    fn tree_from_empty() {
        let alphabet: &[u32] = &[];
        let bl: &[u8] = &[];
        let tree = HuffmanTree::from_alphabet_and_bl_list(alphabet, bl);

        let data = [0];
        let mut s = Stream::new(&data);
        assert_eq!(decode_symb(&mut s, &tree), None);
    }

    #[test]
    fn tree_all_zero_lengths() {
        let tree = HuffmanTree::from_alphabet_and_bl_list(&[1, 2, 3], &[0, 0, 0]);
        let data = [0];
        let mut s = Stream::new(&data);
        assert_eq!(decode_symb(&mut s, &tree), None);
    }

    #[test]
    fn tree_single_symbol() {
        let tree = HuffmanTree::from_alphabet_and_bl_list(&[42], &[1]);

        let data = [0b0];
        let mut s = Stream::new(&data);
        assert_eq!(decode_symb(&mut s, &tree), Some(42));

        let data = [0b1];
        let mut s = Stream::new(&data);
        assert_eq!(decode_symb(&mut s, &tree), None);
    }

    #[test]
    fn tree_canonical_codes() {
        let tree = HuffmanTree::from_alphabet_and_bl_list(&[65, 66, 67], &[1, 2, 2]);

        let data = [0b0];
        let mut s = Stream::new(&data);
        assert_eq!(decode_symb(&mut s, &tree), Some(65));

        let data = [0b01];
        let mut s = Stream::new(&data);
        assert_eq!(decode_symb(&mut s, &tree), Some(66));

        let data = [0b11];
        let mut s = Stream::new(&data);
        assert_eq!(decode_symb(&mut s, &tree), Some(67));
    }

    #[test]
    fn tree_decode_sequence() {
        let tree = HuffmanTree::from_alphabet_and_bl_list(&[65, 66, 67], &[1, 2, 2]);
        let data = [0b11010];
        let mut s = Stream::new(&data);
        assert_eq!(decode_symb(&mut s, &tree), Some(65));
        assert_eq!(decode_symb(&mut s, &tree), Some(66));
        assert_eq!(decode_symb(&mut s, &tree), Some(67));
    }

    #[test]
    fn tree_filters_zero_lengths() {
        let tree = HuffmanTree::from_alphabet_and_bl_list(&[10, 20, 30], &[0, 2, 2]);

        let data = [0b00];
        let mut s = Stream::new(&data);
        assert_eq!(decode_symb(&mut s, &tree), Some(20));

        let data = [0b10];
        let mut s = Stream::new(&data);
        assert_eq!(decode_symb(&mut s, &tree), Some(30));
    }

    #[test]
    fn tree_insert_manual() {
        let mut tree = HuffmanTree::new();
        tree.insert(0b0, 1, 100);
        tree.insert(0b10, 2, 200);
        tree.insert(0b11, 2, 201);

        let data = [0];
        let mut s = Stream::new(&data);
        assert_eq!(decode_symb(&mut s, &tree), Some(100));

        let data = [0b01];
        let mut s = Stream::new(&data);
        assert_eq!(decode_symb(&mut s, &tree), Some(200));

        let data = [0b11];
        let mut s = Stream::new(&data);
        assert_eq!(decode_symb(&mut s, &tree), Some(201));
    }

    #[test]
    fn tree_insert_invalid_path_returns_none() {
        let mut tree = HuffmanTree::new();
        tree.insert(0, 1, 7);
        let data = [0b1u8];
        let mut s = Stream::new(&data);
        assert_eq!(decode_symb(&mut s, &tree), None);
    }

    #[test]
    fn tree_roundtrip_many_symbols() {
        let alphabet = [0, 1, 2, 3, 4];
        let lens = [2, 2, 2, 3, 3];
        let tree = HuffmanTree::from_alphabet_and_bl_list(&alphabet, &lens);

        let cases: [(u8, u32); 5] = [(0b000, 0), (0b010, 1), (0b001, 2), (0b011, 3), (0b111, 4)];

        for &(byte, expected) in &cases {
            let data = [byte];
            let mut s = Stream::new(&data);
            assert_eq!(
                decode_symb(&mut s, &tree),
                Some(expected),
                "byte = {byte:#05b}"
            );
        }
    }
}
