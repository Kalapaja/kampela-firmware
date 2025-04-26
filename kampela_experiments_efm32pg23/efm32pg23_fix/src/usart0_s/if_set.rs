#[doc = "Register `IF_SET` reader"]
pub type R = crate::R<IfSetSpec>;
#[doc = "Register `IF_SET` writer"]
pub type W = crate::W<IfSetSpec>;
#[doc = "Field `TXC` reader - TX Complete Interrupt Flag"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - TX Complete Interrupt Flag"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBL` reader - TX Buffer Level Interrupt Flag"]
pub type TxblR = crate::BitReader;
#[doc = "Field `TXBL` writer - TX Buffer Level Interrupt Flag"]
pub type TxblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDATAV` reader - RX Data Valid Interrupt Flag"]
pub type RxdatavR = crate::BitReader;
#[doc = "Field `RXDATAV` writer - RX Data Valid Interrupt Flag"]
pub type RxdatavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - RX Buffer Full Interrupt Flag"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXFULL` writer - RX Buffer Full Interrupt Flag"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` reader - RX Overflow Interrupt Flag"]
pub type RxofR = crate::BitReader;
#[doc = "Field `RXOF` writer - RX Overflow Interrupt Flag"]
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` reader - RX Underflow Interrupt Flag"]
pub type RxufR = crate::BitReader;
#[doc = "Field `RXUF` writer - RX Underflow Interrupt Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` reader - TX Overflow Interrupt Flag"]
pub type TxofR = crate::BitReader;
#[doc = "Field `TXOF` writer - TX Overflow Interrupt Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUF` reader - TX Underflow Interrupt Flag"]
pub type TxufR = crate::BitReader;
#[doc = "Field `TXUF` writer - TX Underflow Interrupt Flag"]
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
#[doc = "Field `SSM` reader - Chip-Select In Main Mode Interrupt Flag"]
pub type SsmR = crate::BitReader;
#[doc = "Field `SSM` writer - Chip-Select In Main Mode Interrupt Flag"]
pub type SsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCF` reader - Collision Check Fail Interrupt Flag"]
pub type CcfR = crate::BitReader;
#[doc = "Field `CCF` writer - Collision Check Fail Interrupt Flag"]
pub type CcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIDLE` reader - TX Idle Interrupt Flag"]
pub type TxidleR = crate::BitReader;
#[doc = "Field `TXIDLE` writer - TX Idle Interrupt Flag"]
pub type TxidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP0` reader - Timer comparator 0 Interrupt Flag"]
pub type Tcmp0R = crate::BitReader;
#[doc = "Field `TCMP0` writer - Timer comparator 0 Interrupt Flag"]
pub type Tcmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP1` reader - Timer comparator 1 Interrupt Flag"]
pub type Tcmp1R = crate::BitReader;
#[doc = "Field `TCMP1` writer - Timer comparator 1 Interrupt Flag"]
pub type Tcmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCMP2` reader - Timer comparator 2 Interrupt Flag"]
pub type Tcmp2R = crate::BitReader;
#[doc = "Field `TCMP2` writer - Timer comparator 2 Interrupt Flag"]
pub type Tcmp2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX Buffer Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&self) -> RxofR {
        RxofR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX Underflow Interrupt Flag"]
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
    #[doc = "Bit 11 - Chip-Select In Main Mode Interrupt Flag"]
    #[inline(always)]
    pub fn ssm(&self) -> SsmR {
        SsmR::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 14 - Timer comparator 0 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp0(&self) -> Tcmp0R {
        Tcmp0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer comparator 1 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp1(&self) -> Tcmp1R {
        Tcmp1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer comparator 2 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp2(&self) -> Tcmp2R {
        Tcmp2R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<IfSetSpec> {
        TxcW::new(self, 0)
    }
    #[doc = "Bit 1 - TX Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn txbl(&mut self) -> TxblW<IfSetSpec> {
        TxblW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn rxdatav(&mut self) -> RxdatavW<IfSetSpec> {
        RxdatavW::new(self, 2)
    }
    #[doc = "Bit 3 - RX Buffer Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<IfSetSpec> {
        RxfullW::new(self, 3)
    }
    #[doc = "Bit 4 - RX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RxofW<IfSetSpec> {
        RxofW::new(self, 4)
    }
    #[doc = "Bit 5 - RX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<IfSetSpec> {
        RxufW::new(self, 5)
    }
    #[doc = "Bit 6 - TX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<IfSetSpec> {
        TxofW::new(self, 6)
    }
    #[doc = "Bit 7 - TX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn txuf(&mut self) -> TxufW<IfSetSpec> {
        TxufW::new(self, 7)
    }
    #[doc = "Bit 8 - Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<IfSetSpec> {
        PerrW::new(self, 8)
    }
    #[doc = "Bit 9 - Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<IfSetSpec> {
        FerrW::new(self, 9)
    }
    #[doc = "Bit 10 - Multi-Processor Address Frame Interrupt"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MpafW<IfSetSpec> {
        MpafW::new(self, 10)
    }
    #[doc = "Bit 11 - Chip-Select In Main Mode Interrupt Flag"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SsmW<IfSetSpec> {
        SsmW::new(self, 11)
    }
    #[doc = "Bit 12 - Collision Check Fail Interrupt Flag"]
    #[inline(always)]
    pub fn ccf(&mut self) -> CcfW<IfSetSpec> {
        CcfW::new(self, 12)
    }
    #[doc = "Bit 13 - TX Idle Interrupt Flag"]
    #[inline(always)]
    pub fn txidle(&mut self) -> TxidleW<IfSetSpec> {
        TxidleW::new(self, 13)
    }
    #[doc = "Bit 14 - Timer comparator 0 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp0(&mut self) -> Tcmp0W<IfSetSpec> {
        Tcmp0W::new(self, 14)
    }
    #[doc = "Bit 15 - Timer comparator 1 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp1(&mut self) -> Tcmp1W<IfSetSpec> {
        Tcmp1W::new(self, 15)
    }
    #[doc = "Bit 16 - Timer comparator 2 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp2(&mut self) -> Tcmp2W<IfSetSpec> {
        Tcmp2W::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`if_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSetSpec;
impl crate::RegisterSpec for IfSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_set::R`](R) reader structure"]
impl crate::Readable for IfSetSpec {}
#[doc = "`write(|w| ..)` method takes [`if_set::W`](W) writer structure"]
impl crate::Writable for IfSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF_SET to value 0x02"]
impl crate::Resettable for IfSetSpec {
    const RESET_VALUE: u32 = 0x02;
}
