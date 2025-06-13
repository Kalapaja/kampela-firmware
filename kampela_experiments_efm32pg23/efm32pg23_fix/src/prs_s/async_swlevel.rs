#[doc = "Register `ASYNC_SWLEVEL` reader"]
pub type R = crate::R<AsyncSwlevelSpec>;
#[doc = "Register `ASYNC_SWLEVEL` writer"]
pub type W = crate::W<AsyncSwlevelSpec>;
#[doc = "Field `CH0LEVEL` reader - Channel Level"]
pub type Ch0levelR = crate::BitReader;
#[doc = "Field `CH0LEVEL` writer - Channel Level"]
pub type Ch0levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1LEVEL` reader - Channel Level"]
pub type Ch1levelR = crate::BitReader;
#[doc = "Field `CH1LEVEL` writer - Channel Level"]
pub type Ch1levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2LEVEL` reader - Channel Level"]
pub type Ch2levelR = crate::BitReader;
#[doc = "Field `CH2LEVEL` writer - Channel Level"]
pub type Ch2levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3LEVEL` reader - Channel Level"]
pub type Ch3levelR = crate::BitReader;
#[doc = "Field `CH3LEVEL` writer - Channel Level"]
pub type Ch3levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4LEVEL` reader - Channel Level"]
pub type Ch4levelR = crate::BitReader;
#[doc = "Field `CH4LEVEL` writer - Channel Level"]
pub type Ch4levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5LEVEL` reader - Channel Level"]
pub type Ch5levelR = crate::BitReader;
#[doc = "Field `CH5LEVEL` writer - Channel Level"]
pub type Ch5levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6LEVEL` reader - Channel Level"]
pub type Ch6levelR = crate::BitReader;
#[doc = "Field `CH6LEVEL` writer - Channel Level"]
pub type Ch6levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7LEVEL` reader - Channel Level"]
pub type Ch7levelR = crate::BitReader;
#[doc = "Field `CH7LEVEL` writer - Channel Level"]
pub type Ch7levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8LEVEL` reader - Channel Level"]
pub type Ch8levelR = crate::BitReader;
#[doc = "Field `CH8LEVEL` writer - Channel Level"]
pub type Ch8levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9LEVEL` reader - Channel Level"]
pub type Ch9levelR = crate::BitReader;
#[doc = "Field `CH9LEVEL` writer - Channel Level"]
pub type Ch9levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10LEVEL` reader - Channel Level"]
pub type Ch10levelR = crate::BitReader;
#[doc = "Field `CH10LEVEL` writer - Channel Level"]
pub type Ch10levelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11LEVEL` reader - Channel Level"]
pub type Ch11levelR = crate::BitReader;
#[doc = "Field `CH11LEVEL` writer - Channel Level"]
pub type Ch11levelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel Level"]
    #[inline(always)]
    pub fn ch0level(&self) -> Ch0levelR {
        Ch0levelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Level"]
    #[inline(always)]
    pub fn ch1level(&self) -> Ch1levelR {
        Ch1levelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Level"]
    #[inline(always)]
    pub fn ch2level(&self) -> Ch2levelR {
        Ch2levelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Level"]
    #[inline(always)]
    pub fn ch3level(&self) -> Ch3levelR {
        Ch3levelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Level"]
    #[inline(always)]
    pub fn ch4level(&self) -> Ch4levelR {
        Ch4levelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel Level"]
    #[inline(always)]
    pub fn ch5level(&self) -> Ch5levelR {
        Ch5levelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Level"]
    #[inline(always)]
    pub fn ch6level(&self) -> Ch6levelR {
        Ch6levelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Level"]
    #[inline(always)]
    pub fn ch7level(&self) -> Ch7levelR {
        Ch7levelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel Level"]
    #[inline(always)]
    pub fn ch8level(&self) -> Ch8levelR {
        Ch8levelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel Level"]
    #[inline(always)]
    pub fn ch9level(&self) -> Ch9levelR {
        Ch9levelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel Level"]
    #[inline(always)]
    pub fn ch10level(&self) -> Ch10levelR {
        Ch10levelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel Level"]
    #[inline(always)]
    pub fn ch11level(&self) -> Ch11levelR {
        Ch11levelR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Level"]
    #[inline(always)]
    pub fn ch0level(&mut self) -> Ch0levelW<AsyncSwlevelSpec> {
        Ch0levelW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Level"]
    #[inline(always)]
    pub fn ch1level(&mut self) -> Ch1levelW<AsyncSwlevelSpec> {
        Ch1levelW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel Level"]
    #[inline(always)]
    pub fn ch2level(&mut self) -> Ch2levelW<AsyncSwlevelSpec> {
        Ch2levelW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel Level"]
    #[inline(always)]
    pub fn ch3level(&mut self) -> Ch3levelW<AsyncSwlevelSpec> {
        Ch3levelW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel Level"]
    #[inline(always)]
    pub fn ch4level(&mut self) -> Ch4levelW<AsyncSwlevelSpec> {
        Ch4levelW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel Level"]
    #[inline(always)]
    pub fn ch5level(&mut self) -> Ch5levelW<AsyncSwlevelSpec> {
        Ch5levelW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel Level"]
    #[inline(always)]
    pub fn ch6level(&mut self) -> Ch6levelW<AsyncSwlevelSpec> {
        Ch6levelW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel Level"]
    #[inline(always)]
    pub fn ch7level(&mut self) -> Ch7levelW<AsyncSwlevelSpec> {
        Ch7levelW::new(self, 7)
    }
    #[doc = "Bit 8 - Channel Level"]
    #[inline(always)]
    pub fn ch8level(&mut self) -> Ch8levelW<AsyncSwlevelSpec> {
        Ch8levelW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel Level"]
    #[inline(always)]
    pub fn ch9level(&mut self) -> Ch9levelW<AsyncSwlevelSpec> {
        Ch9levelW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel Level"]
    #[inline(always)]
    pub fn ch10level(&mut self) -> Ch10levelW<AsyncSwlevelSpec> {
        Ch10levelW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel Level"]
    #[inline(always)]
    pub fn ch11level(&mut self) -> Ch11levelW<AsyncSwlevelSpec> {
        Ch11levelW::new(self, 11)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_swlevel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_swlevel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsyncSwlevelSpec;
impl crate::RegisterSpec for AsyncSwlevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`async_swlevel::R`](R) reader structure"]
impl crate::Readable for AsyncSwlevelSpec {}
#[doc = "`write(|w| ..)` method takes [`async_swlevel::W`](W) writer structure"]
impl crate::Writable for AsyncSwlevelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ASYNC_SWLEVEL to value 0"]
impl crate::Resettable for AsyncSwlevelSpec {}
