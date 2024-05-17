use embedded_graphics::{
    mono_font::{
        ascii::FONT_10X20,
        MonoTextStyle,
    },
    primitives::{Primitive, PrimitiveStyle},
    Drawable,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

/// Draw the screen informing with fullscreen message
pub fn draw<D>(display: &mut D, message: &str) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let filled = PrimitiveStyle::with_fill(BinaryColor::On);
    let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::Off);
    let textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Middle)
        .build();
    let area = display.bounding_box();
    area.into_styled(filled).draw(display)?;
    TextBox::with_textbox_style(
        message,
        area,
        character_style,
        textbox_style,
    )
    .draw(display)?;
    Ok(())
}
