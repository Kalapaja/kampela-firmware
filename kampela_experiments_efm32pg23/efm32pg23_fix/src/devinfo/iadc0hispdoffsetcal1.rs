#[doc = "Register `IADC0HISPDOFFSETCAL1` reader"]
pub type R = crate::R<Iadc0hispdoffsetcal1Spec>;
#[doc = "Field `OFFSETANA3HISPD` reader - No Description"]
pub type Offsetana3hispdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - No Description"]
    #[inline(always)]
    pub fn offsetana3hispd(&self) -> Offsetana3hispdR {
        Offsetana3hispdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "IADC High Speed Offset Calibration Info\n\nYou can [`read`](crate::Reg::read) this register and get [`iadc0hispdoffsetcal1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iadc0hispdoffsetcal1Spec;
impl crate::RegisterSpec for Iadc0hispdoffsetcal1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iadc0hispdoffsetcal1::R`](R) reader structure"]
impl crate::Readable for Iadc0hispdoffsetcal1Spec {}
#[doc = "`reset()` method sets IADC0HISPDOFFSETCAL1 to value 0"]
impl crate::Resettable for Iadc0hispdoffsetcal1Spec {}
