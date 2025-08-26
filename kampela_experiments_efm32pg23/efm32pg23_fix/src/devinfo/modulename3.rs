#[doc = "Register `MODULENAME3` reader"]
pub type R = crate::R<Modulename3Spec>;
#[doc = "Field `MODCHAR13` reader - No Description"]
pub type Modchar13R = crate::FieldReader;
#[doc = "Field `MODCHAR14` reader - No Description"]
pub type Modchar14R = crate::FieldReader;
#[doc = "Field `MODCHAR15` reader - No Description"]
pub type Modchar15R = crate::FieldReader;
#[doc = "Field `MODCHAR16` reader - No Description"]
pub type Modchar16R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn modchar13(&self) -> Modchar13R {
        Modchar13R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn modchar14(&self) -> Modchar14R {
        Modchar14R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - No Description"]
    #[inline(always)]
    pub fn modchar15(&self) -> Modchar15R {
        Modchar15R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - No Description"]
    #[inline(always)]
    pub fn modchar16(&self) -> Modchar16R {
        Modchar16R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Characters 13-16 of Module Name stored as a null terminated string\n\nYou can [`read`](crate::Reg::read) this register and get [`modulename3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Modulename3Spec;
impl crate::RegisterSpec for Modulename3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modulename3::R`](R) reader structure"]
impl crate::Readable for Modulename3Spec {}
#[doc = "`reset()` method sets MODULENAME3 to value 0xffff_ffff"]
impl crate::Resettable for Modulename3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
