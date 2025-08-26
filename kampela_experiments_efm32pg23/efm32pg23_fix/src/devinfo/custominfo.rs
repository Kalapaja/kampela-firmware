#[doc = "Register `CUSTOMINFO` reader"]
pub type R = crate::R<CustominfoSpec>;
#[doc = "Field `PARTNO` reader - Part Number"]
pub type PartnoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 16:31 - Part Number"]
    #[inline(always)]
    pub fn partno(&self) -> PartnoR {
        PartnoR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Custom information\n\nYou can [`read`](crate::Reg::read) this register and get [`custominfo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CustominfoSpec;
impl crate::RegisterSpec for CustominfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`custominfo::R`](R) reader structure"]
impl crate::Readable for CustominfoSpec {}
#[doc = "`reset()` method sets CUSTOMINFO to value 0"]
impl crate::Resettable for CustominfoSpec {}
