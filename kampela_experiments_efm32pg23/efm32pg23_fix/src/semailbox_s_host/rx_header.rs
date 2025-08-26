#[doc = "Register `RX_HEADER` reader"]
pub type R = crate::R<RxHeaderSpec>;
#[doc = "Field `RXHEADER` reader - RXHEADER"]
pub type RxheaderR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RXHEADER"]
    #[inline(always)]
    pub fn rxheader(&self) -> RxheaderR {
        RxheaderR::new(self.bits)
    }
}
#[doc = "A read access to this register will be mapped to the RX FIFO (only for the header).\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_header::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxHeaderSpec;
impl crate::RegisterSpec for RxHeaderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_header::R`](R) reader structure"]
impl crate::Readable for RxHeaderSpec {}
#[doc = "`reset()` method sets RX_HEADER to value 0"]
impl crate::Resettable for RxHeaderSpec {}
