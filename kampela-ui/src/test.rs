#[cfg(not(feature="std"))]
use alloc::{rc::Rc, vec::Vec, format};
use substrate_parser::cards::Event;
#[cfg(feature="std")]
use std::{rc::Rc, vec::Vec, format};

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

use crate::{display_def::*, uistate::UpdateRequest, widget::view::ViewScreen};
use crate::uistate::{EventResult, Reason, Cause};
use crate::widget::{view::{View, DrawView}, button::Button};

pub struct Test {
    button: Button,
    count: usize,
}

impl Test {
    pub fn new() -> Self {
        Test {
            button: Button::new(
                "hello",
                Rectangle {
                    top_left: Point { x: 0, y: 30 },
                    size: Size { width: 66, height: 44 },
                }
            ),
            count: 0,
        }
    }
    fn inc_count(&mut self) {
        self.count = self.count + 1;
    }
}

pub trait StateOutput {
    fn is_tapped(&mut self) -> bool;
}

impl<D: DrawTarget<Color = BinaryColor>> ViewScreen<D> for Test
{
    fn draw_screen(&self, target: &mut D, reason: &Reason) -> Result<EventResult, D::Error> {
        self.button
        .draw(target, reason)?;

		let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        TextBox::with_textbox_style(
            format!("was pressed: {:?}", self.count).as_str(),
            Rectangle { top_left: SCREEN_ZERO, size: Size { width: SCREEN_SIZE_X, height: 30 } },
            character_style,
            textbox_style,
        )
		.draw(target)?;

        let mut request = UpdateRequest::new();
        let state = None;
        if matches!(reason.cause(), Cause::Tap) && reason.repeats() < 1 {
            request.set_fast();
        }
        Ok(EventResult { request, state })
    }
    fn handle_tap_screen(&mut self, point: Point) -> EventResult {
        let state = None;
        <Button as View<D>>::handle_tap(&mut self.button, point);

        let mut request = UpdateRequest::new();
        if self.button.is_tapped() { // example of changing state based on child request
            self.inc_count();
            request.set_fast() // only screen has authority to set update speed and new state
        }

        EventResult{ request, state }
    }
}