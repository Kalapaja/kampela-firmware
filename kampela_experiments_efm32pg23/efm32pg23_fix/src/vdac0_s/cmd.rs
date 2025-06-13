#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `CH0EN` writer - DAC Channel 0 Enable"]
pub type Ch0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0DIS` writer - DAC Channel 0 Disable"]
pub type Ch0disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1EN` writer - DAC Channel 1 Enable"]
pub type Ch1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1DIS` writer - DAC Channel 1 Disable"]
pub type Ch1disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0FIFOFLUSH` writer - CH0 WFIFO Flush"]
pub type Ch0fifoflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1FIFOFLUSH` writer - CH1 WFIFO Flush"]
pub type Ch1fifoflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINEMODESTART` writer - Start Sine Wave Generation"]
pub type SinemodestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINEMODESTOP` writer - Stop Sine Wave Generation"]
pub type SinemodestopW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC Channel 0 Enable"]
    #[inline(always)]
    pub fn ch0en(&mut self) -> Ch0enW<CmdSpec> {
        Ch0enW::new(self, 0)
    }
    #[doc = "Bit 1 - DAC Channel 0 Disable"]
    #[inline(always)]
    pub fn ch0dis(&mut self) -> Ch0disW<CmdSpec> {
        Ch0disW::new(self, 1)
    }
    #[doc = "Bit 4 - DAC Channel 1 Enable"]
    #[inline(always)]
    pub fn ch1en(&mut self) -> Ch1enW<CmdSpec> {
        Ch1enW::new(self, 4)
    }
    #[doc = "Bit 5 - DAC Channel 1 Disable"]
    #[inline(always)]
    pub fn ch1dis(&mut self) -> Ch1disW<CmdSpec> {
        Ch1disW::new(self, 5)
    }
    #[doc = "Bit 8 - CH0 WFIFO Flush"]
    #[inline(always)]
    pub fn ch0fifoflush(&mut self) -> Ch0fifoflushW<CmdSpec> {
        Ch0fifoflushW::new(self, 8)
    }
    #[doc = "Bit 9 - CH1 WFIFO Flush"]
    #[inline(always)]
    pub fn ch1fifoflush(&mut self) -> Ch1fifoflushW<CmdSpec> {
        Ch1fifoflushW::new(self, 9)
    }
    #[doc = "Bit 10 - Start Sine Wave Generation"]
    #[inline(always)]
    pub fn sinemodestart(&mut self) -> SinemodestartW<CmdSpec> {
        SinemodestartW::new(self, 10)
    }
    #[doc = "Bit 11 - Stop Sine Wave Generation"]
    #[inline(always)]
    pub fn sinemodestop(&mut self) -> SinemodestopW<CmdSpec> {
        SinemodestopW::new(self, 11)
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
