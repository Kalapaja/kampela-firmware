//! display control functions
use crate::peripherals::ldma_ch_usart::{FrameBufferLDMA, LDMAchUSART0, RamCopy, StaticArrayLDMA, Transmittable};
use crate::peripherals::ldma_ch_usart_rx::{LDMAchUSART0Rx, Receivable};
use crate::peripherals::usart::*;
use crate::peripherals::gpio_pins::{display_res_clear, display_res_set};
use crate::in_free;
use crate::parallel::{AsyncOperation, Threads, Timer, DELAY};
use crate::devices::display_transmission::{display_is_busy, EPDCommand, EPDData, EPDDataBuffer};
use kampela_ui::display_def::*;

use super::display_transmission::{epaper_write_command, epaper_write_data};
const LUT_LEN: usize = 0x99;
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

const REVERSED_LUT: [u8; LUT_LEN] = [
    0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 0
    0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 1
    0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT 2
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

pub type Bounds = (u8, u8, u16, u16);

/// Draw sequence
///
/// Iterate through this to perform drawing and send display to proper sleep mode
pub struct Request<R> where
    R: for <'a> RequestType<
        Init = <UpdateUltraFast as AsyncOperation>::Init,
        Input<'a> = (),
        Output = Option<bool>,
    > 
{
    threads: Threads<RequestState<R>, 1>,
    part_options: Option<Bounds>,
    selective_refresh: bool,
    repeats: usize
}

enum RequestState<R> where
    R: for <'a> RequestType<
        Init = <UpdateUltraFast as AsyncOperation>::Init,
        Input<'a> = (),
        Output = Option<bool>,
    > 
{
    Init(Option<EPDInit>),
    PrepareDraw(Option<PrepareDraw>),
    Update(Option<R>),
    PostUpdate(Option<()>),
    DeepSleepEnter(Option<EPDDeepSleepEnter>),
    End,
    Error,
}

impl<R> Default for RequestState<R> where
    R: for <'a> RequestType<
        Init = <UpdateUltraFast as AsyncOperation>::Init,
        Input<'a> = (),
        Output = Option<bool>,
    >
{
    fn default() -> Self { RequestState::<R>::Error }
}

impl<R> AsyncOperation for Request<R> where
    R: for <'a> RequestType<
        Init = <UpdateUltraFast as AsyncOperation>::Init,
        Input<'a> = (),
        Output = Option<bool>,
    >
{
    type Init = (Option<Bounds>, bool);
    type Input<'a> = <PrepareDraw as AsyncOperation>::Input<'a>;
    type Output = Option<Option<bool>>;

    fn new((part_options, selective_refresh): Self::Init) -> Self {
        Self {
            threads: Threads::new(RequestState::Init(None)),
            part_options,
            selective_refresh,
            repeats: 0,
        }
    }

    fn advance(&mut self, data: Self::Input<'_>) -> Self::Output {
        match self.threads.turn() {
            RequestState::Init(state) => {
                match state {
                    None => {
                        self.threads.change(RequestState::Init(Some(EPDInit::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(RequestState::PrepareDraw(None));
                            },
                            Some(false) => {
                                return Some(None)
                            },
                            None => return None
                        };
                    }
                }
                Some(None)
            },
            RequestState::PrepareDraw(state) => {
                match state {
                    None => {
                        self.threads.change(RequestState::PrepareDraw(
                            Some(
                                PrepareDraw::new(self.part_options)
                            )
                        ));
                    },
                    Some(a) => {
                        match a.advance(data) {
                            Some(true) => {
                                self.threads.change(RequestState::Update(None));
                                return Some(Some(false))
                            },
                            Some(false) => {
                                return Some(None)
                            },
                            None => return None
                        };
                    }
                }
                Some(None)
            },
            RequestState::Update(state) => {
                match state {
                    None => {
                        self.threads.change(RequestState::Update(Some(R::new(self.selective_refresh))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(RequestState::PostUpdate(None));
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
            RequestState::PostUpdate(state) => {
                match state {
                    None => {
                        let b = self.part_options.unwrap_or((0, SCREEN_SIZE_WIDTH_ADDRESS as u8 - 1, SCREEN_SIZE_X as u16 - 1, 0));
                        LDMAchUSART0Rx::set_static_cell( 
                            Some(
                                Receivable::RamCopy(
                                    RamCopy::new(b)
                                )
                            )
                        );
                        self.threads.change(RequestState::PostUpdate(Some(())));
                    },
                    Some(_) => {
                        if !LDMAchUSART0Rx::done() {
                            if self.repeats > 2000 {
                                let a = LDMAchUSART0Rx::take_static_cell();
                                match a {
                                    Some(Receivable::RamCopy(a)) => {
                                        panic!("{:x?}", a.buffer)
                                    },
                                    _ => ()
                                };
                                in_free(|peripherals| {
                                    let f = peripherals
                                        .usart0_s
                                        .status()
                                        .read()
                                        .rxdatav()
                                        .bit_is_set();
                                    let h = peripherals.usart0_s.if_().read().rxof().bit_is_set();
                                    panic!("data rxdatav:{f} rxof: {h}");
                                });
                            }
                            self.repeats += 1;
                            return None
                        }

                        /*
                        in_free(|peripherals| {
                            epaper_write_command(peripherals, &[0x32]);
                        });
                        LDMAchUSART0::set_static_cell( 
                            Some(
                                Transmittable::Array(
                                    StaticArrayLDMA::new(&REVERSED_LUT)
                                )
                            )
                        );
                        while !LDMAchUSART0::done() {}
                        LDMAchUSART0::take_static_cell();
                        in_free(|peripherals| {
                            epaper_write_command(peripherals, &[0x22]);
                            epaper_write_data(peripherals, &[0xC7]);
                            epaper_write_command(peripherals, &[0x20]);
                        });
                        while display_is_busy() {}
                        */
                        self.threads.change(RequestState::DeepSleepEnter(None));
                    }
                }
                Some(None)
            },
            RequestState::DeepSleepEnter(state) => {
                match state {
                    None => {
                        self.threads.change(RequestState::DeepSleepEnter(Some(EPDDeepSleepEnter::new(()))));
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
struct EPDInit {
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

pub trait RequestType: AsyncOperation {}
impl RequestType for UpdateFast {}
impl RequestType for UpdateFull {}
impl RequestType for UpdateUltraFast {}

pub struct PrepareDraw {
    threads: Threads<PrepareDrawState, 1>,
    bounds: <EPDDataBuffer::<SCREEN_BUFFER_SIZE> as AsyncOperation>::Init,
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
    //BorderWavefrom,
    BorderWaveformControl(Option<EPDCommand<0x3C>>),
    VBDasVCOM(Option<EPDData<1>>),

    WriteRamBlack(Option<EPDCommand<0x24>>),
    SendBufferData(Option<()>),

    End,
    Error,
}

impl Default for PrepareDrawState {
    fn default() -> Self { PrepareDrawState::Error }
}

impl AsyncOperation for PrepareDraw {
    type Init = <EPDDataBuffer::<SCREEN_BUFFER_SIZE> as AsyncOperation>::Init;
    type Input<'a> = &'a mut Option<FrameBufferLDMA>;
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

    fn advance(&mut self, data: Self::Input<'_>) -> Self::Output {
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
                                self.threads.change(PrepareDrawState::BorderWaveformControl(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            PrepareDrawState::BorderWaveformControl(state) => {
                match state {
                    None => {
                        self.threads.change(PrepareDrawState::BorderWaveformControl(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(PrepareDrawState::VBDasVCOM(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            PrepareDrawState::VBDasVCOM(state) => {
                match state {
                    None => {
                        self.threads.change(PrepareDrawState::VBDasVCOM(Some(EPDData::new(()))));
                    },
                    Some(a) => {
                        match a.advance(&[0x01]) {
                            Some(true) => {
                                self.threads.change(PrepareDrawState::WriteRamBlack(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            PrepareDrawState::WriteRamBlack(state) => {
                match state {
                    None => {
                        self.threads.change(PrepareDrawState::WriteRamBlack(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => self.threads.change(PrepareDrawState::SendBufferData(None)),
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            PrepareDrawState::SendBufferData(state) => {
                match state {
                    None => {
                        let b = self.bounds.unwrap_or((0, SCREEN_SIZE_WIDTH_ADDRESS as u8 - 1, SCREEN_SIZE_X as u16 - 1, 0));
                        data.as_mut().unwrap().set_bounds(b);
                        LDMAchUSART0::set_static_cell(Some(Transmittable::Display(data.take().unwrap())));
                        *state = Some(());
                    },
                    Some(_) => {
                        if !LDMAchUSART0::done() {
                            return None
                        }
                        match LDMAchUSART0::take_static_cell() {
                            Some(Transmittable::Display(a)) => {
                                *data = Some(a)
                            },
                            _ => {unreachable!("FrameBuffer should be in static cell")}
                        }
                        self.threads.change(PrepareDrawState::End)
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


pub struct UpdateFull {
    threads: Threads<UpdateFullState, 1>,
}

enum UpdateFullState {
    // set read temperature from internal TS
    TempSensorControl(Option<EPDCommand<0x18>>),
    InternalTempSensor(Option<EPDData<1>>),
    
    DisplayUpdateControl2(Option<EPDCommand<0x22>>),
    DisplayMode1(Option<EPDData<1>>),

    MasterActivation(Option<EPDCommand<0x20>>),

    End,
    Error,
}

impl Default for UpdateFullState {
    fn default() -> Self { UpdateFullState::Error }
}

impl AsyncOperation for UpdateFull {
    type Init = <UpdateUltraFast as AsyncOperation>::Init;
    type Input<'a> = ();
    type Output = Option<bool>;

    fn new(_: Self::Init) -> Self {
        Self {
            threads: Threads::new(UpdateFullState::TempSensorControl(None)),
        }
    }

    fn advance(&mut self, _: ()) -> Self::Output {
        match self.threads.turn() {
            UpdateFullState::TempSensorControl(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateFullState::TempSensorControl(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(UpdateFullState::InternalTempSensor(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            UpdateFullState::InternalTempSensor(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateFullState::InternalTempSensor(Some(EPDData::new(()))));
                    },
                    Some(a) => {
                        match a.advance(&[0x80]) {
                            Some(true) => {
                                self.threads.change(UpdateFullState::DisplayUpdateControl2(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            UpdateFullState::DisplayUpdateControl2(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateFullState::DisplayUpdateControl2(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(UpdateFullState::DisplayMode1(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            UpdateFullState::DisplayMode1(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateFullState::DisplayMode1(Some(EPDData::new(()))));
                    },
                    Some(a) => {
                        match a.advance(&[0xF7]) {
                            Some(true) => {
                                self.threads.change(UpdateFullState::MasterActivation(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            UpdateFullState::MasterActivation(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateFullState::MasterActivation(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                if display_is_busy() {
                                    return None
                                }
                                self.threads.change(UpdateFullState::End);
                                return Some(true)
                            },
                            r => return r
                        }
                    }
                }
                Some(false)
            },
            UpdateFullState::End => {
                Some(true)
            },
            UpdateFullState::Error => {
                panic!("Unknown UpdateFullState while display")
            }
        }
    }
}

pub struct UpdateFast {
    threads: Threads<UpdateFastState, 1>,
}

enum UpdateFastState {
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

impl Default for UpdateFastState {
    fn default() -> Self { UpdateFastState::Error }
}

impl AsyncOperation for UpdateFast {
    type Init = <UpdateUltraFast as AsyncOperation>::Init;
    type Input<'a> = ();
    type Output = Option<bool>;

    fn new(_: Self::Init) -> Self {
        Self {
            threads: Threads::new(UpdateFastState::WtiteLUTRegister(None)),
        }
    }

    fn advance(&mut self, _: ()) -> Self::Output {
        match self.threads.turn() {
            UpdateFastState::WtiteLUTRegister(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateFastState::WtiteLUTRegister(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(UpdateFastState::CustomLUTData(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            UpdateFastState::CustomLUTData(state) => {
                match state {
                    None => {
                        LDMAchUSART0::set_static_cell( 
                            Some(
                                Transmittable::Array(
                                    StaticArrayLDMA::new(&FAST_LUT)
                                )
                            )
                        );
                        self.threads.change(UpdateFastState::CustomLUTData(Some(())));
                    },
                    Some(_) => {
                        if !LDMAchUSART0::done() {
                            return None
                        }
                        LDMAchUSART0::take_static_cell();
                        self.threads.change(UpdateFastState::DisplayUpdateControl2(None));
                    }
                }
                Some(false)
            },
            UpdateFastState::DisplayUpdateControl2(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateFastState::DisplayUpdateControl2(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(UpdateFastState::DisplayMode1NoLoadLUT(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            UpdateFastState::DisplayMode1NoLoadLUT(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateFastState::DisplayMode1NoLoadLUT(Some(EPDData::new(()))));
                    },
                    Some(a) => {
                        match a.advance(&[0xC7]) {
                            Some(true) => {
                                self.threads.change(UpdateFastState::MasterActivation(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            UpdateFastState::MasterActivation(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateFastState::MasterActivation(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                if display_is_busy() {
                                    return None
                                }
                                self.threads.change(UpdateFastState::End);
                                return Some(true)
                            },
                            r => return r
                        }
                    }
                }
                Some(false)
            },
            UpdateFastState::End => {
                Some(true)
            },
            UpdateFastState::Error => {
                panic!("Unknown UpdateFastState while display")
            }
        }
    }
}

pub struct UpdateUltraFast {
    threads: Threads<UpdateUltraFastState, 1>,
    part_mode: <UpdateUltraFast as AsyncOperation>::Init
}

enum UpdateUltraFastState {
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

impl Default for UpdateUltraFastState {
    fn default() -> Self { UpdateUltraFastState::Error }
}

impl AsyncOperation for UpdateUltraFast {
    type Init = bool;
    type Input<'a> = ();
    type Output = Option<bool>;

    fn new(part_mode: Self::Init) -> Self {
        Self {
            threads: Threads::new(UpdateUltraFastState::WtiteLUTRegister(None)),
            part_mode
        }
    }

    fn advance(&mut self, _: ()) -> Self::Output {
        match self.threads.turn() {
            UpdateUltraFastState::WtiteLUTRegister(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateUltraFastState::WtiteLUTRegister(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(UpdateUltraFastState::CustomLUTData(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            UpdateUltraFastState::CustomLUTData(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateUltraFastState::CustomLUTData(Some(())));
                        LDMAchUSART0::set_static_cell( 
                            Some(
                                Transmittable::Array(
                                    StaticArrayLDMA::new( {
                                        if self.part_mode {
                                            &ULTRAFAST_SELECTIVE_LUT
                                        } else {
                                            &ULTRAFAST_LUT
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
                        self.threads.change(UpdateUltraFastState::DisplayUpdateControl2(None));
                    }
                }
                Some(false)
            },
            UpdateUltraFastState::DisplayUpdateControl2(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateUltraFastState::DisplayUpdateControl2(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                self.threads.change(UpdateUltraFastState::DisplayMode1NoLoadLUT(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            UpdateUltraFastState::DisplayMode1NoLoadLUT(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateUltraFastState::DisplayMode1NoLoadLUT(Some(EPDData::new(()))));
                    },
                    Some(a) => {
                        match a.advance(&[0xC7]) {
                            Some(true) => {
                                self.threads.change(UpdateUltraFastState::MasterActivation(None));
                            },
                            r => return r
                        };
                    }
                }
                Some(false)
            },
            UpdateUltraFastState::MasterActivation(state) => {
                match state {
                    None => {
                        self.threads.change(UpdateUltraFastState::MasterActivation(Some(EPDCommand::new(()))));
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(true) => {
                                if display_is_busy() {
                                    return None
                                }
                                self.threads.change(UpdateUltraFastState::End);
                                return Some(true)
                            },
                            r => return r
                        }
                    }
                }
                Some(false)
            },
            UpdateUltraFastState::End => {
                Some(true)
            },
            UpdateUltraFastState::Error => {
                panic!("Unknown UpdateUltraFastState while display")
            }
        }
    }
}