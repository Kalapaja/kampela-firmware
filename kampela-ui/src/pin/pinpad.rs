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
use crate::widget::view::{View, Widget, DrawView};
use crate::pin::{pinbutton::PinButton};

use crate::uistate::{EventResult, Reason, Cause};

pub const PAD_SIZE_WIDTH: u32 = 200;

/// Shuffle keys
fn get_pinbuttons<R: Rng + ?Sized>(rng: &mut R, bounding_box_absolut: Rectangle) -> [PinButton; 10] {
    let button_size = Size {
        width: bounding_box_absolut.size.width / 3,
        height: bounding_box_absolut.size.height / 4,
    };
    let mut pinnums: [usize; 10] = core::array::from_fn(|i| {
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
                            0 => button_size.width as i32,
                            _ => (i as i32 - 1) % 3 * button_size.width as i32,
                        }
                    },
                    y: {
                        match i {
                            0 => 3 * button_size.height as i32,
                            _ => (i as i32 - 1) / 3 * button_size.height as i32,
                        }
                    }
                },
                size: button_size,
            },
            bounding_box_absolut.top_left,
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
	pub fn new(area: Rectangle, parent_top_left: Point, rng: &mut R) -> Self {
        let widget = Widget::new(area, parent_top_left);

        let button_size = Size {
            width: area.size.width / 3,
            height: area.size.height / 4,
        };
        let buttons: [PinButton; 10] = get_pinbuttons::<R>(rng, widget.bounding_box_absolut());
		Self {
			widget,
            buttons,
            input_type: PhantomData::<R>::default(),
		}
	}
    /// Change pin keys positions; remember to run before new key press
    fn shuffle(&mut self, rng: &mut R) {
        self.buttons = get_pinbuttons::<R>(rng, self.widget.bounding_box_absolut());
    }
}

impl<R> View for Pinpad<R> where
    R: Rng + ?Sized
{
    type DrawInput<'a> = &'a mut R where Self: 'a;
    type TapOutput = usize;
    fn bounding_box(&self) -> Rectangle {
        self.widget.bounding_box()
    }
    fn bounding_box_absolut(&self) -> Rectangle {
        self.widget.bounding_box_absolut()
    }
	fn draw_view<'a, D: DrawTarget<Color = BinaryColor>>(&mut self, target: &mut DrawView<D>, reason: &Reason, rng: Self::DrawInput<'a>) -> Result<(),D::Error> {
        if matches!(reason.cause(), Cause::Tap) && reason.repeats() > 0 {
            self.shuffle(rng);
        }
        for button in self.buttons.iter_mut() {
            button.draw(target, reason, ())?
        }
        Ok(())
	}
    fn handle_tap_view(&mut self, point: Point) -> usize {
        let mut tapped = 0;
        for (i, button) in self.buttons.iter_mut().enumerate() {
            if button.handle_tap(point).is_some() {
                tapped = i;
            }
        }
        tapped
    }
}