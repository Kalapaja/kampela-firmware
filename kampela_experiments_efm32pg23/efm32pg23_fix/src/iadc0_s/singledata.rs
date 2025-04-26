#[doc = "Register `SINGLEDATA` reader"]
pub type R = crate::R<SingledataSpec>;
#[doc = "Field `DATA` reader - Data"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "latest single queue conversion data\n\nYou can [`read`](crate::Reg::read) this register and get [`singledata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SingledataSpec;
impl crate::RegisterSpec for SingledataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singledata::R`](R) reader structure"]
impl crate::Readable for SingledataSpec {}
#[doc = "`reset()` method sets SINGLEDATA to value 0"]
impl crate::Resettable for SingledataSpec {}
