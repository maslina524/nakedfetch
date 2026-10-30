use core::cmp::Ordering;

use alloc::vec::Vec;

use huffman::decode_symb;
use huffman::{HuffmanTree, Stream};

use crate::DeflateError;
use crate::consts;

pub fn decode_block(
    stream: &mut Stream,
    output: &mut Vec<u8>,
    litlen_tree: &HuffmanTree,
    dist_tree: &HuffmanTree,
) -> Result<(), DeflateError> {
    loop {
        let sym = decode_symb(stream, litlen_tree).ok_or(DeflateError::DecodeError)?;

        match sym.cmp(&256) {
            Ordering::Less => output.push(sym as u8),
            Ordering::Greater => {
                let idx = (sym - 257) as usize;
                if idx >= consts::LENGTH_BASE.len() {
                    return Err(DeflateError::InvalidSymbol);
                }
                let (base, extra_bits) = consts::LENGTH_BASE[idx];
                let length = base + stream.read_bits(extra_bits as usize) as usize;

                let dist_sym = decode_symb(stream, dist_tree).ok_or(DeflateError::DecodeError)?;
                if dist_sym >= 30 {
                    return Err(DeflateError::InvalidDistanceSymbol);
                }
                let (dist_base, dist_extra) = consts::DIST_BASE[dist_sym as usize];
                let distance = dist_base + stream.read_bits(dist_extra as usize) as usize;

                if distance > output.len() {
                    return Err(DeflateError::DistanceTooFar);
                }

                let start = output.len() - distance;
                for i in 0..length {
                    output.push(output[start + i]);
                }
            },
            Ordering::Equal => break,
        }
    }
    Ok(())
}