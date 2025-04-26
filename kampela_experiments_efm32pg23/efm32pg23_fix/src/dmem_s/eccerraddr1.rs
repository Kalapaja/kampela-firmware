#[doc = "Register `ECCERRADDR1` reader"]
pub type R = crate::R<Eccerraddr1Spec>;
#[doc = "Field `ADDR` reader - ECC Error Address"]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC Error Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`eccerraddr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eccerraddr1Spec;
impl crate::RegisterSpec for Eccerraddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccerraddr1::R`](R) reader structure"]
impl crate::Readable for Eccerraddr1Spec {}
#[doc = "`reset()` method sets ECCERRADDR1 to value 0"]
impl crate::Resettable for Eccerraddr1Spec {}
