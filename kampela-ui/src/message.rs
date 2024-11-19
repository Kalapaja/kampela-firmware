use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    mono_font::{
        ascii::FONT_10X20,
        MonoTextStyle,
    },
    primitives::{Primitive, PrimitiveStyle},
    Drawable,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

/// Draw the screen informing with fullscreen message
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
    let character_style = MonoTextStyle::new(&FONT_10X20, on);
    let textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Middle)
        .build();
    let bounds = display.bounding_box();
    bounds.into_styled(filled).draw(display)?;
    TextBox::with_textbox_style(
        message,
        bounds,
        character_style,
        textbox_style,
    )
    .draw(display)?;
    Ok(())
}
