#[doc = "Register `SCANDATA` reader"]
pub type R = crate::R<ScandataSpec>;
#[doc = "Field `DATA` reader - Data"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Most recent data data from scan queue conversion\n\nYou can [`read`](crate::Reg::read) this register and get [`scandata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScandataSpec;
impl crate::RegisterSpec for ScandataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scandata::R`](R) reader structure"]
impl crate::Readable for ScandataSpec {}
#[doc = "`reset()` method sets SCANDATA to value 0"]
impl crate::Resettable for ScandataSpec {}
