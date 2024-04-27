use bitvec::prelude::{BitArr, Msb0, bitarr};

use efm32pg23_fix::Peripherals;
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    pixelcolor::BinaryColor,
    primitives::rectangle::Rectangle,
    prelude::Drawable,
    Pixel,
};

use kampela_display_common::display_def::*;
use qrcodegen_noheap::{QrCode, QrCodeEcc, Version};

use crate::devices::display::{FastDraw, FullDraw, PartDraw, Request};
use crate::devices::display_transmission::{epaper_deep_sleep, epaper_hw_init_cs};
use crate::debug_display::{epaper_draw_stuff_differently};

const SCREEN_SIZE_VALUE: usize = (SCREEN_SIZE_X*SCREEN_SIZE_Y) as usize;

use crate::{in_free, parallel::Operation}; 

// x and y of framebuffer and display RAM address are inversed
fn refreshable_area_address(refreshable_area: Rectangle) -> (u8, u8, u16, u16) {
    let x_start_address: u8 = if refreshable_area.top_left.y < 0 {
        0
    } else if refreshable_area.top_left.y > (SCREEN_SIZE_Y - 1) as i32 {
        (SCREEN_SIZE_Y / 8 - 1) as u8
    } else {
        (refreshable_area.top_left.y / 8) as u8
    };

    let y_start_address: u16 = if refreshable_area.top_left.x < 0 {
        (SCREEN_SIZE_X - 1) as u16
    } else if refreshable_area.top_left.x > (SCREEN_SIZE_X - 1) as i32{
        0
    } else {
        ((SCREEN_SIZE_X - 1) as i32 - refreshable_area.top_left.x) as u16
    };

    let bottom_right = refreshable_area.top_left + refreshable_area.size;
    
    let x_end_address: u8 = if bottom_right.y > (SCREEN_SIZE_Y - 1) as i32 {
        (SCREEN_SIZE_Y / 8 - 1) as u8
    } else if bottom_right.y < 0 {
        0
    } else {
        ((bottom_right.y / 8 + (bottom_right.y % 8).signum()) - 1) as u8
    };

    let y_end_address: u16 = if bottom_right.x > (SCREEN_SIZE_X - 1) as i32 {
        0
    } else if bottom_right.x < 0 {
        (SCREEN_SIZE_X - 1) as u16
    } else {
        (SCREEN_SIZE_X as i32 - bottom_right.x) as u16
    };

    (x_start_address, x_end_address, y_start_address, y_end_address)
}

#[derive(Debug)]
pub enum DisplayError {}

/// These are voltage thresholds to allow screen updates;
/// for wired debug, set both well below 5000
///
//TODO tune these values for prod; something like 12k and 8k
const FAST_REFRESH_POWER: i32 = 5000;
const FULL_REFRESH_POWER: i32 = 5000;
const PART_REFRESH_POWER: i32 = 5000;

/// Virtual display data storage
type PixelData = BitArr!(for SCREEN_SIZE_VALUE, in u8, Msb0);


/// A virtual display that could be written to EPD simultaneously
pub struct FrameBuffer {
    data: PixelData,
    display_state: DisplayState,
    timer: usize,
}

impl FrameBuffer {
    /// Create new virtual display and fill it with ON pixels
    pub fn new_white() -> Self {
        Self {
            data: bitarr!(u8, Msb0; 1; SCREEN_SIZE_X as usize*SCREEN_SIZE_Y as usize),
            display_state: DisplayState::Idle,
            timer: 0,
        }
    }

    fn count(&mut self) -> bool {
        if self.timer == 0 {
            false
        } else {
            self.timer -= 1;
            true
        }
    }

    /// Send display data to real EPD; invokes full screen refresh
    ///
    /// this is for cs environment; do not use otherwise
    pub fn apply(&self, peripherals: &mut Peripherals) {
        epaper_draw_stuff_differently(peripherals, self.data.into_inner());
    }

    /// Start full display update sequence
    pub fn request_full(&mut self) {
        self.display_state = DisplayState::FullRequested;
    }

    /// Start partial fast display update sequence
    pub fn request_fast(&mut self) {
        self.display_state = DisplayState::FastRequested;
    }

    /// Start partial fast display update sequence
    pub fn request_part(&mut self, area: Rectangle) {
        self.display_state = DisplayState::PartRequested(area);
    }
}

/// Display's updating progress
///
/// This is intentionally done without typestates, as typesafety it offers is outweighted by
/// reallocations made in new item creation.
pub enum DisplayState {
    /// Initial state, where we can change framebuffer. If this was typestate, this would be Zero.
    Idle,
    /// Fast update was requested; waiting for power
    FastRequested,
    FastOperating(Request<FastDraw>),
    /// Slow update was requested; waiting for power
    FullRequested,
    FullOperating(Request<FullDraw>),
    /// Part update was requested; waiting for power
    PartRequested(Rectangle),
    PartOperating(Request<PartDraw>, (u8, u8, u16, u16)),
    /// Display not available due to update cycle
    UpdatingNow,
}

impl Operation for FrameBuffer {
    type Init = ();
    type Input<'a> = i32;
    type Output = Option<bool>;
    type StateEnum = DisplayState;

    fn new(_: ()) -> Self {
        Self::new_white()
    }

    fn wind(&mut self, state: DisplayState, delay: usize) {
        self.display_state = state;
        self.timer = delay;
    }

    /// Move through display update progress
    fn advance(&mut self, voltage: i32) -> Option<bool> {
        if self.count() { return None };

        match self.display_state {
            DisplayState::Idle => Some(true),
            DisplayState::FastRequested => {
                if voltage > FAST_REFRESH_POWER {        
                    self.display_state = DisplayState::FastOperating(Request::<FastDraw>::new(()));
                };
                None
            },
            DisplayState::FastOperating(ref mut a) => {
                if a.advance(&self.data.data) {
                    self.wind(DisplayState::UpdatingNow, 0)
                }
                Some(false)
            },
            DisplayState::FullRequested => {
                if voltage > FULL_REFRESH_POWER {
                    self.display_state = DisplayState::FullOperating(Request::<FullDraw>::new(()));
                };
                None
            },
            DisplayState::FullOperating(ref mut a) => {
                if a.advance(&self.data.data) {
                    self.wind(DisplayState::UpdatingNow, 0)
                }
                Some(false)
            },
            DisplayState::PartRequested(r) => {
                if voltage > PART_REFRESH_POWER {
                    let d: (u8, u8, u16, u16) = refreshable_area_address(r);
                    self.display_state = DisplayState::PartOperating(Request::<PartDraw>::new(()), d);
                };
                None
            },
            DisplayState::PartOperating(ref mut a, bounds) => {
                if a.advance((&self.data.data, bounds)) {
                    self.wind(DisplayState::UpdatingNow, 0)
                }
                Some(false)
            },
            DisplayState::UpdatingNow => {
                in_free(|peripherals| epaper_deep_sleep(peripherals));
                self.display_state = DisplayState::Idle;
                Some(false)
            },
        }
    }
}

impl Dimensions for FrameBuffer {
    fn bounding_box(&self) -> Rectangle {
            Rectangle {
                top_left: SCREEN_ZERO,
                size: SCREEN_SIZE,
            }
    }
}

// this was an experiment to find Y offset value in memory
//const SHIFT_COEFFICIENT: usize = (SCREEN_SIZE_Y * 7) as usize;

impl DrawTarget for FrameBuffer {
    type Color = BinaryColor;
    type Error = DisplayError;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            if (pixel.0.x<0)|(pixel.0.x>=SCREEN_SIZE_X as i32) {continue}
            if (pixel.0.y<0)|(pixel.0.y>=SCREEN_SIZE_Y as i32) {continue}
            //transposing pizels correctly here
            let n = (pixel.0.y + pixel.0.x*SCREEN_SIZE_Y as i32) /*(pixel.0.y*176 + (175 - pixel.0.x))*/ as usize;
            //let n = if n<SHIFT_COEFFICIENT { n + SCREEN_SIZE_VALUE - SHIFT_COEFFICIENT } else { n - SHIFT_COEFFICIENT };
            let mut pixel_update = self.data.get_mut(n).expect("checked the bounds");
            match pixel.1 {
                BinaryColor::Off => {
                    *pixel_update = true; //white
                },
                BinaryColor::On => {
                    *pixel_update = false; //black
                }
            }
        }
        Ok(())
    }
}

pub fn draw_qr(peripherals: &mut Peripherals, data_to_qr: &[u8]) {

    let len = data_to_qr.len();

    let mut outbuffer = [0u8; Version::new(18).buffer_len()].to_vec();
    let mut dataandtemp = [0u8; Version::new(18).buffer_len()].to_vec();
    
    dataandtemp[..len].copy_from_slice(data_to_qr);
    
    let qr_code = QrCode::encode_binary(&mut dataandtemp, len, &mut outbuffer, QrCodeEcc::Low, Version::MIN, Version::new(18), None, true).unwrap();

    let scaling = {
        if qr_code.version() == Version::new(18) {2}
        else {SCREEN_SIZE_Y as i32/qr_code.size()}
    };

    let mut buffer = FrameBuffer::new_white();

    let size = qr_code.size() * scaling;
    for y in 0..size {
        for x in 0..size {
            let color = {
                if qr_code.get_module(x / scaling, y / scaling) {BinaryColor::On}
                else {BinaryColor::Off}
            };
            let x_point = SCREEN_SIZE_X as i32/2 - size/2 + x;
            let y_point = SCREEN_SIZE_Y as i32/2 - size/2 + y;
            let point = Point::new(x_point, y_point);
            let pixel = Pixel::<BinaryColor>(point, color);
            pixel.draw(&mut buffer).unwrap();
        }
    }
    buffer.apply(peripherals);
}
