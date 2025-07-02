#[cfg(not(feature="std"))]
use alloc::vec::Vec;
#[cfg(feature="std")]
use std::vec::Vec;

use embedded_graphics::{
    mono_font::{
        ascii::FONT_10X20, MonoFont, MonoTextStyle
    },
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, Size},
    primitives::{
        CornerRadii,
        Primitive,
        PrimitiveStyleBuilder,
        Rectangle,
        RoundedRectangle,
    },
    Drawable
};

use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use crate::{uistate::Event, widget::view::{DrawView, View, Widget}};

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
    type EventInput<'a> = ();
    type TapOutput = Option<char>;

    fn bounding_box(&self) -> Rectangle {
        self.widget.bounding_box()
    }

    fn bounding_box_absolut(&self) -> Rectangle {
        self.widget.bounding_box_absolute()
    }

    fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, n: Self::DrawInput<'_>) -> Result<Self::DrawOutput,D::Error>
        where 
            D: DrawTarget<Color = BinaryColor>,
            Self: 'a,
        {
        let mut was_tapped = false;

        self.draw_initial(target, n)?;
        if self.this_tapped {
            was_tapped = true;
            self.draw_tapped(target, n)?;
            self.draw_initial(target, !n)?;
        }
        Ok(was_tapped)
    }

    fn handle_event_view<'a>(&mut self, event: Event, _: ()) -> Self::TapOutput
    where Self: 'a {
        if matches!(event, Event::Tap(_)) {
            self.this_tapped = true;
            Some(self.get_char())
        } else {
            None
        }
    }
}

impl Key {
    fn draw_initial<D>(&self, target: &mut D, n: bool) -> Result<(), D::Error>
        where 
            D: DrawTarget<Color = BinaryColor> {
        let character_style= if n {
            MonoTextStyle::new(&KEY_FONT, BinaryColor::Off)
        } else {
            MonoTextStyle::new(&KEY_FONT, BinaryColor::On)
        };
        
        let bounds = self.bounding_box_view();

        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        TextBox::with_textbox_style(
            &self.label,
            bounds,
            character_style,
            textbox_style,
        ).draw(target)?;

        Ok(())
    }

    fn draw_tapped<D>(&mut self, target: &mut D, n: bool) -> Result<(), D::Error>
        where 
            D: DrawTarget<Color = BinaryColor> {
        let (on, _) = if n {
            (BinaryColor::Off, BinaryColor::On)
        } else {
            (BinaryColor::On, BinaryColor::Off)
        };
        self.this_tapped = false;
        let flll = PrimitiveStyleBuilder::new()
            .fill_color(on)
            .build();
        let area = self.bounding_box_view();
        let rounded = RoundedRectangle::new(
            area,
            CornerRadii::new(Size::new(KEY_RADIUS, KEY_RADIUS))
        );
        rounded.into_styled(flll).draw(target)?;

        Ok(())
    }
}