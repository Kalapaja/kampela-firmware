#[cfg(not(feature="std"))]
use alloc::vec::Vec;
#[cfg(feature="std")]
use std::vec::Vec;

use embedded_graphics::{
    pixelcolor::BinaryColor, prelude::{Drawable, DrawTarget, Point}, primitives::{Primitive, PrimitiveStyle}
};

use rand::Rng;

use crate::{message, uistate::UpdateRequest, widget::view::ViewScreen};
use crate::uistate::EventResult;
use crate::widget::view::View;
use crate::platform::PinCode;

use crate::pin::{
    pinpad::Pinpad,
    pindots::Pindots,
};

pub const PIN_LEN: usize = 4;

#[derive(Debug)]
enum PinpadState {
    Initial,
    Tapped,
    TappedLast,
    DrawTapped,
    DrawWrong,
    DrawOk,
}

pub struct Pincode<R> where
    R: Rng + ?Sized
{
    pinpad: Pinpad<R>,
    pindots: Pindots,
    entered_nums: Vec<u8>,
    tapped: PinpadState,
    pinok: bool,
}

impl<R> Pincode<R> where
    R: Rng + ?Sized
{
    pub fn new(rng: &mut R) -> Self {
        Self {
            pinpad: Pinpad::new(rng),
            pindots: Pindots::new(),
            entered_nums: Vec::new(),
            tapped: PinpadState::Initial,
            pinok: false,
        }
    }
    fn check_pin(&mut self, pin: &PinCode) {
        // TODO: proper attempt counter and pin check
        if self.entered_nums.len() == pin.len() {
            if self.entered_nums == pin {
                self.pinok = true;
            }
            self.tapped = PinpadState::TappedLast;
            self.entered_nums = Vec::new();
        }
    }
    fn push_entered(&mut self, num: u8) {
        if self.entered_nums.len() < PIN_LEN {
            self.entered_nums.push(num);
        }
    }
    fn switch_tapped(&mut self) -> bool {
        match self.tapped {
            PinpadState::Initial => false,
            PinpadState::Tapped => {
                self.tapped = PinpadState::DrawTapped;
                true
            },
            PinpadState::TappedLast => {
                if self.pinok {
                    self.tapped = PinpadState::DrawOk;
                } else {
                    self.tapped = PinpadState::DrawWrong;
                }
                true
            },
            PinpadState::DrawTapped => {
                self.tapped = PinpadState::Initial;
                false
            },
            _ => false,
        }
    }
}

impl<R> ViewScreen for Pincode<R> where
    R: Rng + ?Sized
{
    type DrawInput<'a> = &'a mut R where R: 'a;
    type DrawOutput = bool;
    type TapInput<'a> = &'a PinCode where R: 'a;
    type TapOutput = ();
    fn draw_screen<'a, D>(&mut self, target: &mut D, rng: Self::DrawInput<'a>) -> Result<(EventResult, Self::DrawOutput), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        Self: 'a,
    {
        let mut request = None;
        let state = None;

        if matches!(self.tapped, PinpadState::DrawWrong) {
            message::draw(target, "Pin is wrong", false)?;
            request = Some(UpdateRequest::Fast);
            self.tapped = PinpadState::Initial;
            return Ok((EventResult {request, state}, false))
        }
        if matches!(self.tapped, PinpadState::DrawOk) {
            message::draw(target, "Pin is Ok", false)?;
            return Ok((EventResult {request, state}, true))
        }

        let t = self.switch_tapped();
        let filled = if t {
            PrimitiveStyle::with_fill(BinaryColor::On)
        } else {
            PrimitiveStyle::with_fill(BinaryColor::Off)
        };
        target.bounding_box().into_styled(filled).draw(target)?;
        
        self.pindots.draw(target, (self.entered_nums.len(), t))?;
        self.pinpad.draw(target, (rng, t))?;

        if t {
            request = Some(UpdateRequest::UltraFast);
        }

        Ok((EventResult { request, state }, false))
    }
    fn handle_tap_screen<'a>(&mut self, point: Point, pin: Self::TapInput<'a>) -> (EventResult, Self::TapOutput)
    where Self: 'a {
        let state = None;
        let mut request = None;
        if !matches!(self.tapped, PinpadState::Initial) { // ignore taps until permutated
            return (EventResult{ request, state }, ());
        }
        if let Some(b) = self.pinpad.handle_tap(point, ()) {
            self.tapped = PinpadState::Tapped;
            request = Some(UpdateRequest::UltraFast);
            self.push_entered(self.pinpad.buttons[b].num());
            self.check_pin(pin);
        }

        (EventResult{ request, state }, ())
    }
}