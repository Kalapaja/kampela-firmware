#[cfg(not(feature="std"))]
use alloc::{vec::Vec};
#[cfg(feature="std")]
use std::{vec::Vec};

use embedded_graphics::{
    Drawable,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Pixel, Dimensions},
    primitives::{Rectangle, Primitive, PrimitiveStyle, Styled},
    geometry::{Size}, text::{renderer::{TextRenderer, CharacterStyle}, self, Text},
    pixelcolor::Rgb888,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    TextBox,
    plugin::{PluginMarker as Plugin},
};

use crate::uistate::{EventResult, UpdateRequest};

pub struct DrawWindow<'a, D> where
    D: DrawTarget
{
    area: Rectangle,
    origin: &'a mut D,
}

impl <'a, D: DrawTarget> DrawWindow<'a, D> {
    fn new(area: Rectangle, target: &'a mut D) -> Self {
        DrawWindow { area, origin: target }
    }
}

impl <'a, D: DrawTarget> DrawTarget for DrawWindow<'a, D> {
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

impl <'a, D: DrawTarget> Dimensions for DrawWindow<'a, D> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle { top_left: Point { x: 0, y: 0 }, size: self.area.size }
    }
}

impl <'a, D> DrawWindow<'a, D> where
    D: DrawTarget
{
    pub fn draw_styled<T>(&mut self, primitive: Styled<T, PrimitiveStyle<D::Color>>)
    where
        T: Primitive,
    {
        primitive.draw(self.origin);
    }

    pub fn draw_textbox<F, M>(&mut self, textbox: TextBox<'a, F, M>)
    where
        F: TextRenderer<Color = <F as CharacterStyle>::Color> + CharacterStyle,
        <F as CharacterStyle>::Color: From<Rgb888>,
        M: Plugin<'a, <F as TextRenderer>::Color> + Plugin<'a, <F as CharacterStyle>::Color>,
    {
        textbox.draw(self.origin); // No idea what type is required
    }

    pub fn draw_view(&mut self, view: View) {
        view.draw(self.origin);
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

pub trait View {
    /// Getter for area field in Struct
    fn area(&self) -> Rectangle;

    fn area_view(&self) -> Rectangle {
        Rectangle { top_left: Point { x: 0, y: 0 }, size: self.area().size }
    }

    fn draw_view<D>(&self, target: &mut DrawWindow<D>) -> Result<(),D::Error> where D: DrawTarget;

    fn handle_tap_view<D>(&mut self, point: Point, target: &mut DrawWindow<D>) -> Result<EventResult, D::Error> where D: DrawTarget;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget,
    {
        let mut window_target = DrawWindow::new(self.area(), target);
        self.draw_view(&mut window_target)
    }

	fn handle_tap<D>(&mut self, point: Point, target: &mut D) -> Option<Result<EventResult, D::Error>>
    where
        D: DrawTarget
    {
        if self.area().contains(point) {
            let point_offsetted = Point::new(point.x - self.area().top_left.x, point.y - self.area().top_left.y);
            let mut window_target = DrawWindow::new(self.area(), target);
            Some(self.handle_tap_view(point_offsetted, &mut window_target));
        }
        None
    }
}
