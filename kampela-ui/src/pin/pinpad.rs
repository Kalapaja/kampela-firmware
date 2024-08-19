use core::{array, marker::PhantomData};

use embedded_graphics::{
	pixelcolor::BinaryColor,
	prelude::{DrawTarget, Point, Size, Dimensions},
    primitives:: Rectangle,
};

use rand::seq::SliceRandom;
use crate::{display_def::*, platform::Platform, widget::view::{DrawView, View, Widget}};
use crate::pin::{pinbutton::PinButton, pindots::PINDOT_SIZE};

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

const PINPAD_WIDGET: Widget = Widget::new(PINPAD_AREA, SCREEN_ZERO);

const BUTTON_SIZE: Size = Size {
    width: PINPAD_AREA.size.width / 3,
    height: PINPAD_AREA.size.height / 4,
};

const fn get_pinbutton_widgets() -> [Widget; 10] {
    let mut widgets = [Widget::zero(); 10];
    let mut i = 0;
    while i < 10 {
        widgets[i] = Widget::new(Rectangle{
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
        PINPAD_WIDGET.absolute_top_left
        );
        i = i + 1;
    }
    widgets
}

const PIN_BUTTON_WIDGETS: [Widget; 10] = get_pinbutton_widgets();
/// Shuffle keys
fn get_pinbuttons<P: Platform>(h: &mut <P as Platform>::HAL) -> [PinButton; 10] {
    let mut pinnums: [u8; 10] = core::array::from_fn(|i| {
        (i).try_into()
            .expect("static initialization of numbers 0..15")
    });
    pinnums.shuffle(&mut P::rng(h));
    let pinset: [PinButton; 10] = array::from_fn(
        |i| PinButton::new(
            pinnums[i],
            &PIN_BUTTON_WIDGETS[i],
        )
    );
    pinset
}

#[derive(Debug)]
pub struct Pinpad<P> where
    P: Platform
{
    pub buttons: [PinButton; 10],
    input_type: PhantomData<P>,
}

impl<P> Pinpad<P> where
    P: Platform
{
	pub fn new(h: &mut <P as Platform>::HAL) -> Self {
        let buttons: [PinButton; 10] = get_pinbuttons::<P>(h);
		Self {
            buttons,
            input_type: PhantomData::<P>::default(),
		}
	}
    /// Change pin keys positions; remember to run before new key press
    fn shuffle(&mut self, h: &mut <P as Platform>::HAL) {
        self.buttons = get_pinbuttons::<P>(h);
    }
}

impl<P> View for Pinpad<P> where
    P: Platform
{
    type DrawInput<'a> = (bool, &'a mut <P as Platform>::HAL) where Self: 'a;
    type DrawOutput = ();
    type TapInput<'a> = () where Self: 'a,;
    type TapOutput = usize;
    fn bounding_box(&self) -> Rectangle {
        PINPAD_WIDGET.bounding_box()
    }
    fn bounding_box_absolut(&self) -> Rectangle {
        PINPAD_WIDGET.bounding_box_absolute()
    }
	fn draw_view<'a, D: DrawTarget<Color = BinaryColor>>(&mut self, target: &mut DrawView<D>, (t, h): Self::DrawInput<'a>) -> Result<(), D::Error>
    where Self: 'a {
        for button in self.buttons.iter_mut() {
            button.draw(target, t)?;
        }
        if t {
            self.shuffle(h);
        }
        Ok(())
	}
    fn handle_tap_view<'a>(&mut self, point: Point, _: ()) -> usize
    where Self: 'a {
        let mut tapped = 0;
        for (i, button) in self.buttons.iter_mut().enumerate() {
            if button.handle_tap(point, ()).is_some() {
                tapped = i;
            }
        }
        tapped
    }
}