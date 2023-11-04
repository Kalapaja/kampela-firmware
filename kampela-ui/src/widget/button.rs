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

use crate::{
    widget::view::{View, Widget},
    platform::Platform,
};

use crate::uistate::{EventResult, UpdateRequest, Screen};

pub struct Button {
	label: String,
	pub widget: Widget,
}

impl Button {
	pub fn new(label: &str, area: Rectangle) -> Self {
		Button {
			label: label.to_string(),
			widget: Widget { area },
		}
	}
}

impl <D: DrawTarget<Color = BinaryColor>> View<D> for Button {
	fn draw(&self, target: &mut D) -> Result<(),D::Error> {
		let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 2);

        self.widget.area.into_styled(thin_stroke).draw(target)?;

        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        TextBox::with_textbox_style(
            &self.label,
            self.widget.area.bounding_box(),
            character_style,
            textbox_style,
        )
		.draw(target)?;
		Ok(())
	}
    fn handle_tap(&self, point: Point, target: &mut D) -> Result<EventResult, D::Error> {
        let mut request = UpdateRequest::new();
        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::Off);
        let filled = PrimitiveStyle::with_fill(BinaryColor::On);
    
        self.widget.area.into_styled(filled).draw(target)?;
    
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();
    
        TextBox::with_textbox_style(
            &self.label,
            self.widget.area,
            character_style,
            textbox_style,
        )
        .draw(target)?;
        request.set_both();
        let state = None;
        Ok(EventResult {request, state})
    }
}