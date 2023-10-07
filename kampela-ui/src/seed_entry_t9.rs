//! Screen for seed phrase recovery
#[cfg(not(feature="std"))]
use alloc::{format, string::String, string::ToString, vec::Vec, collections::VecDeque, vec, fmt};

#[cfg(feature="std")]
use std::{format, string::String, string::ToString, vec::Vec, collections::VecDeque, vec, fmt};

use embedded_graphics::{
    mono_font::{
        ascii::{FONT_10X20, FONT_4X6, FONT_6X10},
        MonoTextStyle, self,
    },
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, Rectangle, ellipse,
    },
    Drawable,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::BinaryColor,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};
use rand::{Rng, seq::SliceRandom};
use patches::phrase::{phrase_to_entropy, wordlist_english, WordListElement};

use crate::uistate::{EventResult, Screen, UpdateRequest};
use crate::display_def::*;

const WORD_LENGTH: usize = 8;
const MAX_SEED: usize = 24;

const INPUT_AREA: Rectangle = Rectangle::new(
    Point::new(GAP as i32, GAP as i32),
    Size::new(100, SCREEN_SIZE_Y - GAP * 2),
);
const PHRASE_AREA: Rectangle = Rectangle::new(
    INPUT_AREA.top_left,
    Size::new(INPUT_AREA.size.width, FONT_10X20.character_size.height * 2 + FONT_10X20.character_spacing),
);
const WORD_AREA: Rectangle = Rectangle::new(
    Point::new(INPUT_AREA.top_left.x, INPUT_AREA.top_left.y + PHRASE_AREA.size.height as i32 + GAP as i32),
    Size::new(INPUT_AREA.size.width, INPUT_AREA.size.height - PHRASE_AREA.size.height - GAP),
);
const WORD_FONT:mono_font::MonoFont = FONT_10X20;
const MAX_PROPOSAL: usize = (WORD_AREA.size.height / WORD_FONT.character_size.height) as usize;

const KEY_COUNT: usize = 12;
const PAD_AREA: Rectangle = Rectangle::new(
    Point::new(INPUT_AREA.top_left.x + INPUT_AREA.size.width as i32 + GAP as i32, GAP as i32),
    Size::new(SCREEN_SIZE_X - INPUT_AREA.top_left.x as u32 - INPUT_AREA.size.width - GAP * 2, SCREEN_SIZE_Y - GAP * 2)
);

const KEY_BUTTON_SIZE: Size = Size::new((PAD_AREA.size.width - GAP * 2) / 3, (PAD_AREA.size.height - GAP * 3) / 4);

const NUM_LABELS: [&str; 8] = [
    "ABC",
    "DEF",
    "GHI",
    "JKL",
    "MNO",
    "PQRS",
    "TUV",
    "WXYZ",
];
#[derive(Copy, Clone)]
enum Command {
    Chg,
    Del,
    Sel,
    Ok,
}

impl Command {
    fn as_str(&self) -> &'static str {
        match self {
            Command::Chg => "chg",
            Command::Del => "del",
            Command::Sel => "sel",
            Command::Ok => "ok",
        }
    }
}

fn get_padlabels<R: Rng + ?Sized>(rng: &mut R) -> [&str; 8] {
    let mut padlabels = NUM_LABELS;
    padlabels.shuffle(rng);
    padlabels
}

lazy_static! {
    static ref KEY_BUTTONS: [KeyButton; KEY_COUNT] = {
        let x1: i32 = PAD_AREA.top_left.x;
        let x2: i32 = PAD_AREA.top_left.x + KEY_BUTTON_SIZE.width as i32 + GAP as i32;
        let x3: i32 = PAD_AREA.top_left.x + (KEY_BUTTON_SIZE.width as i32 + GAP as i32) * 2;

        let y1: i32 = PAD_AREA.top_left.y;
        let y2: i32 = PAD_AREA.top_left.y + KEY_BUTTON_SIZE.height as i32 + GAP as i32;
        let y3: i32 = PAD_AREA.top_left.y + (KEY_BUTTON_SIZE.height as i32 + GAP as i32 ) * 2;
        let y4: i32 = PAD_AREA.top_left.y + (KEY_BUTTON_SIZE.height as i32 + GAP as i32) * 3;

        [
            KeyButton::new(ButtonKind::Command(Command::Chg), Point::new(x1, y1)),
            KeyButton::new(ButtonKind::Index(0), Point::new(x2, y1)),
            KeyButton::new(ButtonKind::Index(1), Point::new(x3, y1)),
            KeyButton::new(ButtonKind::Index(2), Point::new(x1, y2)),
            KeyButton::new(ButtonKind::Index(3), Point::new(x2, y2)),
            KeyButton::new(ButtonKind::Index(4), Point::new(x3, y2)),
            KeyButton::new(ButtonKind::Index(5), Point::new(x1, y3)),
            KeyButton::new(ButtonKind::Index(6), Point::new(x2, y3)),
            KeyButton::new(ButtonKind::Index(7), Point::new(x3, y3)),
            KeyButton::new(ButtonKind::Command(Command::Del), Point::new(x1, y4)),
            KeyButton::new(ButtonKind::Command(Command::Sel), Point::new(x2, y4)),
            KeyButton::new(ButtonKind::Command(Command::Ok), Point::new(x3, y4)),
        ]
    };
}

#[derive(Copy, Clone)]
enum ButtonKind {
    Command(Command),
    Index(usize),
}
struct KeyButton {
    label: ButtonKind,
    area: Rectangle,
}

impl KeyButton {
    pub fn new(label: ButtonKind, origin: Point) -> Self {
        KeyButton {
            label,
            area: Rectangle::new(
                origin,
                KEY_BUTTON_SIZE,
            ),
        }
    }

    fn handle(&self, point: Point) -> Option<ButtonKind>{
        if self.area.contains(point) {
            Some(self.label)
        } else {
            None
        }
    }

    fn draw<D>(&self, label: &str, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 2);

        self.area.into_styled(thin_stroke).draw(display)?;

        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        TextBox::with_textbox_style(
            label,
            self.area.bounding_box(),
            character_style,
            textbox_style,
        )
        .draw(display)?;
        Ok(())
    }
}

struct SeedBuffer {
    seed_phrase: Vec<WordListElement>,
    ready: Option<Vec<u8>>,
}
///manual Debug impl, neccessary for #derive in SeedEntryState
impl fmt::Debug for SeedBuffer {
    fn fmt(&self, __arg_0: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SeedBuffer { seed_phrase: ref __self_0_0, ready: ref __self_0_1 } => {
                let mut builder = __arg_0.debug_struct("SeedBuffer");
                let _ = builder.field("ready", &&(*__self_0_1));
                builder.finish()
            }
        }
    }
}
impl SeedBuffer {
    pub fn new() -> Self {
        SeedBuffer {
            seed_phrase: Vec::with_capacity(MAX_SEED),
            ready: None,
        }
    }

    pub fn len(&self) -> usize {
        self.seed_phrase.len()
    }

    pub fn proposed_phrase(&self) -> String {
        self.seed_phrase
            .iter()
            .map(|a| String::from(a.word()))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn remove_last(&mut self) -> bool {
        self.seed_phrase.pop().is_some()
    }

    pub fn submit_word(&mut self, word: WordListElement) {
        if self.seed_phrase.len()<MAX_SEED {
            self.seed_phrase.push(word);
        }
    }

    pub fn validate(&mut self) -> bool {
        match phrase_to_entropy(
            &self
                .seed_phrase
                .iter()
                .map(|a| String::from(a.word()))
                .collect::<Vec<String>>()
                .join(" "),
        ) {
            Ok(a) => {
                self.ready = Some(a);
                true
            }
            Err(_) => false,
        }
    }
}

struct Variants<'a> {
    entry: &'a Vec<VecDeque<char>>,
    perm: Vec<usize>,
    variants: &'a VecDeque<String>,
    vindex: usize,
}

impl<'a> Iterator for Variants<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut variant: String;
        if self.perm.len() == 0 {
            return None;
        }
        if self.variants.len() == 0 {
            if self.vindex != 0 {
                return None
            }
            variant = "".to_string();
        } else {
            if self.vindex == self.variants.len() {
                return None
            }
            variant = self.variants[self.vindex].clone();
        }
        // append each current variant with all possible variants
        // skip calcultaion needed to prevent variant modification
        let skip = variant.len();
        
        if skip >= self.perm.len() {
            self.vindex += 1;
            return Some(variant)
        }

        for (l, c) in self.perm.iter().enumerate().skip(skip) {
            variant.push(self.entry[l][*c]);
        }

        let mut do_next_var = false;
        // increment of permutation from end with carry
        // until non mutated part encountered
        for (l, c) in self.perm.iter_mut().enumerate().skip(skip).rev() {
            if *c != self.entry[l].len() - 1 {
                *c += 1;
                for i in (l + 1)..self.perm.len() {
                    self.perm[i] = 0;
                }
                break;
            }
            if l == skip {
                do_next_var = true
            }
        }
        // if incrementaion ended clear perm, and do next variant
        if do_next_var {
            self.vindex += 1;
            self.perm = vec![0; self.entry.len()];
        }

        Some(variant)
    }
}



/// Key entry state for seed phrase recovery screen
struct Proposal {
    entry: Vec<VecDeque<char>>,
    variants: VecDeque<String>,
    variants_depth: usize,
    guess: Vec<WordListElement>,
}

///manual Debug impl, neccessary for #derive in SeedEntryState
impl fmt::Debug for Proposal {
    fn fmt(&self, __arg_0: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Proposal {
                entry: ref __self_0_0,
                variants: ref __self_0_1,
                variants_depth: ref __self_0_2,
                guess: ref __self_0_3,
            } => {
                let mut builder = __arg_0.debug_struct("Proposal");
                let _ = builder.field("entry", &&(*__self_0_0));
                let _ = builder.field("variants", &&(*__self_0_1));
                let _ = builder.field("variants_depth", &&(*__self_0_2));
                builder.finish()
            }
        }
    }
}

impl Proposal {
    pub fn new() -> Self {
        Self {
            entry: Vec::new(),
            variants: VecDeque::new(),
            variants_depth: 0,
            guess: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        *self = Proposal::new();
    }

    pub fn entry_len(&self) -> usize {
        self.entry.len()
    }

    pub fn proposed_len(&self) -> usize {
        self.guess.len()
    }

    fn clear_variants(&mut self) {
        self.variants = VecDeque::new();
        self.variants_depth = 0;
    }

    fn get_variants(&mut self) -> Variants {
        if self.variants.len() < 2 && self.variants_depth >= self.entry_len(){
            self.clear_variants();
        }
        Variants { entry: &self.entry, perm: vec![0; self.entry_len()], variants: &self.variants, vindex: 0 }
    }


    fn append_guess(guess: &mut Vec<WordListElement>, variants: Variants, prev_variants: Option<VecDeque<String>>) -> VecDeque<String> {
        let mut prev_variants_is_some = false;
        let mut new_variants = if let Some(pvs) = prev_variants {
            prev_variants_is_some = true;
            pvs
        } else {
            VecDeque::new()
        };

        for v in variants {
            if new_variants.iter().any(|pv| pv == &v) {
                // skip precalculated variant if this is second round
                continue;
            }
            
            let mut g = wordlist_english().get_words_by_prefix(&v);
            if g.len() == 0 {
                continue;
            }
            
            new_variants.push_back(v);
            //ascending sort by length
            g.sort_by(|a, b| a.word().len().cmp(&b.word().len()));
            guess.append(&mut g);
            // break if there too many guesses to display
            // except if all variants needed, hence !prev_variaants_is_some
            // if at least found two variants
            // to calculte the skip of letters when chg is pressed
            if guess.len() >= MAX_PROPOSAL && new_variants.len() > 1 && !prev_variants_is_some {
                break;
            }
        }
        new_variants
    }

    fn make_guess(&mut self) -> bool {
        let mut guess: Vec<WordListElement> = vec![];
        let variants = self.get_variants();

        let mut new_variants = Proposal::append_guess(&mut guess, variants,  None);
        //if guesses too few, repeat with all variants
        if guess.len() < MAX_PROPOSAL && self.variants_depth == 0 {
            self.variants = VecDeque::new();
            let variants = self.get_variants();
            new_variants = Proposal::append_guess(&mut guess, variants, Some(new_variants));
            self.variants_depth = self.entry_len(); //sets depth when all variants was obtained for optimization reasons
        }

        if guess.len() > 0 {
            self.variants = new_variants;
            self.guess = guess;
            true
        } else {
            false
        }

    }

    fn rotate_all_entry(&mut self) {
        for l in (0..self.entry_len()).rev() {
            let nth_entry = if let Some(p) = self.entry.get_mut(l){
                p
            } else {
                continue;
            };
            if nth_entry.len() < 2 {
                continue;
            };
            let mut rot: usize = 0;
            // checking for each possible character in nth entry
            // if there any overlap with first variant,
            // then counts rotation needed
            // to skip invalid combinations
            for c in nth_entry.iter() {
                let first_v = if let Some(v) = self.variants.get(0) {
                    v
                } else {
                    continue;
                };
                let nth_v = if let Some(nth) = first_v.chars().nth(l) {
                    nth
                } else {
                    continue;
                };
                if nth_v == *c {
                    break;
                };
                rot += 1;
            }
            if rot > nth_entry.len() - 1 {
                continue;
            }
            nth_entry.rotate_left(rot);
        }
        
    }

    pub fn rearrange(&mut self) -> bool {
        self.variants.pop_front();
        self.rotate_all_entry();
        self.make_guess()
    }

    pub fn add_letters(&mut self, letters: VecDeque<char>) -> bool {
        assert!(
            letters.iter().all(|l| l.is_ascii_alphabetic() && l.is_ascii_lowercase()),
            "chars should be alphabetic and lowercase"
        );
        self.entry.push(letters);
        let is_guesses = self.make_guess();
        if !is_guesses {
            self.entry.pop();
            return is_guesses;
        }
        self.rotate_all_entry();
        is_guesses
    }

    pub fn remove_letter(&mut self) -> bool {
        self.entry.pop();
        self.variants.iter_mut().for_each(|v| {v.pop();});

        if self.variants_depth >= self.entry_len() {
            self.variants_depth = 0;
        }
        self.make_guess()
    }

    pub fn list(&self) -> String {
        self.guess.iter().map(|e| e.word()).collect::<Vec<&str>>().join("\n")
    }

    pub fn select_button_action(&mut self) -> Option<WordListElement> {
        if self.proposed_len() > 0 {
            let out = self.guess.swap_remove(0);
            self.clear();
            Some(out)
        } else {
            None
        }
    }
}

/// UI state for seed phrase recovery
#[derive(Debug)]
pub struct SeedEntryState {
    seed_phrase: SeedBuffer,
    proposal: Proposal,
    pad_permutation: [String; 8],
}

impl SeedEntryState{
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        SeedEntryState {
            seed_phrase: SeedBuffer::new(),
            proposal: Proposal::new(),
            pad_permutation: get_padlabels(rng).map(|l| l.to_string()),
        }
    }

    fn new_state(&self, seed: &mut Option<Vec<u8>>) -> Option<Screen> {
        if let Some(a) = &self.seed_phrase.ready {
            *seed = Some(a.clone());
            Some(Screen::OnboardingBackup) // TODO (entropy_to_phrase(&a).unwrap()))
        } else { None }
    }

    fn update_entry<D>(&self, fast_display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let character_style = MonoTextStyle::new(&WORD_FONT, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Top)
            .build();
        let clear = PrimitiveStyle::with_fill(BinaryColor::Off);
        WORD_AREA.into_styled(clear).draw(fast_display)?;
        
        TextBox::with_textbox_style(
            &format!("{}", self.proposal.list()),
            WORD_AREA,
            character_style,
            textbox_style,
        )
        .draw(fast_display)?;

        let underline: String = vec!["_"; self.proposal.entry_len()].into_iter().collect();
        TextBox::with_textbox_style(
            &format!("{}", underline),
            WORD_AREA,
            character_style,
            textbox_style,
        )
        .draw(fast_display)?;

        Ok(())
    }

    fn update_proposal<D>(&self, fast_display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        self.update_entry(fast_display)?;
        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();
        let clear = PrimitiveStyle::with_fill(BinaryColor::Off);
        PHRASE_AREA.into_styled(clear).draw(fast_display)?;
        let mut proposal = self.seed_phrase.proposed_phrase();
        if proposal == "" {
            proposal = String::from("please enter seed phrase")
        };
        TextBox::with_textbox_style(&proposal, PHRASE_AREA, character_style, textbox_style)
            .draw(fast_display)?;

        Ok(())
    }

    fn update_pad<D, R>(&mut self, rng: &mut R, fast_display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        R: Rng + ?Sized,
    {
        self.pad_permutation = get_padlabels(rng).map(|l| l.to_string());
        self.draw_key_buttons(fast_display)
    }

    fn change_button_event<D>(&mut self, fast_display: &mut D) -> Result<UpdateRequest, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let mut out = UpdateRequest::new();

        self.proposal.rearrange();
        self.update_entry(fast_display)?;

        out.set_slow();
        Ok(out)
    }

    fn back_button_event<D, R>(&mut self, rng: &mut R, fast_display: &mut D) -> Result<UpdateRequest, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        R: Rng + ?Sized,
    {
        let mut out = UpdateRequest::new();
        if self.proposal.entry_len() > 0 {
            self.proposal.remove_letter();
            self.update_entry(fast_display)?;
            out.set_slow();
        } else if self.seed_phrase.len() > 0 {
            self.seed_phrase.remove_last();
            self.update_entry(fast_display)?;
            self.update_proposal(fast_display)?;
            self.update_pad(rng, fast_display)?;
            out.set_slow();
        };
        Ok(out)
    }

    fn select_button_event<D, R>(&mut self, rng: &mut R, fast_display: &mut D) -> Result<UpdateRequest, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        R: Rng + ?Sized,
    {
        let mut out = UpdateRequest::new();
        if let Some(a) = self.proposal.select_button_action() {
            self.seed_phrase.submit_word(a);
            self.update_proposal(fast_display)?;
            self.update_entry(fast_display)?;
            self.update_pad(rng, fast_display)?;
            out.set_slow();
        };
        Ok(out)
    }

    fn forward_button_event<D>(&mut self, fast_display: &mut D) -> Result<UpdateRequest, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let mut out = UpdateRequest::new();
        if self.seed_phrase.validate() {
            out.set_slow();
        };
        Ok(out)
    }

    fn command_button_event<D, R>(&mut self, command: Command, rng: &mut R, fast_display: &mut D) -> Result<UpdateRequest, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        R: Rng + ?Sized,
    {
        match command {
            Command::Chg => self.change_button_event( fast_display),
            Command::Del => self.back_button_event(rng, fast_display),
            Command::Sel => self.select_button_event(rng, fast_display),
            Command::Ok => self.forward_button_event(fast_display),
        }
    }

    fn num_button_event<D>(&mut self, key: usize, fast_display: &mut D) -> Result<UpdateRequest, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let letters = self.pad_permutation[key]
            .to_lowercase()
            .chars()
            .collect();
        let mut out = UpdateRequest::new();
        if self.proposal.add_letters(letters) {
            self.update_entry(fast_display)?;
            out.set_slow();
        }
        Ok(out)
    }

    fn handle_button<D, R: Rng + ?Sized>(&mut self, point: Point, rng: &mut R, fast_display: &mut D) -> Result<UpdateRequest, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let mut out = UpdateRequest::new();
        if WORD_AREA.contains(point) {
            let rel_y = point.y - WORD_AREA.top_left.y;
            let word_index = usize::try_from(rel_y as u32 / WORD_FONT.character_size.height);
            if let Ok(word_index) = word_index {
                if self.proposal.guess.len() > word_index {
                    let word = self.proposal.guess.swap_remove(word_index);
                    self.proposal.clear();
                    self.seed_phrase.submit_word(word);
                    self.update_proposal(fast_display)?;
                    self.update_entry(fast_display)?;
                    self.update_pad(rng, fast_display)?;
                    out.set_slow();
                }
            }
        } else {
            for button in KEY_BUTTONS.iter() {
                if let Some(a) = button.handle(point) {
                    let d = match a {
                        ButtonKind::Command(command) => self.command_button_event(command, rng, fast_display),
                        ButtonKind::Index(index) => self.num_button_event(index, fast_display),
                    };
                    out.propagate(d?);
                    break;
                }
            }
        }
        Ok(out)
    }

    pub fn handle_event<D, R>(&mut self, point: Point, seed: &mut Option<Vec<u8>>, rng: &mut R, fast_display: &mut D) -> Result<EventResult, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        R: Rng + ?Sized,
    {
        let request = self.handle_button(point, rng, fast_display)?;
        let state = self.new_state(seed);
        Ok(EventResult {request, state})
    }

    fn draw_key_buttons<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        for button in KEY_BUTTONS.iter() {
            match &button.label {
                ButtonKind::Command(label) => button.draw(&label.as_str(), display)?,
                ButtonKind::Index(index) => button.draw(&self.pad_permutation[*index], display)?,
            }
        }
        Ok(())
    }

    /// Draw seed recovery screen
    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        //self.draw_progress(display)?;
        self.update_proposal(display)?;
        self.draw_key_buttons(display)?;
        Ok(())
    }
}
