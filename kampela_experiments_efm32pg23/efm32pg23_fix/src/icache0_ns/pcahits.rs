#[doc = "Register `PCAHITS` reader"]
pub type R = crate::R<PcahitsSpec>;
#[doc = "Field `PCAHITS` reader - Performance Counter Advanced Hits"]
pub type PcahitsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Performance Counter Advanced Hits"]
    #[inline(always)]
    pub fn pcahits(&self) -> PcahitsR {
        PcahitsR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pcahits::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcahitsSpec;
impl crate::RegisterSpec for PcahitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcahits::R`](R) reader structure"]
impl crate::Readable for PcahitsSpec {}
#[doc = "`reset()` method sets PCAHITS to value 0"]
impl crate::Resettable for PcahitsSpec {}
