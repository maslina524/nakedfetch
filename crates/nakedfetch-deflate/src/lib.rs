#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

mod block;
mod consts;
mod tree;

use alloc::vec::Vec;
use core::error::Error;

use huffman::Stream;

use crate::{
    block::decode_block,
    tree::{build_dynamic_trees, build_fixed_trees},
};

#[derive(Debug)]
pub enum DeflateError {
    InvalidBtype,
    InvalidNlen,
    DecodeError,
    InvalidSymbol,
    InvalidDistanceSymbol,
    DistanceTooFar,
    DynamicTreeError,
    EndOfStream,
}

impl core::fmt::Display for DeflateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidBtype => write!(f, "Invalid byte"),
            Self::InvalidNlen => write!(f, "Invalid nlen"),
            Self::DecodeError => write!(f, "Decode error"),
            Self::InvalidSymbol => write!(f, "Invalid symbol"),
            Self::InvalidDistanceSymbol => write!(f, "Invalid distance symbol"),
            Self::DistanceTooFar => write!(f, "Distance too far"),
            Self::DynamicTreeError => write!(f, "Dynamic tree error"),
            Self::EndOfStream => write!(f, "End of stream"),
        }
    }
}

impl Error for DeflateError {}

pub(crate) fn extend_at<T: Clone>(dst: &mut [T], offset: usize, src: impl IntoIterator<Item = T>) {
    for (slot, val) in dst[offset..].iter_mut().zip(src) {
        *slot = val;
    }
}

/// # Errors
///
/// `Err` when passing an invalid `deflate` sequence.
pub fn decode(input: &[u8]) -> Result<Vec<u8>, DeflateError> {
    let mut stream = Stream::new(input);
    let mut output = Vec::with_capacity(input.len());

    loop {
        let bfinal = stream.read_bit();
        let btype = stream.read_bits(2) as u8;

        match btype {
            0 => {
                stream.align_to_byte();
                let len = stream.read_bytes(2) as usize;
                let nlen = stream.read_bytes(2) as usize;
                if (len ^ 0xFFFF) != nlen {
                    return Err(DeflateError::InvalidNlen);
                }
                for _ in 0..len {
                    output.push(stream.read_byte());
                }
            },
            1 => {
                let (litlen_tree, dist_tree) = build_fixed_trees();
                decode_block(&mut stream, &mut output, &litlen_tree, &dist_tree)?;
            },
            2 => {
                let (litlen_tree, dist_tree) = build_dynamic_trees(&mut stream)?;
                decode_block(&mut stream, &mut output, &litlen_tree, &dist_tree)?;
            },
            _ => {
                return Err(DeflateError::InvalidBtype);
            },
        }

        if bfinal == 1 {
            break;
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use huffman::HuffmanTree;

    use super::*;

    #[test]
    fn extend_at_full_fill() {
        let mut dst = [0; 4];
        extend_at(&mut dst, 0, [1, 2, 3, 4]);
        assert_eq!(dst, [1, 2, 3, 4]);
    }

    #[test]
    fn extend_at_with_offset() {
        let mut dst = [0; 5];
        extend_at(&mut dst, 2, [7, 8, 9]);
        assert_eq!(dst, [0, 0, 7, 8, 9]);
    }

    #[test]
    fn extend_at_truncates_overflow() {
        let mut dst = [0; 3];
        extend_at(&mut dst, 0, [10, 20, 30, 40, 50]);
        assert_eq!(dst, [10, 20, 30]);
    }

    #[test]
    fn extend_at_empty_iterator() {
        let mut dst = [5; 3];
        extend_at(&mut dst, 1, []);
        assert_eq!(dst, [5, 5, 5]);
    }

    #[test]
    fn decode_stored_hello() {
        let input = [0x01, 0x05, 0x00, 0xFA, 0xFF, 0x48, 0x65, 0x6C, 0x6C, 0x6F];
        assert_eq!(decode(&input).unwrap(), b"Hello".to_vec());
    }

    #[test]
    fn decode_stored_empty() {
        let input = [0x01, 0x00, 0x00, 0xFF, 0xFF];
        assert_eq!(decode(&input).unwrap(), Vec::new());
    }

    #[test]
    fn decode_stored_invalid_nlen() {
        let input = [0x01, 0x05, 0x00, 0x00, 0x00];
        assert!(matches!(decode(&input), Err(DeflateError::InvalidNlen)));
    }

    #[test]
    fn decode_invalid_btype() {
        let input = [0x07];
        assert!(matches!(decode(&input), Err(DeflateError::InvalidBtype)));
    }

    #[test]
    fn decode_multiple_stored_blocks() {
        let input = [
            0x00, 0x03, 0x00, 0xFC, 0xFF, 0x41, 0x42, 0x43, 0x01, 0x02, 0x00, 0xFD, 0xFF, 0x44,
            0x45,
        ];
        assert_eq!(decode(&input).unwrap(), b"ABCDE".to_vec());
    }

    #[test]
    fn build_fixed_trees_smoke() {
        let (litlen, dist) = build_fixed_trees();
        let _ = &litlen;
        let _ = &dist;
    }

    #[test]
    fn decode_block_eob_immediately() {
        let mut litlen = HuffmanTree::new();
        litlen.insert(0, 1, 256);

        let mut dist = HuffmanTree::new();
        dist.insert(0, 1, 0);

        let data = [0x00];
        let mut s = Stream::new(&data);
        let mut out = Vec::new();

        decode_block(&mut s, &mut out, &litlen, &dist).unwrap();
        assert_eq!(out, Vec::new());
    }

    #[test]
    fn decode_block_literals() {
        let mut litlen = HuffmanTree::new();
        litlen.insert(0, 1, 65);
        litlen.insert(3, 2, 256);

        let mut dist = HuffmanTree::new();
        dist.insert(0, 1, 0);

        let data = [0x0C];
        let mut s = Stream::new(&data);
        let mut out = Vec::new();

        decode_block(&mut s, &mut out, &litlen, &dist).unwrap();
        assert_eq!(out, b"AA".to_vec());
    }

    #[test]
    fn decode_block_lz77_match() {
        let mut litlen = HuffmanTree::new();
        litlen.insert(0, 1, 65);
        litlen.insert(2, 2, 257);
        litlen.insert(3, 2, 256);

        let mut dist = HuffmanTree::new();
        dist.insert(0, 1, 0);

        let data = [0x32];
        let mut s = Stream::new(&data);
        let mut out = Vec::new();

        decode_block(&mut s, &mut out, &litlen, &dist).unwrap();
        assert_eq!(out, b"AAAA".to_vec());
    }

    #[test]
    fn decode_block_invalid_symbol() {
        let mut litlen = HuffmanTree::new();
        litlen.insert(0, 1, 65);
        litlen.insert(2, 2, 286);

        let mut dist = HuffmanTree::new();
        dist.insert(0, 1, 0);

        let data = [0x02];
        let mut s = Stream::new(&data);
        let mut out = Vec::new();

        let result = decode_block(&mut s, &mut out, &litlen, &dist);
        assert!(matches!(result, Err(DeflateError::InvalidSymbol)));
    }

    #[test]
    fn decode_block_invalid_distance_symbol() {
        let mut litlen = HuffmanTree::new();
        litlen.insert(0, 1, 65);
        litlen.insert(2, 2, 257);

        let mut dist = HuffmanTree::new();
        dist.insert(0, 1, 30);

        let data = [0x02];
        let mut s = Stream::new(&data);
        let mut out = Vec::new();

        let result = decode_block(&mut s, &mut out, &litlen, &dist);
        assert!(matches!(result, Err(DeflateError::InvalidDistanceSymbol)));
    }

    #[test]
    fn decode_block_distance_too_far() {
        let mut litlen = HuffmanTree::new();
        litlen.insert(0, 1, 65);
        litlen.insert(2, 2, 257);

        let mut dist = HuffmanTree::new();
        dist.insert(0, 1, 4);

        let data = [0x02];
        let mut s = Stream::new(&data);
        let mut out = Vec::new();

        let result = decode_block(&mut s, &mut out, &litlen, &dist);
        assert!(matches!(result, Err(DeflateError::DistanceTooFar)));
    }

    #[test]
    fn decode_block_fixed_eob_only() {
        let data = [0xF3, 0x00];
        assert_eq!(decode(&data).unwrap(), Vec::new());
    }

    #[test]
    fn deflate_error_display() {
        let cases = [
            (DeflateError::InvalidBtype, "Invalid byte"),
            (DeflateError::InvalidNlen, "Invalid nlen"),
            (DeflateError::DecodeError, "Decode error"),
            (DeflateError::InvalidSymbol, "Invalid symbol"),
            (
                DeflateError::InvalidDistanceSymbol,
                "Invalid distance symbol",
            ),
            (DeflateError::DistanceTooFar, "Distance too far"),
            (DeflateError::DynamicTreeError, "Dynamic tree error"),
            (DeflateError::EndOfStream, "End of stream"),
        ];

        for (err, expected) in cases {
            let mut s = String::new();
            core::fmt::Write::write_fmt(&mut s, format_args!("{err}")).unwrap();
            assert_eq!(s, expected);
        }
    }
}
