use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    mono_font::{ascii::FONT_9X18_BOLD, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::PrimitiveStyle,
    text::{Alignment, Text},
    Drawable,
};

use crate::display_def::*;
use crate::widget::nav_bar::nav_bar::NavBar;
use crate::widget::view::View;

/// Simple welcome screen that always renders successfully
pub fn draw<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    // Clear screen with white background
    let filled = PrimitiveStyle::with_fill(BinaryColor::Off);
    display.bounding_box().into_styled(filled).draw(display)?;

    // Draw "Kampela" title
    let title_style = MonoTextStyle::new(&FONT_9X18_BOLD, BinaryColor::On);
    Text::with_alignment(
        "Kampela",
        Point::new(SCREEN_SIZE_X as i32 / 2, 60),
        title_style,
        Alignment::Center,
    )
    .draw(display)?;

    // Draw subtitle
    Text::with_alignment(
        "Ethereum Wallet",
        Point::new(SCREEN_SIZE_X as i32 / 2, 90),
        title_style,
        Alignment::Center,
    )
    .draw(display)?;

    // Draw version or status
    Text::with_alignment(
        "Ready",
        Point::new(SCREEN_SIZE_X as i32 / 2, 200),
        title_style,
        Alignment::Center,
    )
    .draw(display)?;

    // Draw navigation bar with Continue button
    let mut nav = NavBar::new(("", "Continue"));
    nav.draw(display, false)?;

    Ok(())
}
