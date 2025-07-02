//! display control functions
use crate::draw::{Bounds, BoundsTrait, DisplayMode, UpdateMode};
use crate::peripherals::ldma::LdmaCh;
use crate::peripherals::ldma_ch_usart::{LDMAchUSART0, DisplayRamCopy, StaticArrayLDMA, TransmittableUSART};
use crate::peripherals::ldma_ch_usart_rx::{LDMAchUSART0Rx, ReceivableUSART};
use crate::peripherals::usart::*;
use crate::peripherals::gpio_pins::{display_res_clear, display_res_set};
use crate::in_free;
use crate::parallel::{AsyncOperation, Threads, Timer, DELAY};
use crate::devices::display_transmission::{display_is_busy, EPDCommand, EPDData};
use kampela_ui::display_def::*;

const LUT_LEN: usize = 0x99;
const FULL_LUT: [u8; LUT_LEN] = [
    0xA0, 0x90, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 0
    0x50, 0x60, 0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 1
    0xA0, 0x90, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 2
    0x50, 0x60, 0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 3
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 4
//  TPA   TPB   SRAB  TPC   TPD   SRCD  RP
    0x30, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0A, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x06,
    0x06, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    
    0x77, 0x00, 0x00, 0x00, 0x00, 0x00, // FR
    0x00, 0x00, 0x00,                   // XON
];
const FAST_LUT: [u8; LUT_LEN] = [
    0xA0, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 0
    0x50, 0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 1
    0xA0, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 2
    0x50, 0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 3
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 4
//  TPA   TPB   SRAB  TPC   TPD   SRCD  RP
    0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    
    0x77, 0x00, 0x00, 0x00, 0x00, 0x00, // FR
    0x00, 0x00, 0x00,                   // XON
];
const ULTRAFAST_LUT: [u8; LUT_LEN] = [
    0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 0
    0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 1
    0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 2
    0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 3
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 4
//  TPA   TPB   SRAB  TPC   TPD   SRCD  RP
    0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    
    0x70, 0x00, 0x00, 0x00, 0x00, 0x00, // FR
    0x00, 0x00, 0x00,                   // XON
];
const ULTRAFAST_SELECTIVE_LUT: [u8; LUT_LEN] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 0
    0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 1
    0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 2
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 3
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 4
//  TPA   TPB   SRAB  TPC   TPD   SRCD  RP
    0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    
    0x70, 0x00, 0x00, 0x00, 0x00, 0x00, // FR
    0x00, 0x00, 0x00,                   // XON
];

/// Draw sequence
///
/// Iterate through this to perform drawing and send display to proper sleep mode
pub struct Request {
    threads: Threads<RequestState, 1>,
    update_mode: UpdateMode,
    send_area_index: usize,
}

enum RequestState {
    Update(Option<Update>),
    PrepareSend(Option<PrepareSend>),
    PostUpdate(Option<()>),
    DeepSleepEnter(Option<EPDDeepSleepEnter>),
    End,
    Error,
}

impl Default for RequestState {
    fn default() -> Self { RequestState::Error }
}

impl AsyncOperation for Request {
    type Init = UpdateMode;
    type Input<'a> = ();
    type Output = Option<Option<bool>>;

    fn new(update_mode: Self::Init) -> Self {
        let send_area_index = update_mode.bounds_len();
        Self {
            threads: Threads::new(RequestState::Update(None)),
            update_mode,
            send_area_index
        }
    }

    fn advance(&mut self, _: Self::Input<'_>) -> Self::Output {
        match self.threads.turn() {
            RequestState::Update(state) => {
                match state {
                    None => {
                        self.threads.change(RequestState::Update(Some(Update::new(()))));
                    },
                    Some(a) => {
                        match a.advance(&self.update_mode) {
                            Some(true) => {
                                if display_is_busy() {
                                    return Some(Some(false))
                                }
                                self.threads.change(RequestState::PrepareSend(None));
                            },
                            Some(false) => {
                                return Some(None)
                            },
                            None => return None
                        }
                    }
                }
                Some(None)
            },
            RequestState::PrepareSend(state) => {
                match state {
                    None => {
                        let bounds =  self.update_mode.get_bounds(self.update_mode.bounds_len() - self.send_area_index);
                        *state = Some(PrepareSend::new(bounds))
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(RequestState::PostUpdate(None));
                            },
                            _ => ()
                        };
                    }
                }
                Some(None)
            },
            RequestState::PostUpdate(state) => {
                match state {
                    None => {
                        let bounds = if let Some(bounds) = self.update_mode.get_bounds(self.update_mode.bounds_len() - self.send_area_index) {
                            bounds
                        } else {
                            Bounds::fullscreen()
                        };
                        LDMAchUSART0Rx::set_static_cell( 
                            Some(
                                ReceivableUSART::DisplayRamCopy(
                                    DisplayRamCopy::new(bounds)
                                )
                            )
                        );
                        *state = Some(());
                    },
                    Some(_) => {
                        if !LDMAchUSART0Rx::done() {
                            return Some(Some(false))
                        }
                        LDMAchUSART0Rx::take_static_cell();
                        if self.send_area_index <= 1 {
                            self.threads.change(RequestState::DeepSleepEnter(None));
                        } else {
                            self.send_area_index -= 1;
                            self.threads.change(RequestState::PrepareSend(None));
                        }
                    }
                }
                Some(None)
            },
            RequestState::DeepSleepEnter(state) => {
                match state {
                    None => {
                        *state = Some(EPDDeepSleepEnter::new(()));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(RequestState::End);
                                return Some(Some(true))
                            },
                            Some(false) => {
                                return Some(None)
                            },
                            None => return None
                        }
                    }
                }
                Some(None)
            },
            RequestState::End => {
                Some(Some(true))
            },
            RequestState::Error => {
                panic!("Unknown RequestState while display")
            }
        }
    }
}




/// EPD init to wake up display
pub struct EPDInit {
    threads: Threads<EPDInitState, 1>,
}

enum EPDInitState {
    ResSet(Option<Timer>),
    ResClr(Option<Timer>),
    WakeUp(Option<EPDCommand<0x12>>),
    End,
    Error,
}

impl Default for EPDInitState {
    fn default() -> Self { EPDInitState::Error }
}

impl AsyncOperation for EPDInit {
    type Init = ();
    type Input<'a> = ();
    type Output = Option<bool>;

    fn new(_: ()) -> Self {
        Self {
            threads: Threads::new(EPDInitState::ResSet(None)),
        }
    }

    fn advance(&mut self, _: ()) -> Self::Output {
        match self.threads.turn() {
            EPDInitState::ResSet(state) => {
                match state {
                    None => {
                        in_free(|peripherals| {
                            deselect_display(&mut peripherals.gpio_s);
                            display_res_set(&mut peripherals.gpio_s);
                        });
                        self.threads.change(EPDInitState::ResSet(Some(Timer::new(DELAY))));
                    },
                    Some(t) => {
                        if t.tick() {
                            return None
                        }
                        self.threads.change(EPDInitState::ResClr(None));
                    }
                }
                Some(false)
            },
            EPDInitState::ResClr(state) => {
                match state {
                    None => {
                        in_free(|peripherals| {
                            select_display(&mut peripherals.gpio_s);
                            display_res_clear(&mut peripherals.gpio_s);
                        });
                        self.threads.change(EPDInitState::ResClr(Some(Timer::new(DELAY))));
                    },
                    Some(t) => {
                        if t.tick() {
                            return None
                        }
                        self.threads.change(EPDInitState::WakeUp(None));
                    }
                }
                Some(false)
            },
            EPDInitState::WakeUp(state) => {
                match state {
                    None => {
                        self.threads.change(EPDInitState::WakeUp(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                if display_is_busy() {
                                    return None
                                }
                                self.threads.change(EPDInitState::End);
                                return Some(true)
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            EPDInitState::End => {
                Some(true)
            }
            EPDInitState::Error => {
                panic!("Unknown EPDInitState while display")
            },
        }
    }
}

/// Set EPD to sleep saving ram data
struct EPDDeepSleepEnter {
    threads: Threads<EPDDeepSleepEnterState, 1>,
}

enum EPDDeepSleepEnterState {
    DeepSleepMode(Option<EPDCommand<0x10>>),
    // Retain RAM data
    EnterDeepSleepMode1(Option<EPDData<1>>),
    End,
    Error,
}

impl Default for EPDDeepSleepEnterState {
    fn default() -> Self { EPDDeepSleepEnterState::Error }
}

impl AsyncOperation for EPDDeepSleepEnter {
    type Init = ();
    type Input<'a> = ();
    type Output = Option<bool>;

    fn new(_: ()) -> Self {
        Self {
            threads: Threads::new(EPDDeepSleepEnterState::DeepSleepMode(None)),
        }
    }

    fn advance(&mut self, _: ()) -> Self::Output {
        match self.threads.turn() {
            EPDDeepSleepEnterState::DeepSleepMode(state) => {
                match state {
                    None => {
                        self.threads.change(EPDDeepSleepEnterState::DeepSleepMode(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(EPDDeepSleepEnterState::EnterDeepSleepMode1(None));
                            },
                            r => return r
                        }
                    }
                }
                Some(false)
            },
            EPDDeepSleepEnterState::EnterDeepSleepMode1(state) => {
                match state {
                    None => {
                        self.threads.change(EPDDeepSleepEnterState::EnterDeepSleepMode1(Some(EPDData::new(()))));
                    },
                    Some(a) => {
                        match a.advance(&[0x01]) {
                            Some(true) => {
                                in_free(|peripherals| {
                                    deselect_display(&mut peripherals.gpio_s);
                                });
                                self.threads.change(EPDDeepSleepEnterState::End);
                                return Some(true)
                            },
                            r => return r
                        }
                    }
                }
                Some(false)
            },
            EPDDeepSleepEnterState::End => {
                Some(true)
            }
            EPDDeepSleepEnterState::Error => {
                panic!("Unknown EPDInitState while display")
            },
        }
    }
}

pub struct PrepareSend {
    threads: Threads<PrepareDrawState, 1>,
    bounds: Option<Bounds>,
}

enum PrepareDrawState {
    //Set RAM X address start/end postition (which is Y due to orientation)
    SetRamXAddress(Option<EPDCommand<0x44>>),
    RamXStartEnd(Option<(EPDData<2>, [u8; 2])>),
    //Set RAM Y address start/end postition (which is X due to orientation)
    SetRamYAddress(Option<EPDCommand<0x45>>),
    RamYStartEnd(Option<(EPDData<4>, [u8; 4])>),
    //Set RAM X&Y address write starting position
    SetRamXAddressCounter(Option<EPDCommand<0x4E>>),
    RamXAddressCounter(Option<(EPDData<1>, [u8; 1])>),
    SetRamYAddressCounter(Option<EPDCommand<0x4F>>),
    RamYAddressCounter(Option<(EPDData<2>, [u8; 2])>),

    End,
    Error,
}

impl Default for PrepareDrawState {
    fn default() -> Self { PrepareDrawState::Error }
}

impl AsyncOperation for PrepareSend {
    type Init = Option<Bounds>;
    type Input<'a> = ();
    type Output = Option<bool>;

    fn new(bounds: Self::Init) -> Self {
        let init_state = match bounds {
            //skip bounds addresses transmission
            None => {
                PrepareDrawState::SetRamXAddressCounter(None)
            },
            Some(_) => {
                PrepareDrawState::SetRamXAddress(None)
            }
        };
        Self {
            threads: Threads::new(init_state),
            bounds,
        }
    }

    fn advance(&mut self, _: Self::Input<'_>) -> Self::Output {
        match self.threads.turn() {
            PrepareDrawState::SetRamXAddress(state) => {
                match state {
                    None => {
                        self.threads.change(PrepareDrawState::SetRamXAddress(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(PrepareDrawState::RamXStartEnd(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            PrepareDrawState::RamXStartEnd(state) => {
                match state {
                    None => {
                        let mut start_end = [0; 2];
                        if let Some(b) = self.bounds {
                            start_end[0..1].copy_from_slice(&(b.0).to_le_bytes());
                            start_end[1..2].copy_from_slice(&(b.1).to_le_bytes());
                        }
                        
                        self.threads.change(PrepareDrawState::RamXStartEnd(
                            Some(
                                (
                                    EPDData::new(()),
                                    start_end
                                )
                            )
                        ));
                    },
                    Some((a, b)) => {
                        match a.advance(&b) {
                            Some(true) => {
                                self.threads.change(PrepareDrawState::SetRamYAddress(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            PrepareDrawState::SetRamYAddress(state) => {
                match state {
                    None => {
                        self.threads.change(PrepareDrawState::SetRamYAddress(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(PrepareDrawState::RamYStartEnd(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            PrepareDrawState::RamYStartEnd(state) => {
                match state {
                    None => {
                        let mut start_end = [0; 4];
                        if let Some(b) = self.bounds {
                            start_end[0..2].copy_from_slice(&(b.2).to_le_bytes());
                            start_end[2..4].copy_from_slice(&(b.3).to_le_bytes());
                        }
                        
                        self.threads.change(PrepareDrawState::RamYStartEnd(
                            Some(
                                (
                                    EPDData::new(()),
                                    start_end
                                )
                            )
                        ));
                    },
                    Some((a, b)) => {
                        match a.advance(&b) {
                            Some(true) => {
                                self.threads.change(PrepareDrawState::SetRamXAddressCounter(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            PrepareDrawState::SetRamXAddressCounter(state) => {
                match state {
                    None => {
                        self.threads.change(PrepareDrawState::SetRamXAddressCounter(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(PrepareDrawState::RamXAddressCounter(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            PrepareDrawState::RamXAddressCounter(state) => {
                match state {
                    None => {
                        let x = match self.bounds {
                            None => [0],
                            Some(b) => b.0.to_le_bytes()
                        };
                        self.threads.change(PrepareDrawState::RamXAddressCounter(Some((EPDData::new(()), x))));
                    },
                    Some((a, x)) => {
                        match a.advance(x) {
                            Some(true) => {
                                self.threads.change(PrepareDrawState::SetRamYAddressCounter(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            PrepareDrawState::SetRamYAddressCounter(state) => {
                match state {
                    None => {
                        self.threads.change(PrepareDrawState::SetRamYAddressCounter(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(PrepareDrawState::RamYAddressCounter(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            PrepareDrawState::RamYAddressCounter(state) => {
                match state {
                    None => {
                        let y = match self.bounds {
                            None => ((SCREEN_SIZE_X - 1) as u16).to_le_bytes(),
                            Some(b) => b.2.to_le_bytes()
                        };
                        self.threads.change(PrepareDrawState::RamYAddressCounter(Some((EPDData::new(()), y))));
                    },
                    Some((a, y)) => {
                        match a.advance(y) {
                            Some(true) => {
                                self.threads.change(PrepareDrawState::End);
                                return Some(true)
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            PrepareDrawState::End => {
                Some(true)
            },
            PrepareDrawState::Error => {
                panic!("Unknown PrepareDrawState while display")
            }
        }
    }
}

pub struct Update {
    threads: Threads<UpdateState, 1>,
}

enum UpdateState {
    // Load custom LUT
    WtiteLUTRegister(Option<EPDCommand<0x32>>),
    CustomLUTData(Option<()>),
    // Display with mode 1
    DisplayUpdateControl2(Option<EPDCommand<0x22>>),
    DisplayMode1NoLoadLUT(Option<EPDData<1>>),

    MasterActivation(Option<EPDCommand<0x20>>),

    End,
    Error,
}

impl Default for UpdateState {
    fn default() -> Self { UpdateState::Error }
}

impl AsyncOperation for Update {
    type Init = ();
    type Input<'a> = &'a UpdateMode;
    type Output = Option<bool>;

    fn new(_: Self::Init) -> Self {
        Self {
            threads: Threads::new(UpdateState::WtiteLUTRegister(None)),
        }
    }

    fn advance(&mut self, update_mode: Self::Input<'_>) -> Self::Output {
        match self.threads.turn() {
            UpdateState::WtiteLUTRegister(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateState::WtiteLUTRegister(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(UpdateState::CustomLUTData(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            UpdateState::CustomLUTData(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateState::CustomLUTData(Some(())));
                        LDMAchUSART0::set_static_cell( 
                            Some(
                                TransmittableUSART::Array(
                                    StaticArrayLDMA::new( {
                                        match update_mode.get_display_mode() {
                                            DisplayMode::Full => &FULL_LUT,
                                            DisplayMode::Fast => &FAST_LUT,
                                            DisplayMode::UltraFast => &ULTRAFAST_LUT,
                                            DisplayMode::UltraFastSelective => &ULTRAFAST_SELECTIVE_LUT
                                        }
                                    })
                                )
                            )
                        )
                    },
                    Some(_) => {
                        if !LDMAchUSART0::done() {
                            return None
                        }
                        LDMAchUSART0::take_static_cell();
                        self.threads.change(UpdateState::DisplayUpdateControl2(None));
                    }
                }
                Some(false)
            },
            UpdateState::DisplayUpdateControl2(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateState::DisplayUpdateControl2(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(UpdateState::DisplayMode1NoLoadLUT(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            UpdateState::DisplayMode1NoLoadLUT(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateState::DisplayMode1NoLoadLUT(Some(EPDData::new(()))));
                    },
                    Some(a) => {
                        match a.advance(&[0xC7]) {
                            Some(true) => {
                                self.threads.change(UpdateState::MasterActivation(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            UpdateState::MasterActivation(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateState::MasterActivation(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(UpdateState::End);
                                return Some(true)
                            },
                            r => return r
                        }
                    }
                }
                Some(false)
            },
            UpdateState::End => {
                Some(true)
            },
            UpdateState::Error => {
                panic!("Unknown UpdateUltraFastState while display")
            }
        }
    }
}