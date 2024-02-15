#[cfg(not(feature="std"))]
use alloc::{vec::Vec, format};
#[cfg(feature="std")]
use std::{vec::Vec, format};

use embedded_graphics::{
    mono_font::{
        ascii::{FONT_10X20},
        MonoTextStyle,
    },
    primitives::Rectangle,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    geometry::{Size},
    Drawable,
    pixelcolor::PixelColor,
};

use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use rand::Rng;

use crate::{display_def::*, uistate::{UnitScreen, UpdateRequest}, widget::view::ViewScreen};
use crate::uistate::{EventResult, Reason, Cause};
use crate::widget::view::{View};
use crate::platform::PinCode;

use crate::pin::{
    pinpad::Pinpad,
    pindots::Pindots,
};

pub const PIN_LEN: usize = 4;





#[derive(Debug)]
pub struct Pincode<R> where
    R: Rng + ?Sized
{
    pinpad: Pinpad<R>,
    pindots: Pindots,
    entered_nums: Vec<u8>,
    pinok: bool,
}

impl<R> Pincode<R> where
    R: Rng + ?Sized
{
    pub fn new(rng: &mut R) -> Self {

        Self {
            pinpad: Pinpad::new(
                SCREEN_ZERO,
                rng
            ),
            pindots: Pindots::new(
                SCREEN_ZERO,
            ),
            entered_nums: Vec::new(),
            pinok: false,
        }
    }
    fn check_pin(&mut self, pin: &PinCode) -> bool {
        let r: bool;
        // TODO: proper attempt counter and pin check
        if self.entered_nums == pin {
            r = true
        } else {
            r = false
        }
        self.entered_nums = Vec::new();
        r
    }
    fn push_entered(&mut self, num: u8) {
        if self.entered_nums.len() < PIN_LEN {
            self.entered_nums.push(num);
        }
    }
}

impl<R> ViewScreen for Pincode<R> where
    R: Rng + ?Sized
{
    type DrawInput<'a> = &'a mut R where Self: 'a;
    type DrawOutput = ();
    type TapInput<'a> = &'a PinCode;
    type TapOutput = bool;
    fn draw_screen<'a, D>(&mut self, target: &mut D, reason: &Reason, rng: Self::DrawInput<'a>) -> Result<(EventResult, ()), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let mut request = UpdateRequest::new();
        let state = None;

        self.pindots.draw(target, reason, self.entered_nums.len())?;
        let t = self.pinpad.draw(target, reason, rng)?;

        if t {
            request.set_fast();
        }

        Ok((EventResult { request, state }, ()))
    }
    fn handle_tap_screen<'a>(&mut self, point: Point, pin: Self::TapInput<'a>) -> (EventResult, Self::TapOutput) {
        let state = None;
        let mut request = UpdateRequest::new();
        let mut pinok = false;
        if let Some(b) = self.pinpad.handle_tap(point, ()) {
            request.set_part(self.pinpad.buttons[b].bounding_box_absolut());
            self.push_entered(self.pinpad.buttons[b].num());
            if self.entered_nums.len() == pin.len() && self.check_pin(pin) {
                pinok = true;
            }
        }

        (EventResult{ request, state }, pinok)
    }
}