#[doc = "Register `MODULENAME2` reader"]
pub type R = crate::R<Modulename2Spec>;
#[doc = "Field `MODCHAR9` reader - No Description"]
pub type Modchar9R = crate::FieldReader;
#[doc = "Field `MODCHAR10` reader - No Description"]
pub type Modchar10R = crate::FieldReader;
#[doc = "Field `MODCHAR11` reader - No Description"]
pub type Modchar11R = crate::FieldReader;
#[doc = "Field `MODCHAR12` reader - No Description"]
pub type Modchar12R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn modchar9(&self) -> Modchar9R {
        Modchar9R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn modchar10(&self) -> Modchar10R {
        Modchar10R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - No Description"]
    #[inline(always)]
    pub fn modchar11(&self) -> Modchar11R {
        Modchar11R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - No Description"]
    #[inline(always)]
    pub fn modchar12(&self) -> Modchar12R {
        Modchar12R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Characters 9-12 of Module Name stored as a null terminated string\n\nYou can [`read`](crate::Reg::read) this register and get [`modulename2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Modulename2Spec;
impl crate::RegisterSpec for Modulename2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modulename2::R`](R) reader structure"]
impl crate::Readable for Modulename2Spec {}
#[doc = "`reset()` method sets MODULENAME2 to value 0xffff_ffff"]
impl crate::Resettable for Modulename2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
