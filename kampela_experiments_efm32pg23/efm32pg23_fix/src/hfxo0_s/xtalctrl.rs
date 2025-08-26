#[doc = "Register `XTALCTRL` reader"]
pub type R = crate::R<XtalctrlSpec>;
#[doc = "Register `XTALCTRL` writer"]
pub type W = crate::W<XtalctrlSpec>;
#[doc = "Field `COREBIASANA` reader - Core Bias Current"]
pub type CorebiasanaR = crate::FieldReader;
#[doc = "Field `COREBIASANA` writer - Core Bias Current"]
pub type CorebiasanaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CTUNEXIANA` reader - Tuning Capacitance on XI"]
pub type CtunexianaR = crate::FieldReader;
#[doc = "Field `CTUNEXIANA` writer - Tuning Capacitance on XI"]
pub type CtunexianaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CTUNEXOANA` reader - Tuning Capacitance on XO"]
pub type CtunexoanaR = crate::FieldReader;
#[doc = "Field `CTUNEXOANA` writer - Tuning Capacitance on XO"]
pub type CtunexoanaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Fixed Tuning Capacitance\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctunefixana {
    #[doc = "0: Remove fixed capacitance on XI and XO nodes"]
    None = 0,
    #[doc = "1: Adds fixed capacitance on XI node"]
    Xi = 1,
    #[doc = "2: Adds fixed capacitance on XO node"]
    Xo = 2,
    #[doc = "3: Adds fixed capacitance on both XI and XO nodes"]
    Both = 3,
}
impl From<Ctunefixana> for u8 {
    #[inline(always)]
    fn from(variant: Ctunefixana) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctunefixana {
    type Ux = u8;
}
impl crate::IsEnum for Ctunefixana {}
#[doc = "Field `CTUNEFIXANA` reader - Fixed Tuning Capacitance"]
pub type CtunefixanaR = crate::FieldReader<Ctunefixana>;
impl CtunefixanaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctunefixana {
        match self.bits {
            0 => Ctunefixana::None,
            1 => Ctunefixana::Xi,
            2 => Ctunefixana::Xo,
            3 => Ctunefixana::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Remove fixed capacitance on XI and XO nodes"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ctunefixana::None
    }
    #[doc = "Adds fixed capacitance on XI node"]
    #[inline(always)]
    pub fn is_xi(&self) -> bool {
        *self == Ctunefixana::Xi
    }
    #[doc = "Adds fixed capacitance on XO node"]
    #[inline(always)]
    pub fn is_xo(&self) -> bool {
        *self == Ctunefixana::Xo
    }
    #[doc = "Adds fixed capacitance on both XI and XO nodes"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Ctunefixana::Both
    }
}
#[doc = "Field `CTUNEFIXANA` writer - Fixed Tuning Capacitance"]
pub type CtunefixanaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctunefixana, crate::Safe>;
impl<'a, REG> CtunefixanaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Remove fixed capacitance on XI and XO nodes"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ctunefixana::None)
    }
    #[doc = "Adds fixed capacitance on XI node"]
    #[inline(always)]
    pub fn xi(self) -> &'a mut crate::W<REG> {
        self.variant(Ctunefixana::Xi)
    }
    #[doc = "Adds fixed capacitance on XO node"]
    #[inline(always)]
    pub fn xo(self) -> &'a mut crate::W<REG> {
        self.variant(Ctunefixana::Xo)
    }
    #[doc = "Adds fixed capacitance on both XI and XO nodes"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Ctunefixana::Both)
    }
}
#[doc = "Core Degeneration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Coredgenana {
    #[doc = "0: Do not apply core degeneration resistence"]
    None = 0,
    #[doc = "1: Apply 33 ohm core degeneration resistence"]
    Dgen33 = 1,
    #[doc = "2: Apply 50 ohm core degeneration resistence"]
    Dgen50 = 2,
    #[doc = "3: Apply 100 ohm core degeneration resistence"]
    Dgen100 = 3,
}
impl From<Coredgenana> for u8 {
    #[inline(always)]
    fn from(variant: Coredgenana) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Coredgenana {
    type Ux = u8;
}
impl crate::IsEnum for Coredgenana {}
#[doc = "Field `COREDGENANA` reader - Core Degeneration"]
pub type CoredgenanaR = crate::FieldReader<Coredgenana>;
impl CoredgenanaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Coredgenana {
        match self.bits {
            0 => Coredgenana::None,
            1 => Coredgenana::Dgen33,
            2 => Coredgenana::Dgen50,
            3 => Coredgenana::Dgen100,
            _ => unreachable!(),
        }
    }
    #[doc = "Do not apply core degeneration resistence"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Coredgenana::None
    }
    #[doc = "Apply 33 ohm core degeneration resistence"]
    #[inline(always)]
    pub fn is_dgen33(&self) -> bool {
        *self == Coredgenana::Dgen33
    }
    #[doc = "Apply 50 ohm core degeneration resistence"]
    #[inline(always)]
    pub fn is_dgen50(&self) -> bool {
        *self == Coredgenana::Dgen50
    }
    #[doc = "Apply 100 ohm core degeneration resistence"]
    #[inline(always)]
    pub fn is_dgen100(&self) -> bool {
        *self == Coredgenana::Dgen100
    }
}
#[doc = "Field `COREDGENANA` writer - Core Degeneration"]
pub type CoredgenanaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Coredgenana, crate::Safe>;
impl<'a, REG> CoredgenanaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not apply core degeneration resistence"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Coredgenana::None)
    }
    #[doc = "Apply 33 ohm core degeneration resistence"]
    #[inline(always)]
    pub fn dgen33(self) -> &'a mut crate::W<REG> {
        self.variant(Coredgenana::Dgen33)
    }
    #[doc = "Apply 50 ohm core degeneration resistence"]
    #[inline(always)]
    pub fn dgen50(self) -> &'a mut crate::W<REG> {
        self.variant(Coredgenana::Dgen50)
    }
    #[doc = "Apply 100 ohm core degeneration resistence"]
    #[inline(always)]
    pub fn dgen100(self) -> &'a mut crate::W<REG> {
        self.variant(Coredgenana::Dgen100)
    }
}
#[doc = "Field `SKIPCOREBIASOPT` reader - Skip Core Bias Optimization"]
pub type SkipcorebiasoptR = crate::BitReader;
#[doc = "Field `SKIPCOREBIASOPT` writer - Skip Core Bias Optimization"]
pub type SkipcorebiasoptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Core Bias Current"]
    #[inline(always)]
    pub fn corebiasana(&self) -> CorebiasanaR {
        CorebiasanaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Tuning Capacitance on XI"]
    #[inline(always)]
    pub fn ctunexiana(&self) -> CtunexianaR {
        CtunexianaR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Tuning Capacitance on XO"]
    #[inline(always)]
    pub fn ctunexoana(&self) -> CtunexoanaR {
        CtunexoanaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Fixed Tuning Capacitance"]
    #[inline(always)]
    pub fn ctunefixana(&self) -> CtunefixanaR {
        CtunefixanaR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Core Degeneration"]
    #[inline(always)]
    pub fn coredgenana(&self) -> CoredgenanaR {
        CoredgenanaR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 31 - Skip Core Bias Optimization"]
    #[inline(always)]
    pub fn skipcorebiasopt(&self) -> SkipcorebiasoptR {
        SkipcorebiasoptR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Core Bias Current"]
    #[inline(always)]
    pub fn corebiasana(&mut self) -> CorebiasanaW<XtalctrlSpec> {
        CorebiasanaW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Tuning Capacitance on XI"]
    #[inline(always)]
    pub fn ctunexiana(&mut self) -> CtunexianaW<XtalctrlSpec> {
        CtunexianaW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Tuning Capacitance on XO"]
    #[inline(always)]
    pub fn ctunexoana(&mut self) -> CtunexoanaW<XtalctrlSpec> {
        CtunexoanaW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Fixed Tuning Capacitance"]
    #[inline(always)]
    pub fn ctunefixana(&mut self) -> CtunefixanaW<XtalctrlSpec> {
        CtunefixanaW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Core Degeneration"]
    #[inline(always)]
    pub fn coredgenana(&mut self) -> CoredgenanaW<XtalctrlSpec> {
        CoredgenanaW::new(self, 26)
    }
    #[doc = "Bit 31 - Skip Core Bias Optimization"]
    #[inline(always)]
    pub fn skipcorebiasopt(&mut self) -> SkipcorebiasoptW<XtalctrlSpec> {
        SkipcorebiasoptW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalctrlSpec;
impl crate::RegisterSpec for XtalctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtalctrl::R`](R) reader structure"]
impl crate::Readable for XtalctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalctrl::W`](W) writer structure"]
impl crate::Writable for XtalctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTALCTRL to value 0x033c_3c3c"]
impl crate::Resettable for XtalctrlSpec {
    const RESET_VALUE: u32 = 0x033c_3c3c;
}
