#[cfg(not(feature="std"))]
use alloc::vec::Vec;
#[cfg(feature="std")]
use std::vec::Vec;

use embedded_graphics::{
    pixelcolor::BinaryColor, prelude::{Drawable, DrawTarget, Point}, primitives::{Primitive, PrimitiveStyle}
};

use rand::Rng;

use crate::{uistate::UpdateRequest, widget::view::ViewScreen};
use crate::uistate::EventResult;
use crate::widget::view::View;
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
    tapped: bool,
}

impl<R> Pincode<R> where
    R: Rng + ?Sized
{
    pub fn new(rng: &mut R) -> Self {

        Self {
            pinpad: Pinpad::new(rng),
            pindots: Pindots::new(),
            entered_nums: Vec::new(),
            tapped: false,
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
    fn reset_tapped(&mut self) -> bool {
        if self.tapped {
            self.tapped = false;
            true
        } else {
            false
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
    fn draw_screen<'a, D>(&mut self, target: &mut D, rng: Self::DrawInput<'a>) -> Result<(EventResult, ()), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let mut request = UpdateRequest::new();
        let state = None;

        let t = self.reset_tapped();
        let filled = if t {
            PrimitiveStyle::with_fill(BinaryColor::On)
        } else {
            PrimitiveStyle::with_fill(BinaryColor::Off)
        };
        target.bounding_box().into_styled(filled).draw(target)?;
        
        self.pindots.draw(target, (self.entered_nums.len(), t))?;
        self.pinpad.draw(target, (rng, t))?;

        if t {
            if self.entered_nums.is_empty() {
                request.set_fast();
            } else {
                request.set_ultrafast();
            }
        }

        Ok((EventResult { request, state }, ()))
    }
    fn handle_tap_screen<'a>(&mut self, point: Point, pin: Self::TapInput<'a>) -> (EventResult, Self::TapOutput) {
        let state = None;
        let mut request = UpdateRequest::new();
        let mut pinok = false;
        if let Some(b) = self.pinpad.handle_tap(point, ()) {
            self.tapped = true;
            request.set_ultrafast();
            self.push_entered(self.pinpad.buttons[b].num());
            if self.entered_nums.len() == pin.len() && self.check_pin(pin) {
                pinok = true;
            }
        }

        (EventResult{ request, state }, pinok)
    }
}