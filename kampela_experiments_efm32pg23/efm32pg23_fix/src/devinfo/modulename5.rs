#[doc = "Register `MODULENAME5` reader"]
pub type R = crate::R<Modulename5Spec>;
#[doc = "Field `MODCHAR21` reader - No Description"]
pub type Modchar21R = crate::FieldReader;
#[doc = "Field `MODCHAR22` reader - No Description"]
pub type Modchar22R = crate::FieldReader;
#[doc = "Field `MODCHAR23` reader - No Description"]
pub type Modchar23R = crate::FieldReader;
#[doc = "Field `MODCHAR24` reader - No Description"]
pub type Modchar24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn modchar21(&self) -> Modchar21R {
        Modchar21R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn modchar22(&self) -> Modchar22R {
        Modchar22R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - No Description"]
    #[inline(always)]
    pub fn modchar23(&self) -> Modchar23R {
        Modchar23R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - No Description"]
    #[inline(always)]
    pub fn modchar24(&self) -> Modchar24R {
        Modchar24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Characters 21-24 of Module Name stored as a null terminated string\n\nYou can [`read`](crate::Reg::read) this register and get [`modulename5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Modulename5Spec;
impl crate::RegisterSpec for Modulename5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modulename5::R`](R) reader structure"]
impl crate::Readable for Modulename5Spec {}
#[doc = "`reset()` method sets MODULENAME5 to value 0xffff_ffff"]
impl crate::Resettable for Modulename5Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
