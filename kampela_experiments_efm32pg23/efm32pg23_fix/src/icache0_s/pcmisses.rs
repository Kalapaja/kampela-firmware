#[doc = "Register `PCMISSES` reader"]
pub type R = crate::R<PcmissesSpec>;
#[doc = "Field `PCMISSES` reader - Performance Counter Misses"]
pub type PcmissesR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Performance Counter Misses"]
    #[inline(always)]
    pub fn pcmisses(&self) -> PcmissesR {
        PcmissesR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pcmisses::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcmissesSpec;
impl crate::RegisterSpec for PcmissesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcmisses::R`](R) reader structure"]
impl crate::Readable for PcmissesSpec {}
#[doc = "`reset()` method sets PCMISSES to value 0"]
impl crate::Resettable for PcmissesSpec {}
