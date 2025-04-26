#[doc = "Register `CC2_ICOF` reader"]
pub type R = crate::R<Cc2IcofSpec>;
#[doc = "Field `ICOF` reader - Input Capture FIFO Overflow"]
pub type IcofR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Input Capture FIFO Overflow"]
    #[inline(always)]
    pub fn icof(&self) -> IcofR {
        IcofR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_icof::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc2IcofSpec;
impl crate::RegisterSpec for Cc2IcofSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2_icof::R`](R) reader structure"]
impl crate::Readable for Cc2IcofSpec {}
#[doc = "`reset()` method sets CC2_ICOF to value 0"]
impl crate::Resettable for Cc2IcofSpec {}
