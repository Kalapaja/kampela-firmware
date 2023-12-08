#[cfg(not(feature="std"))]
use alloc::{format, string::String, string::ToString, vec::Vec, rc::Rc};
use core::{fmt::Display, cell::{RefCell, Ref}};
#[cfg(feature="std")]
use std::{format, string::String, string::ToString, vec::Vec, rc::Rc};

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

use crate::{widget::view::{View, Widget, DrawView}, test::{Test, StateOutput}};

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

impl StateOutput for Button {
    fn get_state(&self) -> bool {
        true
    }
}

impl <D: DrawTarget<Color = BinaryColor>> View<D> for Button {
    fn area(&self) -> Rectangle {
        self.widget.area()
    }

	fn draw_view(&self, target: &mut DrawView<D>) -> Result<(),D::Error> {
		let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 2);

        let area = <Button as View<D>>::area(self);
        area.into_styled(thin_stroke).draw(target)?;

        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        TextBox::with_textbox_style(
            &self.label,
            area,
            character_style,
            textbox_style,
        )
		.draw(target)?;
		Ok(())
	}
    fn handle_tap_view(&mut self, point: Point, target: &mut DrawView<D>) -> Result<(), D::Error> {
        self.label = String::from("Yay!");

        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::Off);
        let filled = PrimitiveStyle::with_fill(BinaryColor::On);
    
        let area = <Button as View<D>>::area(self);
        area.into_styled(filled).draw(target)?;
    
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();
    
        TextBox::with_textbox_style(
            &self.label,
            area,
            character_style,
            textbox_style,
        )
        .draw(target)?;
        Ok(())
    }
}