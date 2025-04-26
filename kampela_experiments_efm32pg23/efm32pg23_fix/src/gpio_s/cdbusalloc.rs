#[doc = "Register `CDBUSALLOC` reader"]
pub type R = crate::R<CdbusallocSpec>;
#[doc = "Register `CDBUSALLOC` writer"]
pub type W = crate::W<CdbusallocSpec>;
#[doc = "CD Bus Even 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cdeven0 {
    #[doc = "0: The bus is not allocated"]
    Tristate = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    Adc0 = 1,
    #[doc = "2: The bus is allocated to ACMP0"]
    Acmp0 = 2,
    #[doc = "3: The bus is allocated to ACMP1"]
    Acmp1 = 3,
    #[doc = "4: The bus is allocated to VDAC0 CH0"]
    Vdac0ch0 = 4,
}
impl From<Cdeven0> for u8 {
    #[inline(always)]
    fn from(variant: Cdeven0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cdeven0 {
    type Ux = u8;
}
impl crate::IsEnum for Cdeven0 {}
#[doc = "Field `CDEVEN0` reader - CD Bus Even 0"]
pub type Cdeven0R = crate::FieldReader<Cdeven0>;
impl Cdeven0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cdeven0> {
        match self.bits {
            0 => Some(Cdeven0::Tristate),
            1 => Some(Cdeven0::Adc0),
            2 => Some(Cdeven0::Acmp0),
            3 => Some(Cdeven0::Acmp1),
            4 => Some(Cdeven0::Vdac0ch0),
            _ => None,
        }
    }
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == Cdeven0::Tristate
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Cdeven0::Adc0
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Cdeven0::Acmp0
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Cdeven0::Acmp1
    }
    #[doc = "The bus is allocated to VDAC0 CH0"]
    #[inline(always)]
    pub fn is_vdac0ch0(&self) -> bool {
        *self == Cdeven0::Vdac0ch0
    }
}
#[doc = "Field `CDEVEN0` writer - CD Bus Even 0"]
pub type Cdeven0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Cdeven0>;
impl<'a, REG> Cdeven0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(Cdeven0::Tristate)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdeven0::Adc0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdeven0::Acmp0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdeven0::Acmp1)
    }
    #[doc = "The bus is allocated to VDAC0 CH0"]
    #[inline(always)]
    pub fn vdac0ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdeven0::Vdac0ch0)
    }
}
#[doc = "CD Bus Even 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cdeven1 {
    #[doc = "0: The bus is not allocated"]
    Tristate = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    Adc0 = 1,
    #[doc = "2: The bus is allocated to ACMP0"]
    Acmp0 = 2,
    #[doc = "3: The bus is allocated to ACMP1"]
    Acmp1 = 3,
    #[doc = "4: The bus is allocated to VDAC0 CH1"]
    Vdac0ch1 = 4,
}
impl From<Cdeven1> for u8 {
    #[inline(always)]
    fn from(variant: Cdeven1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cdeven1 {
    type Ux = u8;
}
impl crate::IsEnum for Cdeven1 {}
#[doc = "Field `CDEVEN1` reader - CD Bus Even 1"]
pub type Cdeven1R = crate::FieldReader<Cdeven1>;
impl Cdeven1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cdeven1> {
        match self.bits {
            0 => Some(Cdeven1::Tristate),
            1 => Some(Cdeven1::Adc0),
            2 => Some(Cdeven1::Acmp0),
            3 => Some(Cdeven1::Acmp1),
            4 => Some(Cdeven1::Vdac0ch1),
            _ => None,
        }
    }
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == Cdeven1::Tristate
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Cdeven1::Adc0
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Cdeven1::Acmp0
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Cdeven1::Acmp1
    }
    #[doc = "The bus is allocated to VDAC0 CH1"]
    #[inline(always)]
    pub fn is_vdac0ch1(&self) -> bool {
        *self == Cdeven1::Vdac0ch1
    }
}
#[doc = "Field `CDEVEN1` writer - CD Bus Even 1"]
pub type Cdeven1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Cdeven1>;
impl<'a, REG> Cdeven1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(Cdeven1::Tristate)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdeven1::Adc0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdeven1::Acmp0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdeven1::Acmp1)
    }
    #[doc = "The bus is allocated to VDAC0 CH1"]
    #[inline(always)]
    pub fn vdac0ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdeven1::Vdac0ch1)
    }
}
#[doc = "CD Bus Odd 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cdodd0 {
    #[doc = "0: The bus is not allocated"]
    Tristate = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    Adc0 = 1,
    #[doc = "2: The bus is allocated to ACMP0"]
    Acmp0 = 2,
    #[doc = "3: The bus is allocated to ACMP1"]
    Acmp1 = 3,
    #[doc = "4: The bus is allocated to VDAC0 CH0"]
    Vdac0ch0 = 4,
}
impl From<Cdodd0> for u8 {
    #[inline(always)]
    fn from(variant: Cdodd0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cdodd0 {
    type Ux = u8;
}
impl crate::IsEnum for Cdodd0 {}
#[doc = "Field `CDODD0` reader - CD Bus Odd 0"]
pub type Cdodd0R = crate::FieldReader<Cdodd0>;
impl Cdodd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cdodd0> {
        match self.bits {
            0 => Some(Cdodd0::Tristate),
            1 => Some(Cdodd0::Adc0),
            2 => Some(Cdodd0::Acmp0),
            3 => Some(Cdodd0::Acmp1),
            4 => Some(Cdodd0::Vdac0ch0),
            _ => None,
        }
    }
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == Cdodd0::Tristate
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Cdodd0::Adc0
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Cdodd0::Acmp0
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Cdodd0::Acmp1
    }
    #[doc = "The bus is allocated to VDAC0 CH0"]
    #[inline(always)]
    pub fn is_vdac0ch0(&self) -> bool {
        *self == Cdodd0::Vdac0ch0
    }
}
#[doc = "Field `CDODD0` writer - CD Bus Odd 0"]
pub type Cdodd0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Cdodd0>;
impl<'a, REG> Cdodd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(Cdodd0::Tristate)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdodd0::Adc0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdodd0::Acmp0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdodd0::Acmp1)
    }
    #[doc = "The bus is allocated to VDAC0 CH0"]
    #[inline(always)]
    pub fn vdac0ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdodd0::Vdac0ch0)
    }
}
#[doc = "CD Bus Odd 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cdodd1 {
    #[doc = "0: The bus is not allocated"]
    Tristate = 0,
    #[doc = "1: The bus is allocated to ADC0"]
    Adc0 = 1,
    #[doc = "2: The bus is allocated to ACMP0"]
    Acmp0 = 2,
    #[doc = "3: The bus is allocated to ACMP1"]
    Acmp1 = 3,
    #[doc = "4: The bus is allocated to VDAC0 CH1"]
    Vdac0ch1 = 4,
}
impl From<Cdodd1> for u8 {
    #[inline(always)]
    fn from(variant: Cdodd1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cdodd1 {
    type Ux = u8;
}
impl crate::IsEnum for Cdodd1 {}
#[doc = "Field `CDODD1` reader - CD Bus Odd 1"]
pub type Cdodd1R = crate::FieldReader<Cdodd1>;
impl Cdodd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cdodd1> {
        match self.bits {
            0 => Some(Cdodd1::Tristate),
            1 => Some(Cdodd1::Adc0),
            2 => Some(Cdodd1::Acmp0),
            3 => Some(Cdodd1::Acmp1),
            4 => Some(Cdodd1::Vdac0ch1),
            _ => None,
        }
    }
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == Cdodd1::Tristate
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Cdodd1::Adc0
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Cdodd1::Acmp0
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Cdodd1::Acmp1
    }
    #[doc = "The bus is allocated to VDAC0 CH1"]
    #[inline(always)]
    pub fn is_vdac0ch1(&self) -> bool {
        *self == Cdodd1::Vdac0ch1
    }
}
#[doc = "Field `CDODD1` writer - CD Bus Odd 1"]
pub type Cdodd1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Cdodd1>;
impl<'a, REG> Cdodd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(Cdodd1::Tristate)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdodd1::Adc0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Cdodd1::Acmp0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdodd1::Acmp1)
    }
    #[doc = "The bus is allocated to VDAC0 CH1"]
    #[inline(always)]
    pub fn vdac0ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdodd1::Vdac0ch1)
    }
}
impl R {
    #[doc = "Bits 0:3 - CD Bus Even 0"]
    #[inline(always)]
    pub fn cdeven0(&self) -> Cdeven0R {
        Cdeven0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CD Bus Even 1"]
    #[inline(always)]
    pub fn cdeven1(&self) -> Cdeven1R {
        Cdeven1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CD Bus Odd 0"]
    #[inline(always)]
    pub fn cdodd0(&self) -> Cdodd0R {
        Cdodd0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CD Bus Odd 1"]
    #[inline(always)]
    pub fn cdodd1(&self) -> Cdodd1R {
        Cdodd1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CD Bus Even 0"]
    #[inline(always)]
    pub fn cdeven0(&mut self) -> Cdeven0W<CdbusallocSpec> {
        Cdeven0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - CD Bus Even 1"]
    #[inline(always)]
    pub fn cdeven1(&mut self) -> Cdeven1W<CdbusallocSpec> {
        Cdeven1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - CD Bus Odd 0"]
    #[inline(always)]
    pub fn cdodd0(&mut self) -> Cdodd0W<CdbusallocSpec> {
        Cdodd0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - CD Bus Odd 1"]
    #[inline(always)]
    pub fn cdodd1(&mut self) -> Cdodd1W<CdbusallocSpec> {
        Cdodd1W::new(self, 24)
    }
}
#[doc = "CD Bus allocation\n\nYou can [`read`](crate::Reg::read) this register and get [`cdbusalloc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdbusalloc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdbusallocSpec;
impl crate::RegisterSpec for CdbusallocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdbusalloc::R`](R) reader structure"]
impl crate::Readable for CdbusallocSpec {}
#[doc = "`write(|w| ..)` method takes [`cdbusalloc::W`](W) writer structure"]
impl crate::Writable for CdbusallocSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDBUSALLOC to value 0"]
impl crate::Resettable for CdbusallocSpec {}
