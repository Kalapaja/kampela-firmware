//! UI state unit; almost all inerfacing should be done through this "object"

#[cfg(not(feature="std"))]
mod stdwrap {
    pub use alloc::string::String;
    pub use alloc::borrow::ToOwned;
    pub use alloc::vec::Vec;
    pub use alloc::boxed::Box;
    pub use alloc::format;
}
#[cfg(feature="std")]
mod stdwrap {
    pub use std::string::String;
    pub use std::borrow::ToOwned;
    pub use std::vec::Vec;
    pub use std::boxed::Box;
    pub use std::format;
}

use stdwrap::*;

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    pixelcolor::BinaryColor,
    prelude::Primitive,
    primitives::{
        Line,
        PrimitiveStyle,
        Rectangle,
    },
    Drawable,
};

use patches::phrase::WordListElement;

use crate::{dialog::Dialog, display_def::*, pin::pin::Pincode, qr, transaction::{Transaction, TransactionPage}, widget::view::ViewScreen};

use crate::backup::Backup;

use crate::platform::Platform;

use crate::seed_entry::seed_entry::SeedEntry;

use crate::message;

use rand::Rng;

pub struct EventResult {
    pub request: Option<UpdateRequest>,
    pub state: Option<UnitScreen>,
}

#[derive(Clone)]
pub enum UpdateRequest {
    Hidden,
    Slow,
    Fast,
    UltraFast,
    Part(Rectangle),
}

pub trait UpdateRequestMutate {
    fn propagate(&mut self, new_request: Self);
}

impl UpdateRequestMutate for Option<UpdateRequest> {
    fn propagate(&mut self, new_request: Self) {
        if let Some(r) = new_request {
            self.replace(r);
        }
    }
}
/// State of UI
pub struct UIState<P, D> where
    P: Platform,
    D: DrawTarget<Color = BinaryColor>,
{
    screen: Screen<P::Rng>,
    pub platform: P,
    pub display: D,
    unlocked: bool,
}

pub enum UnitScreen {
    OnboardingRestoreOrGenerate,
    OnboardingRestore(Option<Vec<WordListElement>>),
    OnboardingBackup(Vec<u8>),
    ShowMessage(String),
    ShowDialog(
        &'static str,
        (&'static str, &'static str),
        (Box<dyn FnOnce() -> EventResult>, Box<dyn FnOnce() -> EventResult>),
        bool
    ),
    ShowTransaction(TransactionPage),
    QRSignature,
    QRAddress,
    Locked,
}

impl Default for UnitScreen {
    fn default() -> Self {UnitScreen::QRAddress}
}

/// keeps states of screens, initialization can take a lot of memory
pub enum Screen<R: Rng + ?Sized> {
    PinEntry(Pincode<R>, UnitScreen),
    OnboardingRestoreOrGenerate(Dialog),
    OnboardingRestore(SeedEntry),
    OnboardingBackup(Backup),
    ShowMessage(String, Option<UnitScreen>),
    ShowDialog(Dialog),
    ShowTransaction(Transaction),
    QRSignature,
    QRAddress,
    Locked,
}

impl<R: Rng + ?Sized> Screen<R> {
    pub fn get_unit(&self) -> Option<UnitScreen> {
        match self {
            Screen::OnboardingRestoreOrGenerate(_) => Some(UnitScreen::OnboardingRestoreOrGenerate),
            Screen::OnboardingRestore(s) => Some(UnitScreen::OnboardingRestore(Some(s.get_phrase().clone()))),
            Screen::OnboardingBackup(b) => Some(UnitScreen::OnboardingBackup(b.get_entropy())),
            Screen::ShowMessage(s, _) => Some(UnitScreen::ShowMessage(s.to_owned())),
            Screen::ShowTransaction(t) => Some(UnitScreen::ShowTransaction(t.get_page())),
            Screen::QRSignature => Some(UnitScreen::QRSignature),
            Screen::QRAddress => Some(UnitScreen::QRAddress),
            Screen::Locked => Some(UnitScreen::Locked),
            _ => None,
        }
    }
}
impl<R: Rng + ?Sized> Default for Screen<R> {
    fn default() -> Self {Screen::QRAddress}
}

impl <P: Platform, D: DrawTarget<Color = BinaryColor>> UIState<P, D> {
    pub fn new(mut platform: P, display: D, h: &mut <P as Platform>::HAL) -> Self {
        platform.read_entropy();
        let initial_screen: Option<UnitScreen>;
        let unlocked: bool;
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
            display,
            unlocked,
        };
        state.switch_screen(initial_screen, h);
        state
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
                    self.screen = Screen::OnboardingBackup(Backup::new(e, self.screen.get_unit().expect("Backup returns only to unit screens")));
                },
                UnitScreen::ShowMessage(m) => {
                    self.screen = Screen::ShowMessage(m, None);
                },
                UnitScreen::ShowDialog(message, options, routes, negative) => {
                    self.screen = Screen::ShowDialog(Dialog::new(message, options, routes, negative));
                },
                UnitScreen::OnboardingRestoreOrGenerate => {
                    let e = P::generate_seed_entropy(h).to_vec();
                    self.screen = Screen::OnboardingRestoreOrGenerate(Dialog::new(
                        "restore or generate?",
                        ("restore", "generate"),
                        (
                            Box::new(|| EventResult{request: Some(UpdateRequest::Fast), state: Some(UnitScreen::OnboardingRestore(None))}),
                            Box::new(|| EventResult{request: Some(UpdateRequest::Fast), state: Some(UnitScreen::OnboardingBackup(e))}),
                        ),
                        false,
                    ))
                },
                UnitScreen::OnboardingRestore(e) => {
                    self.screen = Screen::OnboardingRestore(SeedEntry::new(e));
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
                UnitScreen::ShowTransaction(p) => {
                    self.screen = Screen::ShowTransaction(Transaction::new(p));
                },
            }
        }
    }

    /// Read user touch event
    pub fn handle_tap(
        &mut self,
        point: Point,
        h: &mut <P as Platform>::HAL,
    ) -> Option<UpdateRequest>
    {
        let mut out = None;
        let mut new_screen = None;
        match self.screen {
            Screen::PinEntry(ref mut a, _) => {
                let (res, _) = a.handle_tap_screen(point, self.platform.pin());
                out = res.request;
                new_screen = res.state;
            },
            Screen::OnboardingRestore(ref mut a) => {
                let (res, _) = a.handle_tap_screen(point, ());
                out = res.request;
                new_screen = res.state;
            },
            Screen::OnboardingBackup(ref mut a) => {
                let (res, _) = a.handle_tap_screen(point, ());
                out = res.request;
                new_screen = res.state;
            },
            Screen::OnboardingRestoreOrGenerate(ref mut a) |
            Screen::ShowDialog(ref mut a) => {
                let (res, _) = a.handle_tap_screen(point, ());
                out = res.request;
                new_screen = res.state;
            },
            Screen::ShowTransaction(ref mut a) => {
                let (res, _) = a.handle_tap_screen(point, ());
                out = res.request;
                new_screen = res.state;
            },
            _ => (),
        }
        self.switch_screen(new_screen, h);
        out
    }
    pub fn handle_message(&mut self, message: String, h: &mut <P as Platform>::HAL) -> Option<UpdateRequest> {
        let screen = Some(UnitScreen::ShowMessage(message));
        self.switch_screen(screen, h);
        Some(UpdateRequest::UltraFast)
    }
    /// Handle NFC message reception.
    /// TODO this correctly
    /// currently it is a quick demo for expo
    pub fn handle_transaction(&mut self, h: &mut <P as Platform>::HAL) -> Option<UpdateRequest> {
        // match self.screen {
            // Screen::OnboardingRestoreOrGenerate => {
        let screen = Some(UnitScreen::ShowTransaction(TransactionPage::Call));
        self.switch_screen(screen, h);
        Some(UpdateRequest::UltraFast)
            // },
            // _ => {},
        // }
        // out
    }

    pub fn handle_address(&mut self, addr: [u8; 76]) -> Option<UpdateRequest> {
        self.platform.set_address(addr);
        self.screen = Screen::QRAddress;
        Some(UpdateRequest::Slow)
    }

    /// Display new screen state; should be called only when needed, is slow
    pub fn render(
        &mut self,
        is_clear_update: bool,
        h: &mut <P as Platform>::HAL,
    ) -> Result<Option<UpdateRequest>, <D as DrawTarget>::Error>
    {
        let display = &mut self.display;
        if is_clear_update {
            let clear = PrimitiveStyle::with_fill(BinaryColor::Off);
            display.bounding_box().into_styled(clear).draw(display)?;
        }
        let mut out = None;
        let mut new_screen = None;

        match self.screen {
            Screen::PinEntry(ref mut a, _) => {
                let (res, pinok) = a.draw_screen(display, P::rng(h))?;
                out = res.request;
                new_screen = res.state;
                if pinok {
                    self.unlocked = true;
                    out = Some(UpdateRequest::UltraFast);
                    new_screen = match core::mem::take(&mut self.screen) {
                        Screen::PinEntry(_, u) => Some(u),
                        _ => None
                    };
                }
            },
            Screen::OnboardingRestore(ref mut entry) => {
                let (res, _) = entry.draw_screen(display, ())?;
                out = res.request;
                new_screen = res.state;
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
                message::draw(display, m, true)?;
                if next.is_some() {
                    out = Some(UpdateRequest::UltraFast)
                }
                new_screen = match core::mem::replace(&mut self.screen, Screen::ShowMessage("".to_owned(), None)) {
                    Screen::ShowMessage(_, n) => n,
                    _ => None
                };
            },
            Screen::OnboardingRestoreOrGenerate(ref mut a) |
            Screen::ShowDialog(ref mut a) => {
                let (res, _) = a.draw_screen(display, ())?;
                out = res.request;
                new_screen = res.state;
            }
            Screen::ShowTransaction(ref mut a) => {
                let (res, _) = a.draw_screen(
                    display,
                    Box::new(|s| {
                        match s {
                            TransactionPage::Call => {
                                self.platform.call().expect("transaction should be stored to display")
                            },
                            TransactionPage::Extension => {
                                self.platform.extensions().expect("transaction should be stored to display")
                            },
                        }
                    })
                )?;
                out = res.request;
                new_screen = res.state;
            },
            Screen::QRSignature => {
                qr::draw(&self.platform.signature(h), display)?
            },
            Screen::QRAddress => {
                let line1 = format!("substrate:0x{}", hex::encode(self.platform.public().expect("no entropy stored, no address could be shown")));

                qr::draw(&line1.as_bytes(), display)?
            },
        }
        self.switch_screen(new_screen, h);
        Ok(out)
    }
}


