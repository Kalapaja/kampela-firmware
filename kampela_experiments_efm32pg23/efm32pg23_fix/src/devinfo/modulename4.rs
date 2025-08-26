#[doc = "Register `MODULENAME4` reader"]
pub type R = crate::R<Modulename4Spec>;
#[doc = "Field `MODCHAR17` reader - No Description"]
pub type Modchar17R = crate::FieldReader;
#[doc = "Field `MODCHAR18` reader - No Description"]
pub type Modchar18R = crate::FieldReader;
#[doc = "Field `MODCHAR19` reader - No Description"]
pub type Modchar19R = crate::FieldReader;
#[doc = "Field `MODCHAR20` reader - No Description"]
pub type Modchar20R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn modchar17(&self) -> Modchar17R {
        Modchar17R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn modchar18(&self) -> Modchar18R {
        Modchar18R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - No Description"]
    #[inline(always)]
    pub fn modchar19(&self) -> Modchar19R {
        Modchar19R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - No Description"]
    #[inline(always)]
    pub fn modchar20(&self) -> Modchar20R {
        Modchar20R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Characters 17-20 of Module Name stored as a null terminated string\n\nYou can [`read`](crate::Reg::read) this register and get [`modulename4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Modulename4Spec;
impl crate::RegisterSpec for Modulename4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modulename4::R`](R) reader structure"]
impl crate::Readable for Modulename4Spec {}
#[doc = "`reset()` method sets MODULENAME4 to value 0xffff_ffff"]
impl crate::Resettable for Modulename4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
