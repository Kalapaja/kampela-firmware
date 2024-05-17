#[cfg(not(feature="std"))]
use alloc::{vec::Vec, string::String};
#[cfg(feature="std")]
use std::{vec::Vec, string::String};

use embedded_graphics::{
    mono_font::{
        ascii::FONT_6X12, MonoFont, MonoTextStyle
    },
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, Point, Size},
    primitives::{
        CornerRadii,
        Primitive,
        PrimitiveStyleBuilder,
        Rectangle,
        RoundedRectangle,
        StrokeAlignment,
    },
    Drawable
};

use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use patches::phrase::{WordListElement, phrase_to_entropy};

use crate::{display_def::*, widget::view::{DrawView, View, Widget}};

use super::keyboard::KEYBOARD_AREA;

const PHRASE_FONT: MonoFont = FONT_6X12;
const PHRASE_RADIUS: u32 = 4;

pub const MAX_PHRASE: usize = 24;

pub const PHRASE_AREA: Rectangle = Rectangle{
    top_left: Point {
        x: 0,
        y: 0,
    },
    size: Size{
        width: SCREEN_SIZE_X,
        height: KEYBOARD_AREA.top_left.y as u32,
    },
};

const PHRASE_WIDGET: Widget = Widget::new(PHRASE_AREA, SCREEN_ZERO);

pub struct Phrase{
    buffer: Vec<WordListElement>,
    invalid: bool,
}

impl Phrase {
    pub fn new() -> Self {
        Phrase {
            buffer: Vec::new(),
            invalid: false,
        }
    }
    pub fn add_word(&mut self, word: WordListElement) {
        self.buffer.push(word);
    }
    pub fn remove_word(&mut self) {
        if !self.buffer.is_empty() {
            self.buffer.pop();
        }
    }
    pub fn validate(&mut self) -> Option<Vec<u8>> {
        match phrase_to_entropy(
            &self
                .buffer
                .iter()
                .map(|a| String::from(a.word()))
                .collect::<Vec<String>>()
                .join(" "),
        ) {
            Ok(a) => {
                Some(a)
            }
            Err(_) => None,
        }
    }
    pub fn is_maxed(&self) -> bool {
        self.buffer.len() >= MAX_PHRASE
    }
}

impl View for Phrase {
    type DrawInput<'a> = bool;
    type DrawOutput = ();
    type TapInput<'a> = ();
    type TapOutput = ();

    fn bounding_box(&self) -> Rectangle {
        PHRASE_WIDGET.bounding_box()
    }

    fn bounding_box_absolut(&self) -> Rectangle {
        PHRASE_WIDGET.bounding_box_absolute()
    }

    fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, t: Self::DrawInput<'_>) -> Result<Self::DrawOutput,D::Error>
        where 
            D: DrawTarget<Color = BinaryColor> {
        
        let character_style = if t {
            MonoTextStyle::new(&PHRASE_FONT, BinaryColor::Off)
        } else {
            MonoTextStyle::new(&PHRASE_FONT, BinaryColor::On)
        };
        
        let area = self.bounding_box_view();

        if self.invalid {
            let thin_stroke = PrimitiveStyleBuilder::new()
                .stroke_color(BinaryColor::On)
                .stroke_width(2)
                .stroke_alignment(StrokeAlignment::Inside)
                .build();
            let area = self.bounding_box_view();
            let rounded = RoundedRectangle::new(
                area,
                CornerRadii::new(Size::new(PHRASE_RADIUS, PHRASE_RADIUS))
            );
            rounded.into_styled(thin_stroke).draw(target)?;
            self.invalid = false;
        }

        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Top)
            .build();

        let text = self.buffer
            .iter()
            .map(|a| String::from(a.word()))
            .collect::<Vec<String>>()
            .join(" ");

        TextBox::with_textbox_style(
            &text,
            area,
            character_style,
            textbox_style,
        ).draw(target)?;

        Ok(())
    }

    fn handle_tap_view<'a>(&mut self, _: Point, _: ()) -> Self::TapOutput {
    }
}