#[doc = "Register `SYNC_PEEK` reader"]
pub type R = crate::R<SyncPeekSpec>;
#[doc = "Field `CH0VAL` reader - Channel Value"]
pub type Ch0valR = crate::BitReader;
#[doc = "Field `CH1VAL` reader - Channel Value"]
pub type Ch1valR = crate::BitReader;
#[doc = "Field `CH2VAL` reader - Channel Value"]
pub type Ch2valR = crate::BitReader;
#[doc = "Field `CH3VAL` reader - Channel Value"]
pub type Ch3valR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel Value"]
    #[inline(always)]
    pub fn ch0val(&self) -> Ch0valR {
        Ch0valR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Value"]
    #[inline(always)]
    pub fn ch1val(&self) -> Ch1valR {
        Ch1valR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Value"]
    #[inline(always)]
    pub fn ch2val(&self) -> Ch2valR {
        Ch2valR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Value"]
    #[inline(always)]
    pub fn ch3val(&self) -> Ch3valR {
        Ch3valR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_peek::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncPeekSpec;
impl crate::RegisterSpec for SyncPeekSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_peek::R`](R) reader structure"]
impl crate::Readable for SyncPeekSpec {}
#[doc = "`reset()` method sets SYNC_PEEK to value 0"]
impl crate::Resettable for SyncPeekSpec {}
