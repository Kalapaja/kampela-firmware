#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `TXC` reader - TX Complete Enable"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - TX Complete Enable"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFL` reader - TX FIFO Level Enable"]
pub type TxflR = crate::BitReader;
#[doc = "Field `TXFL` writer - TX FIFO Level Enable"]
pub type TxflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFL` reader - RX FIFO Level Enable"]
pub type RxflR = crate::BitReader;
#[doc = "Field `RXFL` writer - RX FIFO Level Enable"]
pub type RxflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - RX FIFO Full Enable"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXFULL` writer - RX FIFO Full Enable"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` reader - RX FIFO Overflow Enable"]
pub type RxofR = crate::BitReader;
#[doc = "Field `RXOF` writer - RX FIFO Overflow Enable"]
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` reader - RX FIFO Underflow Enable"]
pub type RxufR = crate::BitReader;
#[doc = "Field `RXUF` writer - RX FIFO Underflow Enable"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` reader - TX FIFO Overflow Enable"]
pub type TxofR = crate::BitReader;
#[doc = "Field `TXOF` writer - TX FIFO Overflow Enable"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUF` reader - TX FIFO Underflow Enable"]
pub type TxufR = crate::BitReader;
#[doc = "Field `TXUF` writer - TX FIFO Underflow Enable"]
pub type TxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` reader - Parity Error Enable"]
pub type PerrR = crate::BitReader;
#[doc = "Field `PERR` writer - Parity Error Enable"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` reader - Framing Error Enable"]
pub type FerrR = crate::BitReader;
#[doc = "Field `FERR` writer - Framing Error Enable"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAF` reader - Multi-Processor Addr Frame Enable"]
pub type MpafR = crate::BitReader;
#[doc = "Field `MPAF` writer - Multi-Processor Addr Frame Enable"]
pub type MpafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOADERR` reader - Load Error Enable"]
pub type LoaderrR = crate::BitReader;
#[doc = "Field `LOADERR` writer - Load Error Enable"]
pub type LoaderrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCF` reader - Collision Check Fail Enable"]
pub type CcfR = crate::BitReader;
#[doc = "Field `CCF` writer - Collision Check Fail Enable"]
pub type CcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIDLE` reader - TX IDLE Enable"]
pub type TxidleR = crate::BitReader;
#[doc = "Field `TXIDLE` writer - TX IDLE Enable"]
pub type TxidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSWU` reader - CS Wake-up Enable"]
pub type CswuR = crate::BitReader;
#[doc = "Field `CSWU` writer - CS Wake-up Enable"]
pub type CswuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTF` reader - Start Frame Enable"]
pub type StartfR = crate::BitReader;
#[doc = "Field `STARTF` writer - Start Frame Enable"]
pub type StartfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIGF` reader - Signal Frame Enable"]
pub type SigfR = crate::BitReader;
#[doc = "Field `SIGF` writer - Signal Frame Enable"]
pub type SigfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOBAUDDONE` reader - Auto Baud Complete Enable"]
pub type AutobauddoneR = crate::BitReader;
#[doc = "Field `AUTOBAUDDONE` writer - Auto Baud Complete Enable"]
pub type AutobauddoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTO` reader - RX Timeout Enable"]
pub type RxtoR = crate::BitReader;
#[doc = "Field `RXTO` writer - RX Timeout Enable"]
pub type RxtoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX Complete Enable"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Level Enable"]
    #[inline(always)]
    pub fn txfl(&self) -> TxflR {
        TxflR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Level Enable"]
    #[inline(always)]
    pub fn rxfl(&self) -> RxflR {
        RxflR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Full Enable"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Overflow Enable"]
    #[inline(always)]
    pub fn rxof(&self) -> RxofR {
        RxofR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX FIFO Underflow Enable"]
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX FIFO Overflow Enable"]
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX FIFO Underflow Enable"]
    #[inline(always)]
    pub fn txuf(&self) -> TxufR {
        TxufR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Enable"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Framing Error Enable"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multi-Processor Addr Frame Enable"]
    #[inline(always)]
    pub fn mpaf(&self) -> MpafR {
        MpafR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Load Error Enable"]
    #[inline(always)]
    pub fn loaderr(&self) -> LoaderrR {
        LoaderrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Collision Check Fail Enable"]
    #[inline(always)]
    pub fn ccf(&self) -> CcfR {
        CcfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX IDLE Enable"]
    #[inline(always)]
    pub fn txidle(&self) -> TxidleR {
        TxidleR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - CS Wake-up Enable"]
    #[inline(always)]
    pub fn cswu(&self) -> CswuR {
        CswuR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Start Frame Enable"]
    #[inline(always)]
    pub fn startf(&self) -> StartfR {
        StartfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Signal Frame Enable"]
    #[inline(always)]
    pub fn sigf(&self) -> SigfR {
        SigfR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto Baud Complete Enable"]
    #[inline(always)]
    pub fn autobauddone(&self) -> AutobauddoneR {
        AutobauddoneR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RX Timeout Enable"]
    #[inline(always)]
    pub fn rxto(&self) -> RxtoR {
        RxtoR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Complete Enable"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<IenSpec> {
        TxcW::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO Level Enable"]
    #[inline(always)]
    pub fn txfl(&mut self) -> TxflW<IenSpec> {
        TxflW::new(self, 1)
    }
    #[doc = "Bit 2 - RX FIFO Level Enable"]
    #[inline(always)]
    pub fn rxfl(&mut self) -> RxflW<IenSpec> {
        RxflW::new(self, 2)
    }
    #[doc = "Bit 3 - RX FIFO Full Enable"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<IenSpec> {
        RxfullW::new(self, 3)
    }
    #[doc = "Bit 4 - RX FIFO Overflow Enable"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RxofW<IenSpec> {
        RxofW::new(self, 4)
    }
    #[doc = "Bit 5 - RX FIFO Underflow Enable"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<IenSpec> {
        RxufW::new(self, 5)
    }
    #[doc = "Bit 6 - TX FIFO Overflow Enable"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<IenSpec> {
        TxofW::new(self, 6)
    }
    #[doc = "Bit 7 - TX FIFO Underflow Enable"]
    #[inline(always)]
    pub fn txuf(&mut self) -> TxufW<IenSpec> {
        TxufW::new(self, 7)
    }
    #[doc = "Bit 8 - Parity Error Enable"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<IenSpec> {
        PerrW::new(self, 8)
    }
    #[doc = "Bit 9 - Framing Error Enable"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<IenSpec> {
        FerrW::new(self, 9)
    }
    #[doc = "Bit 10 - Multi-Processor Addr Frame Enable"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MpafW<IenSpec> {
        MpafW::new(self, 10)
    }
    #[doc = "Bit 11 - Load Error Enable"]
    #[inline(always)]
    pub fn loaderr(&mut self) -> LoaderrW<IenSpec> {
        LoaderrW::new(self, 11)
    }
    #[doc = "Bit 12 - Collision Check Fail Enable"]
    #[inline(always)]
    pub fn ccf(&mut self) -> CcfW<IenSpec> {
        CcfW::new(self, 12)
    }
    #[doc = "Bit 13 - TX IDLE Enable"]
    #[inline(always)]
    pub fn txidle(&mut self) -> TxidleW<IenSpec> {
        TxidleW::new(self, 13)
    }
    #[doc = "Bit 16 - CS Wake-up Enable"]
    #[inline(always)]
    pub fn cswu(&mut self) -> CswuW<IenSpec> {
        CswuW::new(self, 16)
    }
    #[doc = "Bit 18 - Start Frame Enable"]
    #[inline(always)]
    pub fn startf(&mut self) -> StartfW<IenSpec> {
        StartfW::new(self, 18)
    }
    #[doc = "Bit 19 - Signal Frame Enable"]
    #[inline(always)]
    pub fn sigf(&mut self) -> SigfW<IenSpec> {
        SigfW::new(self, 19)
    }
    #[doc = "Bit 24 - Auto Baud Complete Enable"]
    #[inline(always)]
    pub fn autobauddone(&mut self) -> AutobauddoneW<IenSpec> {
        AutobauddoneW::new(self, 24)
    }
    #[doc = "Bit 25 - RX Timeout Enable"]
    #[inline(always)]
    pub fn rxto(&mut self) -> RxtoW<IenSpec> {
        RxtoW::new(self, 25)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
