#[doc = "Register `IADC0NORMALOFFSETCAL0` reader"]
pub type R = crate::R<Iadc0normaloffsetcal0Spec>;
#[doc = "Field `OFFSETANA1NORM` reader - No Description"]
pub type Offsetana1normR = crate::FieldReader<u16>;
#[doc = "Field `OFFSETANA2NORM` reader - No Description"]
pub type Offsetana2normR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - No Description"]
    #[inline(always)]
    pub fn offsetana1norm(&self) -> Offsetana1normR {
        Offsetana1normR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - No Description"]
    #[inline(always)]
    pub fn offsetana2norm(&self) -> Offsetana2normR {
        Offsetana2normR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "IADC0 Normal Offset Calibration Info\n\nYou can [`read`](crate::Reg::read) this register and get [`iadc0normaloffsetcal0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iadc0normaloffsetcal0Spec;
impl crate::RegisterSpec for Iadc0normaloffsetcal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iadc0normaloffsetcal0::R`](R) reader structure"]
impl crate::Readable for Iadc0normaloffsetcal0Spec {}
#[doc = "`reset()` method sets IADC0NORMALOFFSETCAL0 to value 0"]
impl crate::Resettable for Iadc0normaloffsetcal0Spec {}
