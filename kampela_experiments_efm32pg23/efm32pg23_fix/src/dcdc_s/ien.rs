#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `BYPSW` reader - Bypass Switch Enabled Interrupt Enable"]
pub type BypswR = crate::BitReader;
#[doc = "Field `BYPSW` writer - Bypass Switch Enabled Interrupt Enable"]
pub type BypswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARM` reader - DCDC Warmup Time Done Interrupt Enable"]
pub type WarmR = crate::BitReader;
#[doc = "Field `WARM` writer - DCDC Warmup Time Done Interrupt Enable"]
pub type WarmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNNING` reader - DCDC Running Interrupt Enable"]
pub type RunningR = crate::BitReader;
#[doc = "Field `RUNNING` writer - DCDC Running Interrupt Enable"]
pub type RunningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREGINLOW` reader - VREGIN below threshold Interrupt Enable"]
pub type VreginlowR = crate::BitReader;
#[doc = "Field `VREGINLOW` writer - VREGIN below threshold Interrupt Enable"]
pub type VreginlowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREGINHIGH` reader - VREGIN above threshold Interrupt Enable"]
pub type VreginhighR = crate::BitReader;
#[doc = "Field `VREGINHIGH` writer - VREGIN above threshold Interrupt Enable"]
pub type VreginhighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGULATION` reader - DCDC in Regulation Interrupt Enable"]
pub type RegulationR = crate::BitReader;
#[doc = "Field `REGULATION` writer - DCDC in Regulation Interrupt Enable"]
pub type RegulationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMAX` reader - Ton_max Timeout Interrupt Enable"]
pub type TmaxR = crate::BitReader;
#[doc = "Field `TMAX` writer - Ton_max Timeout Interrupt Enable"]
pub type TmaxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4ERR` reader - EM4 Entry Req Interrupt Enable"]
pub type Em4errR = crate::BitReader;
#[doc = "Field `EM4ERR` writer - EM4 Entry Req Interrupt Enable"]
pub type Em4errW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPMODE` reader - Pulse Pairing Mode Interrupt Enable"]
pub type PpmodeR = crate::BitReader;
#[doc = "Field `PPMODE` writer - Pulse Pairing Mode Interrupt Enable"]
pub type PpmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFMXMODE` reader - PFMX Mode Interrupt Enable"]
pub type PfmxmodeR = crate::BitReader;
#[doc = "Field `PFMXMODE` writer - PFMX Mode Interrupt Enable"]
pub type PfmxmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bypass Switch Enabled Interrupt Enable"]
    #[inline(always)]
    pub fn bypsw(&self) -> BypswR {
        BypswR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCDC Warmup Time Done Interrupt Enable"]
    #[inline(always)]
    pub fn warm(&self) -> WarmR {
        WarmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCDC Running Interrupt Enable"]
    #[inline(always)]
    pub fn running(&self) -> RunningR {
        RunningR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VREGIN below threshold Interrupt Enable"]
    #[inline(always)]
    pub fn vreginlow(&self) -> VreginlowR {
        VreginlowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VREGIN above threshold Interrupt Enable"]
    #[inline(always)]
    pub fn vreginhigh(&self) -> VreginhighR {
        VreginhighR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DCDC in Regulation Interrupt Enable"]
    #[inline(always)]
    pub fn regulation(&self) -> RegulationR {
        RegulationR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ton_max Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn tmax(&self) -> TmaxR {
        TmaxR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EM4 Entry Req Interrupt Enable"]
    #[inline(always)]
    pub fn em4err(&self) -> Em4errR {
        Em4errR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pulse Pairing Mode Interrupt Enable"]
    #[inline(always)]
    pub fn ppmode(&self) -> PpmodeR {
        PpmodeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PFMX Mode Interrupt Enable"]
    #[inline(always)]
    pub fn pfmxmode(&self) -> PfmxmodeR {
        PfmxmodeR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass Switch Enabled Interrupt Enable"]
    #[inline(always)]
    pub fn bypsw(&mut self) -> BypswW<IenSpec> {
        BypswW::new(self, 0)
    }
    #[doc = "Bit 1 - DCDC Warmup Time Done Interrupt Enable"]
    #[inline(always)]
    pub fn warm(&mut self) -> WarmW<IenSpec> {
        WarmW::new(self, 1)
    }
    #[doc = "Bit 2 - DCDC Running Interrupt Enable"]
    #[inline(always)]
    pub fn running(&mut self) -> RunningW<IenSpec> {
        RunningW::new(self, 2)
    }
    #[doc = "Bit 3 - VREGIN below threshold Interrupt Enable"]
    #[inline(always)]
    pub fn vreginlow(&mut self) -> VreginlowW<IenSpec> {
        VreginlowW::new(self, 3)
    }
    #[doc = "Bit 4 - VREGIN above threshold Interrupt Enable"]
    #[inline(always)]
    pub fn vreginhigh(&mut self) -> VreginhighW<IenSpec> {
        VreginhighW::new(self, 4)
    }
    #[doc = "Bit 5 - DCDC in Regulation Interrupt Enable"]
    #[inline(always)]
    pub fn regulation(&mut self) -> RegulationW<IenSpec> {
        RegulationW::new(self, 5)
    }
    #[doc = "Bit 6 - Ton_max Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn tmax(&mut self) -> TmaxW<IenSpec> {
        TmaxW::new(self, 6)
    }
    #[doc = "Bit 7 - EM4 Entry Req Interrupt Enable"]
    #[inline(always)]
    pub fn em4err(&mut self) -> Em4errW<IenSpec> {
        Em4errW::new(self, 7)
    }
    #[doc = "Bit 8 - Pulse Pairing Mode Interrupt Enable"]
    #[inline(always)]
    pub fn ppmode(&mut self) -> PpmodeW<IenSpec> {
        PpmodeW::new(self, 8)
    }
    #[doc = "Bit 9 - PFMX Mode Interrupt Enable"]
    #[inline(always)]
    pub fn pfmxmode(&mut self) -> PfmxmodeW<IenSpec> {
        PfmxmodeW::new(self, 9)
    }
}
#[doc = "Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
