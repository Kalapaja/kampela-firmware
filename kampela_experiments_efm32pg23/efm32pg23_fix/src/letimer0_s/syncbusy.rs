#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `CNT` reader - Sync busy for CNT"]
pub type CntR = crate::BitReader;
#[doc = "Field `TOP` reader - Sync busy for TOP"]
pub type TopR = crate::BitReader;
#[doc = "Field `REP0` reader - Sync busy for REP0"]
pub type Rep0R = crate::BitReader;
#[doc = "Field `REP1` reader - Sync busy for REP1"]
pub type Rep1R = crate::BitReader;
#[doc = "Field `START` reader - Sync busy for START"]
pub type StartR = crate::BitReader;
#[doc = "Field `STOP` reader - Sync busy for STOP"]
pub type StopR = crate::BitReader;
#[doc = "Field `CLEAR` reader - Sync busy for CLEAR"]
pub type ClearR = crate::BitReader;
#[doc = "Field `CTO0` reader - Sync busy for CTO0"]
pub type Cto0R = crate::BitReader;
#[doc = "Field `CTO1` reader - Sync busy for CTO1"]
pub type Cto1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Sync busy for CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Sync busy for TOP"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sync busy for REP0"]
    #[inline(always)]
    pub fn rep0(&self) -> Rep0R {
        Rep0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sync busy for REP1"]
    #[inline(always)]
    pub fn rep1(&self) -> Rep1R {
        Rep1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sync busy for START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sync busy for STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sync busy for CLEAR"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sync busy for CTO0"]
    #[inline(always)]
    pub fn cto0(&self) -> Cto0R {
        Cto0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sync busy for CTO1"]
    #[inline(always)]
    pub fn cto1(&self) -> Cto1R {
        Cto1R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
