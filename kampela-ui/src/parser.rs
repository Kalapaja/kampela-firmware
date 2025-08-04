// should it be in external lib?
use core::{iter::Peekable, str::Chars};

#[cfg(not(feature="std"))]
use alloc::{string::{String, ToString}, vec::Vec};
#[cfg(feature="std")]
use std::{string::{String, ToString}, vec::Vec};
#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    NewLine,
    CarriageReturn,
    Tab,
    Whitespace(u32, &'a str),
    Word(&'a str),
    Break(&'a str, &'a str),
    Eof,
    // Additional tokens omitted for brevity
}

// Use the same special chars as embedded-text
const SPEC_CHAR_NBSP: char = '\u{a0}';
const SPEC_CHAR_ZWSP: char = '\u{200b}';
const SPEC_CHAR_SHY: char = '\u{ad}';

fn is_word_char(c: char) -> bool {
    (!c.is_whitespace() || c == SPEC_CHAR_NBSP) && c != SPEC_CHAR_ZWSP && c != SPEC_CHAR_SHY
}

fn is_space_char(c: char) -> bool {
    (c.is_whitespace() && !matches!(c, '\n' | '\r' | '\t' | SPEC_CHAR_NBSP)) || c == SPEC_CHAR_ZWSP
}

pub struct Parser<'a> {
    inner: Chars<'a>,
    str_slice: &'a str,
    eof_emitted: bool,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            inner: text.chars(),
            str_slice: text,
            eof_emitted: false,
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = (Token<'a>, usize); // Token and starting byte offset

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.inner.as_str();
        if s.is_empty() {
            if !self.eof_emitted {
                self.eof_emitted = true;
                let offset = self.str_slice.len();
                return Some((Token::Eof, offset));
            } else {
                return None;
            }
        }

        let first_char = self.inner.next()?;

        let start_offset = self.str_slice.len() - s.len();

        if is_word_char(first_char) {
            // Parse Word token
            // We'll find where this word ends
            let mut end_offset = start_offset + first_char.len_utf8();

            while let Some(ch) = self.inner.clone().next() {
                if !is_word_char(ch) {
                    break;
                }
                let ch_len = ch.len_utf8();
                end_offset += ch_len;
                self.inner.next();
            }

            let word = &self.str_slice[start_offset..end_offset];
            return Some((Token::Word(word), start_offset));
        }

        match first_char {
            '\n' => Some((Token::NewLine, start_offset)),
            '\r' => Some((Token::CarriageReturn, start_offset)),
            '\t' => Some((Token::Tab, start_offset)),
            SPEC_CHAR_ZWSP => Some((Token::Whitespace(0, &self.str_slice[start_offset..start_offset + first_char.len_utf8()]), start_offset)),
            SPEC_CHAR_SHY => Some((Token::Break("-", &self.str_slice[start_offset..start_offset + first_char.len_utf8()]), start_offset)),
            _ if is_space_char(first_char) => {
                // Consume consecutive spaces
                let mut end_offset = start_offset + first_char.len_utf8();
                while let Some(ch) = self.inner.clone().next() {
                    if !is_space_char(ch) {
                        break;
                    }
                    let ch_len = ch.len_utf8();
                    end_offset += ch_len;
                    self.inner.next();
                }
                let whitespace_slice = &self.str_slice[start_offset..end_offset];
                let len = whitespace_slice.chars().count() as u32;
                Some((Token::Whitespace(len, whitespace_slice), start_offset))
            }
            _ => {
                // Unknown char, treat as word of length 1
                Some((
                    Token::Word(&self.str_slice[start_offset..start_offset + first_char.len_utf8()]),
                    start_offset,
                ))
            }
        }
    }
}

pub struct Breaks<'a> {
    parser: Parser<'a>,
    segment: String,
    tab_size: u16
}
impl<'a> Breaks<'a> {
    fn new(text: &'a str, tab_size: u16) -> Self {
        Breaks {
            parser: Parser::new(text),
            segment: "".to_string(),
            tab_size
        }
    }
}
impl<'a> Iterator for Breaks<'a> {
    type Item = (usize, bool, String);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((token, offset)) = self.parser.next() {
            match token {
                // Hard line breaks on newline
                Token::NewLine | Token::Eof => {
                    return Some((offset, true, core::mem::take(&mut self.segment)));
                }
                // Soft break opportunity after whitespace or Break token
                Token::Whitespace(_, s) => {
                    self.segment.push_str(s);
                    return Some((offset, false, core::mem::take(&mut self.segment)));
                }
                Token::Break(s, _) => {
                    self.segment.push_str(s);
                    return Some((offset, false, core::mem::take(&mut self.segment)));
                }
                Token::Word(s) => {
                    self.segment.push_str(s);
                }
                Token::Tab => {
                    for _ in 0..self.tab_size {
                        self.segment.push_str(" ");
                    }
                }
                _ => (),
            }
        }
        None
    }
}

pub struct LineBreaks<'a> {
    break_positions: Peekable<Breaks<'a>>,
    last_break_pos: usize,
    last_break_offset: usize,
    last_offset: usize,
    last_pos: usize,
    max_chars: usize,
}

impl<'a> LineBreaks<'a> {
    fn new(text: &'a str, max_chars: usize, tab_size: u16) -> Self {
        Self {
            break_positions: Breaks::new(text, tab_size).peekable(),
            last_break_pos: 0,
            last_break_offset: 0,
            last_offset: 0,
            last_pos: 0,
            max_chars,
        }
    }
}

impl<'a> Iterator for LineBreaks<'a> {
    type Item = (usize, bool);
    
    fn next(&mut self) -> Option<Self::Item> {
        while {
            if let Some((_, _, segment)) = self.break_positions.peek() {
                if (self.last_pos + segment.chars().count() - self.last_break_pos) > self.max_chars {
                    // with next segment won't fit in line
                    if self.last_break_pos >= self.last_pos {
                        // maximum length reached still wasn't encountered softbreak possibility
                        // chop segment into maximum length lines
                        self.last_break_offset += segment.chars()
                            .skip(self.last_break_pos - self.last_pos)
                            .take(self.max_chars)
                            .fold(0, |s, c| s + c.len_utf8());
                        self.last_break_pos += self.max_chars;
                    } else {
                        // maximum length reached but there is softbreak possibilities
                        // break at the last one
                        self.last_break_pos = self.last_pos;
                        self.last_break_offset = self.last_offset;
                    }
                    return Some((self.last_break_offset, false))
                }
            }
            if let Some((offset, hard, segment)) = self.break_positions.next() {
                self.last_pos += segment.chars().count();
                self.last_offset = offset;
                if hard {
                    // hard line break return immediately
                    self.last_break_pos = self.last_pos;
                    self.last_break_offset = offset;
                    return Some((self.last_break_offset, true))
                }
                true
            } else {
                false
            }
        } {}

        None
    }
}

pub fn count_lines(text: &str, max_chars_in_line: usize, tab_size: u16) -> usize {
    let line_breaks_iter = LineBreaks::new(text, max_chars_in_line, tab_size);
    line_breaks_iter.count()
}

pub fn scroll_str<'a>(text: &'a str, max_chars_in_line: usize, tab_size: u16, scroll_lines: usize) -> &'a str {
    let mut line_break_count = 0;
    let mut last_pos = 0;
    let line_breaks_iter = LineBreaks::new(text, max_chars_in_line, tab_size);
    for (pos, _) in line_breaks_iter {
        if line_break_count >= scroll_lines {
            break;
        }
        last_pos = pos + 1;
        line_break_count += 1;
    }
    &text[last_pos..]
}