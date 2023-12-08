use core::cell::RefCell;
#[cfg(not(feature="std"))]
use alloc::{rc::Rc, vec::Vec};
#[cfg(feature="std")]
use std::{rc::Rc, vec::Vec};

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

use crate::{display_def::*, uistate::UpdateRequest, widget::view::ViewScreen};
use crate::uistate::EventResult;
use crate::widget::{view::{View, DrawView}, button::Button};

pub struct Test {
    button: Button,
    state: bool
}

impl Test {
    pub fn new() -> Self {
        Test {
            button: Button::new(
                "hello",
                Rectangle {
                    top_left: Point { x: 0, y: 0 },
                    size: Size { width: 66, height: 44 },
                }
            ),
            state: false,
        }
    }
}

pub trait StateOutput {
    fn get_state(&self) -> bool;
}

impl <D: DrawTarget<Color = BinaryColor>> ViewScreen<D> for Test {
    fn draw_screen(&self, target: &mut D) -> Result<(), D::Error> {
        self.button
        .draw(target)?;
        Ok(())
    }
    fn handle_tap_screen(&mut self, point: Point, target: &mut D) -> Result<EventResult, D::Error> {
        let mut request = UpdateRequest::new();
        let state = None;
        self.button.handle_tap(point, target)?;
        self.state = self.button.get_state();
        request.set_both(); //only screen has authority to set update speed
        Ok(EventResult { request, state })
    }
}