use embedded_graphics::{
    geometry::Point,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Drawable},
    primitives::{Primitive, PrimitiveStyle}
};

use crate::{
    nav_bar::nav_bar::{NavBar, NavCommand}, uistate::{EventResult, UpdateRequest, UnitScreen}, widget::view::{View, ViewScreen}
};

use crate::seed_entry::{
    keyboard::{Keyboard, REMOVE_KEY_WIDGET},
    key::Key,
    entry::Entry,
    proposal::Proposal,
    phrase::Phrase,
};

pub struct SeedEntry {
    entry: Entry,
    keyboard: Keyboard,
    remove: Key,
    proposal: Proposal,
    phrase: Phrase,
    navbar: NavBar,
    tapped: bool,
}

impl SeedEntry {
    pub fn new() -> Self {
        SeedEntry {
            entry: Entry::new(),
            keyboard: Keyboard::new(),
            remove: Key::new("DEL", &REMOVE_KEY_WIDGET),
            proposal: Proposal::new(),
            phrase: Phrase::new(),
            navbar: NavBar::new(),
            tapped: false,
        }
    }
    fn reset_tapped(&mut self) -> bool {
        if self.tapped {
            self.tapped = false;
            true
        } else {
            false
        }
    }
}

impl ViewScreen for SeedEntry {
    type DrawInput<'a> = ();
    type DrawOutput = ();
    type TapInput<'a> = ();
    type TapOutput = ();

    fn draw_screen<'a, D>(&mut self, target: &mut D, _: ()) -> Result<(EventResult, ()), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let state = None;
        let mut request = UpdateRequest::new();
        
        let t = self.reset_tapped();

        let filled = if t {
            PrimitiveStyle::with_fill(BinaryColor::On)
        } else {
            PrimitiveStyle::with_fill(BinaryColor::Off)
        };
        target.bounding_box().into_styled(filled).draw(target)?;

        if self.remove.draw(target, t)? {
            request.set_ultrafast();
        };
        if self.keyboard.draw(target, t)?.is_some() {
            request.set_ultrafast();
        }
        if self.entry.is_empty() {
            self.phrase.draw(target, t)?;
        } else {
            self.entry.draw(target, t)?;
            self.proposal.draw(target, t)?;
        }
        self.navbar.draw(target, t)?;
        Ok((EventResult { request, state }, ()))
    }

    fn handle_tap_screen<'a>(&mut self, point: Point, _: ()) -> (crate::uistate::EventResult, Self::TapOutput) {
        let mut state = None;
        let mut request = UpdateRequest::new();
        if !self.phrase.is_maxed() {
            if let Some(Some(c)) = self.keyboard.handle_tap(point, ()) {
                self.entry.add_letter(c[0]);
                self.proposal.add_letters(c);
                self.tapped = true;
                request.set_ultrafast();
            };
        }
        if self.remove.handle_tap(point, ()).is_some() {
            if self.entry.is_empty() {
                self.phrase.remove_word();
            } else {
                self.proposal.remove_letter();
                self.entry.remove_letter();
            }
            self.tapped = true;
            request.set_ultrafast();
        }
        if let Some(Some(guess)) = self.proposal.handle_tap(point, ()) {
            self.phrase.add_word(guess);
            self.entry.clear();
            request.set_fast();
        }
        if let Some(Some(c)) = self.navbar.handle_tap(point, ()) {
            match c {
                NavCommand::Back => {
                    state = Some(UnitScreen::OnboardingRestoreOrGenerate);
                    request.set_slow();
                },
                NavCommand::Next => {
                    if let Some(e) = self.phrase.validate() {
                        state = Some(UnitScreen::OnboardingBackup(e));
                        request.set_fast();
                    }
                },
            }
        }
        (EventResult{ request, state }, ())
    }
}