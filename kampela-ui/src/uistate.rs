//! UI state unit; almost all inerfacing should be done through this "object"

#[cfg(not(feature="std"))]
mod stdwrap {
    pub use alloc::string::String;
    pub use alloc::vec::Vec;
    pub use alloc::rc::Rc;
}


#[cfg(feature="std")]
mod stdwrap {
    pub use std::string::String;
    pub use std::vec::Vec;
    pub use std::rc::Rc;
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

use crate::platform::{NfcTransaction, Platform};

use crate::seed_entry::SeedEntryState;

use crate::restore_or_generate;

use rand::{CryptoRng, Rng};

use schnorrkel::{
    context::attach_rng,
    derive::{ChainCode, Derivation},
    keys::Keypair,
    signing_context,
    ExpansionMode,
    MiniSecretKey,
};

const SIGNING_CTX: &[u8] = b"substrate";

pub struct EventResult {
    pub request: UpdateRequest,
    pub state: Option<UnitScreen>,
}

pub struct UpdateRequest {
    fast: bool,
    slow: bool,
    part: Option<Rectangle>,
}

impl UpdateRequest {
    pub fn new() -> Self {
        UpdateRequest {
            fast: false,
            slow: false,
            part: None,
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

    pub fn set_both(&mut self) {
        self.set_slow();
        self.set_fast();
    }

    pub fn propagate(&mut self, mut new: UpdateRequest) {
        if new.read_fast() { self.set_fast() };
        if new.read_slow() { self.set_slow() };
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
}

impl Default for UpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone)]
pub enum Cause {
    NewScreen,
    Tap,
}
pub struct Reason {
    cause: Cause,
    repeats: usize,
}

impl Reason {
    fn new() -> Self {
        Reason{
            cause: Cause::NewScreen,
            repeats: 0,
        }
    }
    fn set_cause(&mut self, cause: Cause) {
        self.cause = cause;
        self.repeats = 0;
    }
    fn inc_repeats(&mut self) {
        self.repeats = self.repeats + 1;
    }
    pub fn cause(&self) -> Cause {
        self.cause
    }
    pub fn repeats(&self) -> usize {
        self.repeats
    }
}

/// State of UI
pub struct UIState<P> where
    P: Platform,
{
    screen: Screen<P::Rng>,
    reason: Reason,
    pub platform: P,
    unlocked: bool,
}
// Some macro can be used to generate this enum from enum Screen
#[derive(Copy, Clone)]
pub enum UnitScreen {
    OnboardingRestoreOrGenerate,
    OnboardingRestore,
    OnboardingBackup,
    ShowTransaction,
    ShowExtension,
    QRSignature,
    QRAddress,
    Locked,
    End,
}

/// keeps states of screens, initialization can take a lot of memory
pub enum Screen<R: Rng + ?Sized> {
    PinEntry((Pincode<R>, UnitScreen)),
    OnboardingRestoreOrGenerate,
    OnboardingRestore(SeedEntryState),
    OnboardingBackup,
    ShowTransaction,
    ShowExtension,
    QRSignature,
    QRAddress,
    Locked,
}

impl <P: Platform> UIState<P> {
    pub fn new(mut platform: P, h: &mut <P as Platform>::HAL) -> Self {
        platform.read_entropy();
        let mut initial_screen: Screen<P::Rng>;
        let mut unlocked: bool;
        if platform.entropy_display().0.is_empty() {
            initial_screen = Screen::OnboardingRestoreOrGenerate;
            unlocked = true;
        } else {
            initial_screen = Screen::PinEntry((Pincode::new(P::rng(h)), UnitScreen::QRAddress));
            unlocked = false;
        }
        UIState {
            screen: initial_screen,
            reason: Reason::new(),
            platform,
            unlocked,
        }
    }

    pub fn display(&mut self) -> &mut <P as Platform>::Display {
        self.platform.display()
    }

    fn switch_screen(&mut self, s: UnitScreen, h: &mut <P as Platform>::HAL) {
        match s {
            UnitScreen::QRAddress => {
                if self.unlocked {
                    self.screen = Screen::QRAddress;
                } else {
                    self.screen = Screen::PinEntry((Pincode::<P::Rng>::new(&mut P::rng(h)), UnitScreen::QRAddress));
                }
            },
            UnitScreen::Locked => {
                self.screen = Screen::Locked;
            },
            UnitScreen::OnboardingBackup => {
                self.screen = Screen::OnboardingBackup;
            },
            UnitScreen::OnboardingRestore => {
                self.screen = Screen::OnboardingRestore(SeedEntryState::new());
            },
            UnitScreen::OnboardingRestoreOrGenerate => {
                self.screen = Screen::OnboardingRestoreOrGenerate;
            },
            UnitScreen::QRSignature => {//should it be protected?
                self.screen = Screen::QRSignature
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
            Screen::PinEntry((ref mut a, u)) => {
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
                    out.set_slow();
                }
                150..=300 => {
                    self.platform.generate_seed(h);
                    new_screen = Some(UnitScreen::OnboardingBackup);
                    out.set_slow();
                }
                _ => {},
            },
            Screen::OnboardingRestore(ref mut a) => {
                let mut seed = None;
                let res = a.handle_event(point, &mut seed, fast_display)?;
                if let Some(b) = seed {
                    self.platform.set_entropy(&b);
                }
                out = res.request;
                new_screen = res.state;
            },
            Screen::OnboardingBackup => {
                self.platform.store_entropy();
                new_screen = Some(UnitScreen::QRAddress);
                out.set_slow();
            },
            Screen::ShowTransaction => match point.x {
                150..=300 => {
                    new_screen = Some(UnitScreen::ShowExtension);
                    out.set_slow();
                }
                _ => {},
            },
            Screen::ShowExtension => match point.x {
                0..=100 => {
                    new_screen = Some(UnitScreen::ShowTransaction);
                    out.set_slow();
                }
                150..=300 => {
                    new_screen = Some(UnitScreen::QRSignature);
                    out.set_slow();
                }
                _ => {},
            },
            _ => (),
        }
        if let Some(s) = new_screen {
            self.switch_screen(s, h);
            self.reason.set_cause(Cause::NewScreen);
            //out.set_slow(); TODO: there seem to be no reason new state would use fast update
        } else {
            self.reason.set_cause(Cause::Tap);
        }
        Ok(out)
    }

    /// Handle NFC message reception.
    /// TODO this correctly
    /// currently it is a quick demo for expo
    pub fn handle_transaction<R: Rng + ?Sized + CryptoRng>(&mut self, rng: &mut R, transaction: NfcTransaction) -> UpdateRequest
    {
        let mut out = UpdateRequest::new();
        let carded = transaction.decoded_transaction.card(&transaction.specs, &transaction.spec_name);
        let call = carded.call.into_iter().map(|card| card.show()).collect::<Vec<String>>().join("\n");
        let extensions = carded.extensions.into_iter().map(|card| card.show()).collect::<Vec<String>>().join("\n");

        let context = signing_context(SIGNING_CTX);
        let signature = self.platform.pair().unwrap().sign(attach_rng(context.bytes(&transaction.data_to_sign), rng));
        let mut signature_with_id: [u8; 65] = [1; 65];
        signature_with_id[1..].copy_from_slice(&signature.to_bytes());

        self.platform.set_transaction(call, extensions, hex::encode(signature_with_id).into_bytes().try_into().expect("static length"));

        // match self.screen {
            // Screen::OnboardingRestoreOrGenerate => {
        self.screen = Screen::ShowTransaction;
        out.set_slow();
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
            Screen::PinEntry((ref mut a, u)) => {
                let (res, _) = a.draw_screen(display, &self.reason, P::rng(h))?;
                if self.unlocked {
                    out.set_slow();
                    new_screen = Some(u);
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
            Screen::OnboardingBackup => {
                self.platform.draw_backup()?;
            },
            Screen::ShowTransaction => {
                self.platform.draw_transaction()?
            },
            Screen::ShowExtension => {
                self.platform.draw_extensions()?
            },
            Screen::QRSignature => {
                self.platform.draw_signature_qr()?
            },
            Screen::QRAddress => {
                self.platform.draw_address_qr()?
            },
            _ => {}
        }

        if let Some(s) = new_screen {
            self.switch_screen(s, h);
            self.reason.set_cause(Cause::NewScreen);
        } else {
            self.reason.inc_repeats();
        }
        Ok(out)
    }
}
