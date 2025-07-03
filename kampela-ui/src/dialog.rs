#[cfg(not(feature="std"))]
use alloc::boxed::Box;
#[cfg(feature="std")]
use std::boxed::Box;

use embedded_graphics::{
    mono_font::{
        ascii::FONT_10X20,
        MonoTextStyle,
    },
    prelude::{Point, Size, DrawTarget},
    primitives::{Primitive, Rectangle, PrimitiveStyle},
    Drawable,
    pixelcolor::BinaryColor,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use crate::{uistate::EventResult, widget::view::View};
use crate::widget::{view::{Widget, ViewScreen}, nav_bar::nav_bar::{NavBar, NavCommand, NAV_BAR_WIDGET}};

use crate::display_def::*;

const HEADER_WIDGET: Widget = Widget::new(
    Rectangle{
        top_left: Point{
            x: 0,
            y: 0,
        },
        size: Size{
            width: SCREEN_SIZE_X,
            height: SCREEN_SIZE_Y - NAV_BAR_WIDGET.bounds.size.height,
        }
    },
    SCREEN_ZERO
);

pub struct Dialog {
    navbar: NavBar,
    routes: Option<(Box<dyn FnOnce() -> EventResult>, Box<dyn FnOnce() -> EventResult>)>,
    message: &'static str,
    negative: bool,
}

impl Dialog {
    pub fn new(
        message: &'static str,
        options: (&'static str, &'static str),
        routes: (Box<dyn FnOnce() -> EventResult>, Box<dyn FnOnce() -> EventResult>),
        negative: bool,
    ) -> Self {
        Dialog{
            navbar: NavBar::new(options),
            routes: Some(routes),
            message,
            negative,
        }
    }
}

impl ViewScreen for Dialog {
    type DrawInput<'a> = ();
    type DrawOutput = ();
    type TapInput<'a> = ();
    type TapOutput = ();

    
    fn draw_screen<'a, D>(&mut self, target: &mut D, _: ()) -> Result<(EventResult, Self::DrawOutput), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        Self: 'a,
    {
        let request = None;
        let state = None;
        
        let (on, off) = if self.negative {
            (BinaryColor::Off, BinaryColor::On)
        } else {
            (BinaryColor::On, BinaryColor::Off)
        };

        let filled = PrimitiveStyle::with_fill(off);
        let character_style = MonoTextStyle::new(&FONT_10X20, on);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        HEADER_WIDGET.bounds.into_styled(filled).draw(target)?;
        TextBox::with_textbox_style(
            &self.message,
            HEADER_WIDGET.bounds,
            character_style,
            textbox_style,
        )
        .draw(target)?;

        self.navbar.draw(target, self.negative)?;

        Ok((EventResult { request, state }, ()))
    }
    fn handle_tap_screen<'a>(&mut self, point: Point, _: ()) -> (EventResult, ())
    where
        Self: 'a
    {
        let event_result = if let Some(Some(c)) = self.navbar.handle_tap(point, ()) {
            let routes = core::mem::take(&mut self.routes).unwrap();
            match c {
                NavCommand::Left => {
                    routes.0()
                },
                NavCommand::Right => {
                    routes.1()
                }
            }
        } else {
            EventResult{request: None, state: None}
        };

        (event_result, ())
    }
}