#[cfg(not(feature="std"))]
use alloc::{string::String, string::ToString};
#[cfg(feature="std")]
use std::{string::String, string::ToString};

use embedded_graphics::{
	pixelcolor::BinaryColor,
	prelude::{DrawTarget, Point, Primitive, Dimensions},
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

use crate::widget::view::{View, Widget, DrawView};
use crate::pin::pin::PIN_LEN;
use crate::uistate::{EventResult, Reason, Cause};

pub const DOT_DIAMETER: u32 = 16;

#[derive(Debug)]
pub struct Pindots {
	pub widget: Widget,
}

impl Pindots {
	pub fn new(area: Rectangle, parent_top_left: Point) -> Self {
		Pindots {
			widget: Widget::new(area, parent_top_left),
		}
	}
}

impl View for Pindots {
    type DrawInput<'a> = usize;
    type TapOutput = ();
    fn bounding_box(&self) -> Rectangle {
        self.widget.bounding_box()
    }
    fn bounding_box_absolut(&self) -> Rectangle {
        self.widget.bounding_box_absolut()
    }
	fn draw_view<D>(&mut self, target: &mut DrawView<D>, reason: &Reason, dots: usize) -> Result<(),D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 2);
        let filled = PrimitiveStyle::with_fill(BinaryColor::On);
        let unfilled = PrimitiveStyle::with_fill(BinaryColor::Off);
        let area = self.bounding_box_view();
        let diameter = area.size.height;
        for i in 0..PIN_LEN {
            let dot = Circle::new(
                Point {
                    x: area.top_left.x + i as i32 * diameter as i32,
                    y: area.top_left.y
                },
                diameter
            );
            if i < dots {
                dot.into_styled(filled).draw(target)?;
            } else {
                dot.into_styled(unfilled).draw(target)?;
                dot.into_styled(thin_stroke).draw(target)?;
            }
        }
        Ok(())
	}
    fn handle_tap_view(&mut self, _point: Point) {
    }
}
