#[doc = "Register `ASYNC_SWPULSE` writer"]
pub type W = crate::W<AsyncSwpulseSpec>;
#[doc = "Field `CH0PULSE` writer - Channel pulse"]
pub type Ch0pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1PULSE` writer - Channel pulse"]
pub type Ch1pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2PULSE` writer - Channel pulse"]
pub type Ch2pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3PULSE` writer - Channel pulse"]
pub type Ch3pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4PULSE` writer - Channel pulse"]
pub type Ch4pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5PULSE` writer - Channel pulse"]
pub type Ch5pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6PULSE` writer - Channel pulse"]
pub type Ch6pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7PULSE` writer - Channel pulse"]
pub type Ch7pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8PULSE` writer - Channel pulse"]
pub type Ch8pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9PULSE` writer - Channel pulse"]
pub type Ch9pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10PULSE` writer - Channel pulse"]
pub type Ch10pulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11PULSE` writer - Channel pulse"]
pub type Ch11pulseW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel pulse"]
    #[inline(always)]
    pub fn ch0pulse(&mut self) -> Ch0pulseW<AsyncSwpulseSpec> {
        Ch0pulseW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel pulse"]
    #[inline(always)]
    pub fn ch1pulse(&mut self) -> Ch1pulseW<AsyncSwpulseSpec> {
        Ch1pulseW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel pulse"]
    #[inline(always)]
    pub fn ch2pulse(&mut self) -> Ch2pulseW<AsyncSwpulseSpec> {
        Ch2pulseW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel pulse"]
    #[inline(always)]
    pub fn ch3pulse(&mut self) -> Ch3pulseW<AsyncSwpulseSpec> {
        Ch3pulseW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel pulse"]
    #[inline(always)]
    pub fn ch4pulse(&mut self) -> Ch4pulseW<AsyncSwpulseSpec> {
        Ch4pulseW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel pulse"]
    #[inline(always)]
    pub fn ch5pulse(&mut self) -> Ch5pulseW<AsyncSwpulseSpec> {
        Ch5pulseW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel pulse"]
    #[inline(always)]
    pub fn ch6pulse(&mut self) -> Ch6pulseW<AsyncSwpulseSpec> {
        Ch6pulseW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel pulse"]
    #[inline(always)]
    pub fn ch7pulse(&mut self) -> Ch7pulseW<AsyncSwpulseSpec> {
        Ch7pulseW::new(self, 7)
    }
    #[doc = "Bit 8 - Channel pulse"]
    #[inline(always)]
    pub fn ch8pulse(&mut self) -> Ch8pulseW<AsyncSwpulseSpec> {
        Ch8pulseW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel pulse"]
    #[inline(always)]
    pub fn ch9pulse(&mut self) -> Ch9pulseW<AsyncSwpulseSpec> {
        Ch9pulseW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel pulse"]
    #[inline(always)]
    pub fn ch10pulse(&mut self) -> Ch10pulseW<AsyncSwpulseSpec> {
        Ch10pulseW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel pulse"]
    #[inline(always)]
    pub fn ch11pulse(&mut self) -> Ch11pulseW<AsyncSwpulseSpec> {
        Ch11pulseW::new(self, 11)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_swpulse::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsyncSwpulseSpec;
impl crate::RegisterSpec for AsyncSwpulseSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`async_swpulse::W`](W) writer structure"]
impl crate::Writable for AsyncSwpulseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ASYNC_SWPULSE to value 0"]
impl crate::Resettable for AsyncSwpulseSpec {}
