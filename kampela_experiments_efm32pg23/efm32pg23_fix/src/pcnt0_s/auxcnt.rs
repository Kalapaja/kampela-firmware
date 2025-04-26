#[doc = "Register `AUXCNT` reader"]
pub type R = crate::R<AuxcntSpec>;
#[doc = "Field `AUXCNT` reader - Auxiliary Counter Value"]
pub type AuxcntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Auxiliary Counter Value"]
    #[inline(always)]
    pub fn auxcnt(&self) -> AuxcntR {
        AuxcntR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`auxcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxcntSpec;
impl crate::RegisterSpec for AuxcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxcnt::R`](R) reader structure"]
impl crate::Readable for AuxcntSpec {}
#[doc = "`reset()` method sets AUXCNT to value 0"]
impl crate::Resettable for AuxcntSpec {}
