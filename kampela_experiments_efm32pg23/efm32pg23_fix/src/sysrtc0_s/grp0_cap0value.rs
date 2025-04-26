#[doc = "Register `GRP0_CAP0VALUE` reader"]
pub type R = crate::R<Grp0Cap0valueSpec>;
#[doc = "Field `CAP0VALUE` reader - Capture 0 Value"]
pub type Cap0valueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Capture 0 Value"]
    #[inline(always)]
    pub fn cap0value(&self) -> Cap0valueR {
        Cap0valueR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grp0_cap0value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Grp0Cap0valueSpec;
impl crate::RegisterSpec for Grp0Cap0valueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp0_cap0value::R`](R) reader structure"]
impl crate::Readable for Grp0Cap0valueSpec {}
#[doc = "`reset()` method sets GRP0_CAP0VALUE to value 0"]
impl crate::Resettable for Grp0Cap0valueSpec {}
