#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `SINGLEQEN` reader - Single Queue Enabled"]
pub type SingleqenR = crate::BitReader;
#[doc = "Field `SINGLEQUEUEPENDING` reader - Single Queue Pending"]
pub type SinglequeuependingR = crate::BitReader;
#[doc = "Field `SCANQEN` reader - Scan Queued Enabled"]
pub type ScanqenR = crate::BitReader;
#[doc = "Field `SCANQUEUEPENDING` reader - Scan Queue Pending"]
pub type ScanqueuependingR = crate::BitReader;
#[doc = "Field `CONVERTING` reader - Converting"]
pub type ConvertingR = crate::BitReader;
#[doc = "Field `SINGLEFIFODV` reader - SINGLEFIFO Data Valid"]
pub type SinglefifodvR = crate::BitReader;
#[doc = "Field `SCANFIFODV` reader - SCANFIFO Data Valid"]
pub type ScanfifodvR = crate::BitReader;
#[doc = "Field `SINGLEFIFOFLUSHING` reader - The Single FIFO is flushing"]
pub type SinglefifoflushingR = crate::BitReader;
#[doc = "Field `SCANFIFOFLUSHING` reader - The Scan FIFO is flushing"]
pub type ScanfifoflushingR = crate::BitReader;
#[doc = "Field `TIMERACTIVE` reader - Timer Active"]
pub type TimeractiveR = crate::BitReader;
#[doc = "Field `SINGLEWRITEPENDING` reader - SINGLE write pending"]
pub type SinglewritependingR = crate::BitReader;
#[doc = "Field `MASKREQWRITEPENDING` reader - MASKREQ write pending"]
pub type MaskreqwritependingR = crate::BitReader;
#[doc = "Field `SYNCBUSY` reader - SYNCBUSY"]
pub type SyncbusyR = crate::BitReader;
#[doc = "Field `ADCWARM` reader - ADCWARM"]
pub type AdcwarmR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Single Queue Enabled"]
    #[inline(always)]
    pub fn singleqen(&self) -> SingleqenR {
        SingleqenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Single Queue Pending"]
    #[inline(always)]
    pub fn singlequeuepending(&self) -> SinglequeuependingR {
        SinglequeuependingR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan Queued Enabled"]
    #[inline(always)]
    pub fn scanqen(&self) -> ScanqenR {
        ScanqenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scan Queue Pending"]
    #[inline(always)]
    pub fn scanqueuepending(&self) -> ScanqueuependingR {
        ScanqueuependingR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Converting"]
    #[inline(always)]
    pub fn converting(&self) -> ConvertingR {
        ConvertingR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SINGLEFIFO Data Valid"]
    #[inline(always)]
    pub fn singlefifodv(&self) -> SinglefifodvR {
        SinglefifodvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCANFIFO Data Valid"]
    #[inline(always)]
    pub fn scanfifodv(&self) -> ScanfifodvR {
        ScanfifodvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - The Single FIFO is flushing"]
    #[inline(always)]
    pub fn singlefifoflushing(&self) -> SinglefifoflushingR {
        SinglefifoflushingR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The Scan FIFO is flushing"]
    #[inline(always)]
    pub fn scanfifoflushing(&self) -> ScanfifoflushingR {
        ScanfifoflushingR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer Active"]
    #[inline(always)]
    pub fn timeractive(&self) -> TimeractiveR {
        TimeractiveR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - SINGLE write pending"]
    #[inline(always)]
    pub fn singlewritepending(&self) -> SinglewritependingR {
        SinglewritependingR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MASKREQ write pending"]
    #[inline(always)]
    pub fn maskreqwritepending(&self) -> MaskreqwritependingR {
        MaskreqwritependingR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - SYNCBUSY"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SyncbusyR {
        SyncbusyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - ADCWARM"]
    #[inline(always)]
    pub fn adcwarm(&self) -> AdcwarmR {
        AdcwarmR::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
