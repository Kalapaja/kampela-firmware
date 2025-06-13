#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `ROW` reader - Row detection"]
pub type RowR = crate::FieldReader;
#[doc = "Field `RUNNING` reader - Running"]
pub type RunningR = crate::BitReader;
#[doc = "Field `COL` reader - Column Latched"]
pub type ColR = crate::FieldReader;
#[doc = "Field `NOKEY` reader - No Key pressed status"]
pub type NokeyR = crate::BitReader;
#[doc = "Field `SYNCBUSY` reader - Sync Busy"]
pub type SyncbusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Row detection"]
    #[inline(always)]
    pub fn row(&self) -> RowR {
        RowR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Running"]
    #[inline(always)]
    pub fn running(&self) -> RunningR {
        RunningR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Column Latched"]
    #[inline(always)]
    pub fn col(&self) -> ColR {
        ColR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - No Key pressed status"]
    #[inline(always)]
    pub fn nokey(&self) -> NokeyR {
        NokeyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Sync Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SyncbusyR {
        SyncbusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x4000_0000"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x4000_0000;
}
