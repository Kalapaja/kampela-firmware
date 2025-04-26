#[doc = "Register `ASYNC_PEEK` reader"]
pub type R = crate::R<AsyncPeekSpec>;
#[doc = "Field `CH0VAL` reader - Channel 0 Current Value"]
pub type Ch0valR = crate::BitReader;
#[doc = "Field `CH1VAL` reader - Channel 1 Current Value"]
pub type Ch1valR = crate::BitReader;
#[doc = "Field `CH2VAL` reader - Channel 2 Current Value"]
pub type Ch2valR = crate::BitReader;
#[doc = "Field `CH3VAL` reader - Channel 3 Current Value"]
pub type Ch3valR = crate::BitReader;
#[doc = "Field `CH4VAL` reader - Channel 4 Current Value"]
pub type Ch4valR = crate::BitReader;
#[doc = "Field `CH5VAL` reader - Channel 5 Current Value"]
pub type Ch5valR = crate::BitReader;
#[doc = "Field `CH6VAL` reader - Channel 6 Current Value"]
pub type Ch6valR = crate::BitReader;
#[doc = "Field `CH7VAL` reader - Channel 7 Current Value"]
pub type Ch7valR = crate::BitReader;
#[doc = "Field `CH8VAL` reader - Channel 8 Current Value"]
pub type Ch8valR = crate::BitReader;
#[doc = "Field `CH9VAL` reader - Channel 9 Current Value"]
pub type Ch9valR = crate::BitReader;
#[doc = "Field `CH10VAL` reader - Channel 10 Current Value"]
pub type Ch10valR = crate::BitReader;
#[doc = "Field `CH11VAL` reader - Channel 11 Current Value"]
pub type Ch11valR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Current Value"]
    #[inline(always)]
    pub fn ch0val(&self) -> Ch0valR {
        Ch0valR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Current Value"]
    #[inline(always)]
    pub fn ch1val(&self) -> Ch1valR {
        Ch1valR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Current Value"]
    #[inline(always)]
    pub fn ch2val(&self) -> Ch2valR {
        Ch2valR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Current Value"]
    #[inline(always)]
    pub fn ch3val(&self) -> Ch3valR {
        Ch3valR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Current Value"]
    #[inline(always)]
    pub fn ch4val(&self) -> Ch4valR {
        Ch4valR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Current Value"]
    #[inline(always)]
    pub fn ch5val(&self) -> Ch5valR {
        Ch5valR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Current Value"]
    #[inline(always)]
    pub fn ch6val(&self) -> Ch6valR {
        Ch6valR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Current Value"]
    #[inline(always)]
    pub fn ch7val(&self) -> Ch7valR {
        Ch7valR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Current Value"]
    #[inline(always)]
    pub fn ch8val(&self) -> Ch8valR {
        Ch8valR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Current Value"]
    #[inline(always)]
    pub fn ch9val(&self) -> Ch9valR {
        Ch9valR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Current Value"]
    #[inline(always)]
    pub fn ch10val(&self) -> Ch10valR {
        Ch10valR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Current Value"]
    #[inline(always)]
    pub fn ch11val(&self) -> Ch11valR {
        Ch11valR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_peek::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsyncPeekSpec;
impl crate::RegisterSpec for AsyncPeekSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`async_peek::R`](R) reader structure"]
impl crate::Readable for AsyncPeekSpec {}
#[doc = "`reset()` method sets ASYNC_PEEK to value 0"]
impl crate::Resettable for AsyncPeekSpec {}
