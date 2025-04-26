#[doc = "Register `EUI64L` reader"]
pub type R = crate::R<Eui64lSpec>;
#[doc = "Field `UNIQUEL` reader - UNIQUEL"]
pub type UniquelR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - UNIQUEL"]
    #[inline(always)]
    pub fn uniquel(&self) -> UniquelR {
        UniquelR::new(self.bits)
    }
}
#[doc = "MA-L compliant EUI64 Unique Identifier (low bits)\n\nYou can [`read`](crate::Reg::read) this register and get [`eui64l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eui64lSpec;
impl crate::RegisterSpec for Eui64lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eui64l::R`](R) reader structure"]
impl crate::Readable for Eui64lSpec {}
#[doc = "`reset()` method sets EUI64L to value 0"]
impl crate::Resettable for Eui64lSpec {}
