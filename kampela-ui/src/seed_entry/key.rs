#[cfg(not(feature="std"))]
use alloc::vec::Vec;
#[cfg(feature="std")]
use std::vec::Vec;

use embedded_graphics::{
    mono_font::{
        ascii::FONT_10X20, MonoFont, MonoTextStyle
    },
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, Point, Size},
    primitives::{
        CornerRadii,
        Primitive,
        PrimitiveStyleBuilder,
        Rectangle,
        RoundedRectangle,
        StrokeAlignment
    },
    Drawable
};

use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use crate::widget::view::{Widget, View, DrawView};

const KEY_FONT: MonoFont = FONT_10X20;
const KEY_RADIUS: u32 = 4;

pub struct Key{
    widget: &'static Widget,
    label: &'static str,
    this_tapped: bool,
}

impl Key {
    pub fn new(label: &'static str, widget: &'static Widget) -> Self {
        Key {
            widget,
            label,
            this_tapped: false,
        }
    }
    pub fn get_char(&self) -> char {
        self.label.chars().collect::<Vec<char>>()[0].to_ascii_lowercase()
    }
}

impl View for Key {
    type DrawInput<'a> = bool;
    type DrawOutput = bool;
    type TapInput<'a> = ();
    type TapOutput = char;

    fn bounding_box(&self) -> Rectangle {
        self.widget.bounding_box()
    }

    fn bounding_box_absolut(&self) -> Rectangle {
        self.widget.bounding_box_absolute()
    }

    fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, t: Self::DrawInput<'_>) -> Result<Self::DrawOutput,D::Error>
        where 
            D: DrawTarget<Color = BinaryColor> {

        let mut was_tapped = false;
        self.draw_initial(target, t)?;
        if self.this_tapped {
            was_tapped = true;
            self.draw_tapped(target)?;
        }
        Ok(was_tapped)
    }

    fn handle_tap_view<'a>(&mut self, _: Point, _: ()) -> Self::TapOutput {
        self.this_tapped = true;
        self.get_char()
    }
}

impl Key {
    fn draw_initial<D>(&self, target: &mut D, t: bool) -> Result<(), D::Error>
        where 
            D: DrawTarget<Color = BinaryColor> {
        let character_style= if t {
            MonoTextStyle::new(&KEY_FONT, BinaryColor::Off)
        } else {
            MonoTextStyle::new(&KEY_FONT, BinaryColor::On)
        };
        
        let area = self.bounding_box_view();

        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        TextBox::with_textbox_style(
            &self.label,
            area,
            character_style,
            textbox_style,
        ).draw(target)?;

        Ok(())
    }

    fn draw_tapped<D>(&mut self, target: &mut D) -> Result<(), D::Error>
        where 
            D: DrawTarget<Color = BinaryColor> {
        self.this_tapped = false;
        let thin_stroke = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::Off)
            .stroke_width(2)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();
        let area = self.bounding_box_view();
        let rounded = RoundedRectangle::new(
            area,
            CornerRadii::new(Size::new(KEY_RADIUS, KEY_RADIUS))
        );
        rounded.into_styled(thin_stroke).draw(target)?;

        Ok(())
    }
}