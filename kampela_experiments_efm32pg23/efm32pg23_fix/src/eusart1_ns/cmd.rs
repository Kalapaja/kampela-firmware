#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBLOCKEN` writer - Receiver Block Enable"]
pub type RxblockenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBLOCKDIS` writer - Receiver Block Disable"]
pub type RxblockdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIEN` writer - Transmitter Tristate Enable"]
pub type TxtrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIDIS` writer - Transmitter Tristate Disable"]
pub type TxtridisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARTX` writer - Clear TX FIFO"]
pub type CleartxW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<CmdSpec> {
        RxenW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver Disable"]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RxdisW<CmdSpec> {
        RxdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<CmdSpec> {
        TxenW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Disable"]
    #[inline(always)]
    pub fn txdis(&mut self) -> TxdisW<CmdSpec> {
        TxdisW::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Block Enable"]
    #[inline(always)]
    pub fn rxblocken(&mut self) -> RxblockenW<CmdSpec> {
        RxblockenW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Block Disable"]
    #[inline(always)]
    pub fn rxblockdis(&mut self) -> RxblockdisW<CmdSpec> {
        RxblockdisW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmitter Tristate Enable"]
    #[inline(always)]
    pub fn txtrien(&mut self) -> TxtrienW<CmdSpec> {
        TxtrienW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmitter Tristate Disable"]
    #[inline(always)]
    pub fn txtridis(&mut self) -> TxtridisW<CmdSpec> {
        TxtridisW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear TX FIFO"]
    #[inline(always)]
    pub fn cleartx(&mut self) -> CleartxW<CmdSpec> {
        CleartxW::new(self, 8)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {}
