#![no_std]
#![deny(unused_crate_dependencies)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec, borrow::ToOwned};
#[cfg(not(feature = "std"))]
use core::cmp::Ordering;

#[cfg(feature = "std")]
use std::{string::String, vec::Vec, cmp::Ordering, borrow::ToOwned};

use bitvec::prelude::{BitSlice, BitVec, Msb0};
use sha2::{Digest, Sha256};

pub mod error;

#[cfg(feature = "sufficient-memory")]
pub mod internal;
#[cfg(feature = "embed")]
pub mod external;

#[cfg(test)]
mod tests;

#[cfg(any(feature = "sufficient-memory", test))]
pub mod wordlist;

pub use crate::error::ErrorWordList;

pub const TOTAL_WORDS: usize = 2048;
pub const WORD_MAX_LEN: usize = 8;
pub const SEPARATOR_LEN: usize = 1;

pub const MAX_SEED_LEN: usize = 24;

#[derive(Clone, Copy, Debug, Ord, Eq, PartialEq, PartialOrd)]
pub struct Bits11(u16);

impl Bits11 {
    pub fn bits(self) -> u16 {
        self.0
    }
    pub fn from(i: u16) -> Result<Self, ErrorWordList> {
        if (i as usize) < TOTAL_WORDS {
            Ok(Self(i))
        } else {
            Err(ErrorWordList::InvalidWordNumber)
        }
    }
    pub fn to_wordlist_element<L: AsWordList>(
        &self,
    ) -> Result<WordListElement, ErrorWordList>{
        match L::get_word(*self) {
            Ok(word) => Ok(word),
            Err(e) => Err(e),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WordListElement {
    pub word: String,
    pub bits11: Bits11,
}

pub trait AsWordList {
    fn get_word(bits: Bits11) -> Result<WordListElement, ErrorWordList>;
    fn get_words_by_prefix(prefix: &str) -> Vec<WordListElement>;
    fn bits11_for_word(word: &str) -> Result<Bits11, ErrorWordList>;
    
    fn words_to_phrase(words: &Vec<WordListElement>) -> String {
        words
            .iter()
            .map(|a| a.word.to_owned())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MnemonicType {
    Words12,
    Words15,
    Words18,
    Words21,
    Words24,
}

impl MnemonicType {
    fn from(len: usize) -> Result<Self, ErrorWordList> {
        match len {
            12 => Ok(Self::Words12),
            15 => Ok(Self::Words15),
            18 => Ok(Self::Words18),
            21 => Ok(Self::Words21),
            24 => Ok(Self::Words24),
            _ => Err(ErrorWordList::WordsNumber),
        }
    }
    fn checksum_bits(&self) -> u8 {
        match &self {
            Self::Words12 => 4,
            Self::Words15 => 5,
            Self::Words18 => 6,
            Self::Words21 => 7,
            Self::Words24 => 8,
        }
    }
    fn entropy_bits(&self) -> usize {
        match &self {
            Self::Words12 => 128,
            Self::Words15 => 160,
            Self::Words18 => 192,
            Self::Words21 => 224,
            Self::Words24 => 256,
        }
    }
    fn total_bits(&self) -> usize {
        self.entropy_bits() + self.checksum_bits() as usize
    }
}

#[derive(Clone, Debug)]
pub struct WordSet {
    pub bits11_set: Vec<Bits11>,
}

pub struct WordSetIterator {
    wordset: WordSet,
    index: usize,
}

impl Iterator for WordSetIterator {
    type Item = Bits11;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.wordset.get(self.index).cloned();
        self.index += 1;
        result
    }
}

impl<'a> FromIterator<&'a WordListElement> for WordSet {
    fn from_iter<T: IntoIterator<Item = &'a WordListElement>>(iter: T) -> Self {
        let mut c = WordSet::new();

        for i in iter {
            c.bits11_set.push(i.bits11);
        }

        c
    }
}

impl FromIterator<WordListElement> for WordSet {
    fn from_iter<T: IntoIterator<Item = WordListElement>>(iter: T) -> Self {
        let mut c = WordSet::new();

        for i in iter {
            c.bits11_set.push(i.bits11);
        }

        c
    }
}

impl<'a> FromIterator<&'a Bits11> for WordSet {
    fn from_iter<T: IntoIterator<Item = &'a Bits11>>(iter: T) -> Self {
        let mut c = WordSet::new();

        for i in iter {
            c.bits11_set.push(*i);
        }

        c
    }
}

impl FromIterator<Bits11> for WordSet {
    fn from_iter<T: IntoIterator<Item = Bits11>>(iter: T) -> Self {
        let mut c = WordSet::new();

        for i in iter {
            c.bits11_set.push(i);
        }

        c
    }
}

impl WordSet {
    pub fn get(&self, index: usize) -> Option<&Bits11> {
        self.bits11_set.get(index)
    }

    pub fn iter(&self) -> WordSetIterator {
        WordSetIterator { wordset: self.to_owned(), index: 0 }
    }

    pub fn len(&self) -> usize {
        self.bits11_set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bits11_set.is_empty()
    }

    pub fn push(&mut self, bits11: Bits11) {
        self.bits11_set.push(bits11)
    }

    pub fn append(&mut self, appendix: &mut Self) {
        self.bits11_set.append(&mut appendix.bits11_set)
    }

    pub fn sort(&mut self) {
        self.bits11_set.sort();
    }

    pub fn sort_by<L, F>(&mut self, mut compare: F)
    where
        L: AsWordList,
        F: FnMut(&WordListElement, &WordListElement) -> Ordering,
    {
        let msg: &'static str = "sorted wordset should contain only wordlist elements";
        self.bits11_set.sort_by(|a: &Bits11, b: &Bits11| {
            compare(&a.to_wordlist_element::<L>().expect(msg), &b.to_wordlist_element::<L>().expect(msg))
        });
    }

    pub fn from_entropy(entropy: &[u8]) -> Result<Self, ErrorWordList> {
        if entropy.len() < 16 || entropy.len() > 32 || entropy.len() % 4 != 0 {
            return Err(ErrorWordList::InvalidEntropy);
        }

        let checksum_byte = sha256_first_byte(entropy);
        let mut entropy_bits: BitVec<u8, Msb0> = BitVec::with_capacity((entropy.len() + 1) * 8);
        entropy_bits.extend_from_bitslice(&BitVec::<u8, Msb0>::from_slice(entropy));
        entropy_bits.extend_from_bitslice(&BitVec::<u8, Msb0>::from_element(checksum_byte));

        let mut bits11_set: Vec<Bits11> = Vec::new();
        for chunk in entropy_bits.chunks_exact(11usize) {
            let mut bits11: u16 = 0;
            for (i, bit) in chunk.into_iter().enumerate() {
                if *bit {
                    bits11 |= 1 << (10 - i)
                }
            }
            bits11_set.push(Bits11(bits11));
        }
        Ok(Self { bits11_set })
    }

    pub fn new() -> Self {
        Self {
            bits11_set: Vec::with_capacity(MAX_SEED_LEN),
        }
    }

    pub fn add_word<L: AsWordList>(
        &mut self,
        word: &str,
    ) -> Result<(), ErrorWordList> {
        if self.bits11_set.len() < MAX_SEED_LEN {
            let bits11 = L::bits11_for_word(word)?;
            self.bits11_set.push(bits11);
        }
        Ok(())
    }

    pub fn is_finalizable(&self) -> bool {
        MnemonicType::from(self.bits11_set.len()).is_ok()
    }

    pub fn to_entropy(&self) -> Result<Vec<u8>, ErrorWordList> {
        let mnemonic_type = MnemonicType::from(self.bits11_set.len())?;

        let mut entropy_bits: BitVec<u8, Msb0> = BitVec::with_capacity(mnemonic_type.total_bits());

        for bits11 in self.bits11_set.iter() {
            entropy_bits.extend_from_bitslice(
                &BitSlice::<u8, Msb0>::from_slice(&bits11.bits().to_be_bytes())[5..16],
            )
        }

        let mut entropy = entropy_bits.into_vec();
        let entropy_len = mnemonic_type.entropy_bits() / 8;

        let actual_checksum = checksum(entropy[entropy_len], mnemonic_type.checksum_bits());

        entropy.truncate(entropy_len);

        let checksum_byte = sha256_first_byte(&entropy);

        let expected_checksum = checksum(checksum_byte, mnemonic_type.checksum_bits());

        if actual_checksum != expected_checksum {
            Err(ErrorWordList::InvalidChecksum)
        } else {
            Ok(entropy)
        }
    }

    pub fn to_wordlist<L: AsWordList>(&self) -> Result<Vec<WordListElement>, ErrorWordList> {
        let mut wordlist = Vec::new();
        for bits in self.iter() {
            wordlist.push(L::get_word(bits)?)
        }
        Ok(wordlist)
    }

    pub fn to_phrase<L: AsWordList>(&self) -> Result<String, ErrorWordList> {
        let mut phrase =
            String::with_capacity(self.bits11_set.len() * (WORD_MAX_LEN + SEPARATOR_LEN) - 1);
        for bits11 in self.bits11_set.iter() {
            if !phrase.is_empty() {
                phrase.push(' ')
            }
            let word = L::get_word(*bits11)?;
            phrase.push_str(word.word.as_ref());
        }
        Ok(phrase)
    }
}

fn checksum(source: u8, bits: u8) -> u8 {
    assert!(bits <= 8);
    source >> (8 - bits)
}

fn sha256_first_byte(input: &[u8]) -> u8 {
    Sha256::digest(input)[0]
}
