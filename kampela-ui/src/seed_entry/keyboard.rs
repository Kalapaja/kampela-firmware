#[cfg(not(feature="std"))]
use alloc::vec::Vec;
#[cfg(feature="std")]
use std::vec::Vec;

use core::{array, cmp::min};

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, Point, Size},
    primitives::{PointsIter, Rectangle},
};
use crate::{display_def::*, uistate::Event, widget::view::{DrawView, View, Widget}};

use crate::seed_entry::key::Key;

use crate::widget::nav_bar::nav_bar::NAV_BAR_WIDGET;

const TAP_RADIUS_SQUARED: i32 = 169;

const KEYBOARD_HEIGHT: u32 = 84;
pub const KEYBOARD_AREA: Rectangle = Rectangle{
    top_left: Point{
        x: 0,
        y: NAV_BAR_WIDGET.bounds.top_left.y - KEYBOARD_HEIGHT as i32,
    },
    size: Size{width: SCREEN_SIZE_X, height: KEYBOARD_HEIGHT}
};
const KEYBOARD_WIDGET: Widget = Widget::new(KEYBOARD_AREA, SCREEN_ZERO);

const KEY_ROWS: [usize; 3] = [10, 9, 7];
const QWERTY: [&str; 26] = [
    "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P",
    "A", "S", "D", "F", "G", "H", "J", "K", "L",
    "Z", "X", "C", "V", "B", "N", "M",
];
const PADDING_LEFT: u32 = 2;
const KEY_SIZE: Size = Size{
    width: (KEYBOARD_AREA.size.width - PADDING_LEFT * 2) / 10,
    height: KEYBOARD_AREA.size.height / 3,
};

const fn get_keys_widgets<const N: usize>() -> [Widget; N] {
    let mut i = 0;
    let mut widgets = [Widget::zero(); N];
    while i < N {
        let mut col = i;
        let mut row = 0;
        let mut r = 0;
        while r < KEY_ROWS.len() {
            if !(col as i32 - KEY_ROWS[r] as i32).is_negative() {
                col = col - KEY_ROWS[r];
            } else {
                row = r;
                break
            }
            r = r + 1;
        }
        let centering = (KEY_ROWS[0] - KEY_ROWS[row]) as u32 * KEY_SIZE.width / 2;
        let top_left = Point{
            x: (PADDING_LEFT + centering + KEY_SIZE.width * col as u32) as i32,
            y: (KEY_SIZE.height * row as u32) as i32,
        };
        widgets[i] = Widget::new(
            Rectangle::new(
                top_left,
                KEY_SIZE,
            ),
            KEYBOARD_WIDGET.top_left_absolute()
        );
        i = i + 1;
    }
    widgets
}

const KEY_WIDGETS: [Widget; 26] = get_keys_widgets::<26>();

const fn get_remove_widget() -> Widget {
    let relative_widget = Widget::new(
        Rectangle{
            top_left: Point{
                x: KEY_WIDGETS[25].bounds.top_left.x + KEY_WIDGETS[25].bounds.size.width as i32,
                y: KEY_WIDGETS[25].bounds.top_left.y,
            },
            size: Size{
                width: (KEY_ROWS[0] - KEY_ROWS[2]) as u32 * KEY_SIZE.width / 2,
                height: KEY_SIZE.height,
            },
        },
        KEYBOARD_WIDGET.top_left_absolute(),
    );
    Widget::new(
        Rectangle{
            top_left: relative_widget.top_left_absolute(),
            size: relative_widget.bounds.size,
        },
        SCREEN_ZERO,
    )
}
pub const REMOVE_KEY_WIDGET: Widget = get_remove_widget();

pub struct Keyboard {
    keys: [Key; 26],
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            keys: array::from_fn(|i| {
                Key::new(&QWERTY[i], &KEY_WIDGETS[i])
            }),
        }
    }
}

impl View for Keyboard {
    type DrawInput<'a> = bool;
    type DrawOutput = Option<Rectangle>;
    type EventInput<'a> = ();
    type TapOutput = Option<(Vec<char>, Rectangle)>;

    fn bounding_box(&self) -> Rectangle {
        KEYBOARD_WIDGET.bounding_box()
    }

    fn bounding_box_absolut(&self) -> Rectangle {
        KEYBOARD_WIDGET.bounding_box_absolute()
    }

    fn draw_view<'a, D>(&mut self, target: &mut DrawView<D>, n: Self::DrawInput<'_>) -> Result<Self::DrawOutput,D::Error>
        where 
            D: DrawTarget<Color = BinaryColor>,
            Self: 'a,
        {
        let mut was_tapped = None;
        for key in self.keys.iter_mut() {
            if key.draw(target, n)? {
                was_tapped = Some(key.bounding_box_absolut());
            }
        }
        Ok(was_tapped)
    }

    fn handle_event_view<'a>(&mut self, event: Event, _: ()) -> Self::TapOutput
        where Self: 'a
    {
        let mut nearest = Vec::new();
        let mut rectangle = SCREEN_AREA;
        if let Event::Tap(point) = event {
            for key in self.keys.iter_mut() {
                if key.handle_event(event, ()).unwrap_or(None).is_some() {
                    rectangle = key.bounding_box_absolut();
                }
                    
                let b = &key.bounding_box();
                
                //calculating square(to avoid sqrt) of distance to edge or vertex of bounding box
                let mut l = i32::max_value();
                let horizontally_contained = b.columns().contains(&point.x);
                let vertically_contained = b.rows().contains(&point.y);
                if vertically_contained && horizontally_contained {
                    l = 0;
                }
                if vertically_contained && !horizontally_contained {
                    l = min((b.columns().start - point.x).pow(2), (b.columns().end - 1 - point.x).pow(2));
                }
                if horizontally_contained && !vertically_contained {
                    l = min((b.rows().start - point.y).pow(2), (b.rows().end - 1 - point.y).pow(2));
                }
                if !vertically_contained && !horizontally_contained {
                    l = b.points()
                        .map(|vertex| {
                            (vertex.x - point.x).pow(2) + (vertex.y - point.y).pow(2)
                        }).min().unwrap();
                };
    
                if l < TAP_RADIUS_SQUARED {
                    nearest.push((key.get_char(), l));
                }
            }
        }
        nearest.sort_by_key(|k| k.1);
        if nearest.is_empty() || nearest[0].1 != 0 {
            //if neither key is pressed
            None
        } else {
            Some((nearest.iter_mut().map(|k| k.0).collect(), rectangle))
        }
    }
}