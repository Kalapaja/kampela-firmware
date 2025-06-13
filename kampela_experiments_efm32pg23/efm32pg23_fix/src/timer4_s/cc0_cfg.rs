#[doc = "Register `CC0_CFG` reader"]
pub type R = crate::R<Cc0CfgSpec>;
#[doc = "Register `CC0_CFG` writer"]
pub type W = crate::W<Cc0CfgSpec>;
#[doc = "CC Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Compare/Capture channel turned off"]
    Off = 0,
    #[doc = "1: Input Capture"]
    Inputcapture = 1,
    #[doc = "2: Output Compare"]
    Outputcompare = 2,
    #[doc = "3: Pulse-Width Modulation"]
    Pwm = 3,
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
#[doc = "Field `MODE` reader - CC Channel Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Off,
            1 => Mode::Inputcapture,
            2 => Mode::Outputcompare,
            3 => Mode::Pwm,
            _ => unreachable!(),
        }
    }
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mode::Off
    }
    #[doc = "Input Capture"]
    #[inline(always)]
    pub fn is_inputcapture(&self) -> bool {
        *self == Mode::Inputcapture
    }
    #[doc = "Output Compare"]
    #[inline(always)]
    pub fn is_outputcompare(&self) -> bool {
        *self == Mode::Outputcompare
    }
    #[doc = "Pulse-Width Modulation"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Mode::Pwm
    }
}
#[doc = "Field `MODE` writer - CC Channel Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Off)
    }
    #[doc = "Input Capture"]
    #[inline(always)]
    pub fn inputcapture(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Inputcapture)
    }
    #[doc = "Output Compare"]
    #[inline(always)]
    pub fn outputcompare(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Outputcompare)
    }
    #[doc = "Pulse-Width Modulation"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Pwm)
    }
}
#[doc = "Field `COIST` reader - Compare Output Initial State"]
pub type CoistR = crate::BitReader;
#[doc = "Field `COIST` writer - Compare Output Initial State"]
pub type CoistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Insel {
    #[doc = "0: TIMERnCCx pin is selected"]
    Pin = 0,
    #[doc = "1: Synchornous PRS selected"]
    Prssync = 1,
    #[doc = "2: Asynchronous Level PRS selected"]
    Prsasynclevel = 2,
    #[doc = "3: Asynchronous Pulse PRS selected"]
    Prsasyncpulse = 3,
}
impl From<Insel> for u8 {
    #[inline(always)]
    fn from(variant: Insel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Insel {
    type Ux = u8;
}
impl crate::IsEnum for Insel {}
#[doc = "Field `INSEL` reader - Input Selection"]
pub type InselR = crate::FieldReader<Insel>;
impl InselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Insel {
        match self.bits {
            0 => Insel::Pin,
            1 => Insel::Prssync,
            2 => Insel::Prsasynclevel,
            3 => Insel::Prsasyncpulse,
            _ => unreachable!(),
        }
    }
    #[doc = "TIMERnCCx pin is selected"]
    #[inline(always)]
    pub fn is_pin(&self) -> bool {
        *self == Insel::Pin
    }
    #[doc = "Synchornous PRS selected"]
    #[inline(always)]
    pub fn is_prssync(&self) -> bool {
        *self == Insel::Prssync
    }
    #[doc = "Asynchronous Level PRS selected"]
    #[inline(always)]
    pub fn is_prsasynclevel(&self) -> bool {
        *self == Insel::Prsasynclevel
    }
    #[doc = "Asynchronous Pulse PRS selected"]
    #[inline(always)]
    pub fn is_prsasyncpulse(&self) -> bool {
        *self == Insel::Prsasyncpulse
    }
}
#[doc = "Field `INSEL` writer - Input Selection"]
pub type InselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Insel, crate::Safe>;
impl<'a, REG> InselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIMERnCCx pin is selected"]
    #[inline(always)]
    pub fn pin(self) -> &'a mut crate::W<REG> {
        self.variant(Insel::Pin)
    }
    #[doc = "Synchornous PRS selected"]
    #[inline(always)]
    pub fn prssync(self) -> &'a mut crate::W<REG> {
        self.variant(Insel::Prssync)
    }
    #[doc = "Asynchronous Level PRS selected"]
    #[inline(always)]
    pub fn prsasynclevel(self) -> &'a mut crate::W<REG> {
        self.variant(Insel::Prsasynclevel)
    }
    #[doc = "Asynchronous Pulse PRS selected"]
    #[inline(always)]
    pub fn prsasyncpulse(self) -> &'a mut crate::W<REG> {
        self.variant(Insel::Prsasyncpulse)
    }
}
#[doc = "PRS Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prsconf {
    #[doc = "0: Each CC event will generate a one EM01GRPACLK cycle high pulse"]
    Pulse = 0,
    #[doc = "1: The PRS channel will follow CC out"]
    Level = 1,
}
impl From<Prsconf> for bool {
    #[inline(always)]
    fn from(variant: Prsconf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRSCONF` reader - PRS Configuration"]
pub type PrsconfR = crate::BitReader<Prsconf>;
impl PrsconfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prsconf {
        match self.bits {
            false => Prsconf::Pulse,
            true => Prsconf::Level,
        }
    }
    #[doc = "Each CC event will generate a one EM01GRPACLK cycle high pulse"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == Prsconf::Pulse
    }
    #[doc = "The PRS channel will follow CC out"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == Prsconf::Level
    }
}
#[doc = "Field `PRSCONF` writer - PRS Configuration"]
pub type PrsconfW<'a, REG> = crate::BitWriter<'a, REG, Prsconf>;
impl<'a, REG> PrsconfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Each CC event will generate a one EM01GRPACLK cycle high pulse"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Prsconf::Pulse)
    }
    #[doc = "The PRS channel will follow CC out"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(Prsconf::Level)
    }
}
#[doc = "Digital Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filt {
    #[doc = "0: Digital Filter Disabled"]
    Disable = 0,
    #[doc = "1: Digital Filter Enabled"]
    Enable = 1,
}
impl From<Filt> for bool {
    #[inline(always)]
    fn from(variant: Filt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILT` reader - Digital Filter"]
pub type FiltR = crate::BitReader<Filt>;
impl FiltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filt {
        match self.bits {
            false => Filt::Disable,
            true => Filt::Enable,
        }
    }
    #[doc = "Digital Filter Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Filt::Disable
    }
    #[doc = "Digital Filter Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Filt::Enable
    }
}
#[doc = "Field `FILT` writer - Digital Filter"]
pub type FiltW<'a, REG> = crate::BitWriter<'a, REG, Filt>;
impl<'a, REG> FiltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Digital Filter Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filt::Disable)
    }
    #[doc = "Digital Filter Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Filt::Enable)
    }
}
#[doc = "Field `ICFWL` reader - Input Capture FIFO watermark level"]
pub type IcfwlR = crate::BitReader;
#[doc = "Field `ICFWL` writer - Input Capture FIFO watermark level"]
pub type IcfwlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Compare Output Initial State"]
    #[inline(always)]
    pub fn coist(&self) -> CoistR {
        CoistR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Input Selection"]
    #[inline(always)]
    pub fn insel(&self) -> InselR {
        InselR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - PRS Configuration"]
    #[inline(always)]
    pub fn prsconf(&self) -> PrsconfR {
        PrsconfR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Digital Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FiltR {
        FiltR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input Capture FIFO watermark level"]
    #[inline(always)]
    pub fn icfwl(&self) -> IcfwlR {
        IcfwlR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<Cc0CfgSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 4 - Compare Output Initial State"]
    #[inline(always)]
    pub fn coist(&mut self) -> CoistW<Cc0CfgSpec> {
        CoistW::new(self, 4)
    }
    #[doc = "Bits 17:18 - Input Selection"]
    #[inline(always)]
    pub fn insel(&mut self) -> InselW<Cc0CfgSpec> {
        InselW::new(self, 17)
    }
    #[doc = "Bit 19 - PRS Configuration"]
    #[inline(always)]
    pub fn prsconf(&mut self) -> PrsconfW<Cc0CfgSpec> {
        PrsconfW::new(self, 19)
    }
    #[doc = "Bit 20 - Digital Filter"]
    #[inline(always)]
    pub fn filt(&mut self) -> FiltW<Cc0CfgSpec> {
        FiltW::new(self, 20)
    }
    #[doc = "Bit 21 - Input Capture FIFO watermark level"]
    #[inline(always)]
    pub fn icfwl(&mut self) -> IcfwlW<Cc0CfgSpec> {
        IcfwlW::new(self, 21)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc0CfgSpec;
impl crate::RegisterSpec for Cc0CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_cfg::R`](R) reader structure"]
impl crate::Readable for Cc0CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cc0_cfg::W`](W) writer structure"]
impl crate::Writable for Cc0CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC0_CFG to value 0"]
impl crate::Resettable for Cc0CfgSpec {}
