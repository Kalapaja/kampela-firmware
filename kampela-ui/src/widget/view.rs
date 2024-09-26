#[cfg(not(feature="std"))]
use alloc::borrow::ToOwned;
#[cfg(feature="std")]
use std::borrow::ToOwned;

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Pixel, Dimensions},
    primitives::Rectangle,
};

use crate::uistate::EventResult;

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
#[derive(Clone, Copy)]
pub struct Widget {
    pub bounds: Rectangle,
    pub absolute_top_left: Point,
}

impl Widget {
    pub const fn new(bounds: Rectangle, parent_absolut_top_left: Point) -> Self {
        Widget{
            bounds,
            absolute_top_left: Point {
                x: bounds.top_left.x + parent_absolut_top_left.x,
                y: bounds.top_left.y + parent_absolut_top_left.y,
            }
        }
    }
    pub const fn zero() -> Self {
        Self::new(Rectangle::zero(), Point::zero())
    }
    pub const fn bounding_box_absolute(&self) -> Rectangle {
        Rectangle::new(self.absolute_top_left, self.bounds.size)
    }
    pub const fn top_left_absolute(&self) -> Point {
        self.absolute_top_left
    }
}

impl Dimensions for Widget {
    fn bounding_box(&self) -> Rectangle {
        self.bounds.to_owned()
    }
}

pub trait View {
    type DrawInput<'a> where Self: 'a;
    type DrawOutput;
    type TapInput<'a> where Self: 'a;
    type TapOutput;
    /// Getter for area field in Struct
    fn bounding_box(&self) -> Rectangle;

    /// Getter for area field in Struct
    fn bounding_box_absolut(&self) -> Rectangle;

    fn bounding_box_view(&self) -> Rectangle {
        Rectangle { top_left: Point { x: 0, y: 0 }, size: self.bounding_box().size }
    }

    fn bounding_box_relative_to(&self, relative_to: &impl View) -> Rectangle {
        Rectangle {
            top_left: Point{
                x: self.bounding_box_absolut().top_left.x - relative_to.bounding_box_absolut().top_left.x,
                y: self.bounding_box_absolut().top_left.y - relative_to.bounding_box_absolut().top_left.y,
            },
            size: self.bounding_box().size
        }
    }

    fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, input: Self::DrawInput<'a>) -> Result<Self::DrawOutput,D::Error>
    where 
        D: DrawTarget<Color = BinaryColor>,
        Self: 'a;

    fn handle_tap_view<'a>(&mut self, point: Point, input: Self::TapInput<'a>) -> Self::TapOutput
    where Self: 'a;

    fn draw<'a, D>(&mut self, target: &mut D, input: Self::DrawInput<'a>) -> Result<Self::DrawOutput,D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        Self: 'a,
    {
        let mut window_target = DrawView::new(self.bounding_box(), target);
        self.draw_view(&mut window_target, input)
    }

	fn handle_tap<'a>(&mut self, point: Point, input: Self::TapInput<'a>) -> Option<Self::TapOutput>
    where Self: 'a {
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
    type DrawOutput;
    type TapInput<'a> where Self: 'a;
    type TapOutput;
    fn draw_screen<'a, D>(&mut self, target: &mut D, input: Self::DrawInput<'a>) -> Result<(EventResult, Self::DrawOutput), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        Self: 'a;
    fn handle_tap_screen<'a>(&mut self, point: Point, input: Self::TapInput<'a>) -> (EventResult, Self::TapOutput) 
    where Self: 'a;
}