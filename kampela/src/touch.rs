use core::cell::RefCell;

use alloc::collections::vec_deque::VecDeque;
use cortex_m::interrupt::{free, Mutex};
use kampela_system::peripherals::timers::Timer2;
use nalgebra::{Affine2, OMatrix, Point2, RowVector3};
use lazy_static::lazy_static;
use embedded_graphics::prelude::Point;

use efm32pg23_fix::interrupt;
use kampela_system::devices::touch::{clear_touch_if, is_touch_int, Read, FT6X36_REG_NUM_TOUCHES, LEN_NUM_TOUCHES};
use kampela_system::parallel::AsyncOperation;
use kampela_ui::display_def::*;

pub const MAX_TOUCH_QUEUE: usize = 2;

lazy_static! {
    static ref TOUCH_READER: Mutex<RefCell<Read<LEN_NUM_TOUCHES, FT6X36_REG_NUM_TOUCHES>>> = Mutex::new(RefCell::new(Read::new(())));
    pub static ref TOUCHES: Mutex<RefCell<Touches>> = Mutex::new(RefCell::new(Touches::new()));
}

#[interrupt]
fn GPIO_EVEN() {
    if is_touch_int() {
        clear_touch_if();
        free(|cs| {
            let mut touch = TOUCH_READER.borrow(cs).borrow_mut();
            *touch = Read::new(());
        });
        Timer2::load(0);
    };
}

#[interrupt]
fn TIMER2() {
    Timer2::reset_if_ien();

    free(|cs| {
        let mut touch = TOUCH_READER.borrow(cs).borrow_mut();
        loop {
            match touch.advance(()).unwrap() {
                None => { Timer2::load(1); break },
                Some(None) => continue,
                Some(Some(touch_data)) => {
                    let mut touches = TOUCHES.borrow(cs).borrow_mut();
                    touches.try_push_touch_data(touch_data);
                    break
                }
            }
        }
    })
}

lazy_static! {
    // MAGIC calibration numbers obtained through KOLIBRI tool
    static ref AFFINE_MATRIX: Affine2<f32> = Affine2::from_matrix_unchecked(
        OMatrix::from_rows(&[
            RowVector3::<f32>::new(1.0022, -0.0216, -4.2725),
            RowVector3::<f32>::new(0.0061, 1.1433, -13.7305),
            RowVector3::<f32>::new(0.0, 0.0, 1.0),
        ])
    );
}

pub fn take_touch_point() -> Option<Point> {
    free(|cs| {
        let mut touchse = TOUCHES.borrow(cs).borrow_mut();
        touchse.take_touch_point()
    })
}

pub struct Touches(VecDeque<Point>);

impl Touches {
    pub fn new() -> Self {
        Self(VecDeque::with_capacity(MAX_TOUCH_QUEUE))
    }

    pub fn try_push_touch_data(&mut self, touch_data: [u8; LEN_NUM_TOUCHES]) -> bool {
        clear_touch_if();
        if self.0.len() < MAX_TOUCH_QUEUE {
            if let Some(point) = Self::convert(touch_data) {
                self.0.push_back(point);
                return true
            }
        }
        false
    }
    
    pub fn take_touch_point(&mut self) -> Option<Point> {
        self.0.pop_front()
    }

    fn convert(touch_data: [u8; LEN_NUM_TOUCHES]) -> Option<Point> {
        if touch_data[0] == 1 {
            let detected_y = (((touch_data[1] as u16 & 0b00001111) << 8) | touch_data[2] as u16) as i32;
            let detected_x = (((touch_data[3] as u16 & 0b00001111) << 8) | touch_data[4] as u16) as i32;
            let touch = Point::new(SCREEN_SIZE_X as i32 - detected_x, detected_y);
    
            let touch_as_point2 = Point2::new(touch.x as f32, touch.y as f32);
            let display_as_point2 = AFFINE_MATRIX.transform_point(&touch_as_point2);
    
            Some(
                Point {
                    x: display_as_point2.coords[0] as i32,
                    y: display_as_point2.coords[1] as i32,
                }
            )
        } else { None }
    }
}


/*
fn blocking_touch_read() -> Point {
    let mut state: Option<Read<LEN_NUM_TOUCHES, FT6X36_REG_NUM_TOUCHES>> = None;
    clear_touch_if();
    let touch_data = loop {
        match &mut state {
            None => {
                if is_touch_int() {
                    state = Some(Read::new(()));
                }
            },
            Some(reader) => {
                match reader.advance(()) {
                    Ok(Some(Some(touch))) => {
                        break touch;
                    },
                    Ok(Some(None)) => {},
                    Ok(None) => {}
                    Err(e) => panic!("{:?}", e),
                }
            }
        }
    };

    let detected_y = (((touch_data[1] as u16 & 0b00001111) << 8) | touch_data[2] as u16) as i32;
    let detected_x = (((touch_data[3] as u16 & 0b00001111) << 8) | touch_data[4] as u16) as i32;
    Point::new(SCREEN_SIZE_X as i32 - detected_x, detected_y)
}

pub fn kolibri_test() {
    // Prepare
    let mut display = FrameBuffer::new_white();

    let mut rng = se_rng::SeRng{};

    let mut state = UIState::init(&mut rng);

    let mut do_update = true;
    loop {
        if do_update {
            state.render(&mut display).unwrap();
            do_update = false;
        }
        in_free(|peripherals| {
            display.apply(peripherals);
        });

        let point = blocking_touch_read();
        do_update = state.process_touch(point, &mut rng).unwrap();
    }
}
*/