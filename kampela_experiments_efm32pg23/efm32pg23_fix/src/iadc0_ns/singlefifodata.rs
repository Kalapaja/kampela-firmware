#[doc = "Register `SINGLEFIFODATA` reader"]
pub type R = crate::R<SinglefifodataSpec>;
#[doc = "Field `DATA` reader - Single FIFO Read Data"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Single FIFO Read Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Read the oldest valid data from the single FIFO and pop the FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`singlefifodata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SinglefifodataSpec;
impl crate::RegisterSpec for SinglefifodataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singlefifodata::R`](R) reader structure"]
impl crate::Readable for SinglefifodataSpec {}
#[doc = "`reset()` method sets SINGLEFIFODATA to value 0"]
impl crate::Resettable for SinglefifodataSpec {}
