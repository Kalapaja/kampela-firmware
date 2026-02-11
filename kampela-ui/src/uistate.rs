//! UI state unit; almost all interfacing should be done through this "object"
//!
//! Simplified for Ethereum-only functionality with 3 main screens + error dialog.

#[cfg(not(feature = "std"))]
mod stdwrap {
    pub use alloc::format;
    pub use alloc::string::String;
}
#[cfg(feature = "std")]
mod stdwrap {
    pub use std::format;
    pub use std::string::String;
}

use stdwrap::*;

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    pixelcolor::BinaryColor,
    prelude::Primitive,
    primitives::{PrimitiveStyle, Rectangle},
    Drawable,
};

use crate::eth_transaction_viewer::EthTransactionViewer;
use crate::{display_def::*, pin::pin::Pincode, qr, widget::view::ViewScreen};

use crate::error::KampelaError;
use crate::platform::Platform;

use alloy_primitives::Address;

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
pub struct UIState<P, D>
where
    P: Platform,
    D: DrawTarget<Color = BinaryColor>,
{
    screen: Screen<P>,
    pub platform: P,
    pub display: D,
    unlocked: bool,
}

/// Unit screen variants - simplified for Ethereum-only
pub enum UnitScreen {
    Welcome,
    QRAddress,
    EthTransaction,
    QRSignature,
    TestMessage(String),
    ErrorDialog(String),
}

impl Default for UnitScreen {
    fn default() -> Self {
        UnitScreen::Welcome
    }
}

/// Keeps states of screens, initialization can take a lot of memory
pub enum Screen<P: Platform> {
    PinEntry(Pincode<P>, UnitScreen),
    Welcome,
    QRAddress,
    EthTransaction(EthTransactionViewer),
    QRSignature,
    TestMessage(String),
    ErrorDialog(String),
}

impl<P: Platform> Screen<P> {
    pub fn get_unit(&self) -> Option<UnitScreen> {
        match self {
            Screen::Welcome => Some(UnitScreen::Welcome),
            Screen::QRAddress => Some(UnitScreen::QRAddress),
            Screen::EthTransaction(_) => Some(UnitScreen::EthTransaction),
            Screen::QRSignature => Some(UnitScreen::QRSignature),
            Screen::TestMessage(msg) => Some(UnitScreen::TestMessage(msg.clone())),
            Screen::ErrorDialog(msg) => Some(UnitScreen::ErrorDialog(msg.clone())),
            _ => None,
        }
    }
}

impl<P: Platform> Default for Screen<P> {
    fn default() -> Self {
        Screen::Welcome
    }
}

impl<P: Platform, D: DrawTarget<Color = BinaryColor>> UIState<P, D> {
    /// Initialize the UI state.
    ///
    /// On first boot: generates entropy, stores to flash, shows Welcome screen.
    /// On subsequent boots: reads entropy from flash, shows Welcome screen.
    /// Always requires PIN (unlocked = false).
    pub fn new(platform: P, display: D, h: &mut <P as Platform>::HAL) -> Self {
        let mut state = UIState {
            screen: Screen::Welcome,
            platform,
            display,
            unlocked: false, // Always require PIN
        };

        // Switch to welcome screen - simple screen that should always work
        state.switch_screen(Some(UnitScreen::Welcome), h);
        state
    }

    fn ensure_key(&mut self, h: &mut <P as Platform>::HAL) -> Result<(), KampelaError> {
        let has_entropy = self.platform.read_entropy().is_ok();

        if !has_entropy {
            let entropy = P::generate_entropy(h);
            self.platform.store_entropy(&entropy)?;
        }

        self.platform.eth_address()?;

        Ok(())
    }

    fn switch_screen(&mut self, s: Option<UnitScreen>, h: &mut <P as Platform>::HAL) {
        if let Some(s) = s {
            match s {
                UnitScreen::Welcome => {
                    self.screen = Screen::Welcome;
                }
                UnitScreen::QRAddress => {
                    self.screen = Screen::QRAddress;
                }
                UnitScreen::EthTransaction => {
                    // Get transaction display text from platform
                    match self.platform.eth_transaction_display() {
                        Ok(display_text) => {
                            let viewer = EthTransactionViewer::new(display_text);
                            self.screen = Screen::EthTransaction(viewer);
                        }
                        Err(e) => {
                            // If we can't get transaction display, show error
                            let error_msg = format!("{}", e);
                            self.screen = Screen::ErrorDialog(error_msg);
                        }
                    }
                }
                UnitScreen::QRSignature => {
                    if self.unlocked {
                        self.screen = Screen::QRSignature;
                    } else {
                        self.screen = Screen::PinEntry(Pincode::new(h), UnitScreen::QRSignature);
                    }
                }
                UnitScreen::TestMessage(msg) => {
                    self.screen = Screen::TestMessage(msg);
                }
                UnitScreen::ErrorDialog(msg) => {
                    self.screen = Screen::ErrorDialog(msg);
                }
            }
        }
    }

    /// Read user touch event
    pub fn handle_tap(
        &mut self,
        point: Point,
        h: &mut <P as Platform>::HAL,
    ) -> Option<UpdateRequest> {
        let mut out = None;
        let mut new_screen = None;

        match self.screen {
            Screen::PinEntry(ref mut pincode, _) => {
                let (res, _) = pincode.handle_tap_screen(point, self.platform.pin());
                out = res.request;
                new_screen = res.state;
            }
            Screen::Welcome => {
                // Right button = Continue (go to address)
                if point.y > SCREEN_SIZE_Y as i32 - 40 {
                    if point.x >= SCREEN_SIZE_X as i32 / 2 {
                        if let Err(e) = self.ensure_key(h) {
                            new_screen = Some(UnitScreen::ErrorDialog(format!("Key generation failed: {}", e)));
                        } else {
                            new_screen = Some(UnitScreen::QRAddress);
                        }
                        out = Some(UpdateRequest::Fast);
                    }
                }
            }
            Screen::EthTransaction(ref mut viewer) => {

                // Check if tap is on navigation bar (bottom 40 pixels)
                if point.y > SCREEN_SIZE_Y as i32 - 40 {
                    if point.x < SCREEN_SIZE_X as i32 / 2 {
                        // Left button - reject (go back to address)
                        new_screen = Some(UnitScreen::QRAddress);
                        out = Some(UpdateRequest::Fast);
                    } else {
                        // Right button - approve and sign
                        new_screen = Some(UnitScreen::QRSignature);
                        out = Some(UpdateRequest::Fast);
                    }
                } else {
                    // Tap in content area - handle scrolling
                    let (res, _) = viewer.handle_tap_screen(point, ());
                    out = res.request;
                }
            }
            Screen::QRSignature => {
                // Left button = back to address
                if point.y > SCREEN_SIZE_Y as i32 - 40 {
                    if point.x < SCREEN_SIZE_X as i32 / 2 {
                        new_screen = Some(UnitScreen::QRAddress);
                        out = Some(UpdateRequest::Fast);
                    }
                }
            }
            Screen::TestMessage(_) => {
                // Right button = dismiss (go back to address)
                if point.y > SCREEN_SIZE_Y as i32 - 40 {
                    if point.x >= SCREEN_SIZE_X as i32 / 2 {
                        new_screen = Some(UnitScreen::QRAddress);
                        out = Some(UpdateRequest::Fast);
                    }
                }
            }
            Screen::ErrorDialog(_) => {
                // Right button = dismiss (go back to welcome)
                if point.y > SCREEN_SIZE_Y as i32 - 40 {
                    if point.x >= SCREEN_SIZE_X as i32 / 2 {
                        new_screen = Some(UnitScreen::Welcome);
                        out = Some(UpdateRequest::Fast);
                    }
                }
            }
            _ => (),
        }

        self.switch_screen(new_screen, h);
        out
    }

    /// Handle NFC transaction reception.
    /// Shows the transaction screen for user approval.
    pub fn handle_transaction(&mut self, h: &mut <P as Platform>::HAL) -> Option<UpdateRequest> {
        let screen = Some(UnitScreen::EthTransaction);
        self.switch_screen(screen, h);
        Some(UpdateRequest::UltraFast)
    }

    /// Handle Ethereum address setting (from external source).
    pub fn handle_address(&mut self, addr: Address) -> Option<UpdateRequest> {
        self.platform.eth_set_address(addr);
        self.screen = Screen::QRAddress;
        Some(UpdateRequest::Slow)
    }

    /// Handle error by showing error dialog.
    pub fn handle_error(
        &mut self,
        error: KampelaError,
        h: &mut <P as Platform>::HAL,
    ) -> Option<UpdateRequest> {
        let error_msg = format!("{}", error);
        self.switch_screen(Some(UnitScreen::ErrorDialog(error_msg)), h);
        Some(UpdateRequest::Fast)
    }

    /// Handle test message display.
    pub fn handle_test_message(
        &mut self,
        message: String,
        h: &mut <P as Platform>::HAL,
    ) -> Option<UpdateRequest> {
        self.switch_screen(Some(UnitScreen::TestMessage(message)), h);
        Some(UpdateRequest::Fast)
    }

    /// Display new screen state; should be called only when needed, is slow
    pub fn render(
        &mut self,
        is_clear_update: bool,
        h: &mut <P as Platform>::HAL,
    ) -> Result<Option<UpdateRequest>, <D as DrawTarget>::Error> {
        {
            let display = &mut self.display;
            if is_clear_update {
                let clear = PrimitiveStyle::with_fill(BinaryColor::Off);
                display.bounding_box().into_styled(clear).draw(display)?;
            }
        }

        let mut out = None;
        let mut new_screen = None;

        match &mut self.screen {
            Screen::PinEntry(ref mut pincode, _) => {
                let display = &mut self.display;
                let (res, pinok) = pincode.draw_screen(display, h)?;
                out = res.request;
                new_screen = res.state;
                if pinok {
                    self.unlocked = true;
                    out = Some(UpdateRequest::UltraFast);
                    new_screen = match core::mem::take(&mut self.screen) {
                        Screen::PinEntry(_, u) => Some(u),
                        _ => None,
                    };
                }
            }
            Screen::Welcome => {
                let display = &mut self.display;
                // Draw simple welcome screen first so boot always shows something
                crate::welcome_screen::draw(display)?;
            }
            Screen::QRAddress => {
                let display = &mut self.display;
                // Get address and display as QR code
                match self.platform.eth_address() {
                    Ok(address) => {
                        let address_hex = hex::encode(&address[..]);
                        let address_string = format!("ethereum:0x{}", address_hex);
                        qr::draw(address_string.as_bytes(), display)?;
                    }
                    Err(e) => {
                        // Error getting address - show error
                        new_screen = Some(UnitScreen::ErrorDialog(format!("{}", e)));
                        out = Some(UpdateRequest::Fast);
                    }
                }
            }
            Screen::EthTransaction(ref mut viewer) => {
                let display = &mut self.display;
                // Render the scrollable transaction viewer
                let (res, _) = viewer.draw_screen(display, ())?;
                out = res.request;
            }
            Screen::QRSignature => {
                let display = &mut self.display;
                // Sign transaction and display raw signed tx as QR code
                match self.platform.eth_sign_transaction() {
                    Ok(signed_tx) => {
                        // Display raw signed transaction as QR code
                        let signed_hex = hex::encode(&signed_tx);
                        qr::draw(signed_hex.as_bytes(), display)?;
                    }
                    Err(e) => {
                        // Error signing - show error
                        new_screen = Some(UnitScreen::ErrorDialog(format!("{}", e)));
                        out = Some(UpdateRequest::Fast);
                    }
                }
            }
            Screen::TestMessage(ref message) => {
                let display = &mut self.display;
                crate::test_message_screen::draw(display, message, false)?;
            }
            Screen::ErrorDialog(ref error_msg) => {
                let display = &mut self.display;
                crate::error_dialog::draw(display, error_msg, false)?;
            }
        }

        self.switch_screen(new_screen, h);
        Ok(out)
    }
}
