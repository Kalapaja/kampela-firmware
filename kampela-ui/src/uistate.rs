//! UI state unit; almost all inerfacing should be done through this "object"

#[cfg(not(feature="std"))]
mod stdwrap {
    pub use alloc::string::String;
    pub use alloc::borrow::ToOwned;
    pub use alloc::boxed::Box;
    pub use alloc::format;
    pub use alloc::vec::Vec;
}
#[cfg(feature="std")]
mod stdwrap {
    pub use std::string::String;
    pub use std::borrow::ToOwned;
    pub use std::boxed::Box;
    pub use std::format;
    pub use std::vec::Vec;
}

use minicbor::data::Tag;
use mnemonic_external::WordSet;
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

use substrate_crypto_light::ecdsa::PairWithChainCode;
use ur::ur;

use crate::{dialog::{Dialog, DialogUnitScreenArgs}, display_def::*, pin::pin::Pincode, qr, transaction::{Transaction, TransactionPage}, widget::view::ViewScreen};

use crate::backup::Backup;

use crate::platform::Platform;

use crate::seed_entry::seed_entry::SeedEntry;

use crate::message;

pub struct EventResult{
    pub request: Option<UpdateRequest>,
    pub state: Option<UnitScreen>,
}

#[derive(Clone)]
pub enum UpdateRequest {
    Invocate,
    Slow,
    Fast,
    UltraFast,
    UltraFastSelective,
    Part(Rectangle),
}

pub trait UpdateRequestMutate {
    fn propagate(&mut self, new_request: Self);

    fn try_add(&mut self, new_request: Self);
}

impl UpdateRequestMutate for Option<UpdateRequest> {
    fn propagate(&mut self, new_request: Self) {
        if let Some(r) = new_request {
            self.replace(r);
        }
    }
    fn try_add(&mut self, new_request: Self) {
        if self.is_some() {
            return
        }
        self.propagate(new_request);
    }
}

#[derive(Clone, Copy)]
pub enum Event {
    Tap(Point),
    Invocation,
}
/// State of UI
pub struct UIState<P> where
    P: Platform
{
    screen: Screen<P>,
    pub platform: P,
    unlocked: bool,
}

pub enum UnitScreen {
    OnboardingRestoreOrGenerate,
    OnboardingRestore(Option<WordSet>),
    OnboardingBackup(Option<Vec<u8>>),
    ShowMessage(String, Option<Box<dyn FnOnce() -> EventResult>>),
    ShowDialog(DialogUnitScreenArgs),
    ShowTransaction(TransactionPage),
    QRSignature,
    QRAddress,
    Locked,
}

impl Default for UnitScreen {
    fn default() -> Self {UnitScreen::QRAddress}
}

/// keeps states of screens, initialization can take a lot of memory
pub enum Screen<P: Platform> {
    PinEntry(Pincode<P>, UnitScreen),
    OnboardingRestore(SeedEntry<P>),
    OnboardingBackup(Backup<P>),
    ShowMessage(String, Option<Box<dyn FnOnce() -> EventResult>>),
    ShowDialog(Dialog),
    ShowTransaction(Transaction),
    QRSignature,
    QRAddress,
    Locked,
}

pub enum ScreenError {
    NoUnitForCurrentScreen
}

impl<P: Platform> Screen<P> {
    pub fn get_unit(self) -> Option<UnitScreen> {
        match self {
            Screen::OnboardingRestore(s) => Some(UnitScreen::OnboardingRestore(Some(s.get_buffer()))),
            Screen::OnboardingBackup(b) => Some(UnitScreen::OnboardingBackup(Some(b.get_entropy().unwrap()))),
            Screen::ShowMessage(s, r) => Some(UnitScreen::ShowMessage(s.to_owned(), r)),
            Screen::ShowDialog(d) => d.get_unit_screen_args().map(|args| UnitScreen::ShowDialog(args)),
            Screen::ShowTransaction(t) => Some(UnitScreen::ShowTransaction(t.get_page())),
            Screen::QRSignature => Some(UnitScreen::QRSignature),
            Screen::QRAddress => Some(UnitScreen::QRAddress),
            Screen::Locked => Some(UnitScreen::Locked),
            _ => None,
        }
    }
    pub fn replace_getting_unit(&mut self, next: Box<dyn FnOnce(UnitScreen) -> Self>) -> Result<(), ScreenError> {
        let owned = core::mem::take(self);
        let unit = owned.get_unit().ok_or(ScreenError::NoUnitForCurrentScreen)?;
        _ = core::mem::replace(self, next(unit));
        Ok(())
    }
}
impl<P: Platform> Default for Screen<P> {
    fn default() -> Self {Screen::QRAddress}
}

impl <P: Platform> UIState<P> {
    pub fn new(mut platform: P, h: &mut <P as Platform>::HAL) -> Self
        where <P as Platform>::AsWordList: Sized {
        let (initial_screen, unlocked) = if platform.read_entropy() {(
            UnitScreen::ShowMessage(
                "Generating Address".to_owned(),
                Some(Box::new(|| EventResult{
                    request: Some(UpdateRequest::UltraFast),
                    state: Some(UnitScreen::QRAddress)
                }))
            ),
            false
        )} else {(
            UnitScreen::OnboardingRestoreOrGenerate,
            true
        )};
        let mut state = UIState {
            screen: Screen::default(),
            platform,
            unlocked,
        };
        state.switch_screen(Some(initial_screen), h);
        state
    }

    fn switch_screen(&mut self, s: Option<UnitScreen>, h: &mut <P as Platform>::HAL )
        where <P as Platform>::AsWordList: Sized {
        if let Some(s) = s {
            match s {
                UnitScreen::QRAddress => {
                    self.screen = Screen::QRAddress;
                },
                UnitScreen::Locked => {
                    self.screen = Screen::Locked;
                },
                UnitScreen::OnboardingBackup(e) => {
                    let entropy = match e {
                        Some(e) => e,
                        None => P::generate_seed_entropy(h).to_vec(),
                    };
                    self.screen.replace_getting_unit(Box::new(|unit| Screen::OnboardingBackup(Backup::new(entropy, unit))));
                },
                UnitScreen::ShowMessage(m, route) => {
                    self.screen = Screen::ShowMessage(m, route);
                },
                UnitScreen::ShowDialog(args) => {
                    self.screen = Screen::ShowDialog(Dialog::new(args));
                },
                UnitScreen::OnboardingRestoreOrGenerate => {
                    self.screen = Screen::ShowDialog(Dialog::new((
                        "restore or generate?",
                        ("restore", "generate"),
                        (
                            Box::new(|| EventResult{request: Some(UpdateRequest::Fast), state: Some(UnitScreen::OnboardingRestore(None))}),
                            Box::new(|| EventResult{request: Some(UpdateRequest::Fast), state: Some(UnitScreen::OnboardingBackup(None))}),
                        ),
                        false,
                    )))
                },
                UnitScreen::OnboardingRestore(p) => {
                    self.screen = Screen::OnboardingRestore(SeedEntry::new(p));
                },
                UnitScreen::QRSignature => {
                    if self.unlocked {
                        if matches!(self.screen, Screen::ShowMessage(_, _)) {
                            self.screen = Screen::QRSignature;
                        } else {
                            self.screen = Screen::ShowMessage("Signing...".to_owned(), Some(Box::new(|| EventResult{request: Some(UpdateRequest::Fast), state: Some(UnitScreen::QRSignature)})));
                        }
                    } else {
                        self.screen = Screen::PinEntry(Pincode::new(h), UnitScreen::QRSignature);
                    }
                },
                UnitScreen::ShowTransaction(p) => {
                    self.screen = Screen::ShowTransaction(Transaction::new(p));
                },
            }
        }
    }

    /// Read user touch event
    pub fn handle_event(
        &mut self,
        event: Event,
        h: &mut <P as Platform>::HAL,
    ) -> Option<UpdateRequest>
    where <P as Platform>::AsWordList: Sized
    {
        let mut out = None;
        let mut new_screen = None;
        match self.screen {
            Screen::PinEntry(ref mut a, _) => {
                let (res, _) = a.handle_event_screen(event, self.platform.pin());
                out = res.request;
                new_screen = res.state;
            },
            Screen::OnboardingRestore(ref mut a) => {
                let (res, _) = a.handle_event_screen(event, ());
                out = res.request;
                new_screen = res.state;
            },
            Screen::OnboardingBackup(ref mut a) => {
                let (res, entropy) = a.handle_event_screen(event, ());
                if let Some(e) = entropy {
                    self.platform.store_entropy(&e);
                }
                out = res.request;
                new_screen = res.state;
            },
            Screen::ShowDialog(ref mut a) => {
                let (res, _) = a.handle_event_screen(event, ());
                out = res.request;
                new_screen = res.state;
            },
            Screen::ShowTransaction(ref mut a) => {
                let (res, _) = a.handle_event_screen(event, ());
                out = res.request;
                new_screen = res.state;
            },
            _ => (),
        }
        self.switch_screen(new_screen, h);
        out
    }
    pub fn handle_message(&mut self, message: String, h: &mut <P as Platform>::HAL) -> Option<UpdateRequest>
        where <P as Platform>::AsWordList: Sized {
        let screen = Some(UnitScreen::ShowMessage(message, None));
        self.switch_screen(screen, h);
        Some(UpdateRequest::Fast)
    }
    /// Handle NFC message reception.
    /// TODO this correctly
    /// currently it is a quick demo for expo
    pub fn handle_transaction(&mut self, h: &mut <P as Platform>::HAL) -> Option<UpdateRequest>
        where <P as Platform>::AsWordList: Sized {
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

    pub fn handle_eth_sign_request(&mut self, h: &mut <P as Platform>::HAL) -> Option<UpdateRequest>
        where <P as Platform>::AsWordList: Sized {
        // match self.screen {
            // Screen::OnboardingRestoreOrGenerate => {
        let screen = Some(UnitScreen::ShowTransaction(TransactionPage::Eth));
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
    pub fn render <D: DrawTarget<Color = BinaryColor>> (
        &mut self,
        display: &mut D,
        h: &mut <P as Platform>::HAL,
    ) -> Result<Option<UpdateRequest>, <D as DrawTarget>::Error>
    where <P as Platform>::AsWordList: Sized
    {
        let mut out = None;
        let mut new_screen = None;

        match self.screen {
            Screen::PinEntry(ref mut a, _) => {
                let (res, pinok) = a.draw_screen(display, h)?;
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
                display.clear(BinaryColor::Off)?;
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
                let (res, _) = a.draw_screen(display, ())?;
                out = res.request;
                new_screen = res.state;
            },
            Screen::ShowMessage(ref m, ref mut next) => {
                message::draw(display, m, true)?;
                if let Some(n) = next.take() {
                    let res = n();
                    out = res.request;
                    new_screen = res.state
                }
            },
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
                            TransactionPage::Eth => {
                                self.platform.ethereum().expect("ethereum sign request should be stored to display")
                            }
                        }
                    })
                )?;
                out = res.request;
                new_screen = res.state;
            },
            Screen::QRSignature => {
                let code = ur::encode(&self.platform.eth_signature(), "eth-signature");
                qr::draw(&code.as_bytes(), display)?
            },
            Screen::QRAddress => {
                let data = hdkey(&self.platform.pair().unwrap()).unwrap();
                let code = ur::encode(&data, "crypto-hdkey");
                qr::draw(&code.as_bytes(), display)?
            },
        }
        self.switch_screen(new_screen, h);
        Ok(out)
    }
}

fn hdkey(pair_with_chain_code: &PairWithChainCode) -> Result<Vec<u8>, minicbor::encode::Error<core::convert::Infallible>> {
    let mut e = minicbor::Encoder::new(Vec::new());
    let public = &pair_with_chain_code.public().unwrap().0;
    let fingerprint = pair_with_chain_code.fingerprint().unwrap();
    
    e.tag(Tag::new(303))?.map(4)?
        // 3 key-data
        .u8(3)?.bytes(public)?
        // 4 chain-code
        .u8(4)?.bytes(&pair_with_chain_code.chain_code())?
        // 5 coin-info
        .u8(5)?.tag(Tag::new(305))?.map(1)?
            // type
            .u8(1)?.u8(0x3c)?
        // origin
        .u8(6)?.tag(Tag::new(304))?.map(2)?
            // components
            .u8(1)?.array(8)?
                .u8(44)?
                .bool(true)?
                .u8(60)?
                .bool(true)?
                .u8(0)?
                .bool(true)?
                .u8(0)?
                .bool(false)?
            // source-fingerprint
            .u8(2)?.u32(fingerprint)?;
        // parent-fingerprint
        //.u8(8)?.u32(fingerprint)?; // unnecessary
 
    Ok(e.into_writer())
}