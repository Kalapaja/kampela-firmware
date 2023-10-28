#[cfg(not(feature="std"))]
use alloc::{format, vec, string::String, str, vec::Vec};

use core::panic::RefUnwindSafe;
#[cfg(feature="std")]
use std::{format, vec, string::String, str, vec::Vec};

use embedded_graphics::{
    mono_font::{
        iso_8859_1::FONT_10X20, ascii::FONT_4X6,
        MonoTextStyle, MonoFont, mapping::StrGlyphMapping,
    },
    primitives::Rectangle,
    Drawable, text::LineHeight,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    pixelcolor::BinaryColor,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};

use crate::display_def::*;

#[path = "./non_printable_list.rs"]
mod non_printable_list;
use non_printable_list::match_representation;

// Added 2px spacing between characters, to match witdth with 4px width hex font and 4px hex spacing: 4px*2+4px.
// Since space u{20} character is White_Space, spacing is not applied to it. To counteract this,
// in code below all White_Space characters is replaced with SUB_CHAR which is mapped to space glyph down here
const SUB_CHAR: char = '\u{2060}'; //should not be White_Space and should be invisible
const TEXT_FONT: MonoFont = MonoFont{
	character_spacing: 2,
	glyph_mapping: &StrGlyphMapping::new(
		"\u{2060}\0\u{21}\u{7f}\0\u{a0}\u{ff}",
		'\u{a4}' as usize - '\u{20}' as usize - ('\u{a0}' as usize - '\u{80}' as usize),
	),
	..FONT_10X20
};

const HEX_FONT: MonoFont = FONT_4X6;

const LINE_MAX_WIDTH: u32 = SCREEN_SIZE_X - HEX_FONT.character_size.width * 2;
const LINE_CAPACITY: usize = ((LINE_MAX_WIDTH + TEXT_FONT.character_spacing) /
                              (TEXT_FONT.character_size.width + TEXT_FONT.character_spacing)
                             ) as usize;

// This function finds indexes of first symbols in each word indexes is calculated in bytes,
// assuming all non-ascii symbols (which is more than one byte in size) will be replaced.
// Words delimeters found by White_Space and Common_Separator properties
fn find_separations(text: &str) -> Vec<usize> {
	let mut seps = Vec::new();
	let mut len_in_bytes = 0;
	let mut prev: Option<char> = None;
	for c in text.chars() {
		if prev.is_some() &&
		   (prev.unwrap().is_whitespace() || prev.unwrap().is_ascii_punctuation()) &&
		   (!c.is_whitespace() || !c.is_ascii_punctuation()) {
			seps.push(len_in_bytes);
		}
		len_in_bytes = len_in_bytes + c.len_utf8();
		prev = Some(c);
	}
	seps.push(len_in_bytes); // adding the last index as separation
	seps
}

// This function finds breaks bypassing embedded_text inner algortithm
fn find_line_breaks(text: &str) -> Vec<usize> {
	let mut breaks = Vec::new();
	let separations = find_separations(text);

	let mut psep = 0;
	for sep in separations {
		let current_line_chars_count = sep - breaks.last().unwrap_or(&0);
		if current_line_chars_count > LINE_CAPACITY {
			if &psep != breaks.last().unwrap_or(&0) { //if bigger than line is not first word in line
				breaks.push(psep);
			}
			let current_line_chars_count = sep - breaks.last().unwrap_or(&0);
			if current_line_chars_count > LINE_CAPACITY { //if wrapped line still not fits in the line, then chop it
				let lines_in_word = current_line_chars_count / LINE_CAPACITY;
				for _ in 0..lines_in_word {
					breaks.push(breaks.last().unwrap_or(&0) + LINE_CAPACITY);
				}
			}
		}
		psep = sep;
	}
	breaks
}

fn break_text(text: &str, breaks: &Vec<usize>) -> String {
	let mut chars = text.chars().collect::<Vec<char>>();
	for (offset, b) in breaks.iter().enumerate() {
		chars.insert(*b + offset, '\n');
	}
	chars.into_iter().collect()
}

fn break_hex(hex: &str, breaks: &Vec<usize>) -> String{
	let mut chars = hex.chars().collect::<Vec<char>>();
	for b in breaks.iter() {
		chars[b * 3 - 1] = '\n';
	}
	chars.into_iter().collect()
}

fn break_rep(hex: &str, breaks: &Vec<usize>) -> String{
	let mut chars = hex.chars().collect::<Vec<char>>();
	for (offset, b) in breaks.iter().enumerate() {
		chars.insert(*b * 3 + offset, '\n');
	}
	chars.into_iter().collect()
}

fn replace_non_ascii(text: &str) -> String {
	let chars = text.chars().collect::<Vec<char>>();
	let mut newchars = Vec::new();
	for c in chars {
		if !c.is_ascii() || c.is_control() || c.is_whitespace() {
			let mut rep = vec![SUB_CHAR; c.len_utf8()];
			newchars.append(&mut rep);
		} else {
			newchars.push(c);
		}
	}
	newchars.into_iter().collect()
}

// This function creates two text fields matching original text in character length.
// Second of the text field will be shifted to half a character width.
// Both will contain substitution symbol for which more than 1 byte in size
// and should be placed in the middle above its hex encoding string
fn create_substituion(text: &str) -> [String; 2] {
	let mut sub = [Vec::new(), Vec::new()];
	let chars = text.chars().collect::<Vec<char>>();
	for c in chars {
		if !c.is_ascii() && !c.is_control() && !c.is_whitespace() {
			let mut nbsps = vec![SUB_CHAR; c.len_utf8()];
			if c.len_utf8() % 2 == 1 {
				let mut substitute = [
					vec![SUB_CHAR; c.len_utf8() / 2],
					vec![c],
					vec![SUB_CHAR; c.len_utf8() / 2],
				].concat();
				sub[0].append(&mut substitute);
				sub[1].append(&mut nbsps);
			} else {
				let mut substitute = [
					vec![SUB_CHAR; c.len_utf8() / 2 - 1],
					vec![c],
					vec![SUB_CHAR; c.len_utf8() / 2],
				].concat();
				sub[0].append(&mut nbsps);
				sub[1].append(&mut substitute);
			}
		} else {
			sub[0].push(SUB_CHAR);
			sub[1].push(SUB_CHAR);
		}
	}
	sub.map(|t| t.into_iter().collect())
}

// Finds if character is non_printable and returns representation string
// in double string format
fn match_and_format_representation(c: char) -> Option<[Vec<char>; 2]> {
	let rep_text_option = match_representation(c);
	match rep_text_option {
		Some(rep_text) => {
			let mut rep = rep_text.lines().collect::<Vec<&str>>();
			rep.truncate(2);
			if rep.len() == 1 {
				rep = [vec![""], rep].concat()
			}
			let rep: [&str; 2] = rep.try_into().unwrap();
	
			Some(rep.map(|t| t.chars().collect::<Vec<char>>()))
		},
		None => None
	}
}

// Will create small two lined descriptions in place of non-printable characters
// Creates four text fields each offsetted in x and y axis,
// each line matched in size with original text.
fn create_representation(text: &str) -> [[String; 2]; 2] {
	let mut rep = [[Vec::new(), Vec::new()], [Vec::new(), Vec::new()]];
	let chars = text.chars().collect::<Vec<char>>();
	for c in chars {
		let r_option = match_and_format_representation(c);
		for i in 0..2 {
			let mut nbsps = vec!['\u{a0}'; c.len_utf8() * 3];
			match &r_option {
				Some(r) => {
					if (c.len_utf8() % 2 == 1) ^ (r[i].len() % 2 == 1) {
						let mut representation = [
							vec!['\u{a0}'; (c.len_utf8() * 3 - r[i].len()) / 2],
							r[i].clone(),
							vec!['\u{a0}'; (c.len_utf8() * 3 - r[i].len()) / 2 + 1],
						].concat();
						rep[i][0].append(&mut nbsps);
						rep[i][1].append(&mut representation);
					} else {
						let mut representation = [
							vec!['\u{a0}'; (c.len_utf8() * 3 - r[i].len()) / 2],
							r[i].clone(),
							vec!['\u{a0}'; (c.len_utf8() * 3 - r[i].len()) / 2],
						].concat();
						rep[i][0].append(&mut representation);
						rep[i][1].append(&mut nbsps);
					}
				},
				None => {
					rep[i][0].append(&mut nbsps.clone());
					rep[i][1].append(&mut nbsps);
				},
			}
		}
	}
	rep.map(|t| t.map(|r| r.into_iter().collect()))
}

pub fn draw<D>(content: &Vec<u8>, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
	let text_character_style = MonoTextStyle::new(&TEXT_FONT, BinaryColor::On);
	let hex_character_style = MonoTextStyle::new(&HEX_FONT, BinaryColor::On);
	let textbox_style = TextBoxStyleBuilder::new()
		.alignment(HorizontalAlignment::Center)
		.vertical_alignment(VerticalAlignment::Middle)
		.line_height(LineHeight::Pixels(HEX_FONT.character_size.height + TEXT_FONT.character_size.height))
		.paragraph_spacing(5)
		.build();
	let text_bounds = Rectangle::new(
		Point::zero(),
		Size::new(SCREEN_SIZE_X,
		SCREEN_SIZE_Y)
	);
	let text_bounds_offsetted = Rectangle::new(
		Point::new((TEXT_FONT.character_size.width / 2) as i32, 0),
		Size::new(SCREEN_SIZE_X, SCREEN_SIZE_Y)
	);
	let rep_bounds = Rectangle::new(
		Point::new(
			0,
			0,
		),
		Size::new(SCREEN_SIZE_X, SCREEN_SIZE_Y)
	);
	let rep_bounds_offsetted_x = Rectangle::new(
		Point::new(
			(HEX_FONT.character_size.width / 2) as i32,
			0,
		),
		Size::new(SCREEN_SIZE_X, SCREEN_SIZE_Y)
	);
	let rep_bounds_offsetted_y = Rectangle::new(
		Point::new(
			0,
			HEX_FONT.character_size.height as i32,
		),
		Size::new(SCREEN_SIZE_X, SCREEN_SIZE_Y)
	);
	let rep_bounds_offsetted_xy = Rectangle::new(
		Point::new(
			(HEX_FONT.character_size.width / 2) as i32,
			HEX_FONT.character_size.height as i32,
		),
		Size::new(SCREEN_SIZE_X, SCREEN_SIZE_Y)
	);
	let hex_bounds = Rectangle::new(
		Point::new(0, (TEXT_FONT.character_size.height / 2 +
		                             HEX_FONT.character_size.height / 2 +
		                             TEXT_FONT.character_spacing) as i32),
		Size::new(SCREEN_SIZE_X, SCREEN_SIZE_Y)
	);
    
	let mut hex = content.iter().map(|c| format!("{:02X}\u{a0}", c)).collect::<String>();
	hex.pop(); //remove tailing nbsp
  let text = String::from_utf8(content.clone()).expect("not UTF-8");
	
	let breaks = find_line_breaks(&text);
	let sub = create_substituion(&text);
	let sub = sub.map(|s| break_text(&s, &breaks));
	let rep = create_representation(&text);
	let rep = rep.map(|t| t.map(|r| break_rep(&r, &breaks)));
	let text = replace_non_ascii(&text);
	let text = break_text(&text, &breaks);
	let hex = break_hex(&hex, &breaks);
    
	TextBox::with_textbox_style(&text, text_bounds, text_character_style, textbox_style).draw(display)?;
	TextBox::with_textbox_style(&sub[0], text_bounds, text_character_style, textbox_style).draw(display)?;
	TextBox::with_textbox_style(&sub[1], text_bounds_offsetted, text_character_style, textbox_style).draw(display)?;
	TextBox::with_textbox_style(&rep[0][0], rep_bounds, hex_character_style, textbox_style).draw(display)?;
	TextBox::with_textbox_style(&rep[0][1], rep_bounds_offsetted_x, hex_character_style, textbox_style).draw(display)?;
	TextBox::with_textbox_style(&rep[1][0], rep_bounds_offsetted_y, hex_character_style, textbox_style).draw(display)?;
	TextBox::with_textbox_style(&rep[1][1], rep_bounds_offsetted_xy, hex_character_style, textbox_style).draw(display)?;
	TextBox::with_textbox_style(&hex, hex_bounds, hex_character_style, textbox_style).draw(display)?;
	Ok(())
}

