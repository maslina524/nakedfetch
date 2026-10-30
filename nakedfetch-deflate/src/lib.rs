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
    use crate::decode;

    #[test]
    fn decode_test() {
        let encoded = b"\xf3H\xcd\xc9\xc9WpIM\xcbI,IU\x84\x00\x00";
        let decoded = decode(encoded).unwrap();
        let string = String::from_utf8(decoded).unwrap();

        assert_eq!("Hello Deflate!!!!!!!", string);
    }
}
