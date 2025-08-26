#[doc = "Register `SCANFIFODATA` reader"]
pub type R = crate::R<ScanfifodataSpec>;
#[doc = "Field `DATA` reader - Data"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Read the oldest valid data from the scan FIFO and pop the FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`scanfifodata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScanfifodataSpec;
impl crate::RegisterSpec for ScanfifodataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanfifodata::R`](R) reader structure"]
impl crate::Readable for ScanfifodataSpec {}
#[doc = "`reset()` method sets SCANFIFODATA to value 0"]
impl crate::Resettable for ScanfifodataSpec {}
