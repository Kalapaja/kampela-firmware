#[cfg(not(feature="std"))]
use alloc::{string::String, string::ToString};
#[cfg(feature="std")]
use std::{string::String, string::ToString};

use embedded_graphics::{
	pixelcolor::BinaryColor,
	prelude::{DrawTarget, Point, Primitive},
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

use crate::uistate::{EventResult, Reason, Cause};

pub struct Button {
	label: String,
	pub widget: Widget,
    tapped: bool,
}

impl Button {
	pub fn new(label: &str, area: Rectangle) -> Self {
		Button {
			label: label.to_string(),
			widget: Widget::new(area),
            tapped: false,
		}
	}
}

impl StateOutput for Button {
    fn is_tapped(&mut self) -> bool {
        if self.tapped {
            self.tapped = false;
            true
        } else {
            false
        }
    }
}

impl <D: DrawTarget<Color = BinaryColor>> View<D> for Button {
    fn area(&self) -> Rectangle {
        self.widget.area()
    }

	fn draw_view(&self, target: &mut DrawView<D>, reason: &Reason) -> Result<(),D::Error> {
        match reason.cause() {
            Cause::NewScreen => {
                self.draw_initial(target)
            }
            Cause::Tap => {
                if reason.repeats() < 1 {
                    self.draw_tapped(target)
                } else {
                    self.draw_untapped(target)
                }
            }
        }
	}
    fn handle_tap_view(&mut self, _point: Point) {
        self.label = String::from("Yay!");
        self.tapped = true; //example of parent state change request


    }
}

impl Button {
    fn draw_initial<D: DrawTarget<Color = BinaryColor>>(&self, target: &mut DrawView<D>) -> Result<(), D::Error> {
		let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 2);

        let area = <Button as View<D>>::area_view(self);
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
    fn draw_tapped<D: DrawTarget<Color = BinaryColor>>(&self, target: &mut DrawView<D>) -> Result<(), D::Error> {        
        let filled = PrimitiveStyle::with_fill(BinaryColor::On);
        let area = <Button as View<D>>::area_view(self);
        area.into_styled(filled).draw(target)?;
    
        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::Off);
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
    fn draw_untapped<D: DrawTarget<Color = BinaryColor>>(&self, target: &mut DrawView<D>) -> Result<(), D::Error> {        
        let filled = PrimitiveStyle::with_fill(BinaryColor::Off);
        let area = <Button as View<D>>::area_view(self);
        area.into_styled(filled).draw(target)?;

        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 2);
        let area = <Button as View<D>>::area_view(self);
        area.into_styled(thin_stroke).draw(target)?;
    
        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
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