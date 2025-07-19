use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Drawable, Dimensions, DrawTarget, Point, Size},
    primitives::{Primitive, Rectangle, PrimitiveStyle},
};
use crate::{display_def::*, uistate::Event, widget::view::{DrawView, View, Widget}};

use crate::widget::nav_bar::nav_button::NavButton;

pub const NAV_BAR_SIZE: Size = Size{
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
    width: 96,
    height: NAV_BAR_SIZE.height,
};
pub const LEFT_KEY_WIDGET: Widget = Widget::new(
    Rectangle{
        top_left: Point{
            x: 0,
            y: 0,
        },
        size: NAV_KEY_SIZE,
    },
    NAV_BAR_WIDGET.top_left_absolute()
);
pub const NRIGHT_KEY_WIDGET: Widget = Widget::new(
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
    Left,
    Right,
}

pub struct NavBar {
    left: NavButton,
    right: NavButton,
}

impl NavBar {
    pub fn new((left_label, right_label): (&'static str, &'static str)) -> Self {
        NavBar{
            left: NavButton::new(left_label, &LEFT_KEY_WIDGET),
            right: NavButton::new(right_label, &NRIGHT_KEY_WIDGET),
        }
    }
    pub fn replace_labels(&mut self, (left_label, right_label): (&'static str, &'static str)) -> (&'static str, &'static str) {
        (self.left.replace_label(left_label), self.right.replace_label(right_label))
    }
    pub fn get_labels(&self) -> (&'static str, &'static str) {
        (self.left.get_label(), self.right.get_label())
    }
}

impl View for NavBar {
    type DrawInput<'a> = bool;
    type DrawOutput = ();
    type EventInput<'a> = ();
    type TapOutput = Option<NavCommand>;

    fn bounding_box(&self) -> Rectangle {
        NAV_BAR_WIDGET.bounding_box()
    }

    fn bounding_box_absolut(&self) -> Rectangle {
        NAV_BAR_WIDGET.bounding_box_absolute()
    }

    fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, n: Self::DrawInput<'_>) -> Result<Self::DrawOutput,D::Error>
        where 
            D: DrawTarget<Color = BinaryColor>,
            Self: 'a,
        {
        let (_, off) = if n {
            (BinaryColor::Off, BinaryColor::On)
        } else {
            (BinaryColor::On, BinaryColor::Off)
        };
        let filled = PrimitiveStyle::with_fill(off);
        self.bounding_box_view().into_styled(filled).draw(target)?;
        self.left.draw(target, n)?;
        self.right.draw(target, n)?;
        Ok(())
    }

    fn handle_event_view<'a>(&mut self, event: Event, _: ()) -> Self::TapOutput
    where Self: 'a
    {
        if self.left.handle_event(event, ()) == Some(true) {
            return Some(NavCommand::Left)
        };
        if self.right.handle_event(event, ()) == Some(true) {
            return Some(NavCommand::Right)
        };
        None
    }
}