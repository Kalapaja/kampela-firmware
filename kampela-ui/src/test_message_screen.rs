//! Test message display screen.
//!
//! Displays a dynamic string message received via NFC.

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    mono_font::{ascii::FONT_10X20, ascii::FONT_6X10, MonoTextStyle},
    primitives::{Primitive, PrimitiveStyle, Rectangle},
    Drawable,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use crate::widget::nav_bar::nav_bar::NavBar;
use crate::widget::view::View;

/// Draw test message screen with the provided text message.
///
/// User can tap right button to dismiss and return to address screen.
pub fn draw<D>(display: &mut D, message: &str, n: bool) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let (on, off) = if n {
        (BinaryColor::Off, BinaryColor::On)
    } else {
        (BinaryColor::On, BinaryColor::Off)
    };

    let filled = PrimitiveStyle::with_fill(off);
    let bounds = display.bounding_box();

    // Clear screen
    bounds.into_styled(filled).draw(display)?;

    // Draw "Test Message" title
    let title_style = MonoTextStyle::new(&FONT_10X20, on);
    let title_textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Center)
        .build();

    let title_bounds = Rectangle::new(
        bounds.top_left,
        embedded_graphics::prelude::Size::new(bounds.size.width, 25),
    );

    TextBox::with_textbox_style(
        "Test Message",
        title_bounds,
        title_style,
        title_textbox_style,
    )
    .draw(display)?;

    // Draw message content
    let content_style = MonoTextStyle::new(&FONT_6X10, on);
    let content_textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Middle)
        .build();

    let content_bounds = Rectangle::new(
        embedded_graphics::prelude::Point::new(5, 30),
        embedded_graphics::prelude::Size::new(bounds.size.width - 10, bounds.size.height - 60),
    );

    TextBox::with_textbox_style(
        message,
        content_bounds,
        content_style,
        content_textbox_style,
    )
    .draw(display)?;

    // Draw navigation bar (right button = OK/dismiss)
    let mut nav = NavBar::new(("", "OK"));
    nav.draw(display, n)?;

    Ok(())
}
