use alloc::string::String;
use efm32pg23_fix::Peripherals;
use cortex_m::asm::delay;

use kampela_display_common::display_def::*;

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    pixelcolor::BinaryColor,
    primitives::rectangle::Rectangle,
    Drawable,
};

use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};

use embedded_text::{
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};

use crate::devices::display_transmission::{
    display_is_busy_cs,
    epaper_deep_sleep,
    epaper_hw_init_cs,
    epaper_reset,
    epaper_write_command,
    epaper_write_data,
    BUFSIZE
};
use crate::draw::FrameBuffer;
//**** Debug stuff ****//

/// Emergency debug function that spits out errors
/// TODO: replace by power drain in production!
pub fn burning_tank(peripherals: &mut Peripherals, text: String) {
    epaper_hw_init_cs(peripherals);
    make_text(peripherals, &text);
    delay(10000000);
    epaper_deep_sleep(peripherals);
}

/// see this <https://github.com/embedded-graphics/embedded-graphics/issues/716>
fn make_text(peripherals: &mut Peripherals, text: &str) {
    let mut buffer = FrameBuffer::new_white();
    let to_print = TextToPrint{line: text};
    to_print.draw(&mut buffer).unwrap();
    buffer.apply(peripherals);
}

struct TextToPrint<'a> {
    pub line: &'a str,
}

/// For custom font, see this <https://github.com/embedded-graphics/examples/blob/main/eg-0.7/examples/text-custom-font.rs>
impl Drawable for TextToPrint<'_> {
    type Color = BinaryColor;
    type Output = ();
    fn draw<D>(
        &self, 
        target: &mut D
    ) -> Result<Self::Output, <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Self::Color> 
    {
        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Left)
            .paragraph_spacing(5)
            .build();
        let bounds = Rectangle::new(Point::zero(), Size::new(SCREEN_SIZE_X, 0));
        TextBox::with_textbox_style(self.line, bounds, character_style, textbox_style).draw(target)?;
        Ok(())
    }
}

/// Last command in drawing protocol; actually starts display action
pub fn epaper_update(peripherals: &mut Peripherals) {
    epaper_write_command(peripherals, &[0x12]);
    delay(100000);
    while display_is_busy_cs(peripherals) {}
    epaper_write_command(peripherals, &[0x22]); // from manual, Y: "Display Update Control"
epaper_write_data(peripherals, &[0xF7]); // ?
    epaper_write_command(peripherals, &[0x20]); // from manual, Y: "Activate Display Update Sequence"
    while display_is_busy_cs(peripherals) {}
}

/// Partial display update; used to initiate display action when performing fast drawing without
/// full clear
pub fn epaper_update_part(peripherals: &mut Peripherals) {
    epaper_write_command(peripherals, &[0x22]); // from manual, Y: "Display Update Control"
    epaper_write_data(peripherals, &[0xFF]); // ?
    epaper_write_command(peripherals, &[0x20]); // from manual, Y: "Activate Display Update Sequence"
    delay(1000); // why delay, from where the number?
    while display_is_busy_cs(peripherals) {}
}



/// Normal drawing protocol, with full screen clearing
pub fn epaper_draw_stuff_differently(peripherals: &mut Peripherals, stuff: [u8; BUFSIZE]) {
    epaper_reset(&mut peripherals.GPIO_S);
    epaper_write_command(peripherals, &[0x4E]);
    epaper_write_data(peripherals, &[0x00]);
    epaper_write_command(peripherals, &[0x4F]);
    epaper_write_data(peripherals, &[0x07]);
    epaper_write_command(peripherals, &[0x24]); // from manual, Y: "Write Black and White image to RAM"
    epaper_write_data(peripherals, &stuff);
    epaper_write_command(peripherals, &[0x26]);
    epaper_write_data(peripherals, &stuff);
    epaper_update(peripherals);
}

/// Fast and dirty refresh drawing
pub fn epaper_draw_stuff_quickly(peripherals: &mut Peripherals, stuff: [u8; BUFSIZE]) {
    epaper_reset(&mut peripherals.GPIO_S);
    epaper_write_command(peripherals, &[0x4E]);
    epaper_write_data(peripherals, &[0x00]);
    epaper_write_command(peripherals, &[0x4F]);
    epaper_write_data(peripherals, &[0x07]);
    epaper_write_command(peripherals, &[0x3C]);
    epaper_write_data(peripherals, &[0x80]);
    epaper_write_command(peripherals, &[0x24]); // from manual, Y: "Write Black and White image to RAM"
    epaper_write_data(peripherals, &stuff);
    epaper_update_part(peripherals);
}
