#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

#[cfg(feature = "std")]
use std::{string::String, vec::Vec};

use crate::error::ErrorWordList;

#[cfg(feature = "sufficient-memory")]
use crate::internal::InternalWordList;

use crate::wordlist::WORDLIST_ENGLISH;
use crate::{AsWordList, Bits11, WordListElement, WordSet, TOTAL_WORDS, WORD_MAX_LEN};

static mut FLASH_MOCK: [u8; TOTAL_WORDS * WORD_MAX_LEN] = [255u8; TOTAL_WORDS * WORD_MAX_LEN];

fn fill_flash_mock() {
    for (i, word) in WORDLIST_ENGLISH.iter().enumerate() {
        let mut data = [255u8; WORD_MAX_LEN];
        data[..word.len()].copy_from_slice(word.as_bytes());
        unsafe {
            FLASH_MOCK[i * WORD_MAX_LEN..(i + 1) * WORD_MAX_LEN].copy_from_slice(&data);
        }
    }
}

struct FlashMockWordList;

impl AsWordList for FlashMockWordList {

    fn get_word(bits: Bits11) -> Result<WordListElement, ErrorWordList> {
        let word_order = bits.bits() as usize;
        let mut word_bytes = unsafe {
            FLASH_MOCK[word_order * WORD_MAX_LEN..(word_order + 1) * WORD_MAX_LEN].to_vec()
        };
        word_bytes = word_bytes.into_iter().take_while(|x| *x != 255).collect();
        match String::from_utf8(word_bytes) {
            Ok(word) => {
                Ok(WordListElement{
                    word,
                    bits11: bits
                })
            },
            Err(err) => Err(ErrorWordList::DamagedWord)
        }
    }

    fn get_words_by_prefix(prefix: &str) -> Vec<WordListElement> {
        let mut words_by_prefix: Vec<WordListElement> = Vec::new();
        for bits_u16 in 0..TOTAL_WORDS {
            let bits11 = Bits11::from(bits_u16 as u16).expect("Should iterate over valid Bits11 range");
            let wordlist_element = Self::get_word(bits11).expect("Wordlist suppose contain no less words than TOTAL_WORDS");
            if wordlist_element.word.starts_with(prefix) {
                words_by_prefix.push(wordlist_element)
            } else if !words_by_prefix.is_empty() {
                break;
            }
        }
        words_by_prefix
    }

    fn bits11_for_word(word: &str) -> Result<Bits11, ErrorWordList> {
        for bits_u16 in 0..TOTAL_WORDS {
            let bits11 = Bits11::from(bits_u16 as u16)?;
            let read_word = Self::get_word(bits11)?;
            if word == read_word.word {
                return Ok(bits11);
            }
        }
        Err(ErrorWordList::NoWord)
    }
}

// Test data taken from `tiny-bip39`.
static KNOWN: &[[&str; 2]] = &[
    [
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
        "00000000000000000000000000000000",
    ],
    [
        "legal winner thank year wave sausage worth useful legal winner thank yellow",
        "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
    ],
    [
        "letter advice cage absurd amount doctor acoustic avoid letter advice cage above",
        "80808080808080808080808080808080",
    ],
    [
        "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong",
        "ffffffffffffffffffffffffffffffff",
    ],
        [
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon agent",
        "000000000000000000000000000000000000000000000000",
    ],
        [
        "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal will",
        "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
    ],
        [
        "letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter always",
        "808080808080808080808080808080808080808080808080",
    ],
        [
        "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo when",
        "ffffffffffffffffffffffffffffffffffffffffffffffff",
    ],
        [
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art",
        "0000000000000000000000000000000000000000000000000000000000000000",
    ],
        [
        "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth title",
        "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
    ],
        [
        "letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic bless",
        "8080808080808080808080808080808080808080808080808080808080808080",
    ],
        [
        "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote",
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    ],
        [
        "ozone drill grab fiber curtain grace pudding thank cruise elder eight picnic",
        "9e885d952ad362caeb4efe34a8e91bd2",
    ],
        [
        "gravity machine north sort system female filter attitude volume fold club stay feature office ecology stable narrow fog",
        "6610b25967cdcca9d59875f5cb50b0ea75433311869e930b",
    ],
        [
        "hamster diagram private dutch cause delay private meat slide toddler razor book happy fancy gospel tennis maple dilemma loan word shrug inflict delay length",
        "68a79eaca2324873eacc50cb9c6eca8cc68ea5d936f98787c60c7ebc74e6ce7c",
    ],
        [
        "scheme spot photo card baby mountain device kick cradle pact join borrow",
        "c0ba5a8e914111210f2bd131f3d5e08d",
    ],
        [
        "horn tenant knee talent sponsor spell gate clip pulse soap slush warm silver nephew swap uncle crack brave",
        "6d9be1ee6ebd27a258115aad99b7317b9c8d28b6d76431c3",
    ],
        [
        "panda eyebrow bullet gorilla call smoke muffin taste mesh discover soft ostrich alcohol speed nation flash devote level hobby quick inner drive ghost inside",
        "9f6a2878b2520799a44ef18bc7df394e7061a224d2c33cd015b157d746869863",
    ],
        [
        "cat swing flag economy stadium alone churn speed unique patch report train",
        "23db8160a31d3e0dca3688ed941adbf3",
    ],
        [
        "light rule cinnamon wrap drastic word pride squirrel upgrade then income fatal apart sustain crack supply proud access",
        "8197a4a47f0425faeaa69deebc05ca29c0a5b5cc76ceacc0",
    ],
        [
        "all hour make first leader extend hole alien behind guard gospel lava path output census museum junior mass reopen famous sing advance salt reform",
        "066dca1a2bb7e8a1db2832148ce9933eea0f3ac9548d793112d9a95c9407efad",
    ],
        [
        "vessel ladder alter error federal sibling chat ability sun glass valve picture",
        "f30f8c1da665478f49b001d94c5fc452",
    ],
        [
        "scissors invite lock maple supreme raw rapid void congress muscle digital elegant little brisk hair mango congress clump",
        "c10ec20dc3cd9f652c7fac2f1230f7a3c828389a14392f05",
    ],
        [
        "void come effort suffer camp survey warrior heavy shoot primary clutch crush open amazing screen patrol group space point ten exist slush involve unfold",
        "f585c11aec520db57dd353c69554b21a89b20fb0650966fa0a9d6f74fd989d8f",
    ]
];

#[test]
fn flash_mock_entropy_to_phrase() {
    fill_flash_mock();
    for known in KNOWN {
        let entropy = hex::decode(known[1]).unwrap();
        let word_set = WordSet::from_entropy(&entropy).unwrap();
        assert_eq!(word_set.to_phrase::<FlashMockWordList>().unwrap(), known[0]);
    }
}

#[cfg(feature = "sufficient-memory")]
#[test]
fn internal_entropy_to_phrase() {
    for known in KNOWN {
        let entropy = hex::decode(known[1]).unwrap();
        let word_set = WordSet::from_entropy(&entropy).unwrap();
        assert_eq!(word_set.to_phrase::<FlashMockWordList>().unwrap(), known[0]);
    }
}

#[test]
fn flash_mock_phrase_to_entropy() {
    fill_flash_mock();
    for known in KNOWN {
        let entropy_set = hex::decode(known[1]).unwrap();
        let mut word_set = WordSet::new();
        for word in known[0].split(' ') {
            word_set.add_word::<FlashMockWordList>(word).unwrap();
        }
        let entropy_calc = word_set.to_entropy().unwrap();
        assert_eq!(entropy_calc, entropy_set);
    }
}

#[cfg(feature = "sufficient-memory")]
#[test]
fn internal_phrase_to_entropy() {
    for known in KNOWN {
        let entropy_set = hex::decode(known[1]).unwrap();
        let mut word_set = WordSet::new();
        for word in known[0].split(' ') {
            word_set.add_word::<FlashMockWordList>(word).unwrap();
        }
        let entropy_calc = word_set.to_entropy().unwrap();
        assert_eq!(entropy_calc, entropy_set);
    }
}

#[test]
fn flash_mock_get_word() {
    fill_flash_mock();
    assert_eq!(
        "access",
        FlashMockWordList
            ::get_word(Bits11::from(10u16).unwrap())
            .unwrap()
            .word
    );
    assert_eq!(
        "arrive",
        FlashMockWordList
            ::get_word(Bits11::from(100u16).unwrap())
            .unwrap()
            .word
    );
    assert_eq!(
        "laptop",
        FlashMockWordList
            ::get_word(Bits11::from(1000u16).unwrap())
            .unwrap()
            .word
    );
    assert_eq!(
        "zoo",
        FlashMockWordList
            ::get_word(Bits11::from(TOTAL_WORDS as u16 - 1).unwrap())
            .unwrap()
            .word
    );
}
