use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Drawable, Dimensions, DrawTarget, Point, Size},
    primitives::{Primitive, Rectangle, PrimitiveStyle},
};
use crate::{display_def::*, widget::view::{DrawView, View, Widget}};

use crate::widget::nav_bar::nav_button::NavButton;

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

    fn handle_tap_view<'a>(&mut self, point: Point, _: ()) -> Self::TapOutput
    where Self: 'a
    {
        if self.left.handle_tap(point, ()).is_some() {
            return Some(NavCommand::Left)
        };
        if self.right.handle_tap(point, ()).is_some() {
            return Some(NavCommand::Right)
        };
        None
    }
}