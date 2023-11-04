use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    primitives::Rectangle,
};

use crate::uistate::EventResult;

pub struct Widget {
    pub area: Rectangle,
}

pub trait View<D> where
    D: DrawTarget<Color = BinaryColor>,
{
	fn draw(&self, target: &mut D) -> Result<(),D::Error>;
	fn handle_tap(&self, point: Point, target: &mut D) -> Result<EventResult, D::Error>;
}
