#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `FLUSH` writer - Flush"]
pub type FlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTPC` writer - Start Performance Counters"]
pub type StartpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPPC` writer - Stop Performance Counters"]
pub type StoppcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Flush"]
    #[inline(always)]
    pub fn flush(&mut self) -> FlushW<CmdSpec> {
        FlushW::new(self, 0)
    }
    #[doc = "Bit 1 - Start Performance Counters"]
    #[inline(always)]
    pub fn startpc(&mut self) -> StartpcW<CmdSpec> {
        StartpcW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop Performance Counters"]
    #[inline(always)]
    pub fn stoppc(&mut self) -> StoppcW<CmdSpec> {
        StoppcW::new(self, 2)
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
