#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `SINGLESTART` writer - Single Queue Start"]
pub type SinglestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLESTOP` writer - Single Queue Stop"]
pub type SinglestopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANSTART` writer - Scan Queue Start"]
pub type ScanstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANSTOP` writer - Scan Queue Stop"]
pub type ScanstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEREN` writer - Timer Enable"]
pub type TimerenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERDIS` writer - Timer Disable"]
pub type TimerdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEFIFOFLUSH` writer - Flush the Single FIFO"]
pub type SinglefifoflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANFIFOFLUSH` writer - Flush the Scan FIFO"]
pub type ScanfifoflushW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Single Queue Start"]
    #[inline(always)]
    pub fn singlestart(&mut self) -> SinglestartW<CmdSpec> {
        SinglestartW::new(self, 0)
    }
    #[doc = "Bit 1 - Single Queue Stop"]
    #[inline(always)]
    pub fn singlestop(&mut self) -> SinglestopW<CmdSpec> {
        SinglestopW::new(self, 1)
    }
    #[doc = "Bit 3 - Scan Queue Start"]
    #[inline(always)]
    pub fn scanstart(&mut self) -> ScanstartW<CmdSpec> {
        ScanstartW::new(self, 3)
    }
    #[doc = "Bit 4 - Scan Queue Stop"]
    #[inline(always)]
    pub fn scanstop(&mut self) -> ScanstopW<CmdSpec> {
        ScanstopW::new(self, 4)
    }
    #[doc = "Bit 16 - Timer Enable"]
    #[inline(always)]
    pub fn timeren(&mut self) -> TimerenW<CmdSpec> {
        TimerenW::new(self, 16)
    }
    #[doc = "Bit 17 - Timer Disable"]
    #[inline(always)]
    pub fn timerdis(&mut self) -> TimerdisW<CmdSpec> {
        TimerdisW::new(self, 17)
    }
    #[doc = "Bit 24 - Flush the Single FIFO"]
    #[inline(always)]
    pub fn singlefifoflush(&mut self) -> SinglefifoflushW<CmdSpec> {
        SinglefifoflushW::new(self, 24)
    }
    #[doc = "Bit 25 - Flush the Scan FIFO"]
    #[inline(always)]
    pub fn scanfifoflush(&mut self) -> ScanfifoflushW<CmdSpec> {
        ScanfifoflushW::new(self, 25)
    }
}
#[doc = "Command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
