#[doc = "Register `RXDATAP` reader"]
pub type R = crate::R<RxdatapSpec>;
#[doc = "Field `RXDATAP` reader - RX Data Peek"]
pub type RxdatapR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX Data Peek"]
    #[inline(always)]
    pub fn rxdatap(&self) -> RxdatapR {
        RxdatapR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdatap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdatapSpec;
impl crate::RegisterSpec for RxdatapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdatap::R`](R) reader structure"]
impl crate::Readable for RxdatapSpec {}
#[doc = "`reset()` method sets RXDATAP to value 0"]
impl crate::Resettable for RxdatapSpec {}
