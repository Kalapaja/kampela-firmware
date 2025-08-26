
use alloc::vec::Vec;
use bitvec::prelude::{BitArr, Msb0, bitarr};
use efm32pg23_fix::Peripherals;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    pixelcolor::BinaryColor,
    primitives::rectangle::Rectangle,
    prelude::Drawable,
    Pixel,
};

use kampela_display_common::display_def::*;
use qrcodegen_no_heap::{QrCode, QrCodeEcc, Version};

use crate::{
    devices::{display::{EPDInit, PrepareSend}, display_transmission::{epaper_deep_sleep, epaper_hw_init_cs}, power::{voltage, wait_for_energy}}, in_free, parallel::{AsyncOperation, Threads}, peripherals::{ldma::LdmaCh, ldma_ch_usart::{FrameBufferLDMA, LDMAchUSART0, TransmittableUSART}}
};

use crate::debug_display::epaper_draw_stuff_differently;

pub type Bounds = (u8, u8, u16, u16);

pub trait BoundsTrait {
    fn fullscreen() -> Self;

    fn from_rectangle(refreshable_area: Rectangle) -> Self;
    fn x_start_bytes(&self) -> u8;
    fn x_end_bytes(&self) -> u8;
    fn y_start_address(&self) -> u16;
    fn y_end_address(&self) -> u16;
    fn y_start(&self) -> usize;
    fn y_end(&self) -> usize;
    fn width_bytes(&self) -> u8;
    fn height(&self) -> u16;
    fn total_bytes(&self) -> u32;
    fn start_bytes(&self) -> usize;
}

impl BoundsTrait for Bounds {
    fn fullscreen() -> Self {
        (0, SCREEN_SIZE_WIDTH_ADDRESS as u8 - 1, SCREEN_SIZE_X as u16 - 1, 0)
    }
// x and y of framebuffer and display RAM address are inversed
    fn from_rectangle(refreshable_area: Rectangle) -> Self {
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

        let bottom_right = refreshable_area.top_left + refreshable_area.size - Point{x: 1, y: 1};
        
        let x_end_address: u8 = if bottom_right.y > (SCREEN_SIZE_Y - 1) as i32 {
            (SCREEN_SIZE_Y / 8 - 1) as u8
        } else if bottom_right.y < 0 {
            0
        } else {
            (bottom_right.y / 8) as u8
        };

        let y_end_address: u16 = if bottom_right.x > (SCREEN_SIZE_X - 1) as i32 {
            0
        } else if bottom_right.x < 0 {
            (SCREEN_SIZE_X - 1) as u16
        } else {
            ((SCREEN_SIZE_X - 1) as i32 - bottom_right.x) as u16
        };

        (x_start_address, x_end_address, y_start_address, y_end_address)
    }

    fn x_start_bytes(&self) -> u8 {
        self.0
    }
    fn x_end_bytes(&self) -> u8 {
        self.1
    }
    fn y_start_address(&self) -> u16 {
        self.2
    }
    fn y_end_address(&self) -> u16 {
        self.3
    }
    fn y_start(&self) -> usize {
        (SCREEN_SIZE_X - 1) as usize - self.y_start_address() as usize
    }
    fn y_end(&self) -> usize {
        (SCREEN_SIZE_X - 1) as usize - self.y_end_address() as usize
    }
    fn width_bytes(&self) -> u8 {
        self.x_end_bytes() - self.x_start_bytes() + 1
    }
    fn height(&self) -> u16 {
        self.y_start_address() - self.y_end_address() + 1
    }
    fn total_bytes(&self) -> u32 {
        self.width_bytes() as u32 * self.height() as u32
    }
    fn start_bytes(&self) -> usize {
        self.y_start() * SCREEN_SIZE_WIDTH_ADDRESS + self.x_start_bytes() as usize
    }
}

#[derive(Debug)]
pub enum DisplayError {}

/// These are voltage thresholds to allow screen updates;
/// for wired debug, set both well below 5000
///
//TODO tune these values for prod; something like 12k and 8k
const FAST_REFRESH_POWER: i32 = 8000;
const FULL_REFRESH_POWER: i32 = 12000;
const PART_REFRESH_POWER: i32 = 6000;

const SEQUENCIAL_SELECTIVE_LIMIT: usize = 5; // more sequencial selective refreshes cause to leave traces, less cause artefacts
/// Virtual display data storage
type PixelBufferData = BitArr!(for SCREEN_RESOLUTION as usize, in u8, Msb0);
#[repr(C)]
pub struct PixelBuffer(PixelBufferData);

impl PixelBuffer {
    pub fn new_white() -> Self {
        Self(bitarr!(u8, Msb0; 1; SCREEN_RESOLUTION as usize))
    }

    /// Send display data to real EPD; invokes full screen refresh
    ///
    /// this is for cs environment; do not use otherwise
    pub fn apply(&self, peripherals: &mut Peripherals) {
        epaper_draw_stuff_differently(peripherals, self.0.into_inner());
    }
}

impl Dimensions for PixelBuffer {
    fn bounding_box(&self) -> Rectangle {
            Rectangle {
                top_left: SCREEN_ZERO,
                size: SCREEN_SIZE,
            }
    }
}

impl DrawTarget for PixelBuffer {
    type Color = BinaryColor;
    type Error = DisplayError;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            if (pixel.0.x<0)|(pixel.0.x>=SCREEN_SIZE_X as i32) { continue }
            if (pixel.0.y<0)|(pixel.0.y>=SCREEN_SIZE_Y as i32) { continue }
            //transposing pizels correctly here
            let n = (pixel.0.y + pixel.0.x*SCREEN_SIZE_Y as i32) as usize;
            let mut pixel_update = self.get_mut(n).expect("checked the bounds");
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

impl core::ops::Deref for PixelBuffer {
    type Target = PixelBufferData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for PixelBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub enum DisplayMode {
    Full,
    Fast,
    UltraFast,
    UltraFastSelective
}

pub struct UpdateMode {
    display_mode: DisplayMode,
    bounds: Vec<Bounds>,
    tap_response: bool
}

impl UpdateMode {
    pub fn new(display_mode: DisplayMode, bounds: Vec<Bounds>, tap_response: bool) -> Self {
        Self {
            display_mode,
            bounds,
            tap_response
        }
    }
    pub fn get_bounds(&self, index: usize) -> Option<Bounds> {
        self.bounds.get(index).cloned()
    }
    pub fn bounds_len(&self) -> usize {
        self.bounds.len()
    }
    pub fn get_display_mode(&self) -> &DisplayMode {
        &self.display_mode
    }
    pub fn set_display_mode(&mut self, display_mode: DisplayMode) {
        self.display_mode = display_mode
    }
}

pub trait UpdateModeMutate {
    fn propagate(&mut self, new_request: Self);
}

impl UpdateModeMutate for Option<UpdateMode> {
    fn propagate(&mut self, new_request: Self) {
        if let Some(r) = new_request {
            self.replace(r);
        }
    }
}

pub enum UpdateState {
    Idle,
    Updating,
    RenderingUpdating(UpdateMode),
    Rendering(UpdateMode),
    RenderedUpdating(UpdateMode),
    Send(UpdateMode),
    UpdateRequest(UpdateMode)
}

impl Default for UpdateState {
    fn default() -> Self {
        UpdateState::Idle
    }
}

type SelectiveCounter = usize;
trait SelectiveCounterTrait {
    fn is_selective_count(&mut self) -> bool;
}

impl SelectiveCounterTrait for SelectiveCounter {
    fn is_selective_count(&mut self) -> bool {
        if *self < SEQUENCIAL_SELECTIVE_LIMIT {
            *self += 1;
            true
        } else {
            *self = 0;
            false
        }
    }
}
/// A virtual display that could be written to EPD simultaneously
pub struct FrameBuffer {
    data: Option<FrameBufferLDMA>,
    display_send_state: Option<DisplaySendState>,
    update_state: UpdateState,
    pending_update: Option<UpdateMode>,
    selective_counter: SelectiveCounter
}

impl FrameBuffer {
    /// Create new virtual display and fill it with ON pixels
    pub fn new_white() -> Self {
        Self {
            data: Some(FrameBufferLDMA::new()),
            display_send_state: None,
            update_state: UpdateState::Idle,
            pending_update: None,
            selective_counter: 0
        }
    }
    
    pub fn propagate(&mut self, new_update_request: Option<UpdateMode>) {
        self.pending_update.propagate(new_update_request);
    }
    pub fn update_is_pending(&mut self) -> bool {
        self.pending_update.is_some()
    }
    // trying to display latest possible request
    pub fn has_request(&mut self) -> bool {
        if let Some(u) = self.pending_update.take() {
            match self.update_state {
                UpdateState::Updating => {
                    self.update_state = UpdateState::RenderingUpdating(u);
                    true
                },
                UpdateState::RenderedUpdating(ref m) => {
                    if !m.tap_response { // do not rerender if the last was tap response
                        self.update_state = UpdateState::RenderingUpdating(u);
                        true
                    } else {
                        false
                    }
                },
                UpdateState::Idle => {
                    self.update_state = UpdateState::Rendering(u);
                    true
                },
                _ => {
                    self.pending_update = Some(u); // set back if not used
                    false
                }
            }
        } else {
            false
        }
    }

    pub fn end_render(&mut self) -> bool {
        match core::mem::take(&mut self.update_state) {
            UpdateState::RenderingUpdating(m) => {
                self.update_state = UpdateState::RenderedUpdating(m);
            },
            UpdateState::Rendering(m) => {
                self.update_state = UpdateState::Send(m);
                return true
            }
            _ => {}
        }
        false
    }

    pub fn can_send(&mut self) -> bool {
        match &self.update_state {
            UpdateState::Send(m) => {
                self.display_send_state = Some(DisplaySendState::new(m));
                true
            },
            _ => {
                false
            }
        }
    }

    pub fn send_is_tap_response(&self) -> bool {
        match &self.update_state {
            UpdateState::Send(m) => {
                return m.tap_response
            },
            _ => false
        }
    }

    pub fn end_send(&mut self) {
        match core::mem::take(&mut self.update_state) {
            UpdateState::Send(m) => {
                self.update_state = UpdateState::UpdateRequest(m)
            },
            _ => {}
        }
    }

    pub fn has_update_request(&mut self) -> Option<UpdateMode> {
        match &self.update_state {
            UpdateState::UpdateRequest(m) => {
                match m.get_display_mode() {
                    DisplayMode::Full => {
                        if voltage() < FULL_REFRESH_POWER {
                            in_free(|peripherals| {
                                epaper_deep_sleep(peripherals);
                            });
                            wait_for_energy(FULL_REFRESH_POWER as u16);
                            in_free(|peripherals| {
                                epaper_hw_init_cs(peripherals);
                            });
                        }
                    },
                    DisplayMode::Fast => {
                        if voltage() < FAST_REFRESH_POWER {
                            in_free(|peripherals| {
                                epaper_deep_sleep(peripherals);
                            });
                            wait_for_energy(FAST_REFRESH_POWER as u16);
                            in_free(|peripherals| {
                                epaper_hw_init_cs(peripherals);
                            });
                        }
                    },
                    DisplayMode::UltraFast |
                    DisplayMode::UltraFastSelective => {
                        if voltage() < PART_REFRESH_POWER {
                            in_free(|peripherals| {
                                epaper_deep_sleep(peripherals);
                            });
                            wait_for_energy(PART_REFRESH_POWER as u16);
                            in_free(|peripherals| {
                                epaper_hw_init_cs(peripherals);
                            });
                        }
                    }
                };
            },
            _ => {}
        };
        match core::mem::take(&mut self.update_state) {
            UpdateState::UpdateRequest(mut m) => {
                match m.get_display_mode() {
                    DisplayMode::UltraFastSelective => {
                        if !self.selective_counter.is_selective_count() {
                            m.set_display_mode(DisplayMode::UltraFast);
                        }
                    },
                    _ => {}
                };
                self.update_state = UpdateState::Updating;
                Some(m)
            },
            _ => None
        }
    }

    pub fn end_update(&mut self) {
        match core::mem::take(&mut self.update_state) {
            UpdateState::RenderingUpdating(m) => {
                self.update_state = UpdateState::Rendering(m);
            },
            UpdateState::RenderedUpdating(m) => {
                self.update_state = UpdateState::Send(m);
            },
            UpdateState::Updating |
            UpdateState::UpdateRequest(_) => {
                self.update_state = UpdateState::Idle;
            },
            _ => {}
        }
    }

    pub fn is_idle(&self) -> bool {
        matches!(self.update_state, UpdateState::Idle)
    }
    
}

impl Dimensions for FrameBuffer {
    fn bounding_box(&self) -> Rectangle {
        let data = self.data.as_ref().expect("FrameBuffer data should return from static cell");
        data.bounding_box()
    }
}

impl DrawTarget for FrameBuffer {
    type Color = BinaryColor;
    type Error = DisplayError;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let data = self.data.as_mut().expect("FrameBuffer data should return from static cell");
        data.draw_iter(pixels)
    }
}

struct DisplaySendState {
    pub state: Threads<DisplaySendStateEnum, 1>,
    send_area_index: usize,
}

impl DisplaySendState {
    fn new(update_mode: &UpdateMode) -> Self {
        DisplaySendState {
            state: Threads::new(DisplaySendStateEnum::Init(None)),
            send_area_index: update_mode.bounds.len(),
        }
    }

    fn advance<'a>(&mut self, data: &mut Option<FrameBufferLDMA>, m: &UpdateMode) -> Option<bool> {
        match self.state.turn() {
            DisplaySendStateEnum::Init(state) => {
                match state {
                    None => {
                        *state = Some(EPDInit::new(()));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.state.change(DisplaySendStateEnum::PrepareSend(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            DisplaySendStateEnum::PrepareSend(state) => {
                match state {
                    None => {
                        let bounds =  m.get_bounds(m.bounds.len() - self.send_area_index);
                        *state = Some(PrepareSend::new(bounds))
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.state.change(DisplaySendStateEnum::DisplaySend(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            DisplaySendStateEnum::DisplaySend(state) => {
                match state {
                    None => {
                        let mut frame_buffer = data.take().expect("FrameBuffer shouldn't be in static cell");
                        let bounds = if let Some(bounds) = m.get_bounds(m.bounds.len() - self.send_area_index) {
                            bounds
                        } else {
                            Bounds::fullscreen()
                        };
                        frame_buffer.set_bounds(bounds);
                        LDMAchUSART0::set_static_cell(Some(TransmittableUSART::Display(frame_buffer)));
                        *state = Some(());
                    },
                    Some(_) => {
                        if !LDMAchUSART0::done() {
                            return None
                        }
                        match LDMAchUSART0::take_static_cell() {
                            Some(TransmittableUSART::Display(a)) => {
                                *data = Some(a)
                            },
                            _ => {unreachable!("FrameBuffer should be in static cell")}
                        }
                        if self.send_area_index <= 1 {
                            self.state.change(DisplaySendStateEnum::End);
                            return Some(true)
                        } else {
                            self.send_area_index -= 1;
                            self.state.change(DisplaySendStateEnum::PrepareSend(None));
                        }
                    }
                }
                Some(false)
            },
            DisplaySendStateEnum::End => {
                return Some(true)
            }
        }
    }
}

enum DisplaySendStateEnum {
    Init(Option<EPDInit>),
    PrepareSend(Option<PrepareSend>),
    DisplaySend(Option<()>),
    End
}

impl Default for DisplaySendStateEnum {
    fn default() -> Self { DisplaySendStateEnum::End }
}

impl AsyncOperation for FrameBuffer {
    type Init = ();
    type Input<'a> = ();
    type Output = Option<bool>;

    fn new(_: Self::Init) -> Self {
        Self::new_white()
    }

    fn advance<'a>(&mut self, _: Self::Input<'a>) -> Self::Output {
        match &self.update_state {
            UpdateState::Send(m) => {
                self.display_send_state.as_mut().expect("send possibility checked").advance(&mut self.data, m)
            },
            _ => { unreachable!("send possibility checked") }
        }
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

    let mut buffer = PixelBuffer::new_white();

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
