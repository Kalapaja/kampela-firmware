#[doc = "Register `IADC0HISPDOFFSETCAL0` reader"]
pub type R = crate::R<Iadc0hispdoffsetcal0Spec>;
#[doc = "Field `OFFSETANA1HISPD` reader - No Description"]
pub type Offsetana1hispdR = crate::FieldReader<u16>;
#[doc = "Field `OFFSETANA2HISPD` reader - No Description"]
pub type Offsetana2hispdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - No Description"]
    #[inline(always)]
    pub fn offsetana1hispd(&self) -> Offsetana1hispdR {
        Offsetana1hispdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - No Description"]
    #[inline(always)]
    pub fn offsetana2hispd(&self) -> Offsetana2hispdR {
        Offsetana2hispdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "IADC High Speed Offset Calibration Info\n\nYou can [`read`](crate::Reg::read) this register and get [`iadc0hispdoffsetcal0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iadc0hispdoffsetcal0Spec;
impl crate::RegisterSpec for Iadc0hispdoffsetcal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iadc0hispdoffsetcal0::R`](R) reader structure"]
impl crate::Readable for Iadc0hispdoffsetcal0Spec {}
#[doc = "`reset()` method sets IADC0HISPDOFFSETCAL0 to value 0"]
impl crate::Resettable for Iadc0hispdoffsetcal0Spec {}
