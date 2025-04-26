#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Operating Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Frequency Lock Mode"]
    Fll = 0,
    #[doc = "1: Phase Lock Mode"]
    Pll = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Operating Mode Control"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Fll,
            true => Mode::Pll,
        }
    }
    #[doc = "Frequency Lock Mode"]
    #[inline(always)]
    pub fn is_fll(&self) -> bool {
        *self == Mode::Fll
    }
    #[doc = "Phase Lock Mode"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == Mode::Pll
    }
}
#[doc = "Field `MODE` writer - Operating Mode Control"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frequency Lock Mode"]
    #[inline(always)]
    pub fn fll(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Fll)
    }
    #[doc = "Phase Lock Mode"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Pll)
    }
}
#[doc = "Field `EDGESEL` reader - Reference Edge Select"]
pub type EdgeselR = crate::BitReader;
#[doc = "Field `EDGESEL` writer - Reference Edge Select"]
pub type EdgeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTORECOVER` reader - Automatic Recovery Control"]
pub type AutorecoverR = crate::BitReader;
#[doc = "Field `AUTORECOVER` writer - Automatic Recovery Control"]
pub type AutorecoverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DITHEN` reader - Dither Enable Control"]
pub type DithenR = crate::BitReader;
#[doc = "Field `DITHEN` writer - Dither Enable Control"]
pub type DithenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operating Mode Control"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reference Edge Select"]
    #[inline(always)]
    pub fn edgesel(&self) -> EdgeselR {
        EdgeselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic Recovery Control"]
    #[inline(always)]
    pub fn autorecover(&self) -> AutorecoverR {
        AutorecoverR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Dither Enable Control"]
    #[inline(always)]
    pub fn dithen(&self) -> DithenR {
        DithenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operating Mode Control"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<CfgSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 1 - Reference Edge Select"]
    #[inline(always)]
    pub fn edgesel(&mut self) -> EdgeselW<CfgSpec> {
        EdgeselW::new(self, 1)
    }
    #[doc = "Bit 2 - Automatic Recovery Control"]
    #[inline(always)]
    pub fn autorecover(&mut self) -> AutorecoverW<CfgSpec> {
        AutorecoverW::new(self, 2)
    }
    #[doc = "Bit 6 - Dither Enable Control"]
    #[inline(always)]
    pub fn dithen(&mut self) -> DithenW<CfgSpec> {
        DithenW::new(self, 6)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}
