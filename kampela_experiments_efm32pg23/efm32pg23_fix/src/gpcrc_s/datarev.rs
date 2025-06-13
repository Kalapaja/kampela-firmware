#[doc = "Register `DATAREV` reader"]
pub type R = crate::R<DatarevSpec>;
#[doc = "Field `DATAREV` reader - Data Reverse Value"]
pub type DatarevR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data Reverse Value"]
    #[inline(always)]
    pub fn datarev(&self) -> DatarevR {
        DatarevR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`datarev::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatarevSpec;
impl crate::RegisterSpec for DatarevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datarev::R`](R) reader structure"]
impl crate::Readable for DatarevSpec {}
#[doc = "`reset()` method sets DATAREV to value 0"]
impl crate::Resettable for DatarevSpec {}
