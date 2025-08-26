#[doc = "Register `MODULENAME1` reader"]
pub type R = crate::R<Modulename1Spec>;
#[doc = "Field `MODCHAR5` reader - No Description"]
pub type Modchar5R = crate::FieldReader;
#[doc = "Field `MODCHAR6` reader - No Description"]
pub type Modchar6R = crate::FieldReader;
#[doc = "Field `MODCHAR7` reader - No Description"]
pub type Modchar7R = crate::FieldReader;
#[doc = "Field `MODCHAR8` reader - No Description"]
pub type Modchar8R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn modchar5(&self) -> Modchar5R {
        Modchar5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn modchar6(&self) -> Modchar6R {
        Modchar6R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - No Description"]
    #[inline(always)]
    pub fn modchar7(&self) -> Modchar7R {
        Modchar7R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - No Description"]
    #[inline(always)]
    pub fn modchar8(&self) -> Modchar8R {
        Modchar8R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Characters 5-8 of Module Name stored as a null terminated string\n\nYou can [`read`](crate::Reg::read) this register and get [`modulename1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Modulename1Spec;
impl crate::RegisterSpec for Modulename1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modulename1::R`](R) reader structure"]
impl crate::Readable for Modulename1Spec {}
#[doc = "`reset()` method sets MODULENAME1 to value 0xffff_ffff"]
impl crate::Resettable for Modulename1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
