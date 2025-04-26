#[doc = "Register `SCANFIFOSTAT` reader"]
pub type R = crate::R<ScanfifostatSpec>;
#[doc = "Field `FIFOREADCNT` reader - FIFO Read Count"]
pub type FiforeadcntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - FIFO Read Count"]
    #[inline(always)]
    pub fn fiforeadcnt(&self) -> FiforeadcntR {
        FiforeadcntR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Scan FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`scanfifostat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScanfifostatSpec;
impl crate::RegisterSpec for ScanfifostatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanfifostat::R`](R) reader structure"]
impl crate::Readable for ScanfifostatSpec {}
#[doc = "`reset()` method sets SCANFIFOSTAT to value 0"]
impl crate::Resettable for ScanfifostatSpec {}
