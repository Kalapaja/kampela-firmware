#[doc = "Register `TX_HEADER` writer"]
pub type W = crate::W<TxHeaderSpec>;
#[doc = "Field `TXHEADER` writer - TXHEADER"]
pub type TxheaderW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - TXHEADER"]
    #[inline(always)]
    pub fn txheader(&mut self) -> TxheaderW<TxHeaderSpec> {
        TxheaderW::new(self, 0)
    }
}
#[doc = "A write access to this register will be mapped to the TX FIFO (only for header).\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_header::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxHeaderSpec;
impl crate::RegisterSpec for TxHeaderSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tx_header::W`](W) writer structure"]
impl crate::Writable for TxHeaderSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_HEADER to value 0"]
impl crate::Resettable for TxHeaderSpec {}
