#[doc = "Register `STATE` reader"]
pub type R = crate::R<StateSpec>;
#[doc = "Field `BUSY` reader - Bus Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `MASTER` reader - Leader"]
pub type MasterR = crate::BitReader;
#[doc = "Field `TRANSMITTER` reader - Transmitter"]
pub type TransmitterR = crate::BitReader;
#[doc = "Field `NACKED` reader - Nack Received"]
pub type NackedR = crate::BitReader;
#[doc = "Field `BUSHOLD` reader - Bus Held"]
pub type BusholdR = crate::BitReader;
#[doc = "Transmission State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    #[doc = "0: No transmission is being performed."]
    Idle = 0,
    #[doc = "1: Waiting for idle. Will send a start condition as soon as the bus is idle."]
    Wait = 1,
    #[doc = "2: Start transmit phase"]
    Start = 2,
    #[doc = "3: Address transmit or receive phase"]
    Addr = 3,
    #[doc = "4: Address ack/nack transmit or receive phase"]
    Addrack = 4,
    #[doc = "5: Data transmit or receive phase"]
    Data = 5,
    #[doc = "6: Data ack/nack transmit or receive phase"]
    Dataack = 6,
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for State {
    type Ux = u8;
}
impl crate::IsEnum for State {}
#[doc = "Field `STATE` reader - Transmission State"]
pub type StateR = crate::FieldReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<State> {
        match self.bits {
            0 => Some(State::Idle),
            1 => Some(State::Wait),
            2 => Some(State::Start),
            3 => Some(State::Addr),
            4 => Some(State::Addrack),
            5 => Some(State::Data),
            6 => Some(State::Dataack),
            _ => None,
        }
    }
    #[doc = "No transmission is being performed."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == State::Idle
    }
    #[doc = "Waiting for idle. Will send a start condition as soon as the bus is idle."]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == State::Wait
    }
    #[doc = "Start transmit phase"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == State::Start
    }
    #[doc = "Address transmit or receive phase"]
    #[inline(always)]
    pub fn is_addr(&self) -> bool {
        *self == State::Addr
    }
    #[doc = "Address ack/nack transmit or receive phase"]
    #[inline(always)]
    pub fn is_addrack(&self) -> bool {
        *self == State::Addrack
    }
    #[doc = "Data transmit or receive phase"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == State::Data
    }
    #[doc = "Data ack/nack transmit or receive phase"]
    #[inline(always)]
    pub fn is_dataack(&self) -> bool {
        *self == State::Dataack
    }
}
impl R {
    #[doc = "Bit 0 - Bus Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Leader"]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter"]
    #[inline(always)]
    pub fn transmitter(&self) -> TransmitterR {
        TransmitterR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Nack Received"]
    #[inline(always)]
    pub fn nacked(&self) -> NackedR {
        NackedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus Held"]
    #[inline(always)]
    pub fn bushold(&self) -> BusholdR {
        BusholdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Transmission State"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 5) & 7) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StateSpec;
impl crate::RegisterSpec for StateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for StateSpec {}
#[doc = "`reset()` method sets STATE to value 0x01"]
impl crate::Resettable for StateSpec {
    const RESET_VALUE: u32 = 0x01;
}
