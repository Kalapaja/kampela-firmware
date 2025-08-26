#[doc = "Register `IADC0NORMALOFFSETCAL1` reader"]
pub type R = crate::R<Iadc0normaloffsetcal1Spec>;
#[doc = "Field `OFFSETANA3NORM` reader - No Description"]
pub type Offsetana3normR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - No Description"]
    #[inline(always)]
    pub fn offsetana3norm(&self) -> Offsetana3normR {
        Offsetana3normR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "IADC0 Normal Offset Calibration Info\n\nYou can [`read`](crate::Reg::read) this register and get [`iadc0normaloffsetcal1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iadc0normaloffsetcal1Spec;
impl crate::RegisterSpec for Iadc0normaloffsetcal1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iadc0normaloffsetcal1::R`](R) reader structure"]
impl crate::Readable for Iadc0normaloffsetcal1Spec {}
#[doc = "`reset()` method sets IADC0NORMALOFFSETCAL1 to value 0"]
impl crate::Resettable for Iadc0normaloffsetcal1Spec {}
