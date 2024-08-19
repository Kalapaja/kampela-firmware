#[cfg(not(feature="std"))]
use alloc::vec::Vec;
use core::marker::PhantomData;
#[cfg(feature="std")]
use std::vec::Vec;

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

use mnemonic_external::{AsWordList, WordListElement, WordSet};

use crate::{platform::Platform, display_def::*, widget::view::{DrawView, View, Widget}};

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

pub struct Phrase<P> where
    P: Platform + ?Sized
{
    buffer: Vec<WordListElement>,
    invalid: bool,
    platform_type: PhantomData<P>,
}

impl<P: Platform> Phrase<P> {
    pub fn new(phrase: Option<Vec<WordListElement>>) -> Self
        where <P as Platform>::AsWordList: Sized {
        let buffer = if let Some(phrase) = phrase {
            phrase
        } else {
            Vec::new()
        };
        Phrase {
            buffer,
            invalid: false,
            platform_type: PhantomData::<P>::default(),
        }
    }
    pub fn add_word(&mut self, word: WordListElement) {
        self.buffer.push(word);
    }
    pub fn remove_word(&mut self) {
        if !self.buffer.is_empty() {
            self.buffer.pop();
        } else {
            self.set_invalid();
        }
    }
    pub fn validate(&self) -> Option<Vec<u8>> {
        self.buffer.iter().collect::<WordSet>().to_entropy().ok()
    }
    pub fn get_phrase(&self) -> &Vec<WordListElement> {
        &self.buffer
    }
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
    pub fn is_maxed(&self) -> bool {
        self.buffer.len() >= MAX_PHRASE
    }
    pub fn set_invalid(&mut self) {
        self.invalid = true;
    }
}

impl<P: Platform> View for Phrase<P> {
    type DrawInput<'a> = bool where P: 'a;
    type DrawOutput = ();
    type TapInput<'a> = () where P: 'a;
    type TapOutput = ();

    fn bounding_box(&self) -> Rectangle {
        PHRASE_WIDGET.bounding_box()
    }

    fn bounding_box_absolut(&self) -> Rectangle {
        PHRASE_WIDGET.bounding_box_absolute()
    }

    fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, n: Self::DrawInput<'a>) -> Result<Self::DrawOutput,D::Error>
        where 
            D: DrawTarget<Color = BinaryColor>,
            Self: 'a,
        {
        
        let character_style = if n {
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

        TextBox::with_textbox_style(
            &<P::AsWordList>::words_to_phrase(&self.buffer),
            area,
            character_style,
            textbox_style,
        ).draw(target)?;

        Ok(())
    }

    fn handle_tap_view<'a>(&mut self, _: Point, _: ()) -> Self::TapOutput
    where Self: 'a {
    }
}