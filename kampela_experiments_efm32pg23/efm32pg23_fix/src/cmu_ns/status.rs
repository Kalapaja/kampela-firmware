#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `CALRDY` reader - Calibration Ready"]
pub type CalrdyR = crate::BitReader;
#[doc = "Configuration Lock Status for WDOG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdoglock {
    #[doc = "0: WDOG configuration lock is unlocked"]
    Unlocked = 0,
    #[doc = "1: WDOG configuration lock is locked"]
    Locked = 1,
}
impl From<Wdoglock> for bool {
    #[inline(always)]
    fn from(variant: Wdoglock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDOGLOCK` reader - Configuration Lock Status for WDOG"]
pub type WdoglockR = crate::BitReader<Wdoglock>;
impl WdoglockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdoglock {
        match self.bits {
            false => Wdoglock::Unlocked,
            true => Wdoglock::Locked,
        }
    }
    #[doc = "WDOG configuration lock is unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Wdoglock::Unlocked
    }
    #[doc = "WDOG configuration lock is locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Wdoglock::Locked
    }
}
#[doc = "Configuration Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: Configuration lock is unlocked"]
    Unlocked = 0,
    #[doc = "1: Configuration lock is locked"]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Configuration Lock Status"]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::Unlocked,
            true => Lock::Locked,
        }
    }
    #[doc = "Configuration lock is unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "Configuration lock is locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
impl R {
    #[doc = "Bit 0 - Calibration Ready"]
    #[inline(always)]
    pub fn calrdy(&self) -> CalrdyR {
        CalrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 30 - Configuration Lock Status for WDOG"]
    #[inline(always)]
    pub fn wdoglock(&self) -> WdoglockR {
        WdoglockR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Configuration Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
