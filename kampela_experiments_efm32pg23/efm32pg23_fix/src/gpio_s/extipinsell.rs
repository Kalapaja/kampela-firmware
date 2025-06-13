#[doc = "Register `EXTIPINSELL` reader"]
pub type R = crate::R<ExtipinsellSpec>;
#[doc = "Register `EXTIPINSELL` writer"]
pub type W = crate::W<ExtipinsellSpec>;
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel0 {
    #[doc = "0: OFFSET=0"]
    Pin0 = 0,
    #[doc = "1: OFFSET=1"]
    Pin1 = 1,
    #[doc = "2: OFFSET=2"]
    Pin2 = 2,
    #[doc = "3: OFFSET=3"]
    Pin3 = 3,
}
impl From<Extipinsel0> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel0 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel0 {}
#[doc = "Field `EXTIPINSEL0` reader - External Interrupt Pin select"]
pub type Extipinsel0R = crate::FieldReader<Extipinsel0>;
impl Extipinsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel0 {
        match self.bits {
            0 => Extipinsel0::Pin0,
            1 => Extipinsel0::Pin1,
            2 => Extipinsel0::Pin2,
            3 => Extipinsel0::Pin3,
            _ => unreachable!(),
        }
    }
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Extipinsel0::Pin0
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Extipinsel0::Pin1
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Extipinsel0::Pin2
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Extipinsel0::Pin3
    }
}
#[doc = "Field `EXTIPINSEL0` writer - External Interrupt Pin select"]
pub type Extipinsel0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel0, crate::Safe>;
impl<'a, REG> Extipinsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel0::Pin0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel0::Pin1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel0::Pin2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel0::Pin3)
    }
}
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel1 {
    #[doc = "0: OFFSET=0"]
    Pin0 = 0,
    #[doc = "1: OFFSET=1"]
    Pin1 = 1,
    #[doc = "2: OFFSET=2"]
    Pin2 = 2,
    #[doc = "3: OFFSET=3"]
    Pin3 = 3,
}
impl From<Extipinsel1> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel1 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel1 {}
#[doc = "Field `EXTIPINSEL1` reader - External Interrupt Pin select"]
pub type Extipinsel1R = crate::FieldReader<Extipinsel1>;
impl Extipinsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel1 {
        match self.bits {
            0 => Extipinsel1::Pin0,
            1 => Extipinsel1::Pin1,
            2 => Extipinsel1::Pin2,
            3 => Extipinsel1::Pin3,
            _ => unreachable!(),
        }
    }
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Extipinsel1::Pin0
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Extipinsel1::Pin1
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Extipinsel1::Pin2
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Extipinsel1::Pin3
    }
}
#[doc = "Field `EXTIPINSEL1` writer - External Interrupt Pin select"]
pub type Extipinsel1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel1, crate::Safe>;
impl<'a, REG> Extipinsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel1::Pin0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel1::Pin1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel1::Pin2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel1::Pin3)
    }
}
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel2 {
    #[doc = "0: OFFSET=0"]
    Pin0 = 0,
    #[doc = "1: OFFSET=1"]
    Pin1 = 1,
    #[doc = "2: OFFSET=2"]
    Pin2 = 2,
    #[doc = "3: OFFSET=3"]
    Pin3 = 3,
}
impl From<Extipinsel2> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel2 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel2 {}
#[doc = "Field `EXTIPINSEL2` reader - External Interrupt Pin select"]
pub type Extipinsel2R = crate::FieldReader<Extipinsel2>;
impl Extipinsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel2 {
        match self.bits {
            0 => Extipinsel2::Pin0,
            1 => Extipinsel2::Pin1,
            2 => Extipinsel2::Pin2,
            3 => Extipinsel2::Pin3,
            _ => unreachable!(),
        }
    }
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Extipinsel2::Pin0
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Extipinsel2::Pin1
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Extipinsel2::Pin2
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Extipinsel2::Pin3
    }
}
#[doc = "Field `EXTIPINSEL2` writer - External Interrupt Pin select"]
pub type Extipinsel2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel2, crate::Safe>;
impl<'a, REG> Extipinsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel2::Pin0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel2::Pin1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel2::Pin2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel2::Pin3)
    }
}
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel3 {
    #[doc = "0: OFFSET=0"]
    Pin0 = 0,
    #[doc = "1: OFFSET=1"]
    Pin1 = 1,
    #[doc = "2: OFFSET=2"]
    Pin2 = 2,
    #[doc = "3: OFFSET=3"]
    Pin3 = 3,
}
impl From<Extipinsel3> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel3 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel3 {}
#[doc = "Field `EXTIPINSEL3` reader - External Interrupt Pin select"]
pub type Extipinsel3R = crate::FieldReader<Extipinsel3>;
impl Extipinsel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel3 {
        match self.bits {
            0 => Extipinsel3::Pin0,
            1 => Extipinsel3::Pin1,
            2 => Extipinsel3::Pin2,
            3 => Extipinsel3::Pin3,
            _ => unreachable!(),
        }
    }
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Extipinsel3::Pin0
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Extipinsel3::Pin1
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Extipinsel3::Pin2
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Extipinsel3::Pin3
    }
}
#[doc = "Field `EXTIPINSEL3` writer - External Interrupt Pin select"]
pub type Extipinsel3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel3, crate::Safe>;
impl<'a, REG> Extipinsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel3::Pin0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel3::Pin1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel3::Pin2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel3::Pin3)
    }
}
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel4 {
    #[doc = "0: OFFSET=0"]
    Pin0 = 0,
    #[doc = "1: OFFSET=1"]
    Pin1 = 1,
    #[doc = "2: OFFSET=2"]
    Pin2 = 2,
    #[doc = "3: OFFSET=3"]
    Pin3 = 3,
}
impl From<Extipinsel4> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel4 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel4 {}
#[doc = "Field `EXTIPINSEL4` reader - External Interrupt Pin select"]
pub type Extipinsel4R = crate::FieldReader<Extipinsel4>;
impl Extipinsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel4 {
        match self.bits {
            0 => Extipinsel4::Pin0,
            1 => Extipinsel4::Pin1,
            2 => Extipinsel4::Pin2,
            3 => Extipinsel4::Pin3,
            _ => unreachable!(),
        }
    }
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Extipinsel4::Pin0
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Extipinsel4::Pin1
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Extipinsel4::Pin2
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Extipinsel4::Pin3
    }
}
#[doc = "Field `EXTIPINSEL4` writer - External Interrupt Pin select"]
pub type Extipinsel4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel4, crate::Safe>;
impl<'a, REG> Extipinsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel4::Pin0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel4::Pin1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel4::Pin2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel4::Pin3)
    }
}
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel5 {
    #[doc = "0: OFFSET=0"]
    Pin0 = 0,
    #[doc = "1: OFFSET=1"]
    Pin1 = 1,
    #[doc = "2: OFFSET=2"]
    Pin2 = 2,
    #[doc = "3: OFFSET=3"]
    Pin3 = 3,
}
impl From<Extipinsel5> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel5 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel5 {}
#[doc = "Field `EXTIPINSEL5` reader - External Interrupt Pin select"]
pub type Extipinsel5R = crate::FieldReader<Extipinsel5>;
impl Extipinsel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel5 {
        match self.bits {
            0 => Extipinsel5::Pin0,
            1 => Extipinsel5::Pin1,
            2 => Extipinsel5::Pin2,
            3 => Extipinsel5::Pin3,
            _ => unreachable!(),
        }
    }
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Extipinsel5::Pin0
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Extipinsel5::Pin1
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Extipinsel5::Pin2
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Extipinsel5::Pin3
    }
}
#[doc = "Field `EXTIPINSEL5` writer - External Interrupt Pin select"]
pub type Extipinsel5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel5, crate::Safe>;
impl<'a, REG> Extipinsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel5::Pin0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel5::Pin1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel5::Pin2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel5::Pin3)
    }
}
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel6 {
    #[doc = "0: OFFSET=0"]
    Pin0 = 0,
    #[doc = "1: OFFSET=1"]
    Pin1 = 1,
    #[doc = "2: OFFSET=2"]
    Pin2 = 2,
    #[doc = "3: OFFSET=3"]
    Pin3 = 3,
}
impl From<Extipinsel6> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel6 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel6 {}
#[doc = "Field `EXTIPINSEL6` reader - External Interrupt Pin select"]
pub type Extipinsel6R = crate::FieldReader<Extipinsel6>;
impl Extipinsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel6 {
        match self.bits {
            0 => Extipinsel6::Pin0,
            1 => Extipinsel6::Pin1,
            2 => Extipinsel6::Pin2,
            3 => Extipinsel6::Pin3,
            _ => unreachable!(),
        }
    }
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Extipinsel6::Pin0
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Extipinsel6::Pin1
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Extipinsel6::Pin2
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Extipinsel6::Pin3
    }
}
#[doc = "Field `EXTIPINSEL6` writer - External Interrupt Pin select"]
pub type Extipinsel6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel6, crate::Safe>;
impl<'a, REG> Extipinsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel6::Pin0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel6::Pin1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel6::Pin2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel6::Pin3)
    }
}
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel7 {
    #[doc = "0: OFFSET=0"]
    Pin0 = 0,
    #[doc = "1: OFFSET=1"]
    Pin1 = 1,
    #[doc = "2: OFFSET=2"]
    Pin2 = 2,
    #[doc = "3: OFFSET=3"]
    Pin3 = 3,
}
impl From<Extipinsel7> for u8 {
    #[inline(always)]
    fn from(variant: Extipinsel7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipinsel7 {
    type Ux = u8;
}
impl crate::IsEnum for Extipinsel7 {}
#[doc = "Field `EXTIPINSEL7` reader - External Interrupt Pin select"]
pub type Extipinsel7R = crate::FieldReader<Extipinsel7>;
impl Extipinsel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipinsel7 {
        match self.bits {
            0 => Extipinsel7::Pin0,
            1 => Extipinsel7::Pin1,
            2 => Extipinsel7::Pin2,
            3 => Extipinsel7::Pin3,
            _ => unreachable!(),
        }
    }
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Extipinsel7::Pin0
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Extipinsel7::Pin1
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Extipinsel7::Pin2
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Extipinsel7::Pin3
    }
}
#[doc = "Field `EXTIPINSEL7` writer - External Interrupt Pin select"]
pub type Extipinsel7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel7, crate::Safe>;
impl<'a, REG> Extipinsel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFFSET=0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel7::Pin0)
    }
    #[doc = "OFFSET=1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel7::Pin1)
    }
    #[doc = "OFFSET=2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel7::Pin2)
    }
    #[doc = "OFFSET=3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel7::Pin3)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel0(&self) -> Extipinsel0R {
        Extipinsel0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel1(&self) -> Extipinsel1R {
        Extipinsel1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel2(&self) -> Extipinsel2R {
        Extipinsel2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel3(&self) -> Extipinsel3R {
        Extipinsel3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel4(&self) -> Extipinsel4R {
        Extipinsel4R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel5(&self) -> Extipinsel5R {
        Extipinsel5R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel6(&self) -> Extipinsel6R {
        Extipinsel6R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel7(&self) -> Extipinsel7R {
        Extipinsel7R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel0(&mut self) -> Extipinsel0W<ExtipinsellSpec> {
        Extipinsel0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel1(&mut self) -> Extipinsel1W<ExtipinsellSpec> {
        Extipinsel1W::new(self, 4)
    }
    #[doc = "Bits 8:9 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel2(&mut self) -> Extipinsel2W<ExtipinsellSpec> {
        Extipinsel2W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel3(&mut self) -> Extipinsel3W<ExtipinsellSpec> {
        Extipinsel3W::new(self, 12)
    }
    #[doc = "Bits 16:17 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel4(&mut self) -> Extipinsel4W<ExtipinsellSpec> {
        Extipinsel4W::new(self, 16)
    }
    #[doc = "Bits 20:21 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel5(&mut self) -> Extipinsel5W<ExtipinsellSpec> {
        Extipinsel5W::new(self, 20)
    }
    #[doc = "Bits 24:25 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel6(&mut self) -> Extipinsel6W<ExtipinsellSpec> {
        Extipinsel6W::new(self, 24)
    }
    #[doc = "Bits 28:29 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel7(&mut self) -> Extipinsel7W<ExtipinsellSpec> {
        Extipinsel7W::new(self, 28)
    }
}
#[doc = "External Interrupt Pin Select Low\n\nYou can [`read`](crate::Reg::read) this register and get [`extipinsell::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipinsell::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtipinsellSpec;
impl crate::RegisterSpec for ExtipinsellSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extipinsell::R`](R) reader structure"]
impl crate::Readable for ExtipinsellSpec {}
#[doc = "`write(|w| ..)` method takes [`extipinsell::W`](W) writer structure"]
impl crate::Writable for ExtipinsellSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTIPINSELL to value 0"]
impl crate::Resettable for ExtipinsellSpec {}
