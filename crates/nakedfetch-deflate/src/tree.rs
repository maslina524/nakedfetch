use alloc::vec::Vec;
use core::iter::repeat_n;

use huffman::{HuffmanTree, Stream, decode_symb};

use crate::{DeflateError, consts, extend_at};

pub fn build_fixed_trees() -> (HuffmanTree, HuffmanTree) {
    let mut litlen_lens = [0u8; 288];
    for (i, item) in litlen_lens.iter_mut().enumerate() {
        *item = match i {
            i if (144..256).contains(&i) => 9,
            i if (256..280).contains(&i) => 7,
            _ => 8,
        }
    }

    let mut litlen_alphabet = [0u32; 288];
    extend_at(&mut litlen_alphabet, 0, 0..288);
    let litlen_tree = HuffmanTree::from_alphabet_and_bl_list(&litlen_alphabet, &litlen_lens);

    let dist_lens = [5u8; 32];
    let mut dist_alphabet = [0u32; 32];
    extend_at(&mut dist_alphabet, 0, 0..32);
    let dist_tree = HuffmanTree::from_alphabet_and_bl_list(&dist_alphabet, &dist_lens);

    (litlen_tree, dist_tree)
}

pub fn build_dynamic_trees(
    stream: &mut Stream<'_>,
) -> Result<(HuffmanTree, HuffmanTree), DeflateError> {
    let hlit = stream.read_bits(5) as usize + 257;
    let hdist = stream.read_bits(5) as usize + 1;
    let hclen = stream.read_bits(4) as usize + 4;

    let mut clen_lens = [0u8; 19];
    for i in 0..hclen {
        clen_lens[consts::CLEN_ORDER[i]] = stream.read_bits(3) as u8;
    }

    let mut clen_alphabet = [0u32; 19];
    extend_at(&mut clen_alphabet, 0, 0..19);
    let clen_tree = HuffmanTree::from_alphabet_and_bl_list(&clen_alphabet, &clen_lens);

    let mut lens: Vec<u8> = Vec::with_capacity(hlit + hdist);
    while lens.len() < hlit + hdist {
        let sym = decode_symb(stream, &clen_tree).ok_or(DeflateError::DecodeError)?;
        match sym {
            0..=15 => lens.push(sym as u8),
            16 => {
                let repeat = stream.read_bits(2) as usize + 3;
                let last = *lens.last().ok_or(DeflateError::DecodeError)?;
                lens.extend(repeat_n(last, repeat));
            },
            17 => {
                let repeat = stream.read_bits(3) as usize + 3;
                lens.extend(repeat_n(0, repeat));
            },
            18 => {
                let repeat = stream.read_bits(7) as usize + 11;
                lens.extend(repeat_n(0, repeat));
            },
            _ => return Err(DeflateError::DynamicTreeError),
        }
    }

    let litlen_lens = &lens[..hlit];
    let dist_lens = &lens[hlit..hlit + hdist];

    let litlen_alphabet: Vec<u32> = (0..hlit as u32).collect();
    let dist_alphabet: Vec<u32> = (0..hdist as u32).collect();

    let litlen_tree = HuffmanTree::from_alphabet_and_bl_list(&litlen_alphabet, litlen_lens);
    let dist_tree = HuffmanTree::from_alphabet_and_bl_list(&dist_alphabet, dist_lens);

    Ok((litlen_tree, dist_tree))
}
