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

use crate::{display_def::*, uistate::{Screen, UpdateRequest}, widget::view::ViewScreen};
use crate::uistate::{EventResult, Reason, Cause};
use crate::widget::view::{View};

use crate::pin::{
    pinpad::{Pinpad, PAD_SIZE_WIDTH},
    pindots::{Pindots, DOT_DIAMETER},
};

pub const PIN_LEN: usize = 4;
const PIN_CODE_MOCK: [usize; PIN_LEN] = [0; PIN_LEN];

#[derive(Debug)]
pub struct Pincode<R> where
    R: Rng + ?Sized
{
    pinpad: Pinpad<R>,
    pindots: Pindots,
    entered_nums: Vec<usize>,
}

impl<R> Pincode<R> where
    R: Rng + ?Sized
{
    pub fn new(rng: &mut R) -> Self {
        let pindots_size = Size {
            width: DOT_DIAMETER * PIN_LEN as u32,
            height: DOT_DIAMETER,
        };
        Self {
            pinpad: Pinpad::new(
                Rectangle {
                    top_left: Point {
                        x: (SCREEN_SIZE_X - PAD_SIZE_WIDTH) as i32 / 2,
                        y: pindots_size.height as i32
                    },
                    size: Size {
                        width: PAD_SIZE_WIDTH,
                        height: SCREEN_SIZE_Y - pindots_size.height
                    },
                },
                SCREEN_ZERO,
                rng
            ),
            pindots: Pindots::new(
                Rectangle {
                    top_left: Point {
                        x: (SCREEN_SIZE_X - pindots_size.width) as i32 / 2,
                        y: 0,
                    },
                    size: pindots_size,
                },
                SCREEN_ZERO,
            ),
            entered_nums: Vec::new(),
        }
    }
    fn check_pin(&mut self) -> bool {
        if self.entered_nums == PIN_CODE_MOCK {
            true
        } else {
            false
        }
    }
    fn push_entered(&mut self, num: usize) {
        if self.entered_nums.len() < PIN_LEN {
            self.entered_nums.push(num);
        }
        if self.entered_nums.len() == PIN_LEN && !self.check_pin() {
            // TODO: proper attempt counter
            self.entered_nums = Vec::new();
        }
    }
}

impl<R> ViewScreen for Pincode<R> where
    R: Rng + ?Sized
{
    type DrawInput<'a> = &'a mut R where Self: 'a;
    type TapOutput = ();
    fn draw_screen<'a, D>(&mut self, target: &mut D, reason: &Reason, rng: &'a mut R) -> Result<EventResult, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        self.pindots.draw(target, reason, self.entered_nums.len())?;
        self.pinpad.draw(target, reason, rng)?;

        let mut request = UpdateRequest::new();
        let mut state = None;
        
        if matches!(reason.cause(), Cause::Tap) && reason.repeats() < 1 {
            if self.entered_nums.len() == PIN_LEN && self.check_pin(){
                state = Some(Screen::PinOk);
            } else {
                request.set_fast();
            }
        }

        Ok(EventResult { request, state })
    }
    fn handle_tap_screen(&mut self, point: Point) -> (EventResult, ()) {
        let state = None;
        let mut request = UpdateRequest::new();

        if let Some(b) = self.pinpad.handle_tap(point) {
            request.set_part(self.pinpad.buttons[b].bounding_box_absolut());
            self.push_entered(self.pinpad.buttons[b].num());
        }

        (EventResult{ request, state }, ())
    }
}