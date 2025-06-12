#[cfg(not(feature="std"))]
use alloc::{vec::Vec, boxed::Box};
#[cfg(feature="std")]
use std::{vec::Vec, boxed::Box};

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Drawable},
    primitives::{Primitive, PrimitiveStyle}
};

use mnemonic_external::{AsWordList, Bits11, WordListElement, WordSet};

use crate::{
    platform::Platform, uistate::{Event, EventResult, UnitScreen, UpdateRequest}, widget::{
        nav_bar::nav_bar::{NavBar, NavCommand}, view::{View, ViewScreen}
    }
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
}

impl<P: Platform> SeedEntry<P> {
    pub fn new(buffer: Option<WordSet>) -> Self
        where <P as Platform>::AsWordList: Sized {
        let wordlist = P::get_wordlist();
        let phrase = buffer
            .map(|ws| {
                ws.bits11_set
                    .iter()
                    .map(|bits| WordListElement{word: wordlist.get_word(*bits).unwrap(), bits11: *bits})
                    .collect()
            });
        let mut state = SeedEntry {
            entry: Entry::new(),
            keyboard: Keyboard::new(),
            remove: Key::new("DEL", &REMOVE_KEY_WIDGET),
            proposal: Proposal::new(wordlist),
            phrase: Phrase::new(phrase),
            navbar_entry: NavBar::new(("clear", "")),
            navbar_phrase: NavBar::new(("back", "")),
            tapped: KeyboardState::Initial,
        };
        Self::update_navbar_phrase(&mut state);
        state
    }
    pub fn get_entropy(&self) -> Option<Vec<u8>> {
        self.phrase.validate()
    }
    pub fn get_buffer(&self) -> WordSet {
        WordSet {
            bits11_set: self.phrase.get_phrase()
            .iter()
            .map(|w| w.bits11)
            .collect::<Vec<Bits11>>()
        }
    }
    fn switch_tapped(&mut self) -> bool {
        match self.tapped {
            KeyboardState::Initial => false,
            KeyboardState::Tapped => {
                self.tapped = KeyboardState::DrawTapped;
                true
            },
            KeyboardState::DrawTapped => {
                self.tapped = KeyboardState::Initial;
                false
            },
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
    type EventInput<'a> = () where P: 'a;
    type EventOutput = ();

    fn draw_screen<'a, D>(&mut self, target: &mut D, _: ()) -> Result<(EventResult, ()), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        Self: 'a,
    {
        let state = None;
        let mut request = None;
        
        let t = self.switch_tapped();
        if !t {
            target.bounding_box().into_styled(PrimitiveStyle::with_fill(BinaryColor::Off)).draw(target)?;
        }

        self.remove.draw(target, (t, false))?;
        self.keyboard.draw(target, (t, false))?;

        if matches!(self.tapped, KeyboardState::Initial) {
            if self.entry.is_empty() {
                self.phrase.draw(target, false)?;
                self.navbar_phrase.draw(target, false)?;
            } else {
                self.entry.draw(target, false)?;
                self.proposal.draw(target, false)?;
                self.navbar_entry.draw(target, false)?;
            }
        }

        if matches!(self.tapped, KeyboardState::DrawTapped) {
            request = Some(UpdateRequest::Invocate);
        }
        
        Ok((EventResult { request, state }, ()))
    }

    fn handle_event_screen<'a>(&mut self, event: Event, _: Self::EventInput<'a>) -> (crate::uistate::EventResult, Self::EventOutput)
    where
        Self: 'a
    {
        let mut state = None;
        let mut request = None;

        if let Some(Some((c, r))) = self.keyboard.handle_event(event, ()) {
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
            request = Some(UpdateRequest::Part(r));
        };

        if self.entry.is_empty() && matches!(self.tapped, KeyboardState::Initial) {
            if self.remove.handle_event(event, ()).unwrap_or(None).is_some() {
                self.phrase.remove_word();
                self.update_navbar_phrase();
                self.tapped = KeyboardState::Tapped;
                request = Some(UpdateRequest::Part(self.remove.bounding_box_absolut()));
            }
        }
        if !self.entry.is_empty() {
            if self.remove.handle_event(event, ()).unwrap_or(None).is_some() {
                self.proposal.remove_letter();
                self.entry.remove_letter();
                self.tapped = KeyboardState::Tapped;
                request = Some(UpdateRequest::Part(self.remove.bounding_box_absolut()));
            }
        }
        match self.proposal.handle_event(event, ()) {
            Some(Some(Some(guess))) => {
                self.phrase.add_word(guess);
                self.entry.clear();
                self.update_navbar_phrase();
                request = Some(UpdateRequest::UltraFastSelective);
            },
            Some(Some(None)) => {
                // after invocate
                request = Some(UpdateRequest::UltraFastSelective);
            },
            _ => {}
        }
        if self.entry.is_empty() {
            if let Some(Some(c)) = self.navbar_phrase.handle_event(event, ()) {
                match c {
                    NavCommand::Left => {
                        if self.phrase.is_empty() {
                            state = Some(UnitScreen::OnboardingRestoreOrGenerate);
                            request = Some(UpdateRequest::Fast);
                        } else {
                            let buffer = self.get_buffer();
                            state = Some(UnitScreen::ShowDialog((
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
                            )));
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
            if matches!(self.navbar_entry.handle_event(event, ()), Some(Some(NavCommand::Left))) {
                self.entry.clear();
                self.proposal.clear();
                request = Some(UpdateRequest::UltraFastSelective);
            }
        }

        (EventResult{ request, state }, ())
    }
}