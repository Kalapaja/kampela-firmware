#[cfg(not(feature="std"))]
use alloc::{string::String, string::ToString, vec::Vec};
use core::{array, marker::PhantomData};
#[cfg(feature="std")]
use std::{string::String, string::ToString, vec::Vec}; 

use embedded_graphics::{
	pixelcolor::BinaryColor,
	prelude::{DrawTarget, Point, Primitive, Size, Dimensions},
	Drawable,
	mono_font::{
        ascii::{FONT_6X10},
        MonoTextStyle,
    },
    primitives::{
        Circle, PrimitiveStyle, Rectangle,
    },
};

use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};
use rand::{Rng, seq::SliceRandom};
use crate::display_def::*;
use crate::widget::view::{View, Widget, DrawView};
use crate::pin::{pinbutton::PinButton, pindots::PINDOT_SIZE};

use crate::uistate::EventResult;

const PAD_SIZE_WIDTH: u32 = 200;

pub const PINPAD_AREA: Rectangle = Rectangle {
    top_left: Point {
        x: (SCREEN_SIZE_X - PAD_SIZE_WIDTH) as i32 / 2,
        y: PINDOT_SIZE.height as i32
    },
    size: Size {
        width: PAD_SIZE_WIDTH,
        height: SCREEN_SIZE_Y - PINDOT_SIZE.height
    },
};

const BUTTON_SIZE: Size = Size {
    width: PINPAD_AREA.size.width / 3,
    height: PINPAD_AREA.size.height / 4,
};
/// Shuffle keys
fn get_pinbuttons<R: Rng + ?Sized>(rng: &mut R) -> [PinButton; 10] {
    let mut pinnums: [u8; 10] = core::array::from_fn(|i| {
        (i).try_into()
            .expect("static initialization of numbers 0..15")
    });
    pinnums.shuffle(rng);
    let pinset: [PinButton; 10] = array::from_fn(
        |i| PinButton::new(
            pinnums[i],
            Rectangle{
                top_left: Point {
                    x: {
                        match i {
                            0 => BUTTON_SIZE.width as i32,
                            _ => (i as i32 - 1) % 3 * BUTTON_SIZE.width as i32,
                        }
                    },
                    y: {
                        match i {
                            0 => 3 * BUTTON_SIZE.height as i32,
                            _ => (i as i32 - 1) / 3 * BUTTON_SIZE.height as i32,
                        }
                    }
                },
                size: BUTTON_SIZE,
            },
            PINPAD_AREA.top_left,
        )
    );
    pinset
}

#[derive(Debug)]
pub struct Pinpad<R> where
    R: Rng + ?Sized
{
	pub widget: Widget,
    pub buttons: [PinButton; 10],
    input_type: PhantomData<R>,
}

impl<R> Pinpad<R> where
    R: Rng + ?Sized
{
	pub fn new(parent_top_left: Point, rng: &mut R) -> Self {
        let widget = Widget::new(PINPAD_AREA, parent_top_left);

        let buttons: [PinButton; 10] = get_pinbuttons::<R>(rng);
		Self {
			widget,
            buttons,
            input_type: PhantomData::<R>::default(),
		}
	}
    /// Change pin keys positions; remember to run before new key press
    fn shuffle(&mut self, rng: &mut R) {
        self.buttons = get_pinbuttons::<R>(rng);
    }
}

impl<R> View for Pinpad<R> where
    R: Rng + ?Sized
{
    type DrawInput<'a> = &'a mut R where Self: 'a;
    type DrawOutput = bool;
    type TapInput<'a> = ();
    type TapOutput = usize;
    fn bounding_box(&self) -> Rectangle {
        self.widget.bounding_box()
    }
    fn bounding_box_absolut(&self) -> Rectangle {
        self.widget.bounding_box_absolut()
    }
	fn draw_view<'a, D: DrawTarget<Color = BinaryColor>>(&mut self, target: &mut DrawView<D>, rng: Self::DrawInput<'a>) -> Result<Self::DrawOutput, D::Error> {
        let mut t = false;
        for button in self.buttons.iter_mut() {
            if button.draw(target, ())? {
                t = true;
            }
        }
        if t {
            self.shuffle(rng);
        }
        Ok(t)
	}
    fn handle_tap_view(&mut self, point: Point, input: ()) -> usize {
        let mut tapped = 0;
        for (i, button) in self.buttons.iter_mut().enumerate() {
            if button.handle_tap(point, ()).is_some() {
                tapped = i;
            }
        }
        tapped
    }
}