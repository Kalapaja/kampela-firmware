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



use core::{cell::RefCell, mem};


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

use crate::{display_def::*, widget::view::ViewScreen};

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
    pub state: Option<Screen>,
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
    screen: Screen,
    reason: Reason,
    protected_screen: Option<Screen>,
    pub platform: P,
}

pub enum Screen {
    PinOk,
    PinEntry,
    OnboardingRestoreOrGenerate,
    OnboardingRestore(SeedEntryState),
    OnboardingBackup,
    ShowTransaction,
    ShowExtension,
    QRSignature,
    QRAddress,
    Locked,
    End,
}

impl <P: Platform> UIState<P> {
    pub fn new(mut platform: P) -> Self {
        platform.read_entropy();
        let mut initial_screen: Screen;
        let mut protected_screen: Option<Screen> = None;
        if platform.entropy_display().0.is_empty() {
            initial_screen = Screen::OnboardingRestoreOrGenerate;
        } else {
            initial_screen = Screen::PinEntry;
            protected_screen = Some(Screen::QRAddress);
        }
        UIState {
            screen: initial_screen,
            reason: Reason::new(),
            protected_screen,
            platform,
        }
    }

    pub fn is_initial(&self) -> bool {
        if let Screen::OnboardingRestoreOrGenerate = self.screen {
            return true;
        }
        false
    }

    pub fn is_end(&self) -> bool {
        if let Screen::End = self.screen {
            return true;
        }
        false
    }

    pub fn protected_screen(&mut self) -> Option<Screen> {
        mem::take(&mut self.protected_screen)
    }

    pub fn display(&mut self) -> &mut <P as Platform>::Display {
        self.platform.display()
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
            Screen::PinEntry => {
                let res = self.platform.handle_pin_event(point, h);
                out = res.request;
                new_screen = res.state;
            },
            Screen::OnboardingRestoreOrGenerate => match point.x {
                0..=100 => {
                    new_screen = Some(Screen::OnboardingRestore(SeedEntryState::new()));
                    out.set_slow();
                }
                150..=300 => {
                    self.platform.generate_seed(h);
                    new_screen = Some(Screen::OnboardingBackup);
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
                new_screen = Some(Screen::QRAddress);
                out.set_slow();
            },
            Screen::ShowTransaction => match point.x {
                150..=300 => {
                    new_screen = Some(Screen::ShowExtension);
                    out.set_slow();
                }
                _ => {},
            },
            Screen::ShowExtension => match point.x {
                0..=100 => {
                    new_screen = Some(Screen::ShowTransaction);
                    out.set_slow();
                }
                150..=300 => {
                    new_screen = Some(Screen::QRSignature);
                    out.set_slow();
                }
                _ => {},
            },
            _ => (),
        }
        if let Some(a) = new_screen {
            self.screen = a;
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
            Screen::PinEntry => {
                let res: EventResult = self.platform.draw_pincode(&self.reason, h)?;
                if let Some(s) = res.state {
                    if matches!(s, Screen::PinOk) {
                        out.set_slow();
                        new_screen = self.protected_screen();
                    }
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

        if let Some(a) = new_screen {
            self.screen = a;
            self.reason.set_cause(Cause::NewScreen);
        } else {
            self.reason.inc_repeats();
        }
        Ok(out)
    }
}
