#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
#![deny(unused_crate_dependencies)]

extern crate alloc;
extern crate core;

use alloc::format;
use core::{alloc::Layout, panic::PanicInfo};
use cortex_m::interrupt::free;
use cortex_m_rt::entry;
use efm32pg23_fix::Peripherals;
use embedded_alloc::Heap;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Point, Primitive},
    primitives::{Circle, PrimitiveStyle},
    Drawable,
};
use kampela_display_common::display_def::SCREEN_SIZE_X;
use kampela_system::{
    devices::{
        se_rng::SeRng,
        touch::{Read, FT6X36_REG_NUM_TOUCHES, LEN_NUM_TOUCHES},
    },
    draw::{burning_tank, FrameBuffer},
    parallel::Operation,
    peripherals::{cmu::init_cmu, gpio_pins::init_gpio, i2c::init_i2c, usart::init_usart},
    PERIPHERALS,
};
use kolibri::uistate::UIState;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[alloc_error_handler]
fn oom(layout: Layout) -> ! {
    panic!("Out of memory! {layout:?}")
}

#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    let mut peripherals = unsafe { Peripherals::steal() };
    burning_tank(&mut peripherals, format!("{:?}", panic));
    loop {}
}

/// Initialize peripherals for screen calibration
pub fn init_peripherals_calibration(peripherals: &mut Peripherals) {
    // first, start clocking
    init_cmu(&mut peripherals.CMU_S);

    // map GPIO pins to their functions and set their starting values
    init_gpio(&mut peripherals.GPIO_S);

    // Setting up USART0, for epaper display and flash memory
    init_usart(peripherals);

    // set up i2c line to communicate with touch pad
    init_i2c(peripherals);
}

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 16384;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let mut peripherals = Peripherals::take().unwrap();
    init_peripherals_calibration(&mut peripherals);

    free(|cs| {
        PERIPHERALS.borrow(cs).replace(Some(peripherals));
    });

    let mut do_update = true;
    let mut state = UIState::init(&mut SeRng {});

    let mut reader = Read::<LEN_NUM_TOUCHES, FT6X36_REG_NUM_TOUCHES>::new();

    let measured_affine = loop {
        if do_update {
            let mut frame_buffer = FrameBuffer::new_white();
            state.render(&mut frame_buffer).unwrap();
            frame_buffer.request_full();
            while !frame_buffer.advance(20000i32) {}
            do_update = false;
        } else if let Some(touch_data) = reader.advance(()).unwrap() {
            if touch_data[0] == 1 {
                if let UIState::Complete(a) = state {
                    break a;
                } else {
                    let detected_y =
                        ((touch_data[1] as u16 & 0b00001111) << 8) | touch_data[2] as u16;
                    let detected_x =
                        ((touch_data[3] as u16 & 0b00001111) << 8) | touch_data[4] as u16;
                    let point = Point {
                        x: SCREEN_SIZE_X as i32 - detected_x as i32,
                        y: detected_y as i32,
                    };
                    do_update = state.process_touch(point, &mut SeRng {}).unwrap();
                }
            }
        }
    };

    let mut reader = Read::<LEN_NUM_TOUCHES, FT6X36_REG_NUM_TOUCHES>::new();

    loop {
        if let Some(touch_data) = reader.advance(()).unwrap() {
            if touch_data[0] == 1 {
                let detected_y = ((touch_data[1] as u16 & 0b00001111) << 8) | touch_data[2] as u16;
                let detected_x = ((touch_data[3] as u16 & 0b00001111) << 8) | touch_data[4] as u16;
                let point = Point {
                    x: SCREEN_SIZE_X as i32 - detected_x as i32,
                    y: detected_y as i32,
                };
                let point_on_display = measured_affine.transform(&point);
                let mut frame_buffer = FrameBuffer::new_white();
                Circle::with_center(point_on_display, 20)
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                    .draw(&mut frame_buffer)
                    .unwrap();
                frame_buffer.request_full();
                while !frame_buffer.advance(20000i32) {}
            }
        }
    }
}
