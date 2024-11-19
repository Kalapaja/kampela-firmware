#[cfg(not(feature="std"))]
use alloc::{borrow::ToOwned, string::String, vec::Vec, vec};
#[cfg(feature="std")]
use std::{borrow::ToOwned, string::String, vec::Vec, vec};

use embedded_graphics::{
    mono_font::{
        ascii::FONT_10X20, MonoFont, MonoTextStyleBuilder,
    },
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, Point, Size},
    primitives::Rectangle,
    Drawable
};

use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use mnemonic_external::{AsWordList, WordListElement};
const MAX_PROPOSAL: usize = 3;

use crate::{platform::Platform, display_def::*, widget::view::{DrawView, View, Widget}};

use crate::seed_entry::phrase::PHRASE_AREA;

const PROPOSAL_FONT: MonoFont = FONT_10X20;
const ENOUGH_LEN: usize = 4;

const PROPOSAL_SIZE: Size = Size{
    width: SCREEN_SIZE_X,
    height: 24,
};

pub const PROPOSAL_AREA: Rectangle = Rectangle{
    top_left: Point{
        x: 0,
        y: PHRASE_AREA.size.height as i32 - PROPOSAL_SIZE.height as i32 - 4,
    },
    size: PROPOSAL_SIZE,
};
const PROPOSAL_WIDGET: Widget = Widget::new(PROPOSAL_AREA, SCREEN_ZERO);

const PROPOSAL_SECTION_SIZE: Size = Size{
    width: PROPOSAL_AREA.size.width / 3,
    height: PROPOSAL_AREA.size.height,
};
const PROPOSAL_SECTIONS: [Rectangle; 3] = [
    Rectangle{
        top_left: Point{
            x: PROPOSAL_SECTION_SIZE.width as i32,
            y: 0,
        },
        size: PROPOSAL_SECTION_SIZE,
    },
    Rectangle{
        top_left: Point{
            x: PROPOSAL_SECTION_SIZE.width as i32 * 2,
            y: 0,
        },
        size: PROPOSAL_SECTION_SIZE,
    },
    Rectangle{
        top_left: Point{
            x: 0,
            y: 0,
        },
        size: PROPOSAL_SECTION_SIZE,
    },
];

pub struct Proposal<P> where
    P: Platform + ?Sized
{
    pub entered: Vec<Vec<char>>,
    entered_count: usize,
    variants: Vec<String>,
    guess: Vec<WordListElement<P::AsWordList>>,
    guess_depth: usize,
    wordlist: P::AsWordList,
}

impl<P: Platform> Proposal<P> {
    pub fn new(wordlist: P::AsWordList) -> Self {
        Proposal {
            entered: Vec::new(),
            entered_count: 0,
            variants: Vec::new(),
            guess: Vec::new(),
            guess_depth: 0,
            wordlist,
        }
    }

    pub fn clear(&mut self) {
        self.entered = Vec::new();
        self.entered_count = 0;
        self.variants = Vec::new();
        self.guess = Vec::new();
        self.guess_depth = 0;
    }
    pub fn add_letters(&mut self, letters: Vec<char>) {
        if self.entered.len() < ENOUGH_LEN {
            self.entered.push(letters);
        }
        self.entered_count += 1;
    }
    
    pub fn remove_letter(&mut self) {
        if self.entered_count > self.entered.len() {
            self.entered_count -= 1;
            return
        }
        if self.entered.len() <= 1 {
            self.clear()
        } else {
            self.entered.pop();
            self.entered_count -= 1;
            self.variants.iter_mut().for_each(|v| {v.pop();});
            self.variants.dedup();
        }
    }

    fn make_guess(&mut self) {
        let mut guess = Vec::<WordListElement<P::AsWordList>>::new();
        if self.variants.len() < 4 {
            self.variants = Vec::new();
        }
        let variants = Variants::new(&self.entered, &self.variants);

        let mut new_variants = Vec::new();
        for v in variants {
            let mut g = self.wordlist.get_words_by_prefix(&v).unwrap();

            if g.is_empty() {
                continue;
            }
            
            new_variants.push(v);
            //ascending sort by length
            g.sort_by(|a, b| a.word.as_ref().len().cmp(&b.word.as_ref().len()));
            guess.append(&mut g);
            // break if there too many guesses to display
            // and if at least found two variants
            // except if all variants needed, hence !guessed_variants_is_some
            if guess.len() >= MAX_PROPOSAL && new_variants.len() > 1 {
                break;
            }
        }
        if guess.len() > 0 {
            self.variants = new_variants;
            self.guess = guess;
        }
    }
}

impl<P: Platform> View for Proposal<P> {
    type DrawInput<'a> = (bool, bool) where P: 'a;
    type DrawOutput = ();
    type TapInput<'a> = () where P: 'a;
    type TapOutput = Option<WordListElement<P::AsWordList>>;

    fn bounding_box(&self) -> Rectangle {
        PROPOSAL_WIDGET.bounding_box()
    }

    fn bounding_box_absolut(&self) -> Rectangle {
        PROPOSAL_WIDGET.bounding_box_absolute()
    }

    fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, (t, n): Self::DrawInput<'a>) -> Result<(), D::Error>
        where 
            D: DrawTarget<Color = BinaryColor>,
            Self: 'a,
        {

        if t == false {
            let (on, _) = if n {
                (BinaryColor::Off, BinaryColor::On)
            } else {
                (BinaryColor::On, BinaryColor::Off)
            };

            if self.guess_depth != self.entered.len() { // to guess only first draw in row
                self.guess_depth = self.entered.len();
                self.make_guess();
            }
    
            let character_style = MonoTextStyleBuilder::new()
                .font(&PROPOSAL_FONT)
                .text_color(on)
                .underline()
                .build();

            let textbox_style = TextBoxStyleBuilder::new()
                .alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Middle)
                .build();
            for (i, section) in PROPOSAL_SECTIONS.iter().enumerate() {
                let text = match self.guess.get(i) {
                    Some(w) => {
                        w.word.as_ref()
                    },
                    None => "",
                };
                TextBox::with_textbox_style(
                    &text,
                    *section,
                    character_style,
                    textbox_style,
                ).draw(target)?;
            }
        }

        Ok(())
    }

    fn handle_tap_view<'a>(&mut self, point: Point, _: ()) -> Self::TapOutput
    where Self: 'a
    {
        let mut guess_tapped = None;
        for (i, section) in PROPOSAL_SECTIONS.iter().enumerate() {
            if section.contains(point) {
                if i < self.guess.len() {
                    guess_tapped = Some(self.guess.swap_remove(i));
                };
                self.clear();
            }
        }
        guess_tapped
    }
}

struct Variants<'a> {
    entry: &'a Vec<Vec<char>>,
    permutations: Vec<usize>,
    base_variants: &'a Vec<String>,
    vindex: usize,
}

impl<'a> Variants<'a> {
    fn new(entry: &'a Vec<Vec<char>>, base_variants: &'a Vec<String>) -> Self {
        Self {
            entry,
            permutations: vec![0; entry.len()],
            base_variants,
            vindex: 0,
        }
    }
}
impl<'a> Iterator for Variants<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut variant: String;
        if self.permutations.len() == 0 {
            return None;
        }
        if self.base_variants.len() == 0 {
            if self.vindex != 0 {
                return None
            }
            variant = "".to_owned();
        } else {
            if self.vindex == self.base_variants.len() {
                return None
            }
            variant = self.base_variants[self.vindex].clone();
        }
        // append each base variant with remaining entry combination
        let skip = variant.len();
        
        if skip >= self.permutations.len() {
            self.vindex += 1;
            return Some(variant)
        }

        for (l, c) in self.permutations.iter().enumerate().skip(skip) {
            variant.push(self.entry[l][*c]);
        }

        // incrementing permutation from end with carry
        // until non mutated part encountered
        let mut do_next_var = false;
        for (l, c) in self.permutations.iter_mut().enumerate().skip(skip).rev() {
            if *c != self.entry[l].len() - 1 {
                *c += 1;
                for i in (l + 1)..self.permutations.len() {
                    self.permutations[i] = 0;
                }
                break;
            }
            if l == skip {
                do_next_var = true
            }
        }
        // if incrementaion ended clear permutations, and do next base variant
        if do_next_var {
            self.vindex += 1;
            self.permutations = vec![0; self.entry.len()];
        }

        Some(variant)
    }
}