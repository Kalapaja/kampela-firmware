#[cfg(not(feature="std"))]
use alloc::{format, string::String, string::ToString, vec::Vec};
use core::fmt::Display;
#[cfg(feature="std")]
use std::{format, string::String, string::ToString, vec::Vec};

use embedded_graphics::{
	pixelcolor::BinaryColor,
	prelude::{DrawTarget, Point, Primitive},
	geometry::Dimensions,
	Drawable,
	mono_font::{
        ascii::{FONT_6X10},
        MonoTextStyle,
    },
    primitives::{
        Circle, PrimitiveStyle, Rectangle,
    },
};

use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use crate::{widget::view::{View, Widget, DrawWindow, Display}};

use crate::uistate::{EventResult, UpdateRequest};


pub struct Button {
	label: String,
	pub widget: Widget,
}

impl Button {
	pub fn new(label: &str, area: Rectangle) -> Self {
		Button {
			label: label.to_string(),
			widget: Widget::new(area),
		}
	}
}

impl View for Button {
    fn area(&self) -> Rectangle {
        self.widget.area()
    }

	fn draw_view<D>(&self, target: &mut DrawWindow<D>) -> Result<(),D::Error>
    where
        D: DrawTarget,
    {
		let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 2);

        let area = <Button as View>::area_view(self);
        area.into_styled(thin_stroke).draw(target)?;

        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();
        let textbox = TextBox::with_textbox_style(
            &self.label,
            area,
            character_style,
            textbox_style,
        );
        target.draw_textbox(textbox);
		Ok(())
	}

    fn handle_tap_view<D>(&mut self, point: Point, target: &mut DrawWindow<D>) -> Result<EventResult, D::Error>
    where
        D: DrawTarget
    {
        let state = None;
        self.label = String::from("Yay!");

        let mut request = UpdateRequest::new();
        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::Off);
        let filled = PrimitiveStyle::with_fill(BinaryColor::On);
    
        let mut area = <Button as View>::area_view(self);
        area.into_styled(filled);
        target.draw_styled(area);
    
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();
    
        let textbox = TextBox::with_textbox_style(
            &self.label,
            area,
            character_style,
            textbox_style,
        );
        target.draw_textbox(textbox);
        request.set_both();
        Ok(EventResult {request, state})
    }
}