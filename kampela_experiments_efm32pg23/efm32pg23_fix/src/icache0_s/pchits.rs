#[doc = "Register `PCHITS` reader"]
pub type R = crate::R<PchitsSpec>;
#[doc = "Field `PCHITS` reader - Performance Counter Hits"]
pub type PchitsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Performance Counter Hits"]
    #[inline(always)]
    pub fn pchits(&self) -> PchitsR {
        PchitsR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pchits::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PchitsSpec;
impl crate::RegisterSpec for PchitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pchits::R`](R) reader structure"]
impl crate::Readable for PchitsSpec {}
#[doc = "`reset()` method sets PCHITS to value 0"]
impl crate::Resettable for PchitsSpec {}
