use embedded_graphics::{
    mono_font::{
        ascii::{FONT_10X20},
        MonoTextStyle,
    },
    primitives::Rectangle,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    geometry::{Size},
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use crate::{display_def::*, uistate::UpdateRequest};
use crate::uistate::EventResult;
use crate::widget::{view::View, button::Button};

pub struct Test {
    button: Button
}
impl Test {
    pub fn new() -> Self {
        Test {
            button: Button::new(
                "hello",
                Rectangle {
                    top_left: Point { x: 100, y: 100 },
                    size: Size { width: 100, height: 20 }
                }
            )
        }

    }
    pub fn draw<D: DrawTarget<Color = BinaryColor>> (&self, target: &mut D) -> Result<(), D::Error> {
        self.button
        .draw(target)?;
        Ok(())
    }
    pub fn handle_tap<D: DrawTarget<Color = BinaryColor>> (&self, point: Point, target: &mut D) -> Result<EventResult, D::Error> {
        let out = UpdateRequest::new();
        let mut res = Ok(EventResult{request: out, state: None});
        if self.button.widget.area.contains(point){
            res = self.button.handle_tap(point, target)
        }
        res
    }
}
