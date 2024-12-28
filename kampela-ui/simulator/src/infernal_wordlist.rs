use mnemonic_external::{
    error::ErrorWordList, regular::InternalWordList, wordlist::WORDLIST_ENGLISH, AsWordList,
    Bits11, WordListElement,
};

use crc32fast::Hasher as CRC32;
use std::fs::File;
use std::path::PathBuf;

/**
 * This is a wasteful implementation of a wordlist,
 * it takes mnemonics from the internal wordlist
 * in case if no file has been provided,
 * or reads it from the file.
 */
#[derive(Debug)]
pub struct InfernalWordList {
    words: Option<[String; 2048]>,
}

impl InfernalWordList {
    pub fn new() -> Self {
        Self { words: None }
    }

    pub fn load_words(&mut self, buf: Vec<u8>) {
        let binding = String::from_utf8(buf).expect("Failed to convert buffer to string");
        let words = binding.split_whitespace();
        self.words = Some([const { String::new() }; 2048]);
        let words_ref = self.words.as_mut().unwrap();
        let mut hasher = CRC32::new();
        for (i, word) in words.enumerate() {
            hasher.update(word.as_bytes());
            words_ref[i] = word.to_string();
        }
        let checksum = hasher.finalize();
        const EXPECTED_CHECKSUM: u32 = 0x81b9dda4;
        if checksum != EXPECTED_CHECKSUM {
            panic!(
                "Checksum mismatch: 0x{:x}, expected 0x{:x}",
                checksum, EXPECTED_CHECKSUM
            );
        }
    }

    pub fn from_file(path: &str) -> Self {
        println!("Reading wordlist from file: {}", path);
        let buf: Vec<u8> =
            std::fs::read(path).expect(format!("Failed to read file: {}", path).as_str());
        let mut wordlist = Self::new();
        wordlist.load_words(buf);
        return wordlist;
    }
}

impl AsWordList for InfernalWordList {
    type Word = String;

    fn get_word(&self, bits: Bits11) -> Result<Self::Word, ErrorWordList> {
        if let Some(words) = &self.words {
            let pos = bits.bits() as usize;
            return Ok(words[pos].clone());
        }
        let result = InternalWordList.get_word(bits);
        match result {
            Ok(word) => {
                return Ok(word.to_string());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    fn get_words_by_prefix(
        &self,
        prefix: &str,
    ) -> Result<Vec<WordListElement<Self>>, ErrorWordList> {
        if let Some(words) = &self.words {
            let mut result: Vec<WordListElement<Self>> = Vec::new();
            for (i, word) in words.iter().enumerate() {
                if word.starts_with(prefix) {
                    result.push(WordListElement {
                        word: word.clone(),
                        bits11: Bits11::from(i as u16)?,
                    });
                }
            }
            return Ok(result);
        }
        let result = InternalWordList.get_words_by_prefix(prefix);
        match result {
            Ok(words) => {
                let mut result: Vec<WordListElement<Self>> = Vec::new();
                for word in words {
                    result.push(WordListElement {
                        word: word.word.to_string(),
                        bits11: word.bits11,
                    });
                }
                return Ok(result);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    fn bits11_for_word(&self, word: &str) -> Result<Bits11, ErrorWordList> {
        if let Some(words) = &self.words {
            for (i, w) in words.iter().enumerate() {
                if w == word {
                    return Ok(Bits11::from(i as u16)?);
                }
            }
            return Err(ErrorWordList::NoWord);
        }
        let result = InternalWordList.bits11_for_word(word);
        match result {
            Ok(bits) => {
                return Ok(bits);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_wordlist_buf() -> Vec<u8> {
        let mut buf = Vec::new();
        for (i, word) in WORDLIST_ENGLISH.iter().enumerate() {
            buf.extend(word.as_bytes());
            if i < WORDLIST_ENGLISH.len() - 1 {
                buf.push(b' ');
            }
        }
        return buf;
    }

    fn wordlist_to_file(filename: &str) {
        let path = PathBuf::from(filename);
        let mut file = File::create(&path).expect("Failed to create file");
        let buf = get_wordlist_buf();
        use std::io::Write;
        file.write_all(&buf).expect("Failed to write to file");
        drop(file);
    }

    fn get_wordlist_external() -> InfernalWordList {
        let buf = get_wordlist_buf();
        let mut wordlist = InfernalWordList::new();
        wordlist.load_words(buf);
        return wordlist;
    }

    #[test]
    fn test_load_words() {
        let mut wordlist = InfernalWordList::new();
        let buf = get_wordlist_buf();
        wordlist.load_words(buf);
        assert!(wordlist.words.is_some());
        let words = wordlist.words.unwrap();
        for (i, word) in words.iter().enumerate() {
            assert_eq!(
                word,
                InternalWordList
                    .get_word(Bits11::from(i as u16).expect("??? o.O"))
                    .expect("??? o.O")
            );
        }
    }

    #[test]
    fn test_read_mnemonic_from_file() {
        let filename = format!("wordlist_{}.txt", rand::random::<u32>()).to_string();
        wordlist_to_file(&filename);
        let wordlist = InfernalWordList::from_file(&filename);
        assert!(wordlist.words.is_some());
        let words = wordlist.words.unwrap();
        for (i, word) in words.iter().enumerate() {
            assert_eq!(
                word,
                InternalWordList
                    .get_word(Bits11::from(i as u16).expect("??? o.O"))
                    .expect("??? o.O")
            );
        }
        std::fs::remove_file(PathBuf::from(&filename)).expect("Failed to remove file");
    }

    #[test]
    fn test_get_word_internal() {
        let wordlist = InfernalWordList::new();
        for i in 0..2048 {
            let word = wordlist
                .get_word(Bits11::from(i as u16).expect("??? o.O"))
                .expect("??? o.O");
            assert_eq!(
                word,
                InternalWordList
                    .get_word(Bits11::from(i as u16).expect("??? o.O"))
                    .expect("??? o.O")
            );
        }
    }

    #[test]
    fn test_get_word_external() {
        let wordlist = get_wordlist_external();
        for i in 0..2048 {
            let word = wordlist
                .get_word(Bits11::from(i as u16).expect("??? o.O"))
                .expect("??? o.O");
            assert_eq!(
                word,
                InternalWordList
                    .get_word(Bits11::from(i as u16).expect("??? o.O"))
                    .expect("??? o.O")
            );
        }
    }

    #[test]
    fn test_get_words_by_prefix_internal() {
        let wordlist = InfernalWordList::new();
        for i in 0..2048 {
            let prefix = &InternalWordList
                .get_word(Bits11::from(i as u16).expect("??? o.O"))
                .expect("??? o.O")[..2];
            let words = wordlist.get_words_by_prefix(prefix).expect("??? o.O");
            let expected_words = InternalWordList
                .get_words_by_prefix(prefix)
                .expect("??? o.O");
            assert_eq!(words.len(), expected_words.len(), "Prefix: {}", prefix);
            for (i, word) in words.iter().enumerate() {
                assert_eq!(word.word, expected_words[i].word);
                assert_eq!(word.bits11.bits(), expected_words[i].bits11.bits());
            }
        }
    }

    #[test]
    fn test_get_words_by_prefix_external() {
        let wordlist = get_wordlist_external();
        for i in 0..2048 {
            let prefix = &InternalWordList
                .get_word(Bits11::from(i as u16).expect("??? o.O"))
                .expect("??? o.O")[..2];
            let words = wordlist.get_words_by_prefix(prefix).expect("??? o.O");
            let expected_words = InternalWordList
                .get_words_by_prefix(prefix)
                .expect("??? o.O");
            assert_eq!(words.len(), expected_words.len(), "Prefix: {}", prefix);
            for (i, word) in words.iter().enumerate() {
                assert_eq!(word.word, expected_words[i].word);
                assert_eq!(word.bits11.bits(), expected_words[i].bits11.bits());
            }
        }
    }

    #[test]
    fn test_bits11_for_word_internal() {
        let wordlist = InfernalWordList::new();
        for i in 0..2048 {
            let word = InternalWordList
                .get_word(Bits11::from(i as u16).expect("??? o.O"))
                .expect("??? o.O");
            let bits = wordlist.bits11_for_word(&word).expect("??? o.O");
            assert_eq!(bits.bits(), i as u16);
        }
    }

    #[test]
    fn test_bits11_for_word_external() {
        let wordlist = get_wordlist_external();
        for i in 0..2048 {
            let word = InternalWordList
                .get_word(Bits11::from(i as u16).expect("??? o.O"))
                .expect("??? o.O");
            let bits = wordlist.bits11_for_word(&word).expect("??? o.O");
            assert_eq!(bits.bits(), i as u16);
        }
    }
}
