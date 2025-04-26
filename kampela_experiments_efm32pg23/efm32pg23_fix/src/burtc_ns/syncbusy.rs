#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `START` reader - Sync busy for START"]
pub type StartR = crate::BitReader;
#[doc = "Field `STOP` reader - Sync busy for STOP"]
pub type StopR = crate::BitReader;
#[doc = "Field `PRECNT` reader - Sync busy for PRECNT"]
pub type PrecntR = crate::BitReader;
#[doc = "Field `CNT` reader - Sync busy for CNT"]
pub type CntR = crate::BitReader;
#[doc = "Field `COMP` reader - Sync busy for COMP"]
pub type CompR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Sync busy for START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sync busy for STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sync busy for PRECNT"]
    #[inline(always)]
    pub fn precnt(&self) -> PrecntR {
        PrecntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sync busy for CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sync busy for COMP"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(((self.bits >> 4) & 1) != 0)
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
