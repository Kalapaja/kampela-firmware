use embedded_graphics::{
    mono_font::{
        ascii::FONT_6X10,
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
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};

pub fn draw<D>(content: &str, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let filled = PrimitiveStyle::with_fill(BinaryColor::Off);
    let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    let textbox_style = TextBoxStyleBuilder::new()
        .height_mode(HeightMode::FitToText)
        .alignment(HorizontalAlignment::Left)
        .paragraph_spacing(5)
        .build();
    let area = display.bounding_box();
    area.into_styled(filled).draw(display)?;
    TextBox::with_textbox_style(content, area, character_style, textbox_style).draw(display)?;
    Ok(())
}

