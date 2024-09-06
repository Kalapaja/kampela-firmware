#[cfg(not(feature="std"))]
use alloc::{vec::Vec, boxed::Box};
#[cfg(feature="std")]
use std::{vec::Vec, boxed::Box};

use embedded_graphics::{
    geometry::Point,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Drawable},
    primitives::{Primitive, PrimitiveStyle}
};

use mnemonic_external::WordSet;

use crate::{
    platform::Platform,
    widget::{
        view::{View, ViewScreen},
        nav_bar::nav_bar::{NavBar, NavCommand}
    },
    uistate::{EventResult, UpdateRequest, UnitScreen},
};

use crate::seed_entry::{
    keyboard::{Keyboard, REMOVE_KEY_WIDGET},
    key::Key,
    entry::Entry,
    proposal::Proposal,
    phrase::Phrase,
};

enum KeyboardState {
    Initial,
    Tapped,
    DrawTapped,
    InitialInverse,
}

pub struct SeedEntry<P> where
    P: Platform
{
    entry: Entry,
    keyboard: Keyboard,
    remove: Key,
    proposal: Proposal<P>,
    phrase: Phrase<P>,
    navbar_entry: NavBar,
    navbar_phrase: NavBar,
    tapped: KeyboardState,
    negative: bool,
}

impl<P: Platform> SeedEntry<P> {
    pub fn new(buffer: Option<WordSet>) -> Self
        where <P as Platform>::AsWordList: Sized {
        let mut wordlist = P::get_wordlist();
        let phrase = buffer.map(|ws| ws.to_wordlist_elements(&mut wordlist).unwrap());
        let mut state = SeedEntry {
            entry: Entry::new(),
            keyboard: Keyboard::new(),
            remove: Key::new("DEL", &REMOVE_KEY_WIDGET),
            proposal: Proposal::new(wordlist),
            phrase: Phrase::new(phrase),
            navbar_entry: NavBar::new(("clear", "")),
            navbar_phrase: NavBar::new(("back", "")),
            tapped: KeyboardState::Initial,
            negative: false,
        };
        Self::update_navbar_phrase(&mut state);
        state
    }
    pub fn get_entropy(&self) -> Option<Vec<u8>> {
        self.phrase.validate()
    }
    pub fn get_buffer(&self) -> WordSet {
        self.phrase.get_phrase()
            .iter()
            .collect()
    }
    fn switch_tapped(&mut self) -> bool {
        match self.tapped {
            KeyboardState::Initial => false,
            KeyboardState::Tapped => {
                if self.negative {
                    self.tapped = KeyboardState::InitialInverse;
                } else {
                    self.tapped = KeyboardState::DrawTapped;
                }
                self.negative = !self.negative;
                true
            },
            KeyboardState::DrawTapped => {
                self.tapped = KeyboardState::Initial;
                self.negative = false;
                false
            },
            KeyboardState::InitialInverse => {
                self.tapped = KeyboardState::DrawTapped;
                self.negative = true;
                false
            }
        }
    }
    fn update_navbar_phrase(&mut self) {
        if self.phrase.validate().is_some() {
            self.navbar_phrase = NavBar::new(("back", "next"))
        } else {
            self.navbar_phrase = NavBar::new(("back", ""))
        }
    }
}

impl<P: Platform> ViewScreen for SeedEntry<P> {
    type DrawInput<'a> = () where P: 'a;
    type DrawOutput = ();
    type TapInput<'a> = () where P: 'a;
    type TapOutput = ();

    fn draw_screen<'a, D>(&mut self, target: &mut D, _: ()) -> Result<(EventResult, ()), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        Self: 'a,
    {
        let state = None;
        let mut request = None;
        
        let t = self.switch_tapped();

        let filled = if self.negative {
            PrimitiveStyle::with_fill(BinaryColor::On)
        } else {
            PrimitiveStyle::with_fill(BinaryColor::Off)
        };
        target.bounding_box().into_styled(filled).draw(target)?;

        self.remove.draw(target, self.negative)?;
        self.keyboard.draw(target, self.negative)?;

        if self.entry.is_empty() {
            self.phrase.draw(target, self.negative)?;
            self.navbar_phrase.draw(target, self.negative)?;
        } else {
            self.entry.draw(target, self.negative)?;
            self.proposal.draw(target, (t, self.negative))?;
            self.navbar_entry.draw(target, self.negative)?;
        }

        match self.tapped {
            KeyboardState::DrawTapped |
            KeyboardState::InitialInverse => {
                request = Some(UpdateRequest::UltraFast);
            },
            _ => {},
        }
        Ok((EventResult { request, state }, ()))
    }

    fn handle_tap_screen<'a>(&mut self, point: Point, _: Self::TapInput<'a>) -> (crate::uistate::EventResult, Self::TapOutput)
    where
        Self: 'a
    {
        let mut state = None;
        let mut request = None;

        if let Some(Some(c)) = self.keyboard.handle_tap(point, ()) {
            if !self.phrase.is_maxed() {
                if !self.entry.is_maxed() {
                    self.entry.add_letter(c[0]);
                    self.proposal.add_letters(c);
                } else {
                    self.entry.set_invalid();
                }
            } else {
                self.phrase.set_invalid();
            }
            self.tapped = KeyboardState::Tapped;
            request = Some(UpdateRequest::UltraFast);
        };

        if self.entry.is_empty() && matches!(self.tapped, KeyboardState::Initial) {
            if self.remove.handle_tap(point, ()).is_some() {
                self.phrase.remove_word();
                self.update_navbar_phrase();
                self.tapped = KeyboardState::Tapped;
                request = Some(UpdateRequest::UltraFast);
            }
        }
        if !self.entry.is_empty() {
            if self.remove.handle_tap(point, ()).is_some() {
                self.proposal.remove_letter();
                self.entry.remove_letter();
                self.tapped = KeyboardState::Tapped;
                request = Some(UpdateRequest::UltraFast);
            }
        }
        if let Some(Some(guess)) = self.proposal.handle_tap(point, ()) {
            self.phrase.add_word(guess);
            self.entry.clear();
            self.update_navbar_phrase();
            request = Some(UpdateRequest::Fast);
        }
        if self.entry.is_empty() {
            if let Some(Some(c)) = self.navbar_phrase.handle_tap(point, ()) {
                match c {
                    NavCommand::Left => {
                        if self.phrase.is_empty() {
                            state = Some(UnitScreen::OnboardingRestoreOrGenerate);
                            request = Some(UpdateRequest::Fast);
                        } else {
                            let buffer = self.get_buffer();
                            state = Some(UnitScreen::ShowDialog(
                                "Are you sure?\nEntered data will be lost",
                                ("no", "yes"),
                                (
                                    Box::new(|| EventResult {
                                        request: Some(UpdateRequest::UltraFast),
                                        state: Some(UnitScreen::OnboardingRestore(Some(buffer))),
                                    }),
                                    Box::new(|| EventResult {
                                        request: Some(UpdateRequest::UltraFast),
                                        state: Some(UnitScreen::OnboardingRestoreOrGenerate)
                                    })
                                ),
                                true,
                            ));
                            request = Some(UpdateRequest::UltraFast);
                        }
                    },
                    NavCommand::Right => {
                        if let Some(e) = self.get_entropy() {
                            state = Some(UnitScreen::OnboardingBackup(Some(e)));
                            request = Some(UpdateRequest::Fast);
                        } else {
                            self.phrase.set_invalid();
                            request = Some(UpdateRequest::UltraFast);
                        }
                    },
                }
            }
        } else {
            if matches!(self.navbar_entry.handle_tap(point, ()), Some(Some(NavCommand::Left))) {
                self.entry.clear();
                self.proposal.clear();
                request = Some(UpdateRequest::Fast);
            }
        }

        (EventResult{ request, state }, ())
    }
}