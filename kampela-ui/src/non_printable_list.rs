#[cfg(not(feature="std"))]
use alloc::{string::{String, ToString}};

use core::{ops::RangeInclusive};

#[cfg(feature="std")]
use std::{string::{String, ToString}};

pub struct RepMappingSingle<'a> {
	pub character: char,
	pub representation: &'a str,
}

pub struct RepMappingRange<'a> {
	pub character_range: RangeInclusive<char>,
	pub representations: &'a [&'a str],
}

pub enum RepMapping<'a> {
	Single(RepMappingSingle<'a>),
	Range(RepMappingRange<'a>),
}

pub const REPRESENTATION_LIST: [RepMapping; 9] = [
	RepMapping::Range(RepMappingRange { //C0
		character_range: '\u{0000}' ..= '\u{001f}',
		representations: &[
			"NUL", "SOH", "STX", "ETX", "EOT", "ENQ", "ACK", "BEL",
			"BS", "HT", "LF", "VT", "FF", "CR", "SO", "SI",
			"DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB",
			"CAN", "EM", "SUB", "ESC", "FS", "GS", "RS", "US"
		],
	}),
	RepMapping::Range(RepMappingRange { //C1
		character_range: '\u{007F}' ..= '\u{009F}',
		representations: &[
			"DEL",
			"PAD", "HOP", "BPH", "NBH", "IND", "NEL", "SSA", "ESA",
			"HTS", "HTJ", "VTS", "PLD", "PLU", "RI", "SS2", "SS3",
			"DCS", "PU1", "PU2", "STS", "CCH", "MW", "SPA", "EPA",
			"SOS", "SGC\nI", "SCI", "CSI", "ST", "OSC", "PM", "APC"
		],
	}),
	RepMapping::Single(RepMappingSingle { character: '\u{00a0}', representation: "NB\nSP" }),
	RepMapping::Single(RepMappingSingle { character: '\u{00ad}', representation: "SHY" }), //usually displayed as hyphen-minus
	RepMapping::Single(RepMappingSingle { character: '\u{1680}', representation: "OGH\nSM" }),
	RepMapping::Range(RepMappingRange { //WhiteSpaces
		character_range: '\u{2000}' ..= '\u{200F}',
		representations: &[
			"NQ\nSP", "MQ\nSP", "EN\nSP", "EM\nSP", "3/M\nSP", "4/M\nSP", "6/M\nSP", "F\nSP",
			"P\nSP", "TH\nSP", "H\nSP", "ZW\nSP", "ZW\nNJ", "ZW\nJ", "LRM", "RLM"
		],
	}),
	RepMapping::Range(RepMappingRange { //Bidirectional
		character_range: '\u{2028}' ..= '\u{202F}' ,
		representations: &["L\nSEP", "P\nSEP", "LRE", "RLE", "PDF", "LRO", "RLO", "NNB\nSP"],
	}),
	RepMapping::Range(RepMappingRange { //Isolates
		character_range: '\u{205F}' ..= '\u{206F}',
		representations: &[
			"MM\nSP",
			"WJ", "f()", "I\nTMS", "I\nSEP", "I\nPLS", "NA", "LRI", "RLI",
			"FSI", "PDI", "I\nSS", "A\nSS", "I\nAFS", "A\nAFS", "NA\nDS", "NO\nDS"
		],
	}),
	RepMapping::Single(RepMappingSingle { character: '\u{3000}', representation: "ID\nSP" }),
];

pub fn match_representation (c: char) -> Option<String> {
	let mut rep_text_option = None;
	for m in REPRESENTATION_LIST {
		match m {
			RepMapping::Single(s) => {
				if s.character == c {
					rep_text_option = Some(s.representation.to_string());
					break;
				};
			},
			RepMapping::Range(r) => {
				if r.character_range.contains(&c) {
					let ri = c as usize - *r.character_range.start() as usize;
					rep_text_option = Some(r.representations[ri].to_string());
					break;
				};
			}
		}
	}
	rep_text_option
}