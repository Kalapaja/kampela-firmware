#[doc = "Register `IADC0GAIN1` reader"]
pub type R = crate::R<Iadc0gain1Spec>;
#[doc = "Field `GAINCANA3` reader - No Description"]
pub type Gaincana3R = crate::FieldReader<u16>;
#[doc = "Field `GAINCANA4` reader - No Description"]
pub type Gaincana4R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - No Description"]
    #[inline(always)]
    pub fn gaincana3(&self) -> Gaincana3R {
        Gaincana3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - No Description"]
    #[inline(always)]
    pub fn gaincana4(&self) -> Gaincana4R {
        Gaincana4R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "IADC0 Gain Calibration Info\n\nYou can [`read`](crate::Reg::read) this register and get [`iadc0gain1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iadc0gain1Spec;
impl crate::RegisterSpec for Iadc0gain1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iadc0gain1::R`](R) reader structure"]
impl crate::Readable for Iadc0gain1Spec {}
#[doc = "`reset()` method sets IADC0GAIN1 to value 0"]
impl crate::Resettable for Iadc0gain1Spec {}
