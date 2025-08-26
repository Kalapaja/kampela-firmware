#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `BYPSW` reader - Bypass Switch is currently enabled"]
pub type BypswR = crate::BitReader;
#[doc = "Field `WARM` reader - DCDC Warmup Done"]
pub type WarmR = crate::BitReader;
#[doc = "Field `RUNNING` reader - DCDC is running"]
pub type RunningR = crate::BitReader;
#[doc = "Field `VREGIN` reader - VREGVDD comparator status"]
pub type VreginR = crate::BitReader;
#[doc = "Field `BYPCMPOUT` reader - Bypass Comparator Output"]
pub type BypcmpoutR = crate::BitReader;
#[doc = "Field `PPMODE` reader - DCDC in pulse-pairing mode"]
pub type PpmodeR = crate::BitReader;
#[doc = "Field `PFMXMODE` reader - DCDC in PFMX mode"]
pub type PfmxmodeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Bypass Switch is currently enabled"]
    #[inline(always)]
    pub fn bypsw(&self) -> BypswR {
        BypswR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCDC Warmup Done"]
    #[inline(always)]
    pub fn warm(&self) -> WarmR {
        WarmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCDC is running"]
    #[inline(always)]
    pub fn running(&self) -> RunningR {
        RunningR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VREGVDD comparator status"]
    #[inline(always)]
    pub fn vregin(&self) -> VreginR {
        VreginR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass Comparator Output"]
    #[inline(always)]
    pub fn bypcmpout(&self) -> BypcmpoutR {
        BypcmpoutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DCDC in pulse-pairing mode"]
    #[inline(always)]
    pub fn ppmode(&self) -> PpmodeR {
        PpmodeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DCDC in PFMX mode"]
    #[inline(always)]
    pub fn pfmxmode(&self) -> PfmxmodeR {
        PfmxmodeR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "DCDC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
