#[cfg(not(feature="std"))]
use alloc::string::String;
#[cfg(feature="std")]
use std::string::String;

use embedded_graphics::{
    mono_font::{
        ascii::{FONT_10X20},
        MonoTextStyle,
    },
    primitives::Rectangle,
    Drawable,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    pixelcolor::BinaryColor,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use crate::display_def::*;

/// Draw the screen informing that device receives nfc packets
pub fn draw<D>(display: &mut D, message: &str) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Middle)
        .build();
    let bounds = Rectangle::new(Point::new(0, 0), Size::new(SCREEN_SIZE_X, SCREEN_SIZE_Y));
    TextBox::with_textbox_style(
        message,
        bounds,
        character_style,
        textbox_style,
    )
    .draw(display)?;
    Ok(())
}
