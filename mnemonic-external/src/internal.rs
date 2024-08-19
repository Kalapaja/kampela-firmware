#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String};

#[cfg(feature = "std")]
use std::{vec::Vec, string::String};

use crate::error::ErrorWordList;
use crate::wordlist::WORDLIST_ENGLISH;
use crate::{AsWordList, Bits11, WordListElement};

pub struct InternalWordList;

impl AsWordList for InternalWordList {

    fn get_word(bits: Bits11) -> Result<WordListElement, ErrorWordList> {
        let word_order = bits.bits() as usize;
        match WORDLIST_ENGLISH.get(word_order) {
            Some(word) => Ok(WordListElement{word: String::from(*word), bits11: bits}),
            None => Err(ErrorWordList::InvalidWordNumber)
        }
    }

    fn get_words_by_prefix(prefix: &str) -> Vec<WordListElement> {
        let mut out = Vec::<WordListElement>::new();
        for (i, word) in WORDLIST_ENGLISH.iter().enumerate() {
            if word.starts_with(prefix) {
                out.push(WordListElement{word: String::from(*word), bits11: Bits11::from(i as u16).expect("Wordlist suppose contain no more words than TOTAL_WORDS")});
            }
        }
        out
    }

    fn bits11_for_word(word: &str) -> Result<Bits11, ErrorWordList> {
        for (i, element) in WORDLIST_ENGLISH.iter().enumerate() {
            if element == &word {
                return Bits11::from(i as u16);
            }
        }
        Err(ErrorWordList::NoWord)
    }
}