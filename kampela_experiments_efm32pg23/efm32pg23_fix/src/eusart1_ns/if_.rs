#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `TXC` reader - TX Complete Interrupt Flag"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - TX Complete Interrupt Flag"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFL` reader - TX FIFO Level Interrupt Flag"]
pub type TxflR = crate::BitReader;
#[doc = "Field `TXFL` writer - TX FIFO Level Interrupt Flag"]
pub type TxflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFL` reader - RX FIFO Level Interrupt Flag"]
pub type RxflR = crate::BitReader;
#[doc = "Field `RXFL` writer - RX FIFO Level Interrupt Flag"]
pub type RxflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - RX FIFO Full Interrupt Flag"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXFULL` writer - RX FIFO Full Interrupt Flag"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` reader - RX FIFO Overflow Interrupt Flag"]
pub type RxofR = crate::BitReader;
#[doc = "Field `RXOF` writer - RX FIFO Overflow Interrupt Flag"]
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` reader - RX FIFO Underflow Interrupt Flag"]
pub type RxufR = crate::BitReader;
#[doc = "Field `RXUF` writer - RX FIFO Underflow Interrupt Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` reader - TX FIFO Overflow Interrupt Flag"]
pub type TxofR = crate::BitReader;
#[doc = "Field `TXOF` writer - TX FIFO Overflow Interrupt Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUF` reader - TX FIFO Underflow Interrupt Flag"]
pub type TxufR = crate::BitReader;
#[doc = "Field `TXUF` writer - TX FIFO Underflow Interrupt Flag"]
pub type TxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` reader - Parity Error Interrupt Flag"]
pub type PerrR = crate::BitReader;
#[doc = "Field `PERR` writer - Parity Error Interrupt Flag"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` reader - Framing Error Interrupt Flag"]
pub type FerrR = crate::BitReader;
#[doc = "Field `FERR` writer - Framing Error Interrupt Flag"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAF` reader - Multi-Processor Address Frame Interrupt"]
pub type MpafR = crate::BitReader;
#[doc = "Field `MPAF` writer - Multi-Processor Address Frame Interrupt"]
pub type MpafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOADERR` reader - Load Error Interrupt Flag"]
pub type LoaderrR = crate::BitReader;
#[doc = "Field `LOADERR` writer - Load Error Interrupt Flag"]
pub type LoaderrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCF` reader - Collision Check Fail Interrupt Flag"]
pub type CcfR = crate::BitReader;
#[doc = "Field `CCF` writer - Collision Check Fail Interrupt Flag"]
pub type CcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIDLE` reader - TX Idle Interrupt Flag"]
pub type TxidleR = crate::BitReader;
#[doc = "Field `TXIDLE` writer - TX Idle Interrupt Flag"]
pub type TxidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTF` reader - Start Frame Interrupt Flag"]
pub type StartfR = crate::BitReader;
#[doc = "Field `STARTF` writer - Start Frame Interrupt Flag"]
pub type StartfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIGF` reader - Signal Frame Interrupt Flag"]
pub type SigfR = crate::BitReader;
#[doc = "Field `SIGF` writer - Signal Frame Interrupt Flag"]
pub type SigfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOBAUDDONE` reader - Auto Baud Complete Interrupt Flag"]
pub type AutobauddoneR = crate::BitReader;
#[doc = "Field `AUTOBAUDDONE` writer - Auto Baud Complete Interrupt Flag"]
pub type AutobauddoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTO` reader - RX Timeout Interrupt Flag"]
pub type RxtoR = crate::BitReader;
#[doc = "Field `RXTO` writer - RX Timeout Interrupt Flag"]
pub type RxtoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Level Interrupt Flag"]
    #[inline(always)]
    pub fn txfl(&self) -> TxflR {
        TxflR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Level Interrupt Flag"]
    #[inline(always)]
    pub fn rxfl(&self) -> RxflR {
        RxflR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&self) -> RxofR {
        RxofR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn txuf(&self) -> TxufR {
        TxufR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multi-Processor Address Frame Interrupt"]
    #[inline(always)]
    pub fn mpaf(&self) -> MpafR {
        MpafR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Load Error Interrupt Flag"]
    #[inline(always)]
    pub fn loaderr(&self) -> LoaderrR {
        LoaderrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Collision Check Fail Interrupt Flag"]
    #[inline(always)]
    pub fn ccf(&self) -> CcfR {
        CcfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX Idle Interrupt Flag"]
    #[inline(always)]
    pub fn txidle(&self) -> TxidleR {
        TxidleR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 18 - Start Frame Interrupt Flag"]
    #[inline(always)]
    pub fn startf(&self) -> StartfR {
        StartfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Signal Frame Interrupt Flag"]
    #[inline(always)]
    pub fn sigf(&self) -> SigfR {
        SigfR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto Baud Complete Interrupt Flag"]
    #[inline(always)]
    pub fn autobauddone(&self) -> AutobauddoneR {
        AutobauddoneR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RX Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn rxto(&self) -> RxtoR {
        RxtoR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<IfSpec> {
        TxcW::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO Level Interrupt Flag"]
    #[inline(always)]
    pub fn txfl(&mut self) -> TxflW<IfSpec> {
        TxflW::new(self, 1)
    }
    #[doc = "Bit 2 - RX FIFO Level Interrupt Flag"]
    #[inline(always)]
    pub fn rxfl(&mut self) -> RxflW<IfSpec> {
        RxflW::new(self, 2)
    }
    #[doc = "Bit 3 - RX FIFO Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<IfSpec> {
        RxfullW::new(self, 3)
    }
    #[doc = "Bit 4 - RX FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RxofW<IfSpec> {
        RxofW::new(self, 4)
    }
    #[doc = "Bit 5 - RX FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<IfSpec> {
        RxufW::new(self, 5)
    }
    #[doc = "Bit 6 - TX FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<IfSpec> {
        TxofW::new(self, 6)
    }
    #[doc = "Bit 7 - TX FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn txuf(&mut self) -> TxufW<IfSpec> {
        TxufW::new(self, 7)
    }
    #[doc = "Bit 8 - Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<IfSpec> {
        PerrW::new(self, 8)
    }
    #[doc = "Bit 9 - Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<IfSpec> {
        FerrW::new(self, 9)
    }
    #[doc = "Bit 10 - Multi-Processor Address Frame Interrupt"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MpafW<IfSpec> {
        MpafW::new(self, 10)
    }
    #[doc = "Bit 11 - Load Error Interrupt Flag"]
    #[inline(always)]
    pub fn loaderr(&mut self) -> LoaderrW<IfSpec> {
        LoaderrW::new(self, 11)
    }
    #[doc = "Bit 12 - Collision Check Fail Interrupt Flag"]
    #[inline(always)]
    pub fn ccf(&mut self) -> CcfW<IfSpec> {
        CcfW::new(self, 12)
    }
    #[doc = "Bit 13 - TX Idle Interrupt Flag"]
    #[inline(always)]
    pub fn txidle(&mut self) -> TxidleW<IfSpec> {
        TxidleW::new(self, 13)
    }
    #[doc = "Bit 18 - Start Frame Interrupt Flag"]
    #[inline(always)]
    pub fn startf(&mut self) -> StartfW<IfSpec> {
        StartfW::new(self, 18)
    }
    #[doc = "Bit 19 - Signal Frame Interrupt Flag"]
    #[inline(always)]
    pub fn sigf(&mut self) -> SigfW<IfSpec> {
        SigfW::new(self, 19)
    }
    #[doc = "Bit 24 - Auto Baud Complete Interrupt Flag"]
    #[inline(always)]
    pub fn autobauddone(&mut self) -> AutobauddoneW<IfSpec> {
        AutobauddoneW::new(self, 24)
    }
    #[doc = "Bit 25 - RX Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn rxto(&mut self) -> RxtoW<IfSpec> {
        RxtoW::new(self, 25)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`write(|w| ..)` method takes [`if_::W`](W) writer structure"]
impl crate::Writable for IfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
