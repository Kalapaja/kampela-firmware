use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Pixel, Dimensions},
    primitives::{Rectangle, Primitive},
    geometry::{Size},
};
use rand::{Rng};

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

#[derive(Debug)]
pub struct Widget {
    area: Rectangle,
    absolut_top_left: Point,
}

impl Widget {
    pub fn new(area: Rectangle, parent_top_left: Point) -> Self {
        Self {
            area,
            absolut_top_left: Point {
                x: area.top_left.x + parent_top_left.x,
                y: area.top_left.y + parent_top_left.y,
            }
        }
    }
    pub fn bounding_box_absolut(&self) -> Rectangle {
        Rectangle::new(self.absolut_top_left, self.area.size)
    }
}

impl Dimensions for Widget {
    fn bounding_box(&self) -> Rectangle {
        self.area
    }
}

pub trait View {
    type DrawInput<'a> where Self: 'a;
    type TapInput<'a>;
    type TapOutput;
    /// Getter for area field in Struct
    fn bounding_box(&self) -> Rectangle;

    /// Getter for area field in Struct
    fn bounding_box_absolut(&self) -> Rectangle;

    fn bounding_box_view(&self) -> Rectangle {
        Rectangle { top_left: Point { x: 0, y: 0 }, size: self.bounding_box().size }
    }

    fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, reason: &Reason, input: Self::DrawInput<'a>) -> Result<(),D::Error>
    where 
        D: DrawTarget<Color = BinaryColor>;

    fn handle_tap_view<'a>(&mut self, point: Point, input: Self::TapInput<'a>) -> Self::TapOutput;

    fn draw<'a, D>(&mut self, target: &mut D, reason: &Reason, input: Self::DrawInput<'a>) -> Result<(),D::Error>
    where
        D: DrawTarget<Color = BinaryColor>
    {
        let mut window_target = DrawView::new(self.bounding_box(), target);
        self.draw_view(&mut window_target, reason, input)
    }

	fn handle_tap<'a>(&mut self, point: Point, input: Self::TapInput<'a>) -> Option<Self::TapOutput> {
        if self.bounding_box().contains(point) {
            let point_offsetted = Point::new(point.x - self.bounding_box().top_left.x, point.y - self.bounding_box().top_left.y);
            Some(self.handle_tap_view(point_offsetted, input))
        } else {
            None
        }
    }
}

pub trait ViewScreen {
    type DrawInput<'a> where Self: 'a;
    type TapInput<'a>;
    type TapOutput;
    fn draw_screen<'a, D>(&mut self, target: &mut D, reason: &Reason, input: Self::DrawInput<'a>) -> Result<EventResult, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>;
    fn handle_tap_screen<'a>(&mut self, point: Point, input: Self::TapInput<'a>) -> (EventResult, Self::TapOutput);
}