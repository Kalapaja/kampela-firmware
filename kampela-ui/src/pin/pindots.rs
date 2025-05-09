use embedded_graphics::{
	pixelcolor::BinaryColor,
	prelude::{DrawTarget, Point, Primitive, Dimensions},
	Drawable,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment,
    },
    geometry::Size,
};

use crate::{display_def::*, uistate::Event, widget::view::{DrawView, View, Widget}};
use crate::pin::pin::PIN_LEN;

const DOT_DIAMETER: u32 = 16;
const DOT_SPACING: u32 = 2;
pub const PINDOT_SIZE: Size = Size {
    width: DOT_DIAMETER * PIN_LEN as u32 + DOT_SPACING * 3,
    height: DOT_DIAMETER,
};

const PINDOTS_AREA: Rectangle = Rectangle {
    top_left: Point {
        x: (SCREEN_SIZE_X - PINDOT_SIZE.width) as i32 / 2,
        y: 0,
    },
    size: PINDOT_SIZE,
};

const PINDOTS_WIDGET: Widget = Widget::new(PINDOTS_AREA, SCREEN_ZERO);

#[derive(Debug)]
pub struct Pindots {
}

impl Pindots {
	pub fn new() -> Self {
		Pindots {}
	}
}

impl View for Pindots {
    type DrawInput<'a> = usize;
    type DrawOutput = ();
    type EventInput<'a> = ();
    type TapOutput = ();
    fn bounding_box(&self) -> Rectangle {
        PINDOTS_WIDGET.bounding_box()
    }
    fn bounding_box_absolut(&self) -> Rectangle {
        PINDOTS_WIDGET.bounding_box_absolute()
    }
	fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, dots: Self::DrawInput<'_>) -> Result<(),D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        Self: 'a,
    {
        let thin_stroke = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(2)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();
        let filled = PrimitiveStyle::with_fill(BinaryColor::On);
        let area = self.bounding_box_view();
        for i in 0..PIN_LEN {
            let dot = Circle::new(
                Point {
                    x: area.top_left.x + i as i32 * (DOT_DIAMETER as i32 + DOT_SPACING as i32),
                    y: area.top_left.y
                },
                DOT_DIAMETER
            );
            if i < dots {
                dot.into_styled(filled).draw(target)?;
                dot.into_styled(thin_stroke).draw(target)?;
            } else {
                dot.into_styled(thin_stroke).draw(target)?;
            }
        }
        Ok(())
	}
    fn handle_event_view<'a>(&mut self, _event: Event, _: ())
    where Self: 'a {
    }
}
