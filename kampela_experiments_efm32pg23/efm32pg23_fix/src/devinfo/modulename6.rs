#[doc = "Register `MODULENAME6` reader"]
pub type R = crate::R<Modulename6Spec>;
#[doc = "Field `MODCHAR25` reader - No Description"]
pub type Modchar25R = crate::FieldReader;
#[doc = "Field `MODCHAR26` reader - No Description"]
pub type Modchar26R = crate::FieldReader;
#[doc = "Field `RSV` reader - No Description"]
pub type RsvR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn modchar25(&self) -> Modchar25R {
        Modchar25R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn modchar26(&self) -> Modchar26R {
        Modchar26R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - No Description"]
    #[inline(always)]
    pub fn rsv(&self) -> RsvR {
        RsvR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Characters 25-26 of Module Name stored as a null terminated string\n\nYou can [`read`](crate::Reg::read) this register and get [`modulename6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Modulename6Spec;
impl crate::RegisterSpec for Modulename6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modulename6::R`](R) reader structure"]
impl crate::Readable for Modulename6Spec {}
#[doc = "`reset()` method sets MODULENAME6 to value 0xffff_ffff"]
impl crate::Resettable for Modulename6Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
