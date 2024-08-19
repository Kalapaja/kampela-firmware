#[cfg(not(feature="std"))]
use alloc::string::String;
#[cfg(feature="std")]
use std::string::String;

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
        StrokeAlignment,
    },
    Drawable
};

use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use mnemonic_external::WORD_MAX_LEN;

use crate::{display_def::*, widget::view::{Widget,View, DrawView}};

use crate::seed_entry::proposal::PROPOSAL_AREA;

use super::phrase::PHRASE_AREA;

const ENTRY_FONT: MonoFont = FONT_10X20;
const ENTRY_RADIUS: u32 = 4;

pub const ENTRY_AREA: Rectangle = Rectangle{
    top_left: Point{
        x: 0,
        y: 0,
    },
    size: Size{
        width: SCREEN_SIZE_X,
        height: PHRASE_AREA.size.height - PROPOSAL_AREA.size.height,
    },
};

const TEXTBOX_SIZE: Size = Size{
    width: ENTRY_FONT.character_size.width * 8 + ENTRY_FONT.character_spacing * 7,
    height: ENTRY_FONT.character_size.height
};
const TEXT_AREA: Rectangle = Rectangle{
    top_left: Point{
        x: (ENTRY_AREA.size.width as i32 - TEXTBOX_SIZE.width as i32) / 2,
        y: (ENTRY_AREA.size.height as i32 - TEXTBOX_SIZE.height as i32) / 2
    },
    size: TEXTBOX_SIZE,
};

const ENTRY_WIDGET: Widget = Widget::new(ENTRY_AREA, SCREEN_ZERO);

pub struct Entry{
    entered: String,
    maxed: bool,
}

impl Entry {
    pub fn new() -> Self {
        Entry {
            entered: String::new(),
            maxed: false,
        }
    }
    pub fn add_letter(&mut self, c: char) -> bool {
        if self.entered.len() < WORD_MAX_LEN {
            self.entered.push(c);
            true
        } else {
            self.maxed = true;
            false
        }
    }
    pub fn remove_letter(&mut self) -> bool {
        if !self.is_empty() {
            self.entered.pop();
            true
        } else {
            false
        }
    }
    pub fn clear(&mut self) {
        self.entered = String::new();
    }
    pub fn is_empty(&self) -> bool {
        self.entered.is_empty()
    }
}

impl View for Entry {
    type DrawInput<'a> = bool;
    type DrawOutput = ();
    type TapInput<'a> = ();
    type TapOutput = ();

    fn bounding_box(&self) -> Rectangle {
        ENTRY_WIDGET.bounding_box()
    }

    fn bounding_box_absolut(&self) -> Rectangle {
        ENTRY_WIDGET.bounding_box_absolute()
    }

    fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, n: Self::DrawInput<'_>) -> Result<(), D::Error>
        where 
            D: DrawTarget<Color = BinaryColor>,
            Self: 'a,
    {
        let (on, _) = if n {
            (BinaryColor::Off, BinaryColor::On)
        } else {
            (BinaryColor::On, BinaryColor::Off)
        };

        if self.maxed {
            let thin_stroke = PrimitiveStyleBuilder::new()
                .stroke_color(on)
                .stroke_width(2)
                .stroke_alignment(StrokeAlignment::Inside)
                .build();
            let rounded = RoundedRectangle::new(
                TEXT_AREA.offset(ENTRY_RADIUS as i32),
                CornerRadii::new(Size::new(ENTRY_RADIUS, ENTRY_RADIUS))
            );
            rounded.into_styled(thin_stroke).draw(target)?;
            self.maxed = false;
        }

        let character_style = MonoTextStyle::new(&ENTRY_FONT, on);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        TextBox::with_textbox_style(
            &self.entered,
            TEXT_AREA,
            character_style,
            textbox_style,
        ).draw(target)?;

        Ok(())
    }

    fn handle_tap_view<'a>(&mut self, _: Point, _: ()) -> ()
    where Self: 'a {
    }
}
