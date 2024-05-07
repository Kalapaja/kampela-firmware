//! UI state unit; almost all inerfacing should be done through this "object"

#[cfg(not(feature="std"))]
mod stdwrap {
    pub use alloc::string::String;
    pub use alloc::vec::Vec;
    pub use alloc::rc::Rc;
    pub use alloc::borrow::ToOwned;
}


#[cfg(feature="std")]
mod stdwrap {
    pub use std::string::String;
    pub use std::vec::Vec;
    pub use std::rc::Rc;
    pub use std::borrow::ToOwned;
}


use core::mem::take;

use stdwrap::*;

use embedded_graphics::{
    prelude::Primitive,
    primitives::{
        Line, PrimitiveStyle},
    Drawable,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    pixelcolor::BinaryColor, primitives::Rectangle,
};

use crate::{display_def::*, pin::pin::Pincode, widget::view::ViewScreen};

use crate::backup::Backup;

use crate::platform::Platform;

use crate::seed_entry::SeedEntryState;

use crate::restore_or_generate;

use crate::message;

use rand::{CryptoRng, Rng};

pub struct EventResult {
    pub request: UpdateRequest,
    pub state: Option<UnitScreen>,
}

//TODO: enum
pub struct UpdateRequest {
    fast: bool,
    slow: bool,
    part: Option<Rectangle>,
    hidden: bool,
}

impl UpdateRequest {
    pub fn new() -> Self {
        UpdateRequest {
            fast: false,
            slow: false,
            part: None,
            hidden: false,
        }
    }

    pub fn set_slow(&mut self) {
        self.slow = true;
    }

    pub fn set_fast(&mut self) {
        self.fast = true;
    }
    pub fn set_part(&mut self, area: Rectangle) {
        self.part = Some(area);
    }

    pub fn set_both(&mut self) { //unnecessary
        self.set_slow();
        self.set_fast();
    }

    pub fn set_hidden(&mut self) {
        self.hidden = true;
    }

    pub fn propagate(&mut self, mut new: UpdateRequest) {
        if new.read_fast() { self.set_fast() };
        if new.read_slow() { self.set_slow() };
        if new.read_hidden() { self.set_hidden() };
        if let Some(a) = new.read_part() { self.set_part(a) };
    }

    pub fn read_slow(&mut self) -> bool {
        if self.slow {
            self.slow = false;
            true
        } else { false }
    }

    pub fn read_fast(&mut self) -> bool {
        if self.fast {
            self.fast = false;
            true
        } else { false }
    }

    pub fn read_part(&mut self) -> Option<Rectangle> {
        if self.part.is_some() {
            let area = self.part;
            self.part = None;
            area
        } else { None }
    }

    pub fn read_hidden(&mut self) -> bool {
        if self.hidden {
            self.hidden = false;
            true
        } else { false }
    }
}

impl Default for UpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// State of UI
pub struct UIState<P> where
    P: Platform,
{
    screen: Screen<P::Rng>,
    pub platform: P,
    unlocked: bool,
}

#[derive(Clone)]
pub enum UnitScreen {
    OnboardingRestoreOrGenerate,
    OnboardingRestore,
    OnboardingBackup(Vec<u8>),
    ShowMessage(String),
    ShowTransaction,
    ShowExtension,
    QRSignature,
    QRAddress,
    Locked,
    End,
}

/// keeps states of screens, initialization can take a lot of memory
pub enum Screen<R: Rng + ?Sized> {
    PinEntry(Pincode<R>, UnitScreen),
    OnboardingRestoreOrGenerate,
    OnboardingRestore(SeedEntryState),
    OnboardingBackup(Backup),
    ShowMessage(String, Option<UnitScreen>),
    ShowTransaction,
    ShowExtension,
    QRSignature,
    QRAddress,
    Locked,
}

impl <P: Platform> UIState<P> {
    pub fn new(mut platform: P, h: &mut <P as Platform>::HAL) -> Self {
        platform.read_entropy();
        let mut initial_screen: Option<UnitScreen>;
        let mut unlocked: bool;
        if platform.public().is_none() {
            initial_screen = Some(UnitScreen::OnboardingRestoreOrGenerate);
            unlocked = true;
        } else {
            initial_screen = Some(UnitScreen::QRAddress);
            unlocked = false;
        }
        let mut state = UIState {
            screen: Screen::Locked, // doesn't matter
            platform,
            unlocked,
        };
        state.switch_screen(initial_screen, h);
        state
    }

    pub fn display(&mut self) -> &mut <P as Platform>::Display {
        self.platform.display()
    }

    fn switch_screen(&mut self, s: Option<UnitScreen>, h: &mut <P as Platform>::HAL) {
        if let Some(s) = s {
            match s {
                UnitScreen::QRAddress => {
                    self.screen = Screen::QRAddress;
                },
                UnitScreen::Locked => {
                    self.screen = Screen::Locked;
                },
                UnitScreen::OnboardingBackup(e) => {
                    self.screen = Screen::OnboardingBackup(Backup::new(e));
                },
                UnitScreen::ShowMessage(m) => {
                    self.screen = Screen::ShowMessage(m, None);
                },
                UnitScreen::OnboardingRestore => {
                    self.screen = Screen::OnboardingRestore(SeedEntryState::new());
                },
                UnitScreen::OnboardingRestoreOrGenerate => {
                    self.screen = Screen::OnboardingRestoreOrGenerate;
                },
                UnitScreen::QRSignature => {
                    if self.unlocked {
                        if matches!(self.screen, Screen::ShowMessage(_, _)) {
                            self.screen = Screen::QRSignature;
                        } else {
                            self.screen = Screen::ShowMessage("Signing...".to_owned(), Some(UnitScreen::QRSignature));
                        }
                    } else {
                        self.screen = Screen::PinEntry(Pincode::<P::Rng>::new(&mut P::rng(h)), UnitScreen::QRSignature);
                    }
                },
                UnitScreen::ShowExtension => {
                    self.screen = Screen::ShowExtension;
                },
                UnitScreen::ShowTransaction => {
                    self.screen = Screen::ShowTransaction;
                },
                _ => {}
            }
        }
    }

    /// Read user touch event
    pub fn handle_tap<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        point: Point,
        h: &mut <P as Platform>::HAL,
    ) -> Result<UpdateRequest, <<P as Platform>::Display as DrawTarget>::Error>
    {
        let fast_display = self.platform.display();
        let mut out = UpdateRequest::new();
        let mut new_screen = None;
        match self.screen {
            Screen::PinEntry(ref mut a, ref u) => {
                let (res, pinok) = a.handle_tap_screen(point, self.platform.pin());
                out = res.request;
                new_screen = res.state;
                if pinok {
                    self.unlocked = true;
                }
            },
            Screen::OnboardingRestoreOrGenerate => match point.x {
                0..=100 => {
                    new_screen = Some(UnitScreen::OnboardingRestore);
                    out.set_fast();
                }
                150..=300 => {
                    let e = P::generate_seed_entropy(h).to_vec();
                    new_screen = Some(UnitScreen::OnboardingBackup(e));
                    out.set_fast();
                }
                _ => {},
            },
            Screen::OnboardingRestore(ref mut a) => {
                let res = a.handle_event(point, fast_display)?;
                out = res.request;
                new_screen = res.state;
            },
            Screen::OnboardingBackup(ref mut a) => {
                let (res, _) = a.handle_tap_screen(point, ());
                out = res.request;
                new_screen = res.state;
            },
            Screen::ShowTransaction => match point.x {
                150..=300 => {
                    new_screen = Some(UnitScreen::ShowExtension);
                    out.set_fast();
                }
                _ => {},
            },
            Screen::ShowExtension => match point.x {
                0..=100 => {
                    new_screen = Some(UnitScreen::ShowTransaction);
                    out.set_fast();
                }
                150..=300 => {
                    new_screen = Some(UnitScreen::QRSignature);
                    out.set_fast();
                }
                _ => {},
            },
            _ => (),
        }
        self.switch_screen(new_screen, h);
        Ok(out)
    }
    pub fn handle_message(&mut self, message: String, h: &mut <P as Platform>::HAL) -> UpdateRequest {
        let mut out = UpdateRequest::new();
        let screen = Some(UnitScreen::ShowMessage(message));
        self.switch_screen(screen, h);
        out.set_fast();
        out
    }
    /// Handle NFC message reception.
    /// TODO this correctly
    /// currently it is a quick demo for expo
    pub fn handle_transaction(&mut self, h: &mut <P as Platform>::HAL) -> UpdateRequest
    {
        let mut out = UpdateRequest::new();
        // match self.screen {
            // Screen::OnboardingRestoreOrGenerate => {
        let screen = Some(UnitScreen::ShowTransaction);
        self.switch_screen(screen, h);
        out.set_fast();
        out
            // },
            // _ => {},
        // }
        // out
    }

    pub fn handle_address(&mut self, addr: [u8; 76]) -> UpdateRequest {
        let mut out = UpdateRequest::new();
        self.platform.set_address(addr);
        self.screen = Screen::QRAddress;
        out.set_slow();
        out
    }

    /// Display new screen state; should be called only when needed, is slow
    pub fn render<D>(
        &mut self,
        is_clear_update: bool,
        h: &mut <P as Platform>::HAL,
    ) -> Result<UpdateRequest, <<P as Platform>::Display as DrawTarget>::Error>
    {
        let display = self.platform.display();
        if is_clear_update {
            let clear = PrimitiveStyle::with_fill(BinaryColor::Off);
            display.bounding_box().into_styled(clear).draw(display)?;
        }
        let mut out = UpdateRequest::new();
        let mut new_screen = None;

        match self.screen {
            Screen::PinEntry(ref mut a, ref u) => {
                let (res, _) = a.draw_screen(display, P::rng(h))?;
                if self.unlocked {
                    out.set_fast();
                    new_screen = Some(u.clone());
                } else {
                    out = res.request;
                    new_screen = res.state;
                }
            },
            Screen::OnboardingRestoreOrGenerate => {
                restore_or_generate::draw(display)?;
            },
            Screen::OnboardingRestore(ref entry) => {
                entry.draw(display)?;
            },
            Screen::Locked => {
                let linestyle = PrimitiveStyle::with_stroke(BinaryColor::On, 5);
                Line::new(
                    Point::new(0, 0),
                    Point::new(SCREEN_SIZE_X as i32, SCREEN_SIZE_Y as i32),
                )
                .into_styled(linestyle)
                .draw(display)?;
                Line::new(
                    Point::new(SCREEN_SIZE_X as i32, 0),
                    Point::new(0, SCREEN_SIZE_Y as i32),
                )
                .into_styled(linestyle)
                .draw(display)?;
            },
            Screen::OnboardingBackup(ref mut a) => {
                let (res, entropy) = a.draw_screen(display, ())?;
                if let Some(e) = entropy {
                    self.platform.store_entropy(&e);
                }
                out = res.request;
                new_screen = res.state;
            },
            Screen::ShowMessage(ref m, ref next) => {
                message::draw(display, m)?;
                if next.is_some() {
                    out.set_fast();
                }
                new_screen = next.clone();
            },
            Screen::ShowTransaction => {
                self.platform.draw_transaction()?
            },
            Screen::ShowExtension => {
                self.platform.draw_extensions()?
            },
            Screen::QRSignature => {
                self.platform.draw_signature_qr(h)?
            },
            Screen::QRAddress => {
                self.platform.draw_address_qr()?
            },
            _ => {}
        }
        self.switch_screen(new_screen, h);
        Ok(out)
    }
}
