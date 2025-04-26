use embedded_graphics_core::{geometry::{Point, Size}, primitives::Rectangle};
pub const SCREEN_SIZE_X: u32 = 264;
pub const SCREEN_SIZE_Y: u32 = 176;
pub const SCREEN_RESOLUTION: u32 = SCREEN_SIZE_X * SCREEN_SIZE_Y;
pub const SCREEN_BIT_DEPTH: usize = 1;
pub const SCREEN_BIT_SIZE: usize = SCREEN_RESOLUTION as usize * SCREEN_BIT_DEPTH;
pub const SCREEN_BUFFER_SIZE: usize = SCREEN_BIT_SIZE / 8;
pub const SCREEN_SIZE_WIDTH_ADDRESS: usize = (SCREEN_SIZE_Y / 8) as usize;
pub const SCREEN_SIZE: Size = Size{width: SCREEN_SIZE_X, height: SCREEN_SIZE_Y};
pub const SCREEN_ZERO: Point = Point{x: 0, y: 0};
pub const SCREEN_AREA: Rectangle = Rectangle{top_left: SCREEN_ZERO, size: SCREEN_SIZE};

