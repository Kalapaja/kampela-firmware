use alloc::{borrow::ToOwned, string::String, vec::Vec};

use core::ops::Rem;

//use crate::wordlist::WORDLIST_ENGLISH;
use mnemonic_external::{AsWordList, Bits11, WordListElement, TOTAL_WORDS, WORD_MAX_LEN, error::ErrorMnemonic};

use crate::{devices::psram::{psram_read_at_address, psram_read_byte, psram_write_byte, psram_write_read_byte, psram_write_slice, AddressPsram, PSRAM_PAGE_SIZE, PSRAM_READ}, in_free, peripherals::eusart::{deselect_psram, select_psram}};
const WORDLIST_STARTS: [u32; 26] = [
    0, 136, 253, 439, 551, 651, 757,
    833, 897, 952, 972, 992, 1068, 1173,
    1214, 1269, 1401, 1409, 1517, 1767, 1888,
    1923, 1969, 2038, 2038, 2044
];
const FIRST_WORDLIST_STARTS: u8 = 0x61;
pub const WORDLIST_SIZE: u32 = TOTAL_WORDS as u32 * WORD_MAX_LEN as u32; //aligned to pages
pub const WORDLIST_PAGES: u32 = WORDLIST_SIZE / PSRAM_PAGE_SIZE;
pub const WORDS_IN_PAGE:u32 = PSRAM_PAGE_SIZE / WORD_MAX_LEN as u32;

pub const MAX_PROPOSAL: usize = 3;
pub const WORDLIST_BASE: u32 = 128*256;

struct Bits11ForPrefixIterator {
    prefix: String,
    page: u32,
    word_in_page: u32,
    matches_max: usize
}
impl Bits11ForPrefixIterator {
    pub fn new(prefix: &str) -> Self {
        let first_letter = prefix.as_bytes().get(0).unwrap();
        let start_word = WORDLIST_STARTS[(first_letter - FIRST_WORDLIST_STARTS) as usize];
        let start_page = start_word / WORDS_IN_PAGE;
        let start_word_in_page = start_word.rem(WORDS_IN_PAGE);

        Self {
            prefix: prefix.to_owned(),
            page: start_page,
            word_in_page: start_word_in_page,
            matches_max: 0
        }
    }
    fn end(&mut self) {
        self.page = WORDLIST_PAGES;
    }
}
impl Iterator for Bits11ForPrefixIterator {
    type Item = Bits11;
    fn next(&mut self) -> Option<Self::Item> {
        let mut out = None;
        'search: while self.page < WORDLIST_PAGES {
            in_free(|peripherals| {
                select_psram(&mut peripherals.gpio_s);
                psram_write_read_byte(peripherals, PSRAM_READ);
                let start_address = AddressPsram::new(
                        (self.page * WORDS_IN_PAGE + self.word_in_page) * WORD_MAX_LEN as u32 + WORDLIST_BASE
                    )
                    .expect("wordlist valid address");
                psram_write_slice(peripherals, &start_address.inner());
                psram_write_byte(peripherals, 0); // dummy_byte)
            });
            'word: while self.word_in_page < WORDS_IN_PAGE {
                let mut b = 0;
                for (j, c) in self.prefix.as_bytes().iter().enumerate() {
                    in_free(|peripherals| {
                        b = psram_read_byte(peripherals);
                        psram_write_byte(peripherals, 0); // dummy_byte
                    });
                    if *c != b {
                        if j < self.matches_max {
                            self.end();
                            break 'search
                        }
                        self.matches_max = j;
                        in_free(|peripherals| {
                            if j < WORD_MAX_LEN {
                                for _ in 0..(WORD_MAX_LEN - 1 - j) {
                                    psram_read_byte(peripherals);
                                    psram_write_byte(peripherals, 0); // dummy_byte
                                }
                            }
                        });
                        self.word_in_page += 1;
                        continue 'word
                    }
                }
                let bits11 = Bits11::from((self.page * WORDS_IN_PAGE + self.word_in_page) as u16)
                    .expect("Wordlist suppose contain no more words than TOTAL_WORDS");
                out = Some(bits11);
                self.word_in_page += 1;
                break 'search
            }
            self.page += 1;
            self.word_in_page = 0;
            if self.page < WORDLIST_PAGES {
                in_free(|peripherals| {
                    deselect_psram(&mut peripherals.gpio_s);
                    psram_read_byte(peripherals);
                })
            }
        }
        in_free(|peripherals| {
            psram_read_byte(peripherals);
            deselect_psram(&mut peripherals.gpio_s);
        });
        out
    }
}

pub struct PsramWordList;

impl AsWordList for PsramWordList {
    type Word = String;
    fn get_word(&self, bits: Bits11) -> Result<Self::Word, ErrorMnemonic> {
        let word_order = bits.bits() as usize;
        if word_order > TOTAL_WORDS {
            Err(ErrorMnemonic::InvalidWordNumber)
        } else {
            let address = AddressPsram::new(word_order as u32 * WORD_MAX_LEN as u32 + WORDLIST_BASE)
                .expect("checked valid wordlist address");
            let mut word_bytes = Vec::new();
            in_free(|peripherals| {
                word_bytes = psram_read_at_address(peripherals, address, WORD_MAX_LEN)
                    .expect("checked valid wordlist address");
            });
            let word_bytes_stripped = word_bytes.iter().take_while(|&ch| *ch != b' ').cloned().collect();
            let word = String::from_utf8(word_bytes_stripped).unwrap();
            Ok(word)
        }
    }

    fn get_words_by_prefix(&self, prefix: &str) -> Result<Vec<WordListElement<Self>>, ErrorMnemonic> {
        let mut out = Vec::<WordListElement<Self>>::new();
        for bits11 in Bits11ForPrefixIterator::new(prefix) {
            out.push(WordListElement {
                word: self.get_word(bits11)?,
                bits11,
            });
            if out.len() >= MAX_PROPOSAL {
                break;
            }
        }

        Ok(out)
    }

    fn bits11_for_word(&self, word: &str) -> Result<Bits11, ErrorMnemonic> {
        for bits11 in Bits11ForPrefixIterator::new(word) {
            return Ok(bits11)
        }
        Err(ErrorMnemonic::NoWord)
    }
}
/*
pub fn store_wordlist() {
    for (i, chunk) in WORDLIST_ENGLISH.chunks(32).enumerate() {
        let mut data: [u8; 256] = [0x20u8; 256];
        for (j, w) in chunk.iter().enumerate() {
            data[j*8..j*8+w.len()].copy_from_slice((*w).as_bytes())
        }
        if let Err(e) = store_data(((i+128)*256) as u32, &data) {
            panic!("could not store wordlist chunk {}", i)
        };
    }
}*/