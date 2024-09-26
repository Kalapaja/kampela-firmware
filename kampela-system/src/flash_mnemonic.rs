#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String, collections::VecDeque};

//use crate::wordlist::WORDLIST_ENGLISH;
use mnemonic_external::{AsWordList, Bits11, WordListElement, TOTAL_WORDS, WORD_MAX_LEN, error::ErrorWordList};

use crate::devices::flash::read_data;
const WORDLIST_STARTS: [usize; 26] = [
    0, 4, 7, 13, 17, 20, 23,
    26, 28, 29, 30, 31, 33, 36,
    37, 39, 43, 44, 47, 55, 59,
    60, 61, 63, 63, 63
];
const FIRST_WORDLIST_STARTS: u8 = 0x61;

const CACHE_SIZE: usize = 5;
pub const MAX_PROPOSAL: usize = 3;
const WORDLIST_BASE: u32 = 128*256;

struct CachedChunk {
    chunk_index: usize,
    cache: [u8; 256]
}
pub struct FlashWordList {
    cached_chunks: VecDeque<CachedChunk>
}

impl FlashWordList {
    pub fn new() -> Self {
        Self {
            cached_chunks: VecDeque::with_capacity(CACHE_SIZE)
        }
    }

    fn read_wordlist_chunk<'a>(&'a mut self, chunk_index: usize) -> &'a [u8; 256] {
        for (i, c) in self.cached_chunks.iter().enumerate() {
            if c.chunk_index == chunk_index {
                return &self.cached_chunks.get(i).unwrap().cache;
            }
        }
        let mut c = CachedChunk { chunk_index, cache: [0; 256]};
        if let Err(_) = read_data(WORDLIST_BASE + chunk_index as u32 * 256, &mut c.cache) {
            panic!("couldn't read from flash wordlist chunk №{}", chunk_index)
        };
        if self.cached_chunks.len() >= CACHE_SIZE {
            self.cached_chunks.pop_front();
        }
        self.cached_chunks.push_back(c);
        &self.cached_chunks.get(self.cached_chunks.len() - 1).unwrap().cache
    }
}

impl AsWordList for FlashWordList {
    type Word = String;
    fn get_word(&mut self, bits: Bits11) -> Result<Self::Word, ErrorWordList> {
        let word_order = bits.bits() as usize;
        let chunk_index = word_order / 32;
        let index_inchunk = word_order - chunk_index * 32;
        let chunk = self.read_wordlist_chunk(chunk_index);
        if chunk_index >= 64 {
            Err(ErrorWordList::InvalidWordNumber)
        } else {
            let word_bytes = &chunk[index_inchunk * WORD_MAX_LEN..(index_inchunk + 1) * WORD_MAX_LEN].to_vec();
            let word_bytes_stripped = word_bytes.iter().take_while(|&ch| *ch != b' ').cloned().collect();
            let word = String::from_utf8(word_bytes_stripped).unwrap();
            Ok(word)
        }
    }

    fn get_words_by_prefix(&mut self, prefix: &str) -> Result<Vec<WordListElement<Self>>, ErrorWordList> {
        let mut out = Vec::<WordListElement<Self>>::new();

        let first_letter = prefix.as_bytes().get(0).unwrap();
        let start_chunk = WORDLIST_STARTS[(first_letter - FIRST_WORDLIST_STARTS) as usize];
        let mut matches_max: usize = 0;
        'search: for chunk_index in start_chunk..(TOTAL_WORDS / 32) {
            let chunk = self.read_wordlist_chunk(chunk_index);
            'chunk: for (i, word_bytes) in chunk.chunks(WORD_MAX_LEN).enumerate() {
                for (j, c) in prefix.as_bytes().iter().enumerate() {
                    if *c != word_bytes[j] {
                        if j < matches_max {
                            break 'search
                        } else {
                            matches_max = j;
                        }
                        continue 'chunk
                    }
                }
                let word_bytes_stripped = word_bytes.iter().take_while(|&ch| *ch != b' ').cloned().collect();
                let word = String::from_utf8(word_bytes_stripped).unwrap();
                out.push(
                    WordListElement{
                        word,
                        bits11: Bits11::from((chunk_index as usize * 32 + i) as u16)
                            .expect("Wordlist suppose contain no more words than TOTAL_WORDS")
                    }
                );
                if out.len() >= MAX_PROPOSAL {
                    break 'search
                }
            }
        }
        Ok(out)
    }

    fn bits11_for_word(&mut self, word: &str) -> Result<Bits11, ErrorWordList> {
        let first_letter = word.as_bytes().get(0).unwrap();
        let start_chunk = WORDLIST_STARTS[(first_letter - FIRST_WORDLIST_STARTS) as usize];
        let mut matches_max: usize = 0;
        'search: for chunk_index in start_chunk..(TOTAL_WORDS / 32) {
            let chunk = self.read_wordlist_chunk(chunk_index);
            'chunk: for (i, word_bytes) in chunk.chunks(WORD_MAX_LEN).enumerate() {
                for (j, c) in word.as_bytes().iter().enumerate() {
                    if *c != word_bytes[j] {
                        if j < matches_max {
                            break 'search
                        } else {
                            matches_max = j;
                        }
                        continue 'chunk
                    }
                }
                let word_bytes_stripped: Vec<u8> = word_bytes.iter().take_while(|&ch| *ch != b' ').cloned().collect();
                if word_bytes_stripped == word.as_bytes() {
                    return Bits11::from((chunk_index as usize * 32 + i) as u16);
                }
            }
        }
        Err(ErrorWordList::NoWord)
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