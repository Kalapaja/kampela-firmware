#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Single input EM23GRPACLK oversampling mode (available in EM0-EM3)."]
    Ovssingle = 0,
    #[doc = "1: Externally clocked single input counter mode (available in EM0-EM3)."]
    Extclksingle = 1,
    #[doc = "2: Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    Extclkquad = 2,
    #[doc = "3: EM23GRPACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    Ovsquad1x = 3,
    #[doc = "4: EM23GRPACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    Ovsquad2x = 4,
    #[doc = "5: EM23GRPACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    Ovsquad4x = 5,
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
#[doc = "Field `MODE` reader - Mode Select"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Ovssingle),
            1 => Some(Mode::Extclksingle),
            2 => Some(Mode::Extclkquad),
            3 => Some(Mode::Ovsquad1x),
            4 => Some(Mode::Ovsquad2x),
            5 => Some(Mode::Ovsquad4x),
            _ => None,
        }
    }
    #[doc = "Single input EM23GRPACLK oversampling mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovssingle(&self) -> bool {
        *self == Mode::Ovssingle
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_extclksingle(&self) -> bool {
        *self == Mode::Extclksingle
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_extclkquad(&self) -> bool {
        *self == Mode::Extclkquad
    }
    #[doc = "EM23GRPACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovsquad1x(&self) -> bool {
        *self == Mode::Ovsquad1x
    }
    #[doc = "EM23GRPACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovsquad2x(&self) -> bool {
        *self == Mode::Ovsquad2x
    }
    #[doc = "EM23GRPACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn is_ovsquad4x(&self) -> bool {
        *self == Mode::Ovsquad4x
    }
}
#[doc = "Field `MODE` writer - Mode Select"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single input EM23GRPACLK oversampling mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovssingle(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ovssingle)
    }
    #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclksingle(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Extclksingle)
    }
    #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn extclkquad(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Extclkquad)
    }
    #[doc = "EM23GRPACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad1x(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ovsquad1x)
    }
    #[doc = "EM23GRPACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad2x(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ovsquad2x)
    }
    #[doc = "EM23GRPACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
    #[inline(always)]
    pub fn ovsquad4x(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ovsquad4x)
    }
}
#[doc = "Debug Mode Halt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debughalt {
    #[doc = "0: PCNT is running in debug mode."]
    Disable = 0,
    #[doc = "1: PCNT is frozen in debug mode."]
    Enable = 1,
}
impl From<Debughalt> for bool {
    #[inline(always)]
    fn from(variant: Debughalt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUGHALT` reader - Debug Mode Halt Enable"]
pub type DebughaltR = crate::BitReader<Debughalt>;
impl DebughaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Debughalt {
        match self.bits {
            false => Debughalt::Disable,
            true => Debughalt::Enable,
        }
    }
    #[doc = "PCNT is running in debug mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Debughalt::Disable
    }
    #[doc = "PCNT is frozen in debug mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Debughalt::Enable
    }
}
#[doc = "Field `DEBUGHALT` writer - Debug Mode Halt Enable"]
pub type DebughaltW<'a, REG> = crate::BitWriter<'a, REG, Debughalt>;
impl<'a, REG> DebughaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PCNT is running in debug mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Debughalt::Disable)
    }
    #[doc = "PCNT is frozen in debug mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Debughalt::Enable)
    }
}
#[doc = "Field `FILTEN` reader - Enable Digital Pulse Width Filter"]
pub type FiltenR = crate::BitReader;
#[doc = "Field `FILTEN` writer - Enable Digital Pulse Width Filter"]
pub type FiltenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Enable Hysteresis"]
pub type HystR = crate::BitReader;
#[doc = "Field `HYST` writer - Enable Hysteresis"]
pub type HystW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S0PRSEN` reader - S0IN PRS Enable"]
pub type S0prsenR = crate::BitReader;
#[doc = "Field `S0PRSEN` writer - S0IN PRS Enable"]
pub type S0prsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1PRSEN` reader - S1IN PRS Enable"]
pub type S1prsenR = crate::BitReader;
#[doc = "Field `S1PRSEN` writer - S1IN PRS Enable"]
pub type S1prsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Debug Mode Halt Enable"]
    #[inline(always)]
    pub fn debughalt(&self) -> DebughaltR {
        DebughaltR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    pub fn filten(&self) -> FiltenR {
        FiltenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - S0IN PRS Enable"]
    #[inline(always)]
    pub fn s0prsen(&self) -> S0prsenR {
        S0prsenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - S1IN PRS Enable"]
    #[inline(always)]
    pub fn s1prsen(&self) -> S1prsenR {
        S1prsenR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mode Select"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<CfgSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 4 - Debug Mode Halt Enable"]
    #[inline(always)]
    pub fn debughalt(&mut self) -> DebughaltW<CfgSpec> {
        DebughaltW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Digital Pulse Width Filter"]
    #[inline(always)]
    pub fn filten(&mut self) -> FiltenW<CfgSpec> {
        FiltenW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<CfgSpec> {
        HystW::new(self, 6)
    }
    #[doc = "Bit 8 - S0IN PRS Enable"]
    #[inline(always)]
    pub fn s0prsen(&mut self) -> S0prsenW<CfgSpec> {
        S0prsenW::new(self, 8)
    }
    #[doc = "Bit 9 - S1IN PRS Enable"]
    #[inline(always)]
    pub fn s1prsen(&mut self) -> S1prsenW<CfgSpec> {
        S1prsenW::new(self, 9)
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
