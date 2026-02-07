//! Ethereum transaction display screen.
//!
//! Shows transaction details using clear-signing format and allows
//! the user to approve (right button) or reject (left button).

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

/// Draw the Ethereum transaction screen with transaction details.
///
/// `display_text` should be the formatted transaction from clear-signing.
/// Returns Ok(()) if drawn successfully.
pub fn draw<D>(display: &mut D, display_text: &str, n: bool) -> Result<(), D::Error>
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

    // Draw title
    let title_style = MonoTextStyle::new(&FONT_10X20, on);
    let title_textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Center)
        .build();

    let title_bounds = Rectangle::new(
        bounds.top_left,
        embedded_graphics::prelude::Size::new(bounds.size.width, 25),
    );

    TextBox::with_textbox_style(
        "Sign Transaction?",
        title_bounds,
        title_style,
        title_textbox_style,
    )
    .draw(display)?;

    // Draw transaction details
    let content_style = MonoTextStyle::new(&FONT_6X10, on);
    let content_textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Left)
        .vertical_alignment(VerticalAlignment::Top)
        .build();

    let content_bounds = Rectangle::new(
        embedded_graphics::prelude::Point::new(5, 30),
        embedded_graphics::prelude::Size::new(bounds.size.width - 10, bounds.size.height - 60),
    );

    TextBox::with_textbox_style(
        display_text,
        content_bounds,
        content_style,
        content_textbox_style,
    )
    .draw(display)?;

    // Draw navigation bar (left = reject, right = approve)
    let mut nav = NavBar::new(("Reject", "Approve"));
    nav.draw(display, n)?;

    Ok(())
}
