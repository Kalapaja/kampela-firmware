#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `BIAS` reader - Bias Configuration"]
pub type BiasR = crate::FieldReader;
#[doc = "Field `BIAS` writer - Bias Configuration"]
pub type BiasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Hysteresis mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hyst {
    #[doc = "0: Hysteresis disabled"]
    Disabled = 0,
    #[doc = "1: 10mV symmetrical hysteresis"]
    Sym10mv = 1,
    #[doc = "2: 20mV symmetrical hysteresis"]
    Sym20mv = 2,
    #[doc = "3: 30mV symmetrical hysteresis"]
    Sym30mv = 3,
    #[doc = "4: 10mV hysteresis on positive edge transitions"]
    Pos10mv = 4,
    #[doc = "5: 20mV hysteresis on positive edge transitions"]
    Pos20mv = 5,
    #[doc = "6: 30mV hysteresis on positive edge transitions"]
    Pos30mv = 6,
    #[doc = "8: 10mV hysteresis on negative edge transitions"]
    Neg10mv = 8,
    #[doc = "9: 20mV hysteresis on negative edge transitions"]
    Neg20mv = 9,
    #[doc = "10: 30mV hysteresis on negative edge transitions"]
    Neg30mv = 10,
}
impl From<Hyst> for u8 {
    #[inline(always)]
    fn from(variant: Hyst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hyst {
    type Ux = u8;
}
impl crate::IsEnum for Hyst {}
#[doc = "Field `HYST` reader - Hysteresis mode"]
pub type HystR = crate::FieldReader<Hyst>;
impl HystR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hyst> {
        match self.bits {
            0 => Some(Hyst::Disabled),
            1 => Some(Hyst::Sym10mv),
            2 => Some(Hyst::Sym20mv),
            3 => Some(Hyst::Sym30mv),
            4 => Some(Hyst::Pos10mv),
            5 => Some(Hyst::Pos20mv),
            6 => Some(Hyst::Pos30mv),
            8 => Some(Hyst::Neg10mv),
            9 => Some(Hyst::Neg20mv),
            10 => Some(Hyst::Neg30mv),
            _ => None,
        }
    }
    #[doc = "Hysteresis disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hyst::Disabled
    }
    #[doc = "10mV symmetrical hysteresis"]
    #[inline(always)]
    pub fn is_sym10mv(&self) -> bool {
        *self == Hyst::Sym10mv
    }
    #[doc = "20mV symmetrical hysteresis"]
    #[inline(always)]
    pub fn is_sym20mv(&self) -> bool {
        *self == Hyst::Sym20mv
    }
    #[doc = "30mV symmetrical hysteresis"]
    #[inline(always)]
    pub fn is_sym30mv(&self) -> bool {
        *self == Hyst::Sym30mv
    }
    #[doc = "10mV hysteresis on positive edge transitions"]
    #[inline(always)]
    pub fn is_pos10mv(&self) -> bool {
        *self == Hyst::Pos10mv
    }
    #[doc = "20mV hysteresis on positive edge transitions"]
    #[inline(always)]
    pub fn is_pos20mv(&self) -> bool {
        *self == Hyst::Pos20mv
    }
    #[doc = "30mV hysteresis on positive edge transitions"]
    #[inline(always)]
    pub fn is_pos30mv(&self) -> bool {
        *self == Hyst::Pos30mv
    }
    #[doc = "10mV hysteresis on negative edge transitions"]
    #[inline(always)]
    pub fn is_neg10mv(&self) -> bool {
        *self == Hyst::Neg10mv
    }
    #[doc = "20mV hysteresis on negative edge transitions"]
    #[inline(always)]
    pub fn is_neg20mv(&self) -> bool {
        *self == Hyst::Neg20mv
    }
    #[doc = "30mV hysteresis on negative edge transitions"]
    #[inline(always)]
    pub fn is_neg30mv(&self) -> bool {
        *self == Hyst::Neg30mv
    }
}
#[doc = "Field `HYST` writer - Hysteresis mode"]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 4, Hyst>;
impl<'a, REG> HystW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Hysteresis disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Disabled)
    }
    #[doc = "10mV symmetrical hysteresis"]
    #[inline(always)]
    pub fn sym10mv(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Sym10mv)
    }
    #[doc = "20mV symmetrical hysteresis"]
    #[inline(always)]
    pub fn sym20mv(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Sym20mv)
    }
    #[doc = "30mV symmetrical hysteresis"]
    #[inline(always)]
    pub fn sym30mv(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Sym30mv)
    }
    #[doc = "10mV hysteresis on positive edge transitions"]
    #[inline(always)]
    pub fn pos10mv(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Pos10mv)
    }
    #[doc = "20mV hysteresis on positive edge transitions"]
    #[inline(always)]
    pub fn pos20mv(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Pos20mv)
    }
    #[doc = "30mV hysteresis on positive edge transitions"]
    #[inline(always)]
    pub fn pos30mv(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Pos30mv)
    }
    #[doc = "10mV hysteresis on negative edge transitions"]
    #[inline(always)]
    pub fn neg10mv(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Neg10mv)
    }
    #[doc = "20mV hysteresis on negative edge transitions"]
    #[inline(always)]
    pub fn neg20mv(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Neg20mv)
    }
    #[doc = "30mV hysteresis on negative edge transitions"]
    #[inline(always)]
    pub fn neg30mv(self) -> &'a mut crate::W<REG> {
        self.variant(Hyst::Neg30mv)
    }
}
#[doc = "Input Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inputrange {
    #[doc = "0: Use this setting when the input to the comparator core can be from 0 to AVDD."]
    Full = 0,
    #[doc = "1: It is recommended to use this setting when the input to the comparator core will always be less than AVDD-0.7V."]
    Reduced = 1,
}
impl From<Inputrange> for bool {
    #[inline(always)]
    fn from(variant: Inputrange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUTRANGE` reader - Input Range"]
pub type InputrangeR = crate::BitReader<Inputrange>;
impl InputrangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inputrange {
        match self.bits {
            false => Inputrange::Full,
            true => Inputrange::Reduced,
        }
    }
    #[doc = "Use this setting when the input to the comparator core can be from 0 to AVDD."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Inputrange::Full
    }
    #[doc = "It is recommended to use this setting when the input to the comparator core will always be less than AVDD-0.7V."]
    #[inline(always)]
    pub fn is_reduced(&self) -> bool {
        *self == Inputrange::Reduced
    }
}
#[doc = "Field `INPUTRANGE` writer - Input Range"]
pub type InputrangeW<'a, REG> = crate::BitWriter<'a, REG, Inputrange>;
impl<'a, REG> InputrangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use this setting when the input to the comparator core can be from 0 to AVDD."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Inputrange::Full)
    }
    #[doc = "It is recommended to use this setting when the input to the comparator core will always be less than AVDD-0.7V."]
    #[inline(always)]
    pub fn reduced(self) -> &'a mut crate::W<REG> {
        self.variant(Inputrange::Reduced)
    }
}
#[doc = "ACMP accuracy mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accuracy {
    #[doc = "0: ACMP operates in low-accuracy mode but consumes less current."]
    Low = 0,
    #[doc = "1: ACMP operates in high-accuracy mode but consumes more current."]
    High = 1,
}
impl From<Accuracy> for bool {
    #[inline(always)]
    fn from(variant: Accuracy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCURACY` reader - ACMP accuracy mode"]
pub type AccuracyR = crate::BitReader<Accuracy>;
impl AccuracyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Accuracy {
        match self.bits {
            false => Accuracy::Low,
            true => Accuracy::High,
        }
    }
    #[doc = "ACMP operates in low-accuracy mode but consumes less current."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Accuracy::Low
    }
    #[doc = "ACMP operates in high-accuracy mode but consumes more current."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Accuracy::High
    }
}
#[doc = "Field `ACCURACY` writer - ACMP accuracy mode"]
pub type AccuracyW<'a, REG> = crate::BitWriter<'a, REG, Accuracy>;
impl<'a, REG> AccuracyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ACMP operates in low-accuracy mode but consumes less current."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Accuracy::Low)
    }
    #[doc = "ACMP operates in high-accuracy mode but consumes more current."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Accuracy::High)
    }
}
impl R {
    #[doc = "Bits 0:2 - Bias Configuration"]
    #[inline(always)]
    pub fn bias(&self) -> BiasR {
        BiasR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - Hysteresis mode"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Input Range"]
    #[inline(always)]
    pub fn inputrange(&self) -> InputrangeR {
        InputrangeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ACMP accuracy mode"]
    #[inline(always)]
    pub fn accuracy(&self) -> AccuracyR {
        AccuracyR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bias Configuration"]
    #[inline(always)]
    pub fn bias(&mut self) -> BiasW<CfgSpec> {
        BiasW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Hysteresis mode"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<CfgSpec> {
        HystW::new(self, 8)
    }
    #[doc = "Bit 16 - Input Range"]
    #[inline(always)]
    pub fn inputrange(&mut self) -> InputrangeW<CfgSpec> {
        InputrangeW::new(self, 16)
    }
    #[doc = "Bit 17 - ACMP accuracy mode"]
    #[inline(always)]
    pub fn accuracy(&mut self) -> AccuracyW<CfgSpec> {
        AccuracyW::new(self, 17)
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
#[doc = "`reset()` method sets CFG to value 0x04"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x04;
}
