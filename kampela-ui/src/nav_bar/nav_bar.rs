use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, Point, Size},
    primitives::Rectangle,
};
use crate::{display_def::*, widget::view::{DrawView, View, Widget}};

use crate::nav_bar::nav_button::NavButton;

const NAV_BAR_SIZE: Size = Size{
    width: SCREEN_SIZE_X,
    height: 32,
};
pub const NAV_BAR_WIDGET: Widget = Widget::new(
    Rectangle{
        top_left: Point{
            x: 0,
            y: SCREEN_SIZE_Y as i32 - NAV_BAR_SIZE.height as i32,
        },
        size: NAV_BAR_SIZE
    },
    SCREEN_ZERO //should be on screen_widget
);
const NAV_KEY_SIZE: Size = Size{
    width: 64,
    height: NAV_BAR_SIZE.height,
};
const BACK_KEY_WIDGET: Widget = Widget::new(
    Rectangle{
        top_left: Point{
            x: 0,
            y: 0,
        },
        size: NAV_KEY_SIZE,
    },
    NAV_BAR_WIDGET.top_left_absolute()
);
const NEXT_KEY_WIDGET: Widget = Widget::new(
    Rectangle{
        top_left: Point{
            x: NAV_BAR_SIZE.width as i32 - NAV_KEY_SIZE.width as i32,
            y: 0,
        },
        size: NAV_KEY_SIZE
    },
    NAV_BAR_WIDGET.top_left_absolute()
);

pub enum NavCommand {
    Back,
    Next,
}

pub struct NavBar {
    back: NavButton,
    next: NavButton,
}

impl NavBar {
    pub fn new() -> Self {
        NavBar{
            back: NavButton::new("back", &BACK_KEY_WIDGET),
            next: NavButton::new("next", &NEXT_KEY_WIDGET),
        }
    }
}

impl View for NavBar {
    type DrawInput<'a> = bool;
    type DrawOutput = ();
    type TapInput<'a> = ();
    type TapOutput = Option<NavCommand>;

    fn bounding_box(&self) -> Rectangle {
        NAV_BAR_WIDGET.bounding_box()
    }

    fn bounding_box_absolut(&self) -> Rectangle {
        NAV_BAR_WIDGET.bounding_box_absolute()
    }

    fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, t: Self::DrawInput<'_>) -> Result<Self::DrawOutput,D::Error>
        where 
            D: DrawTarget<Color = BinaryColor> {
        self.back.draw(target, t)?;
        self.next.draw(target, t)?;
        Ok(())
    }

    fn handle_tap_view<'a>(&mut self, point: Point, _: ()) -> Self::TapOutput {
        if self.back.handle_tap(point, ()).is_some() {
            return Some(NavCommand::Back)
        };
        if self.next.handle_tap(point, ()).is_some() {
            return Some(NavCommand::Next)
        };
        None
    }
}