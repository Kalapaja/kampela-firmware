//! display control functions

use crate::peripherals::usart::*;
use crate::peripherals::gpio_pins::{display_res_clear, display_res_set};
use crate::in_free;
use crate::parallel::Operation;
use crate::devices::display_transmission::{display_is_busy, EPDCommand, EPDData, EPDDataB, EPDDataPart, BUFSIZE};

/// Draw sequence
///
/// Iterate through this to perform drawing and send display to proper sleep mode
pub struct Request<R> {
    state: RequestState<R>,
}

pub enum RequestState<R> {
    Init(EPDInit),
    Draw(R),
}

impl<R> Operation for Request<R> where
    R: for <'a> RequestType<
        Init = (),
        Output = bool,
    >
{
    type Init = ();
    type Input<'a> = R::Input<'a>;
    type Output = bool;
    type StateEnum = RequestState<R>;

    fn new(_: ()) -> Self {
        Self {
            state: RequestState::Init(EPDInit::new(())),
        }
    }

    fn wind(&mut self, state: RequestState<R>, _delay: usize) {
        self.state = state;
    }

    fn advance(&mut self, data: Self::Input<'_>) -> bool {
        match self.state {
            RequestState::Init(ref mut a) => {
                if a.advance(()) {
                    let new_state = RequestState::Draw(R::new(()));
                    self.wind_d(new_state);
                };
                false
            },
            RequestState::Draw(ref mut a) => {
                if display_is_busy() != Ok(false) { return false };
                a.advance(data)
                /*
        epaper_draw_stuff_quickly(peripherals, self.data.into_inner());
        or
        epaper_draw_stuff_differently(peripherals, self.data.into_inner());
                */
            }
        }
    }
}


/// EPD init to wake up display
pub struct EPDInit {
    state: EPDInitState,
}

pub enum EPDInitState {
    Reset(Reset),
    WakeUp(EPDCommand<0x12>),
}

impl Operation for EPDInit {
    type Init = ();
    type Input<'a> = ();
    type Output = bool;
    type StateEnum = EPDInitState;

    fn new(_: ()) -> Self {
        Self {
            state: EPDInitState::Reset(Reset::new(())),
        }
    }

    fn wind(&mut self, state: EPDInitState, _: usize) {
        self.state = state;
    }

    fn advance(&mut self, _: ()) -> bool {
        match self.state{
            EPDInitState::Reset(ref mut a) => {
                if a.advance(()) {
                    self.wind(EPDInitState::WakeUp(EPDCommand::<0x12>::new(())), 10)
                }
                false
            },
            EPDInitState::WakeUp(ref mut a) => {
                if display_is_busy() != Ok(false) { return false };
                a.advance(())
            }
        }
    }
}

/// Reset display
///
/// notably used for waking up
pub struct Reset {
    state: ResetState,
    timer: usize,
}

pub enum ResetState {
    R0,
    R1,
    R2,
    R3,
}

impl Reset {
    fn count(&mut self) -> bool {
        if self.timer == 0 {
            false
        } else {
            self.timer -= 1;
            true
        }
    }
}

impl Operation for Reset {
    type Init = ();
    type Input<'a> = ();
    type Output = bool;
    type StateEnum = ResetState;

    fn new(_: ()) -> Self {
        Self {
            state: ResetState::R0,
            timer: 0
        }
    }

    
    fn wind(&mut self, state: ResetState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }


    fn advance(&mut self, _: ()) -> bool {
        if self.count() { return false };
        match self.state {
            ResetState::R0 => {
                in_free(|peripherals| display_res_clear(&mut peripherals.GPIO_S));
                self.wind_d(ResetState::R1);
                false
            },
            ResetState::R1 => {
                in_free(|peripherals| display_res_set(&mut peripherals.GPIO_S));
                self.wind_d(ResetState::R2);
                false
            },
            ResetState::R2 => {
                in_free(|peripherals| display_res_clear(&mut peripherals.GPIO_S));
                self.wind_d(ResetState::R3);
                false
            },
            ResetState::R3 => { // TODO: this is not ZERO operation, should it be here?
                in_free(|peripherals| deselect_display(&mut peripherals.GPIO_S));
                true
            },
        }
    }
}


pub trait RequestType: Operation {}
impl RequestType for FastDraw {}
impl RequestType for FullDraw {}
impl RequestType for PartDraw {}

/// Fast draw sequence without full refresh
///
/// display should be awake
pub struct FastDraw {
    state: FastDrawState,
    timer: usize,
}

pub enum FastDrawState {
    PrepareC1(EPDCommand<0x4E>),
    PrepareD1(EPDDataB<0x00>),
    PrepareC2(EPDCommand<0x4F>),
    PrepareD2(EPDDataB<0x07>),

    SendC1(EPDCommand<0x24>),
    SendD1(EPDData<BUFSIZE>),

    Update(UpdateFast),
}

impl Operation for FastDraw {
    type Init = ();
    type Input<'a> = &'a [u8];
    type Output = bool;
    type StateEnum = FastDrawState;

    fn new(_: ()) -> Self {
        Self {
            state: FastDrawState::PrepareC1(EPDCommand::<0x4E>::new(())),
            timer: 0,
        }
    }

    fn wind(&mut self, state: FastDrawState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self, data: Self::Input<'_>) -> bool {
        match self.state{
            FastDrawState::PrepareC1(ref mut a) => {
                if a.advance(()) {
                    self.change(FastDrawState::PrepareD1(EPDDataB::<0x00>::new(())));
                }
                false
            },
            FastDrawState::PrepareD1(ref mut a) => {
                if a.advance(()) {
                    self.change(FastDrawState::PrepareC2(EPDCommand::<0x4F>::new(())));
                }
                false
            },
            FastDrawState::PrepareC2(ref mut a) => {
                if a.advance(()) {
                    self.change(FastDrawState::PrepareD2(EPDDataB::<0x07>::new(())));
                }
                false
            },
            FastDrawState::PrepareD2(ref mut a) => {
                if a.advance(()) {
                    self.change(FastDrawState::SendC1(EPDCommand::<0x24>::new(())));
                }
                false
            },
            FastDrawState::SendC1(ref mut a) => {
                if display_is_busy() != Ok(false) { return false };
                if a.advance(()) {
                    self.change(FastDrawState::SendD1(EPDData::<BUFSIZE>::new(())));
                }
                false
            },
            FastDrawState::SendD1(ref mut a) => {
                if a.advance(data) {
                    self.change(FastDrawState::Update(UpdateFast::new(())));
                }
                false
            },
            FastDrawState::Update(ref mut a) => {
                a.advance(())
            }
        }
    }

}

/// Slow drawing sequence with full refresh;
///
/// display should be awake
pub struct FullDraw {
    state: FullDrawState,
    timer: usize,
}

pub enum FullDrawState {
    PrepareC1(EPDCommand<0x4E>),
    PrepareD1(EPDDataB<0x00>),
    PrepareC2(EPDCommand<0x4F>),
    PrepareD2(EPDDataB<0x07>),
    SendC1(EPDCommand<0x24>),
    SendD1(EPDData<BUFSIZE>),
    Update(UpdateFull),
}

impl Operation for FullDraw {
    type Init = ();
    type Input<'a> = &'a [u8];
    type Output = bool;
    type StateEnum = FullDrawState;

    fn new(_: ()) -> Self {
        Self {
            state: FullDrawState::PrepareC1(EPDCommand::<0x4E>::new(())),
            timer: 0,
        }
    }

    fn wind(&mut self, state: FullDrawState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self, data: Self::Input<'_>) -> bool {
        match self.state{
            FullDrawState::PrepareC1(ref mut a) => {
                if a.advance(()) {
                    self.change(FullDrawState::PrepareD1(EPDDataB::<0x00>::new(())));
                }
                false
            },
            FullDrawState::PrepareD1(ref mut a) => {
                if a.advance(()) {
                    self.change(FullDrawState::PrepareC2(EPDCommand::<0x4F>::new(())));
                }
                false
            },
            FullDrawState::PrepareC2(ref mut a) => {
                if a.advance(()) {
                    self.change(FullDrawState::PrepareD2(EPDDataB::<0x07>::new(())));
                }
                false
            },
            FullDrawState::PrepareD2(ref mut a) => {
                if a.advance(()) {
                    self.change(FullDrawState::SendC1(EPDCommand::<0x24>::new(())));
                }
                false
            },
            FullDrawState::SendC1(ref mut a) => {
                if a.advance(()) {
                    self.change(FullDrawState::SendD1(EPDData::<BUFSIZE>::new(())));
                }
                false
            },
            FullDrawState::SendD1(ref mut a) => {
                if a.advance(data) {
                    self.change(FullDrawState::Update(UpdateFull::new(())));
                }
                false
            },
            FullDrawState::Update(ref mut a) => {
                a.advance(())
            }
        }
    }
}

pub struct PartDraw {
    state: PartDrawState,
    timer: usize,
}

pub enum PartDrawState {
    //BorderWavefrom,
    PrepareC1(EPDCommand<0x3C>),
    PrepareD1(EPDDataB<0x80>),
    //Set RAM X address start/end postition (which is Y due to orientation)
    PrepareC2(EPDCommand<0x44>),
    PrepareD21(EPDData<1>),
    PrepareD22(EPDData<1>),
    //Set RAM Y address start/end postition (which is X due to orientation)
    PrepareC3(EPDCommand<0x45>),
    PrepareD31(EPDData<2>),
    PrepareD32(EPDData<2>),
    //Set RAM X&Y address write starting position
    PrepareC4(EPDCommand<0x4E>),
    PrepareD4(EPDData<1>),
    PrepareC5(EPDCommand<0x4F>),
    PrepareD5(EPDData<2>),

    SendC1(EPDCommand<0x24>),
    SendD1(EPDDataPart<BUFSIZE>),
    SendC2(EPDCommand<0x26>), //for some reason red ram still used in mode 2
    SendD2(EPDDataPart<BUFSIZE>),

    Update(UpdateUltraFast),
}

impl Operation for PartDraw {
    type Init = ();
    type Input<'a> = (&'a [u8], (u8, u8, u16, u16));
    type Output = bool;
    type StateEnum = PartDrawState;

    fn new(_: ()) -> Self {
        Self {
            state: PartDrawState::PrepareC1(EPDCommand::<0x3C>::new(())),
            timer: 0,
        }
    }

    fn wind(&mut self, state: PartDrawState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self, data: Self::Input<'_>) -> bool {
        match self.state{
            PartDrawState::PrepareC1(ref mut a) => {
                if a.advance(()) {
                    self.change(PartDrawState::PrepareD1(EPDDataB::<0x80>::new(())));
                }
                false
            },
            PartDrawState::PrepareD1(ref mut a) => {
                if a.advance(()) {
                    self.change(PartDrawState::PrepareC2(EPDCommand::<0x44>::new(())));
                }
                false
            },
            PartDrawState::PrepareC2(ref mut a) => {
                if a.advance(()) {
                    self.change(PartDrawState::PrepareD21(EPDData::<1>::new(())));
                }
                false
            },
            PartDrawState::PrepareD21(ref mut a) => {
                if a.advance(&data.1.0.to_le_bytes()) {
                    self.change(PartDrawState::PrepareD22(EPDData::<1>::new(())));
                }
                false
            },
            PartDrawState::PrepareD22(ref mut a) => {
                if a.advance(&data.1.1.to_le_bytes()) {
                    self.change(PartDrawState::PrepareC3(EPDCommand::<0x45>::new(())));
                }
                false
            },
            PartDrawState::PrepareC3(ref mut a) => {
                if a.advance(()) {
                    self.change(PartDrawState::PrepareD31(EPDData::<2>::new(())));
                }
                false
            },
            PartDrawState::PrepareD31(ref mut a) => {
                if a.advance(&data.1.2.to_le_bytes()) {
                    self.change(PartDrawState::PrepareD32(EPDData::<2>::new(())));
                }
                false
            },
            PartDrawState::PrepareD32(ref mut a) => {
                if a.advance(&data.1.3.to_le_bytes()) {
                    self.change(PartDrawState::PrepareC4(EPDCommand::<0x4E>::new(())));
                }
                false
            },
            PartDrawState::PrepareC4(ref mut a) => {
                if a.advance(()) {
                    self.change(PartDrawState::PrepareD4(EPDData::<1>::new(())));
                }
                false
            },
            PartDrawState::PrepareD4(ref mut a) => {
                if a.advance(&data.1.0.to_le_bytes()) {
                    self.change(PartDrawState::PrepareC5(EPDCommand::<0x4F>::new(())));
                }
                false
            },
            PartDrawState::PrepareC5(ref mut a) => {
                if a.advance(()) {
                    self.change(PartDrawState::PrepareD5(EPDData::<2>::new(())));
                }
                false
            },
            PartDrawState::PrepareD5(ref mut a) => {
                if a.advance(&data.1.2.to_le_bytes()) {
                    self.change(PartDrawState::SendC1(EPDCommand::<0x24>::new(())));
                }
                false
            },
            PartDrawState::SendC1(ref mut a) => {
                if a.advance(()) {
                    self.change(PartDrawState::SendD1(EPDDataPart::<BUFSIZE>::new(data.1)));
                }
                false
            },
            PartDrawState::SendD1(ref mut a) => {
                if a.advance(data.0) {
                    self.change(PartDrawState::SendC2(EPDCommand::<0x26>::new(())));
                }
                false
            },
            PartDrawState::SendC2(ref mut a) => {
                if a.advance(()) {
                    self.change(PartDrawState::SendD2(EPDDataPart::<BUFSIZE>::new(data.1)));
                }
                false
            },
            PartDrawState::SendD2(ref mut a) => {
                if a.advance(data.0) {
                    self.change(PartDrawState::Update(UpdateUltraFast::new(())));
                }
                false
            },
            PartDrawState::Update(ref mut a) => {
                a.advance(())
            }
        }
    }
}


pub struct UpdateFull {
    state: UpdateFullState,
    timer: usize,
}

pub enum UpdateFullState {
     //bypass RED RAM
    UpdateC1(EPDCommand<0x21>),
    UpdateD11(EPDDataB<0x40>),
    UpdateD12(EPDDataB<0x00>),
    // set read temperature from internal TS
    UpdateC2(EPDCommand<0x18>),
    UpdateD2(EPDDataB<0x80>),
    
    UpdateC3(EPDCommand<0x22>),
    UpdateD3(EPDDataB<0xF7>),

    UpdateC4(EPDCommand<0x20>),
}

impl UpdateFull {
    fn count(&mut self) -> bool {
        if self.timer == 0 {
            false
        } else {
            self.timer -= 1;
            true
        }
    }
}

impl Operation for UpdateFull {
    type Init = ();
    type Input<'a> = ();
    type Output = bool;
    type StateEnum = UpdateFullState;

    fn new(_: ()) -> Self {
        Self {
            state: UpdateFullState::UpdateC1(EPDCommand::<0x21>::new(())),
            timer: 0
        }
    }
    
    fn wind(&mut self, state: UpdateFullState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }


    fn advance(&mut self, _: ()) -> bool {
        if self.count() { return false };
        match self.state {
            UpdateFullState::UpdateC1(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFullState::UpdateD11(EPDDataB::<0x40>::new(())));
                }
                false
            },
            UpdateFullState::UpdateD11(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFullState::UpdateD12(EPDDataB::<0x00>::new(())));
                }
                false
            },
            UpdateFullState::UpdateD12(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFullState::UpdateC2(EPDCommand::<0x18>::new(())));
                }
                false
            },
            UpdateFullState::UpdateC2(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFullState::UpdateD2(EPDDataB::<0x80>::new(())));
                }
                false
            },
            UpdateFullState::UpdateD2(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFullState::UpdateC3(EPDCommand::<0x22>::new(())));
                }
                false
            },
            UpdateFullState::UpdateC3(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFullState::UpdateD3(EPDDataB::<0xF7>::new(())));
                }
                false
            },
            UpdateFullState::UpdateD3(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFullState::UpdateC4(EPDCommand::<0x20>::new(())));
                }
                false
            },
            UpdateFullState::UpdateC4(ref mut a) => {
                if display_is_busy() != Ok(false) { return false };
                if a.advance(()) {
                    true
                } else {
                    false
                }
            },
        }
    }
}

pub struct UpdateFast {
    state: UpdateFastState,
    timer: usize,
}

pub enum UpdateFastState {
    //set read from internal temperature sensor
    PrepareC1(EPDCommand<0x18>),
    PrepareD1(EPDDataB<0x80>),
    //set temperature register at 100deg
    PrepareC2(EPDCommand<0x1A>),
    PrepareD21(EPDDataB<0x64>),
    PrepareD22(EPDDataB<0x00>),
    //load LUT with new temperature value
    PrepareC3(EPDCommand<0x22>),
    PrepareD3(EPDDataB<0x91>),
    PrepareC4(EPDCommand<0x20>),
    // bypass RED RAM
    UpdateC1(EPDCommand<0x21>), 
    UpdateD11(EPDDataB<0x40>),
    UpdateD12(EPDDataB<0x00>),
    // set to display with new LUT
    UpdateC2(EPDCommand<0x22>),
    UpdateD2(EPDDataB<0xC7>),

    UpdateC3(EPDCommand<0x20>),
}

impl UpdateFast {
    fn count(&mut self) -> bool {
        if self.timer == 0 {
            false
        } else {
            self.timer -= 1;
            true
        }
    }
}

impl Operation for UpdateFast {
    type Init = ();
    type Input<'a> = ();
    type Output = bool;
    type StateEnum = UpdateFastState;

    fn new(_: ()) -> Self {
        Self {
            state: UpdateFastState::PrepareC1(EPDCommand::<0x18>::new(())),
            timer: 0
        }
    }
    
    fn wind(&mut self, state: UpdateFastState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self, _: ()) -> bool {
        if self.count() { return false };
        match self.state {
            UpdateFastState::PrepareC1(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFastState::PrepareD1(EPDDataB::<0x80>::new(())));
                }
                false
            },
            UpdateFastState::PrepareD1(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFastState::PrepareC2(EPDCommand::<0x1A>::new(())));
                }
                false
            },
            UpdateFastState::PrepareC2(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFastState::PrepareD21(EPDDataB::<0x64>::new(())));
                }
                false
            },
            UpdateFastState::PrepareD21(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFastState::PrepareD22(EPDDataB::<0x00>::new(())));
                }
                false
            },
            UpdateFastState::PrepareD22(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFastState::PrepareC3(EPDCommand::<0x22>::new(())));
                }
                false
            },
            UpdateFastState::PrepareC3(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFastState::PrepareD3(EPDDataB::<0x91>::new(())));
                }
                false
            },
            UpdateFastState::PrepareD3(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFastState::PrepareC4(EPDCommand::<0x20>::new(())));
                }
                false
            },
            UpdateFastState::PrepareC4(ref mut a) => {
                if display_is_busy() != Ok(false) { return false };
                if a.advance(()) {
                    self.change(UpdateFastState::UpdateC1(EPDCommand::<0x21>::new(())));
                }
                false
            },
            UpdateFastState::UpdateC1(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFastState::UpdateD11(EPDDataB::<0x40>::new(())));
                }
                false
            },
            UpdateFastState::UpdateD11(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFastState::UpdateD12(EPDDataB::<0x00>::new(())));
                }
                false
            },
            UpdateFastState::UpdateD12(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFastState::UpdateC2(EPDCommand::<0x22>::new(())));
                }
                false
            },
            UpdateFastState::UpdateC2(ref mut a) => {
                if display_is_busy() != Ok(false) { return false };
                if a.advance(()) {
                    self.change(UpdateFastState::UpdateD2(EPDDataB::<0xC7>::new(())));
                }
                false
            },
            UpdateFastState::UpdateD2(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateFastState::UpdateC3(EPDCommand::<0x20>::new(())));
                }
                false
            },
            UpdateFastState::UpdateC3(ref mut a) => {
                if display_is_busy() != Ok(false) { return false };
                a.advance(())
            },
        }
    }
}

pub struct UpdateUltraFast {
    state: UpdateUltraFastState,
    timer: usize,
}

pub enum UpdateUltraFastState {
    //inverse RED RAM (for some reason red ram still used in mode 2)
    UpdateC1(EPDCommand<0x21>),
    UpdateD11(EPDDataB<0x80>),
    UpdateD12(EPDDataB<0x00>),
    
    UpdateC2(EPDCommand<0x22>),
    UpdateD2(EPDDataB<0xFF>),

    UpdateC3(EPDCommand<0x20>),
}

impl UpdateUltraFast {
    fn count(&mut self) -> bool {
        if self.timer == 0 {
            false
        } else {
            self.timer -= 1;
            true
        }
    }
}

impl Operation for UpdateUltraFast {
    type Init = ();
    type Input<'a> = ();
    type Output = bool;
    type StateEnum = UpdateUltraFastState;

    fn new(_: ()) -> Self {
        Self {
            state: UpdateUltraFastState::UpdateC1(EPDCommand::<0x21>::new(())),
            timer: 0
        }
    }
    
    fn wind(&mut self, state: UpdateUltraFastState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self, _: ()) -> bool {
        if self.count() { return false };
        match self.state {
            UpdateUltraFastState::UpdateC1(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateUltraFastState::UpdateD11(EPDDataB::<0x80>::new(())));
                }
                false
            },
            UpdateUltraFastState::UpdateD11(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateUltraFastState::UpdateD12(EPDDataB::<0x00>::new(())));
                }
                false
            },
            UpdateUltraFastState::UpdateD12(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateUltraFastState::UpdateC2(EPDCommand::<0x22>::new(())));
                }
                false
            },
            UpdateUltraFastState::UpdateC2(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateUltraFastState::UpdateD2(EPDDataB::<0xFF>::new(())));
                }
                false
            },
            UpdateUltraFastState::UpdateD2(ref mut a) => {
                if a.advance(()) {
                    self.change(UpdateUltraFastState::UpdateC3(EPDCommand::<0x20>::new(())));
                }
                false
            },

            UpdateUltraFastState::UpdateC3(ref mut a) => {
                if display_is_busy() != Ok(false) { return false };
                if a.advance(()) {
                    true
                } else {
                    false
                }
            },
        }
    }
}