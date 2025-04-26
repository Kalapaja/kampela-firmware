#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Crystal Oscillator Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: crystal oscillator"]
    Xtal = 0,
    #[doc = "1: external sinusoidal clock can be supplied on XI pin."]
    Extclk = 1,
    #[doc = "2: external sinusoidal clock can be supplied on XI pin (peak detector used)."]
    Extclkpkdet = 2,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Crystal Oscillator Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Xtal),
            1 => Some(Mode::Extclk),
            2 => Some(Mode::Extclkpkdet),
            _ => None,
        }
    }
    #[doc = "crystal oscillator"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == Mode::Xtal
    }
    #[doc = "external sinusoidal clock can be supplied on XI pin."]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == Mode::Extclk
    }
    #[doc = "external sinusoidal clock can be supplied on XI pin (peak detector used)."]
    #[inline(always)]
    pub fn is_extclkpkdet(&self) -> bool {
        *self == Mode::Extclkpkdet
    }
}
#[doc = "Field `MODE` writer - Crystal Oscillator Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Xtal)
    }
    #[doc = "external sinusoidal clock can be supplied on XI pin."]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Extclk)
    }
    #[doc = "external sinusoidal clock can be supplied on XI pin (peak detector used)."]
    #[inline(always)]
    pub fn extclkpkdet(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Extclkpkdet)
    }
}
#[doc = "Field `ENXIDCBIASANA` reader - Enable XI Internal DC Bias"]
pub type EnxidcbiasanaR = crate::BitReader;
#[doc = "Field `ENXIDCBIASANA` writer - Enable XI Internal DC Bias"]
pub type EnxidcbiasanaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Squaring Buffer Schmitt Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sqbufschtrgana {
    #[doc = "0: Squaring buffer schmitt trigger is disabled"]
    Disable = 0,
    #[doc = "1: Squaring buffer schmitt trigger is enabled"]
    Enable = 1,
}
impl From<Sqbufschtrgana> for bool {
    #[inline(always)]
    fn from(variant: Sqbufschtrgana) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQBUFSCHTRGANA` reader - Squaring Buffer Schmitt Trigger"]
pub type SqbufschtrganaR = crate::BitReader<Sqbufschtrgana>;
impl SqbufschtrganaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sqbufschtrgana {
        match self.bits {
            false => Sqbufschtrgana::Disable,
            true => Sqbufschtrgana::Enable,
        }
    }
    #[doc = "Squaring buffer schmitt trigger is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sqbufschtrgana::Disable
    }
    #[doc = "Squaring buffer schmitt trigger is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sqbufschtrgana::Enable
    }
}
#[doc = "Field `SQBUFSCHTRGANA` writer - Squaring Buffer Schmitt Trigger"]
pub type SqbufschtrganaW<'a, REG> = crate::BitWriter<'a, REG, Sqbufschtrgana>;
impl<'a, REG> SqbufschtrganaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Squaring buffer schmitt trigger is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sqbufschtrgana::Disable)
    }
    #[doc = "Squaring buffer schmitt trigger is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sqbufschtrgana::Enable)
    }
}
#[doc = "Field `FORCELFTIMEOUT` reader - Force Low Frequency Timeout"]
pub type ForcelftimeoutR = crate::BitReader;
#[doc = "Field `FORCELFTIMEOUT` writer - Force Low Frequency Timeout"]
pub type ForcelftimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Crystal Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Enable XI Internal DC Bias"]
    #[inline(always)]
    pub fn enxidcbiasana(&self) -> EnxidcbiasanaR {
        EnxidcbiasanaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Squaring Buffer Schmitt Trigger"]
    #[inline(always)]
    pub fn sqbufschtrgana(&self) -> SqbufschtrganaR {
        SqbufschtrganaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 28 - Force Low Frequency Timeout"]
    #[inline(always)]
    pub fn forcelftimeout(&self) -> ForcelftimeoutR {
        ForcelftimeoutR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Crystal Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<CfgSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 2 - Enable XI Internal DC Bias"]
    #[inline(always)]
    pub fn enxidcbiasana(&mut self) -> EnxidcbiasanaW<CfgSpec> {
        EnxidcbiasanaW::new(self, 2)
    }
    #[doc = "Bit 3 - Squaring Buffer Schmitt Trigger"]
    #[inline(always)]
    pub fn sqbufschtrgana(&mut self) -> SqbufschtrganaW<CfgSpec> {
        SqbufschtrganaW::new(self, 3)
    }
    #[doc = "Bit 28 - Force Low Frequency Timeout"]
    #[inline(always)]
    pub fn forcelftimeout(&mut self) -> ForcelftimeoutW<CfgSpec> {
        ForcelftimeoutW::new(self, 28)
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
#[doc = "`reset()` method sets CFG to value 0x1000_0000"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
