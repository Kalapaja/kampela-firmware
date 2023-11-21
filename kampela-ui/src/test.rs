#[cfg(not(feature="std"))]
use alloc::{vec::Vec};
#[cfg(feature="std")]
use std::{vec::Vec};

use embedded_graphics::{
    mono_font::{
        ascii::{FONT_10X20},
        MonoTextStyle,
    },
    primitives::Rectangle,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    geometry::{Size},
};

use crate::{display_def::*, uistate::UpdateRequest};
use crate::uistate::EventResult;
use crate::widget::{view::{View, DrawWindow}, button::{Button}};

pub struct Test {
    area: Rectangle,
    button: Button,
    state: bool,
}

impl Test {
    pub fn new() -> Self {
        Test {
            area: Rectangle {
                top_left: Point { x:  132, y: 88 },
                size: Size { width: 132, height: 88 },
            },
            button: Button::new(
                "hello",
                Rectangle {
                    top_left: Point { x: 0, y: 0 },
                    size: Size { width: 66, height: 44 },
                },
            ),
            state: false
        }

    }
    pub fn onclick(&mut self) -> () {
        self.state = !self.state;
    }
}

impl View for Test {
    fn area(&self) -> Rectangle {
        self.area
    }
    fn draw_view<D>(&self, target: &mut DrawWindow<D>) -> Result<(), D::Error>
    where
        D: DrawTarget
    {
        target.draw_view(self.button);
        Ok(())
    }
    fn handle_tap_view<D>(&mut self, point: Point, target: &mut DrawWindow<D>) -> Result<EventResult, D::Error>
    where
        D: DrawTarget
    {
        let mut res = Ok(EventResult{request: UpdateRequest::new(), state: None});
        if let Some(a) = self.button.handle_tap(point, target) {
            res = a;
        };
        res
    }
}
