#[doc = "Register `MODULENAME0` reader"]
pub type R = crate::R<Modulename0Spec>;
#[doc = "Field `MODCHAR1` reader - No Description"]
pub type Modchar1R = crate::FieldReader;
#[doc = "Field `MODCHAR2` reader - No Description"]
pub type Modchar2R = crate::FieldReader;
#[doc = "Field `MODCHAR3` reader - No Description"]
pub type Modchar3R = crate::FieldReader;
#[doc = "Field `MODCHAR4` reader - No Description"]
pub type Modchar4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn modchar1(&self) -> Modchar1R {
        Modchar1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn modchar2(&self) -> Modchar2R {
        Modchar2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - No Description"]
    #[inline(always)]
    pub fn modchar3(&self) -> Modchar3R {
        Modchar3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - No Description"]
    #[inline(always)]
    pub fn modchar4(&self) -> Modchar4R {
        Modchar4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Characters 1-4 of Module Name stored as a null terminated string\n\nYou can [`read`](crate::Reg::read) this register and get [`modulename0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Modulename0Spec;
impl crate::RegisterSpec for Modulename0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modulename0::R`](R) reader structure"]
impl crate::Readable for Modulename0Spec {}
#[doc = "`reset()` method sets MODULENAME0 to value 0xffff_ffff"]
impl crate::Resettable for Modulename0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
