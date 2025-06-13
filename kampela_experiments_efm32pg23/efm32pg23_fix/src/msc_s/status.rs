#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `BUSY` reader - Erase/Write Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `LOCKED` reader - Access Locked"]
pub type LockedR = crate::BitReader;
#[doc = "Field `INVADDR` reader - Invalid Write Address or Erase Page"]
pub type InvaddrR = crate::BitReader;
#[doc = "Field `WDATAREADY` reader - WDATA Write Ready"]
pub type WdatareadyR = crate::BitReader;
#[doc = "Field `ERASEABORTED` reader - Erase Operation Aborted"]
pub type EraseabortedR = crate::BitReader;
#[doc = "Field `PENDING` reader - Write Command In Queue"]
pub type PendingR = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - Write Command Timeout"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `RANGEPARTIAL` reader - EraseRange with skipped locked pages"]
pub type RangepartialR = crate::BitReader;
#[doc = "Register Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reglock {
    #[doc = "0: UNLOCKED"]
    Unlocked = 0,
    #[doc = "1: LOCKED"]
    Locked = 1,
}
impl From<Reglock> for bool {
    #[inline(always)]
    fn from(variant: Reglock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGLOCK` reader - Register Lock Status"]
pub type ReglockR = crate::BitReader<Reglock>;
impl ReglockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reglock {
        match self.bits {
            false => Reglock::Unlocked,
            true => Reglock::Locked,
        }
    }
    #[doc = "UNLOCKED"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Reglock::Unlocked
    }
    #[doc = "LOCKED"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Reglock::Locked
    }
}
#[doc = "Field `PWRON` reader - Flash power on status"]
pub type PwronR = crate::BitReader;
#[doc = "Field `WREADY` reader - Flash Write Ready"]
pub type WreadyR = crate::BitReader;
#[doc = "Field `PWRUPCKBDFAILCOUNT` reader - Flash power up checkerboard pattern chec"]
pub type PwrupckbdfailcountR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Erase/Write Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Access Locked"]
    #[inline(always)]
    pub fn locked(&self) -> LockedR {
        LockedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Invalid Write Address or Erase Page"]
    #[inline(always)]
    pub fn invaddr(&self) -> InvaddrR {
        InvaddrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WDATA Write Ready"]
    #[inline(always)]
    pub fn wdataready(&self) -> WdatareadyR {
        WdatareadyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Erase Operation Aborted"]
    #[inline(always)]
    pub fn eraseaborted(&self) -> EraseabortedR {
        EraseabortedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Command In Queue"]
    #[inline(always)]
    pub fn pending(&self) -> PendingR {
        PendingR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Command Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EraseRange with skipped locked pages"]
    #[inline(always)]
    pub fn rangepartial(&self) -> RangepartialR {
        RangepartialR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Register Lock Status"]
    #[inline(always)]
    pub fn reglock(&self) -> ReglockR {
        ReglockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Flash power on status"]
    #[inline(always)]
    pub fn pwron(&self) -> PwronR {
        PwronR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - Flash Write Ready"]
    #[inline(always)]
    pub fn wready(&self) -> WreadyR {
        WreadyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Flash power up checkerboard pattern chec"]
    #[inline(always)]
    pub fn pwrupckbdfailcount(&self) -> PwrupckbdfailcountR {
        PwrupckbdfailcountR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x0800_0008"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x0800_0008;
}
