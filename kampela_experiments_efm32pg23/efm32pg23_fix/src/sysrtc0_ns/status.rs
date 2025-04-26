#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RUNNING` reader - SYSRTC running status"]
pub type RunningR = crate::BitReader;
#[doc = "Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockstatus {
    #[doc = "0: SYSRTC registers are unlocked"]
    Unlocked = 0,
    #[doc = "1: SYSRTC registers are locked"]
    Locked = 1,
}
impl From<Lockstatus> for bool {
    #[inline(always)]
    fn from(variant: Lockstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKSTATUS` reader - Lock Status"]
pub type LockstatusR = crate::BitReader<Lockstatus>;
impl LockstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockstatus {
        match self.bits {
            false => Lockstatus::Unlocked,
            true => Lockstatus::Locked,
        }
    }
    #[doc = "SYSRTC registers are unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lockstatus::Unlocked
    }
    #[doc = "SYSRTC registers are locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lockstatus::Locked
    }
}
impl R {
    #[doc = "Bit 0 - SYSRTC running status"]
    #[inline(always)]
    pub fn running(&self) -> RunningR {
        RunningR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Status"]
    #[inline(always)]
    pub fn lockstatus(&self) -> LockstatusR {
        LockstatusR::new(((self.bits >> 1) & 1) != 0)
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
