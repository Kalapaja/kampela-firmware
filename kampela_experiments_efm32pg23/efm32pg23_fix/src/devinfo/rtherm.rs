#[doc = "Register `RTHERM` reader"]
pub type R = crate::R<RthermSpec>;
#[doc = "Field `RTHERM` reader - No Description"]
pub type RthermR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - No Description"]
    #[inline(always)]
    pub fn rtherm(&self) -> RthermR {
        RthermR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RTHERM\n\nYou can [`read`](crate::Reg::read) this register and get [`rtherm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RthermSpec;
impl crate::RegisterSpec for RthermSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtherm::R`](R) reader structure"]
impl crate::Readable for RthermSpec {}
#[doc = "`reset()` method sets RTHERM to value 0"]
impl crate::Resettable for RthermSpec {}
