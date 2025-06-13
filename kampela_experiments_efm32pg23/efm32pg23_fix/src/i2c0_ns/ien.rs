#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `START` reader - START condition Interrupt Flag"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - START condition Interrupt Flag"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTART` reader - Repeated START condition Interrupt Flag"]
pub type RstartR = crate::BitReader;
#[doc = "Field `RSTART` writer - Repeated START condition Interrupt Flag"]
pub type RstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` reader - Address Interrupt Flag"]
pub type AddrR = crate::BitReader;
#[doc = "Field `ADDR` writer - Address Interrupt Flag"]
pub type AddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXC` reader - Transfer Completed Interrupt Flag"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - Transfer Completed Interrupt Flag"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBL` reader - Transmit Buffer Level Interrupt Flag"]
pub type TxblR = crate::BitReader;
#[doc = "Field `TXBL` writer - Transmit Buffer Level Interrupt Flag"]
pub type TxblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDATAV` reader - Receive Data Valid Interrupt Flag"]
pub type RxdatavR = crate::BitReader;
#[doc = "Field `RXDATAV` writer - Receive Data Valid Interrupt Flag"]
pub type RxdatavW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - Acknowledge Received Interrupt Flag"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - Acknowledge Received Interrupt Flag"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - Not Acknowledge Received Interrupt Flag"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - Not Acknowledge Received Interrupt Flag"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTOP` reader - Leader STOP Condition Interrupt Flag"]
pub type MstopR = crate::BitReader;
#[doc = "Field `MSTOP` writer - Leader STOP Condition Interrupt Flag"]
pub type MstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOST` reader - Arbitration Lost Interrupt Flag"]
pub type ArblostR = crate::BitReader;
#[doc = "Field `ARBLOST` writer - Arbitration Lost Interrupt Flag"]
pub type ArblostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERR` reader - Bus Error Interrupt Flag"]
pub type BuserrR = crate::BitReader;
#[doc = "Field `BUSERR` writer - Bus Error Interrupt Flag"]
pub type BuserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSHOLD` reader - Bus Held Interrupt Flag"]
pub type BusholdR = crate::BitReader;
#[doc = "Field `BUSHOLD` writer - Bus Held Interrupt Flag"]
pub type BusholdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` reader - Transmit Buffer Overflow Interrupt Flag"]
pub type TxofR = crate::BitReader;
#[doc = "Field `TXOF` writer - Transmit Buffer Overflow Interrupt Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` reader - Receive Buffer Underflow Interrupt Flag"]
pub type RxufR = crate::BitReader;
#[doc = "Field `RXUF` writer - Receive Buffer Underflow Interrupt Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITO` reader - Bus Idle Timeout Interrupt Flag"]
pub type BitoR = crate::BitReader;
#[doc = "Field `BITO` writer - Bus Idle Timeout Interrupt Flag"]
pub type BitoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTO` reader - Clock Low Timeout Interrupt Flag"]
pub type CltoR = crate::BitReader;
#[doc = "Field `CLTO` writer - Clock Low Timeout Interrupt Flag"]
pub type CltoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTOP` reader - Follower STOP condition Interrupt Flag"]
pub type SstopR = crate::BitReader;
#[doc = "Field `SSTOP` writer - Follower STOP condition Interrupt Flag"]
pub type SstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - Receive Buffer Full Interrupt Flag"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXFULL` writer - Receive Buffer Full Interrupt Flag"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLERR` reader - Clock Low Error Interrupt Flag"]
pub type ClerrR = crate::BitReader;
#[doc = "Field `CLERR` writer - Clock Low Error Interrupt Flag"]
pub type ClerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLERR` reader - SCL Error Interrupt Flag"]
pub type SclerrR = crate::BitReader;
#[doc = "Field `SCLERR` writer - SCL Error Interrupt Flag"]
pub type SclerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDAERR` reader - SDA Error Interrupt Flag"]
pub type SdaerrR = crate::BitReader;
#[doc = "Field `SDAERR` writer - SDA Error Interrupt Flag"]
pub type SdaerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - START condition Interrupt Flag"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeated START condition Interrupt Flag"]
    #[inline(always)]
    pub fn rstart(&self) -> RstartR {
        RstartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Address Interrupt Flag"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer Completed Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn txbl(&self) -> TxblR {
        TxblR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RxdatavR {
        RxdatavR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Not Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Leader STOP Condition Interrupt Flag"]
    #[inline(always)]
    pub fn mstop(&self) -> MstopR {
        MstopR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Flag"]
    #[inline(always)]
    pub fn arblost(&self) -> ArblostR {
        ArblostR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn buserr(&self) -> BuserrR {
        BuserrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus Held Interrupt Flag"]
    #[inline(always)]
    pub fn bushold(&self) -> BusholdR {
        BusholdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive Buffer Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bus Idle Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn bito(&self) -> BitoR {
        BitoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock Low Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn clto(&self) -> CltoR {
        CltoR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Follower STOP condition Interrupt Flag"]
    #[inline(always)]
    pub fn sstop(&self) -> SstopR {
        SstopR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive Buffer Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clock Low Error Interrupt Flag"]
    #[inline(always)]
    pub fn clerr(&self) -> ClerrR {
        ClerrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCL Error Interrupt Flag"]
    #[inline(always)]
    pub fn sclerr(&self) -> SclerrR {
        SclerrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SDA Error Interrupt Flag"]
    #[inline(always)]
    pub fn sdaerr(&self) -> SdaerrR {
        SdaerrR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - START condition Interrupt Flag"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<IenSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Repeated START condition Interrupt Flag"]
    #[inline(always)]
    pub fn rstart(&mut self) -> RstartW<IenSpec> {
        RstartW::new(self, 1)
    }
    #[doc = "Bit 2 - Address Interrupt Flag"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<IenSpec> {
        AddrW::new(self, 2)
    }
    #[doc = "Bit 3 - Transfer Completed Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<IenSpec> {
        TxcW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn txbl(&mut self) -> TxblW<IenSpec> {
        TxblW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn rxdatav(&mut self) -> RxdatavW<IenSpec> {
        RxdatavW::new(self, 5)
    }
    #[doc = "Bit 6 - Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<IenSpec> {
        AckW::new(self, 6)
    }
    #[doc = "Bit 7 - Not Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<IenSpec> {
        NackW::new(self, 7)
    }
    #[doc = "Bit 8 - Leader STOP Condition Interrupt Flag"]
    #[inline(always)]
    pub fn mstop(&mut self) -> MstopW<IenSpec> {
        MstopW::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Flag"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ArblostW<IenSpec> {
        ArblostW::new(self, 9)
    }
    #[doc = "Bit 10 - Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BuserrW<IenSpec> {
        BuserrW::new(self, 10)
    }
    #[doc = "Bit 11 - Bus Held Interrupt Flag"]
    #[inline(always)]
    pub fn bushold(&mut self) -> BusholdW<IenSpec> {
        BusholdW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<IenSpec> {
        TxofW::new(self, 12)
    }
    #[doc = "Bit 13 - Receive Buffer Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<IenSpec> {
        RxufW::new(self, 13)
    }
    #[doc = "Bit 14 - Bus Idle Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn bito(&mut self) -> BitoW<IenSpec> {
        BitoW::new(self, 14)
    }
    #[doc = "Bit 15 - Clock Low Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn clto(&mut self) -> CltoW<IenSpec> {
        CltoW::new(self, 15)
    }
    #[doc = "Bit 16 - Follower STOP condition Interrupt Flag"]
    #[inline(always)]
    pub fn sstop(&mut self) -> SstopW<IenSpec> {
        SstopW::new(self, 16)
    }
    #[doc = "Bit 17 - Receive Buffer Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<IenSpec> {
        RxfullW::new(self, 17)
    }
    #[doc = "Bit 18 - Clock Low Error Interrupt Flag"]
    #[inline(always)]
    pub fn clerr(&mut self) -> ClerrW<IenSpec> {
        ClerrW::new(self, 18)
    }
    #[doc = "Bit 19 - SCL Error Interrupt Flag"]
    #[inline(always)]
    pub fn sclerr(&mut self) -> SclerrW<IenSpec> {
        SclerrW::new(self, 19)
    }
    #[doc = "Bit 20 - SDA Error Interrupt Flag"]
    #[inline(always)]
    pub fn sdaerr(&mut self) -> SdaerrW<IenSpec> {
        SdaerrW::new(self, 20)
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
