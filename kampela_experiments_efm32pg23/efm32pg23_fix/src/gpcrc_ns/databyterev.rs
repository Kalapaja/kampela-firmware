#[doc = "Register `DATABYTEREV` reader"]
pub type R = crate::R<DatabyterevSpec>;
#[doc = "Field `DATABYTEREV` reader - Data Byte Reverse Value"]
pub type DatabyterevR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data Byte Reverse Value"]
    #[inline(always)]
    pub fn databyterev(&self) -> DatabyterevR {
        DatabyterevR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`databyterev::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatabyterevSpec;
impl crate::RegisterSpec for DatabyterevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`databyterev::R`](R) reader structure"]
impl crate::Readable for DatabyterevSpec {}
#[doc = "`reset()` method sets DATABYTEREV to value 0"]
impl crate::Resettable for DatabyterevSpec {}
