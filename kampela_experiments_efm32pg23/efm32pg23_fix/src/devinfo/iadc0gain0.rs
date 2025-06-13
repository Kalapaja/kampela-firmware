#[doc = "Register `IADC0GAIN0` reader"]
pub type R = crate::R<Iadc0gain0Spec>;
#[doc = "Field `GAINCANA1` reader - No Description"]
pub type Gaincana1R = crate::FieldReader<u16>;
#[doc = "Field `GAINCANA2` reader - No Description"]
pub type Gaincana2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - No Description"]
    #[inline(always)]
    pub fn gaincana1(&self) -> Gaincana1R {
        Gaincana1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - No Description"]
    #[inline(always)]
    pub fn gaincana2(&self) -> Gaincana2R {
        Gaincana2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "IADC0 Gain Calibration Info\n\nYou can [`read`](crate::Reg::read) this register and get [`iadc0gain0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iadc0gain0Spec;
impl crate::RegisterSpec for Iadc0gain0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iadc0gain0::R`](R) reader structure"]
impl crate::Readable for Iadc0gain0Spec {}
#[doc = "`reset()` method sets IADC0GAIN0 to value 0"]
impl crate::Resettable for Iadc0gain0Spec {}
