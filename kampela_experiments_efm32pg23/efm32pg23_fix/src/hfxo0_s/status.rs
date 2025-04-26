#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RDY` reader - Ready Status"]
pub type RdyR = crate::BitReader;
#[doc = "Field `COREBIASOPTRDY` reader - Core Bias Optimization Ready"]
pub type CorebiasoptrdyR = crate::BitReader;
#[doc = "Field `PRSRDY` reader - PRS Ready Status"]
pub type PrsrdyR = crate::BitReader;
#[doc = "Field `BUFOUTRDY` reader - BUFOUT Ready Status"]
pub type BufoutrdyR = crate::BitReader;
#[doc = "Field `BUFOUTFROZEN` reader - BUFOUT Frozen"]
pub type BufoutfrozenR = crate::BitReader;
#[doc = "Field `ENS` reader - Enabled Status"]
pub type EnsR = crate::BitReader;
#[doc = "Field `HWREQ` reader - Oscillator Requested by Digital Clock"]
pub type HwreqR = crate::BitReader;
#[doc = "Field `ISWARM` reader - Oscillator Is Kept Warm"]
pub type IswarmR = crate::BitReader;
#[doc = "Field `PRSHWREQ` reader - Oscillator Requested by PRS Request"]
pub type PrshwreqR = crate::BitReader;
#[doc = "Field `BUFOUTHWREQ` reader - Oscillator Requested by BUFOUT Request"]
pub type BufouthwreqR = crate::BitReader;
#[doc = "Field `SYNCBUSY` reader - Sync Busy"]
pub type SyncbusyR = crate::BitReader;
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
    #[doc = "Bit 0 - Ready Status"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core Bias Optimization Ready"]
    #[inline(always)]
    pub fn corebiasoptrdy(&self) -> CorebiasoptrdyR {
        CorebiasoptrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PRS Ready Status"]
    #[inline(always)]
    pub fn prsrdy(&self) -> PrsrdyR {
        PrsrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUFOUT Ready Status"]
    #[inline(always)]
    pub fn bufoutrdy(&self) -> BufoutrdyR {
        BufoutrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - BUFOUT Frozen"]
    #[inline(always)]
    pub fn bufoutfrozen(&self) -> BufoutfrozenR {
        BufoutfrozenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enabled Status"]
    #[inline(always)]
    pub fn ens(&self) -> EnsR {
        EnsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Oscillator Requested by Digital Clock"]
    #[inline(always)]
    pub fn hwreq(&self) -> HwreqR {
        HwreqR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Oscillator Is Kept Warm"]
    #[inline(always)]
    pub fn iswarm(&self) -> IswarmR {
        IswarmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Oscillator Requested by PRS Request"]
    #[inline(always)]
    pub fn prshwreq(&self) -> PrshwreqR {
        PrshwreqR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Oscillator Requested by BUFOUT Request"]
    #[inline(always)]
    pub fn bufouthwreq(&self) -> BufouthwreqR {
        BufouthwreqR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - Sync Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SyncbusyR {
        SyncbusyR::new(((self.bits >> 30) & 1) != 0)
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
