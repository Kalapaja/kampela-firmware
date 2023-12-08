use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Pixel, Dimensions},
    primitives::{Rectangle, Primitive},
    geometry::{Size},
};

use crate::uistate::{EventResult, Reason};
use crate::display_def::*;

pub struct DrawView<'a, D> where
    D: DrawTarget
{
    area: Rectangle,
    origin: &'a mut D,
}

impl <'a, D: DrawTarget> DrawView<'a, D> {
    fn new(area: Rectangle, target: &'a mut D) -> Self {
        DrawView { area, origin: target }
    }
}

impl <'a, D: DrawTarget> DrawTarget for DrawView<'a, D> {
    type Color = D::Color;
    type Error = D::Error;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>> {
        let pixels_offsetted = pixels.into_iter().map(
            |p| {
                Pixel(Point {
                    x: p.0.x + self.area.top_left.x,
                    y: p.0.y + self.area.top_left.y
                }, p.1)
            });
        self.origin.draw_iter(pixels_offsetted)
    }
}

impl <'a, D: DrawTarget> Dimensions for DrawView<'a, D> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle { top_left: Point { x: 0, y: 0 }, size: self.area.size }
    }
}

pub struct Widget {
    area: Rectangle,
}

impl Widget {
    pub fn new(area: Rectangle) -> Self {
        Widget { area }
    }
    pub fn area(&self) -> Rectangle {
        self.area
    }
}

pub trait View<D> where
    D: DrawTarget,
{
    /// Getter for area field in Struct
    fn area(&self) -> Rectangle;

    fn area_view(&self) -> Rectangle {
        Rectangle { top_left: Point { x: 0, y: 0 }, size: self.area().size }
    }

    fn draw_view(&self, target: &mut DrawView<D>, reason: &Reason) -> Result<(),D::Error>;
    fn handle_tap_view(&mut self, point: Point);

    fn draw(&self, target: &mut D, reason: &Reason) -> Result<(),D::Error> {
        let mut window_target = DrawView::new(self.area(), target);
        self.draw_view(&mut window_target, reason)
    }

	fn handle_tap(&mut self, point: Point) {
        if self.area().contains(point) {
            let point_offsetted = Point::new(point.x - self.area().top_left.x, point.y - self.area().top_left.y);
            self.handle_tap_view(point_offsetted);
        }
    }
}

pub trait ViewScreen<D> where
    D: DrawTarget,
{
    fn draw_screen(&self, target: &mut D, reason: &Reason) -> Result<EventResult, D::Error>;
    fn handle_tap_screen(&mut self, point: Point) -> EventResult;
}