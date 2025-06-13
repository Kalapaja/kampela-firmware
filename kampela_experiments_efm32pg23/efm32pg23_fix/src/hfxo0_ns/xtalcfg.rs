#[doc = "Register `XTALCFG` reader"]
pub type R = crate::R<XtalcfgSpec>;
#[doc = "Register `XTALCFG` writer"]
pub type W = crate::W<XtalcfgSpec>;
#[doc = "Field `COREBIASSTARTUPI` reader - Intermediate Startup Core Bias Current"]
pub type CorebiasstartupiR = crate::FieldReader;
#[doc = "Field `COREBIASSTARTUPI` writer - Intermediate Startup Core Bias Current"]
pub type CorebiasstartupiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `COREBIASSTARTUP` reader - Startup Core Bias Current"]
pub type CorebiasstartupR = crate::FieldReader;
#[doc = "Field `COREBIASSTARTUP` writer - Startup Core Bias Current"]
pub type CorebiasstartupW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CTUNEXISTARTUP` reader - Startup Tuning Capacitance on XI"]
pub type CtunexistartupR = crate::FieldReader;
#[doc = "Field `CTUNEXISTARTUP` writer - Startup Tuning Capacitance on XI"]
pub type CtunexistartupW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CTUNEXOSTARTUP` reader - Startup Tuning Capacitance on XO"]
pub type CtunexostartupR = crate::FieldReader;
#[doc = "Field `CTUNEXOSTARTUP` writer - Startup Tuning Capacitance on XO"]
pub type CtunexostartupW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Steady State Timeout\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timeoutsteady {
    #[doc = "0: The steady state timeout is set to 16 us minimum. The maximum can be +40%."]
    T4us = 0,
    #[doc = "1: The steady state timeout is set to 41 us minimum. The maximum can be +40%."]
    T16us = 1,
    #[doc = "2: The steady state timeout is set to 83 us minimum. The maximum can be +40%."]
    T41us = 2,
    #[doc = "3: The steady state timeout is set to 125 us minimum. The maximum can be +40%."]
    T83us = 3,
    #[doc = "4: The steady state timeout is set to 166 us minimum. The maximum can be +40%."]
    T125us = 4,
    #[doc = "5: The steady state timeout is set to 208 us minimum. The maximum can be +40%."]
    T166us = 5,
    #[doc = "6: The steady state timeout is set to 250 us minimum. The maximum can be +40%."]
    T208us = 6,
    #[doc = "7: The steady state timeout is set to 333 us minimum. The maximum can be +40%."]
    T250us = 7,
    #[doc = "8: The steady state timeout is set to 416 us minimum. The maximum can be +40%."]
    T333us = 8,
    #[doc = "9: The steady state timeout is set to 500 us minimum. The maximum can be +40%."]
    T416us = 9,
    #[doc = "10: The steady state timeout is set to 666 us minimum. The maximum can be +40%."]
    T500us = 10,
    #[doc = "11: The steady state timeout is set to 833 us minimum. The maximum can be +40%."]
    T666us = 11,
    #[doc = "12: The steady state timeout is set to 1666 us minimum. The maximum can be +40%."]
    T833us = 12,
    #[doc = "13: The steady state timeout is set to 2500 us minimum. The maximum can be +40%."]
    T1666us = 13,
    #[doc = "14: The steady state timeout is set to 4166 us minimum. The maximum can be +40%."]
    T2500us = 14,
    #[doc = "15: The steady state timeout is set to 7500 us minimum. The maximum can be +40%."]
    T4166us = 15,
}
impl From<Timeoutsteady> for u8 {
    #[inline(always)]
    fn from(variant: Timeoutsteady) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timeoutsteady {
    type Ux = u8;
}
impl crate::IsEnum for Timeoutsteady {}
#[doc = "Field `TIMEOUTSTEADY` reader - Steady State Timeout"]
pub type TimeoutsteadyR = crate::FieldReader<Timeoutsteady>;
impl TimeoutsteadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timeoutsteady {
        match self.bits {
            0 => Timeoutsteady::T4us,
            1 => Timeoutsteady::T16us,
            2 => Timeoutsteady::T41us,
            3 => Timeoutsteady::T83us,
            4 => Timeoutsteady::T125us,
            5 => Timeoutsteady::T166us,
            6 => Timeoutsteady::T208us,
            7 => Timeoutsteady::T250us,
            8 => Timeoutsteady::T333us,
            9 => Timeoutsteady::T416us,
            10 => Timeoutsteady::T500us,
            11 => Timeoutsteady::T666us,
            12 => Timeoutsteady::T833us,
            13 => Timeoutsteady::T1666us,
            14 => Timeoutsteady::T2500us,
            15 => Timeoutsteady::T4166us,
            _ => unreachable!(),
        }
    }
    #[doc = "The steady state timeout is set to 16 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t4us(&self) -> bool {
        *self == Timeoutsteady::T4us
    }
    #[doc = "The steady state timeout is set to 41 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t16us(&self) -> bool {
        *self == Timeoutsteady::T16us
    }
    #[doc = "The steady state timeout is set to 83 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t41us(&self) -> bool {
        *self == Timeoutsteady::T41us
    }
    #[doc = "The steady state timeout is set to 125 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t83us(&self) -> bool {
        *self == Timeoutsteady::T83us
    }
    #[doc = "The steady state timeout is set to 166 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t125us(&self) -> bool {
        *self == Timeoutsteady::T125us
    }
    #[doc = "The steady state timeout is set to 208 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t166us(&self) -> bool {
        *self == Timeoutsteady::T166us
    }
    #[doc = "The steady state timeout is set to 250 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t208us(&self) -> bool {
        *self == Timeoutsteady::T208us
    }
    #[doc = "The steady state timeout is set to 333 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t250us(&self) -> bool {
        *self == Timeoutsteady::T250us
    }
    #[doc = "The steady state timeout is set to 416 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t333us(&self) -> bool {
        *self == Timeoutsteady::T333us
    }
    #[doc = "The steady state timeout is set to 500 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t416us(&self) -> bool {
        *self == Timeoutsteady::T416us
    }
    #[doc = "The steady state timeout is set to 666 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t500us(&self) -> bool {
        *self == Timeoutsteady::T500us
    }
    #[doc = "The steady state timeout is set to 833 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t666us(&self) -> bool {
        *self == Timeoutsteady::T666us
    }
    #[doc = "The steady state timeout is set to 1666 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t833us(&self) -> bool {
        *self == Timeoutsteady::T833us
    }
    #[doc = "The steady state timeout is set to 2500 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t1666us(&self) -> bool {
        *self == Timeoutsteady::T1666us
    }
    #[doc = "The steady state timeout is set to 4166 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t2500us(&self) -> bool {
        *self == Timeoutsteady::T2500us
    }
    #[doc = "The steady state timeout is set to 7500 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t4166us(&self) -> bool {
        *self == Timeoutsteady::T4166us
    }
}
#[doc = "Field `TIMEOUTSTEADY` writer - Steady State Timeout"]
pub type TimeoutsteadyW<'a, REG> = crate::FieldWriter<'a, REG, 4, Timeoutsteady, crate::Safe>;
impl<'a, REG> TimeoutsteadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The steady state timeout is set to 16 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t4us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T4us)
    }
    #[doc = "The steady state timeout is set to 41 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t16us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T16us)
    }
    #[doc = "The steady state timeout is set to 83 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t41us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T41us)
    }
    #[doc = "The steady state timeout is set to 125 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t83us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T83us)
    }
    #[doc = "The steady state timeout is set to 166 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t125us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T125us)
    }
    #[doc = "The steady state timeout is set to 208 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t166us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T166us)
    }
    #[doc = "The steady state timeout is set to 250 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t208us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T208us)
    }
    #[doc = "The steady state timeout is set to 333 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t250us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T250us)
    }
    #[doc = "The steady state timeout is set to 416 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t333us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T333us)
    }
    #[doc = "The steady state timeout is set to 500 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t416us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T416us)
    }
    #[doc = "The steady state timeout is set to 666 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t500us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T500us)
    }
    #[doc = "The steady state timeout is set to 833 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t666us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T666us)
    }
    #[doc = "The steady state timeout is set to 1666 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t833us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T833us)
    }
    #[doc = "The steady state timeout is set to 2500 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t1666us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T1666us)
    }
    #[doc = "The steady state timeout is set to 4166 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t2500us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T2500us)
    }
    #[doc = "The steady state timeout is set to 7500 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t4166us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutsteady::T4166us)
    }
}
#[doc = "Core Bias LSB Change Timeout\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timeoutcblsb {
    #[doc = "0: The core bias LSB change timeout is set to 8 us minimum. The maximum can be +40%."]
    T8us = 0,
    #[doc = "1: The core bias LSB change timeout is set to 20 us minimum. The maximum can be +40%."]
    T20us = 1,
    #[doc = "2: The core bias LSB change timeout is set to 41 us minimum. The maximum can be +40%."]
    T41us = 2,
    #[doc = "3: The core bias LSB change timeout is set to 62 us minimum. The maximum can be +40%."]
    T62us = 3,
    #[doc = "4: The core bias LSB change timeout is set to 83 us minimum. The maximum can be +40%."]
    T83us = 4,
    #[doc = "5: The core bias LSB change timeout is set to 104 us minimum. The maximum can be +40%."]
    T104us = 5,
    #[doc = "6: The core bias LSB change timeout is set to 125 us minimum. The maximum can be +40%."]
    T125us = 6,
    #[doc = "7: The core bias LSB change timeout is set to 166 us minimum. The maximum can be +40%."]
    T166us = 7,
    #[doc = "8: The core bias LSB change timeout is set to 208 us minimum. The maximum can be +40%."]
    T208us = 8,
    #[doc = "9: The core bias LSB change timeout is set to 250 us minimum. The maximum can be +40%."]
    T250us = 9,
    #[doc = "10: The core bias LSB change timeout is set to 333 us minimum. The maximum can be +40%."]
    T333us = 10,
    #[doc = "11: The core bias LSB change timeout is set to 416 us minimum. The maximum can be +40%."]
    T416us = 11,
    #[doc = "12: The core bias LSB change timeout is set to 833 us minimum. The maximum can be +40%."]
    T833us = 12,
    #[doc = "13: The core bias LSB change timeout is set to 1250 us minimum. The maximum can be +40%."]
    T1250us = 13,
    #[doc = "14: The core bias LSB change timeout is set to 2083 us minimum. The maximum can be +40%."]
    T2083us = 14,
    #[doc = "15: The core bias LSB change timeout is set to 3750 us minimum. The maximum can be +40%."]
    T3750us = 15,
}
impl From<Timeoutcblsb> for u8 {
    #[inline(always)]
    fn from(variant: Timeoutcblsb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timeoutcblsb {
    type Ux = u8;
}
impl crate::IsEnum for Timeoutcblsb {}
#[doc = "Field `TIMEOUTCBLSB` reader - Core Bias LSB Change Timeout"]
pub type TimeoutcblsbR = crate::FieldReader<Timeoutcblsb>;
impl TimeoutcblsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timeoutcblsb {
        match self.bits {
            0 => Timeoutcblsb::T8us,
            1 => Timeoutcblsb::T20us,
            2 => Timeoutcblsb::T41us,
            3 => Timeoutcblsb::T62us,
            4 => Timeoutcblsb::T83us,
            5 => Timeoutcblsb::T104us,
            6 => Timeoutcblsb::T125us,
            7 => Timeoutcblsb::T166us,
            8 => Timeoutcblsb::T208us,
            9 => Timeoutcblsb::T250us,
            10 => Timeoutcblsb::T333us,
            11 => Timeoutcblsb::T416us,
            12 => Timeoutcblsb::T833us,
            13 => Timeoutcblsb::T1250us,
            14 => Timeoutcblsb::T2083us,
            15 => Timeoutcblsb::T3750us,
            _ => unreachable!(),
        }
    }
    #[doc = "The core bias LSB change timeout is set to 8 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t8us(&self) -> bool {
        *self == Timeoutcblsb::T8us
    }
    #[doc = "The core bias LSB change timeout is set to 20 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t20us(&self) -> bool {
        *self == Timeoutcblsb::T20us
    }
    #[doc = "The core bias LSB change timeout is set to 41 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t41us(&self) -> bool {
        *self == Timeoutcblsb::T41us
    }
    #[doc = "The core bias LSB change timeout is set to 62 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t62us(&self) -> bool {
        *self == Timeoutcblsb::T62us
    }
    #[doc = "The core bias LSB change timeout is set to 83 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t83us(&self) -> bool {
        *self == Timeoutcblsb::T83us
    }
    #[doc = "The core bias LSB change timeout is set to 104 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t104us(&self) -> bool {
        *self == Timeoutcblsb::T104us
    }
    #[doc = "The core bias LSB change timeout is set to 125 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t125us(&self) -> bool {
        *self == Timeoutcblsb::T125us
    }
    #[doc = "The core bias LSB change timeout is set to 166 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t166us(&self) -> bool {
        *self == Timeoutcblsb::T166us
    }
    #[doc = "The core bias LSB change timeout is set to 208 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t208us(&self) -> bool {
        *self == Timeoutcblsb::T208us
    }
    #[doc = "The core bias LSB change timeout is set to 250 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t250us(&self) -> bool {
        *self == Timeoutcblsb::T250us
    }
    #[doc = "The core bias LSB change timeout is set to 333 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t333us(&self) -> bool {
        *self == Timeoutcblsb::T333us
    }
    #[doc = "The core bias LSB change timeout is set to 416 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t416us(&self) -> bool {
        *self == Timeoutcblsb::T416us
    }
    #[doc = "The core bias LSB change timeout is set to 833 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t833us(&self) -> bool {
        *self == Timeoutcblsb::T833us
    }
    #[doc = "The core bias LSB change timeout is set to 1250 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t1250us(&self) -> bool {
        *self == Timeoutcblsb::T1250us
    }
    #[doc = "The core bias LSB change timeout is set to 2083 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t2083us(&self) -> bool {
        *self == Timeoutcblsb::T2083us
    }
    #[doc = "The core bias LSB change timeout is set to 3750 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn is_t3750us(&self) -> bool {
        *self == Timeoutcblsb::T3750us
    }
}
#[doc = "Field `TIMEOUTCBLSB` writer - Core Bias LSB Change Timeout"]
pub type TimeoutcblsbW<'a, REG> = crate::FieldWriter<'a, REG, 4, Timeoutcblsb, crate::Safe>;
impl<'a, REG> TimeoutcblsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The core bias LSB change timeout is set to 8 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t8us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T8us)
    }
    #[doc = "The core bias LSB change timeout is set to 20 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t20us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T20us)
    }
    #[doc = "The core bias LSB change timeout is set to 41 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t41us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T41us)
    }
    #[doc = "The core bias LSB change timeout is set to 62 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t62us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T62us)
    }
    #[doc = "The core bias LSB change timeout is set to 83 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t83us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T83us)
    }
    #[doc = "The core bias LSB change timeout is set to 104 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t104us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T104us)
    }
    #[doc = "The core bias LSB change timeout is set to 125 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t125us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T125us)
    }
    #[doc = "The core bias LSB change timeout is set to 166 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t166us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T166us)
    }
    #[doc = "The core bias LSB change timeout is set to 208 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t208us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T208us)
    }
    #[doc = "The core bias LSB change timeout is set to 250 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t250us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T250us)
    }
    #[doc = "The core bias LSB change timeout is set to 333 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t333us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T333us)
    }
    #[doc = "The core bias LSB change timeout is set to 416 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t416us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T416us)
    }
    #[doc = "The core bias LSB change timeout is set to 833 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t833us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T833us)
    }
    #[doc = "The core bias LSB change timeout is set to 1250 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t1250us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T1250us)
    }
    #[doc = "The core bias LSB change timeout is set to 2083 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t2083us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T2083us)
    }
    #[doc = "The core bias LSB change timeout is set to 3750 us minimum. The maximum can be +40%."]
    #[inline(always)]
    pub fn t3750us(self) -> &'a mut crate::W<REG> {
        self.variant(Timeoutcblsb::T3750us)
    }
}
impl R {
    #[doc = "Bits 0:5 - Intermediate Startup Core Bias Current"]
    #[inline(always)]
    pub fn corebiasstartupi(&self) -> CorebiasstartupiR {
        CorebiasstartupiR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Startup Core Bias Current"]
    #[inline(always)]
    pub fn corebiasstartup(&self) -> CorebiasstartupR {
        CorebiasstartupR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:15 - Startup Tuning Capacitance on XI"]
    #[inline(always)]
    pub fn ctunexistartup(&self) -> CtunexistartupR {
        CtunexistartupR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Startup Tuning Capacitance on XO"]
    #[inline(always)]
    pub fn ctunexostartup(&self) -> CtunexostartupR {
        CtunexostartupR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Steady State Timeout"]
    #[inline(always)]
    pub fn timeoutsteady(&self) -> TimeoutsteadyR {
        TimeoutsteadyR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Core Bias LSB Change Timeout"]
    #[inline(always)]
    pub fn timeoutcblsb(&self) -> TimeoutcblsbR {
        TimeoutcblsbR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Intermediate Startup Core Bias Current"]
    #[inline(always)]
    pub fn corebiasstartupi(&mut self) -> CorebiasstartupiW<XtalcfgSpec> {
        CorebiasstartupiW::new(self, 0)
    }
    #[doc = "Bits 6:11 - Startup Core Bias Current"]
    #[inline(always)]
    pub fn corebiasstartup(&mut self) -> CorebiasstartupW<XtalcfgSpec> {
        CorebiasstartupW::new(self, 6)
    }
    #[doc = "Bits 12:15 - Startup Tuning Capacitance on XI"]
    #[inline(always)]
    pub fn ctunexistartup(&mut self) -> CtunexistartupW<XtalcfgSpec> {
        CtunexistartupW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Startup Tuning Capacitance on XO"]
    #[inline(always)]
    pub fn ctunexostartup(&mut self) -> CtunexostartupW<XtalcfgSpec> {
        CtunexostartupW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Steady State Timeout"]
    #[inline(always)]
    pub fn timeoutsteady(&mut self) -> TimeoutsteadyW<XtalcfgSpec> {
        TimeoutsteadyW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Core Bias LSB Change Timeout"]
    #[inline(always)]
    pub fn timeoutcblsb(&mut self) -> TimeoutcblsbW<XtalcfgSpec> {
        TimeoutcblsbW::new(self, 24)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalcfgSpec;
impl crate::RegisterSpec for XtalcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtalcfg::R`](R) reader structure"]
impl crate::Readable for XtalcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalcfg::W`](W) writer structure"]
impl crate::Writable for XtalcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTALCFG to value 0x0bb0_0820"]
impl crate::Resettable for XtalcfgSpec {
    const RESET_VALUE: u32 = 0x0bb0_0820;
}
