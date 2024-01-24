#[cfg(not(feature="std"))]
use alloc::{string::String, string::ToString};
#[cfg(feature="std")]
use std::{string::String, string::ToString};

use embedded_graphics::{
	pixelcolor::BinaryColor,
	prelude::{DrawTarget, Point, Primitive, Dimensions},
	Drawable,
	mono_font::{
        ascii::{FONT_10X20},
        MonoTextStyle,
        MonoFont,
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

use crate::{widget::view::{View, Widget, DrawView}};

use crate::uistate::{EventResult, Reason, Cause};

pub const BUTTON_FONT: MonoFont = FONT_10X20;

#[derive(Debug)]
pub struct PinButton {
	num: usize,
	pub widget: Widget,
    tapped: bool,
}

impl PinButton {
	pub fn new(num: usize, area: Rectangle, parent_top_left: Point) -> Self {
		Self {
			num,
			widget: Widget::new(area, parent_top_left),
            tapped: false,
		}
	}
    fn reset_tapped(&mut self) -> bool {
        if self.tapped {
            self.tapped = false;
            true
        } else {
            false
        }
    }
    pub fn num(&self) -> usize {
        self.num
    }
}

impl View for PinButton {
    type DrawInput<'a> = ();
    type TapOutput = bool;
    fn bounding_box(&self) -> Rectangle {
        self.widget.bounding_box()
    }
    fn bounding_box_absolut(&self) -> Rectangle {
        self.widget.bounding_box_absolut()
    }
	fn draw_view<D: DrawTarget<Color = BinaryColor>>(&mut self, target: &mut DrawView<D>, reason: &Reason, input: ()) -> Result<(),D::Error> {
        match reason.cause() {
            Cause::NewScreen => {
                self.draw_initial(target)
            }
            Cause::Tap => {
                if self.reset_tapped() && reason.repeats() < 1 {
                    self.draw_tapped(target)
                } else {
                    self.draw_initial(target)
                }
            }
        }
	}
    fn handle_tap_view(&mut self, _point: Point) -> bool {
        self.tapped = true;
        true
    }
}

impl PinButton {
    fn draw_initial<D: DrawTarget<Color = BinaryColor>>(&self, target: &mut D) -> Result<(), D::Error> {
		let character_style = MonoTextStyle::new(&BUTTON_FONT, BinaryColor::On);
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 2);
        let filled = PrimitiveStyle::with_fill(BinaryColor::Off);

        let area = self.bounding_box_view();
        area.into_styled(filled).draw(target)?;
        area.into_styled(thin_stroke).draw(target)?;


        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        TextBox::with_textbox_style(
            &self.num.to_string(),
            area,
            character_style,
            textbox_style,
        )
		.draw(target)?;
		Ok(())
    }
    fn draw_tapped<D: DrawTarget<Color = BinaryColor>>(&self, target: &mut DrawView<D>) -> Result<(), D::Error> {     
        let filled = PrimitiveStyle::with_fill(BinaryColor::On);
        let area = self.bounding_box_view();
        area.into_styled(filled).draw(target)?;
    
        let character_style = MonoTextStyle::new(&BUTTON_FONT, BinaryColor::Off);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();
    
        TextBox::with_textbox_style(
            &self.num.to_string(),
            area,
            character_style,
            textbox_style,
        )
        .draw(target)?;
        Ok(())
    }
}