#[doc = "Register `REQPEND` reader"]
pub type R = crate::R<ReqpendSpec>;
#[doc = "Field `REQPEND` reader - DMA Requests Pending"]
pub type ReqpendR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - DMA Requests Pending"]
    #[inline(always)]
    pub fn reqpend(&self) -> ReqpendR {
        ReqpendR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`reqpend::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqpendSpec;
impl crate::RegisterSpec for ReqpendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reqpend::R`](R) reader structure"]
impl crate::Readable for ReqpendSpec {}
#[doc = "`reset()` method sets REQPEND to value 0"]
impl crate::Resettable for ReqpendSpec {}
