#[doc = "Register `SINGLEFIFOSTAT` reader"]
pub type R = crate::R<SinglefifostatSpec>;
#[doc = "Field `FIFOREADCNT` reader - FIFO Read Count"]
pub type FiforeadcntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - FIFO Read Count"]
    #[inline(always)]
    pub fn fiforeadcnt(&self) -> FiforeadcntR {
        FiforeadcntR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Single FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`singlefifostat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SinglefifostatSpec;
impl crate::RegisterSpec for SinglefifostatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singlefifostat::R`](R) reader structure"]
impl crate::Readable for SinglefifostatSpec {}
#[doc = "`reset()` method sets SINGLEFIFOSTAT to value 0"]
impl crate::Resettable for SinglefifostatSpec {}
