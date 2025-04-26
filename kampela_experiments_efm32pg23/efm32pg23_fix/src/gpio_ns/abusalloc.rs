#[doc = "Register `ABUSALLOC` reader"]
pub type R = crate::R<AbusallocSpec>;
#[doc = "Register `ABUSALLOC` writer"]
pub type W = crate::W<AbusallocSpec>;
#[doc = "A Bus Even 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aeven0 {
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
impl From<Aeven0> for u8 {
    #[inline(always)]
    fn from(variant: Aeven0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aeven0 {
    type Ux = u8;
}
impl crate::IsEnum for Aeven0 {}
#[doc = "Field `AEVEN0` reader - A Bus Even 0"]
pub type Aeven0R = crate::FieldReader<Aeven0>;
impl Aeven0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aeven0> {
        match self.bits {
            0 => Some(Aeven0::Tristate),
            1 => Some(Aeven0::Adc0),
            2 => Some(Aeven0::Acmp0),
            3 => Some(Aeven0::Acmp1),
            4 => Some(Aeven0::Vdac0ch0),
            _ => None,
        }
    }
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == Aeven0::Tristate
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Aeven0::Adc0
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Aeven0::Acmp0
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Aeven0::Acmp1
    }
    #[doc = "The bus is allocated to VDAC0 CH0"]
    #[inline(always)]
    pub fn is_vdac0ch0(&self) -> bool {
        *self == Aeven0::Vdac0ch0
    }
}
#[doc = "Field `AEVEN0` writer - A Bus Even 0"]
pub type Aeven0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Aeven0>;
impl<'a, REG> Aeven0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(Aeven0::Tristate)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Aeven0::Adc0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Aeven0::Acmp0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Aeven0::Acmp1)
    }
    #[doc = "The bus is allocated to VDAC0 CH0"]
    #[inline(always)]
    pub fn vdac0ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Aeven0::Vdac0ch0)
    }
}
#[doc = "A Bus Even 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aeven1 {
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
impl From<Aeven1> for u8 {
    #[inline(always)]
    fn from(variant: Aeven1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aeven1 {
    type Ux = u8;
}
impl crate::IsEnum for Aeven1 {}
#[doc = "Field `AEVEN1` reader - A Bus Even 1"]
pub type Aeven1R = crate::FieldReader<Aeven1>;
impl Aeven1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aeven1> {
        match self.bits {
            0 => Some(Aeven1::Tristate),
            1 => Some(Aeven1::Adc0),
            2 => Some(Aeven1::Acmp0),
            3 => Some(Aeven1::Acmp1),
            4 => Some(Aeven1::Vdac0ch1),
            _ => None,
        }
    }
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == Aeven1::Tristate
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Aeven1::Adc0
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Aeven1::Acmp0
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Aeven1::Acmp1
    }
    #[doc = "The bus is allocated to VDAC0 CH1"]
    #[inline(always)]
    pub fn is_vdac0ch1(&self) -> bool {
        *self == Aeven1::Vdac0ch1
    }
}
#[doc = "Field `AEVEN1` writer - A Bus Even 1"]
pub type Aeven1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Aeven1>;
impl<'a, REG> Aeven1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(Aeven1::Tristate)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Aeven1::Adc0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Aeven1::Acmp0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Aeven1::Acmp1)
    }
    #[doc = "The bus is allocated to VDAC0 CH1"]
    #[inline(always)]
    pub fn vdac0ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Aeven1::Vdac0ch1)
    }
}
#[doc = "A Bus Odd 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aodd0 {
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
impl From<Aodd0> for u8 {
    #[inline(always)]
    fn from(variant: Aodd0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aodd0 {
    type Ux = u8;
}
impl crate::IsEnum for Aodd0 {}
#[doc = "Field `AODD0` reader - A Bus Odd 0"]
pub type Aodd0R = crate::FieldReader<Aodd0>;
impl Aodd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aodd0> {
        match self.bits {
            0 => Some(Aodd0::Tristate),
            1 => Some(Aodd0::Adc0),
            2 => Some(Aodd0::Acmp0),
            3 => Some(Aodd0::Acmp1),
            4 => Some(Aodd0::Vdac0ch0),
            _ => None,
        }
    }
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == Aodd0::Tristate
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Aodd0::Adc0
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Aodd0::Acmp0
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Aodd0::Acmp1
    }
    #[doc = "The bus is allocated to VDAC0 CH0"]
    #[inline(always)]
    pub fn is_vdac0ch0(&self) -> bool {
        *self == Aodd0::Vdac0ch0
    }
}
#[doc = "Field `AODD0` writer - A Bus Odd 0"]
pub type Aodd0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Aodd0>;
impl<'a, REG> Aodd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(Aodd0::Tristate)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Aodd0::Adc0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Aodd0::Acmp0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Aodd0::Acmp1)
    }
    #[doc = "The bus is allocated to VDAC0 CH0"]
    #[inline(always)]
    pub fn vdac0ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Aodd0::Vdac0ch0)
    }
}
#[doc = "A Bus Odd 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aodd1 {
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
impl From<Aodd1> for u8 {
    #[inline(always)]
    fn from(variant: Aodd1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aodd1 {
    type Ux = u8;
}
impl crate::IsEnum for Aodd1 {}
#[doc = "Field `AODD1` reader - A Bus Odd 1"]
pub type Aodd1R = crate::FieldReader<Aodd1>;
impl Aodd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aodd1> {
        match self.bits {
            0 => Some(Aodd1::Tristate),
            1 => Some(Aodd1::Adc0),
            2 => Some(Aodd1::Acmp0),
            3 => Some(Aodd1::Acmp1),
            4 => Some(Aodd1::Vdac0ch1),
            _ => None,
        }
    }
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == Aodd1::Tristate
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Aodd1::Adc0
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Aodd1::Acmp0
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Aodd1::Acmp1
    }
    #[doc = "The bus is allocated to VDAC0 CH1"]
    #[inline(always)]
    pub fn is_vdac0ch1(&self) -> bool {
        *self == Aodd1::Vdac0ch1
    }
}
#[doc = "Field `AODD1` writer - A Bus Odd 1"]
pub type Aodd1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Aodd1>;
impl<'a, REG> Aodd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(Aodd1::Tristate)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Aodd1::Adc0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Aodd1::Acmp0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Aodd1::Acmp1)
    }
    #[doc = "The bus is allocated to VDAC0 CH1"]
    #[inline(always)]
    pub fn vdac0ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Aodd1::Vdac0ch1)
    }
}
impl R {
    #[doc = "Bits 0:3 - A Bus Even 0"]
    #[inline(always)]
    pub fn aeven0(&self) -> Aeven0R {
        Aeven0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - A Bus Even 1"]
    #[inline(always)]
    pub fn aeven1(&self) -> Aeven1R {
        Aeven1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - A Bus Odd 0"]
    #[inline(always)]
    pub fn aodd0(&self) -> Aodd0R {
        Aodd0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - A Bus Odd 1"]
    #[inline(always)]
    pub fn aodd1(&self) -> Aodd1R {
        Aodd1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - A Bus Even 0"]
    #[inline(always)]
    pub fn aeven0(&mut self) -> Aeven0W<AbusallocSpec> {
        Aeven0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - A Bus Even 1"]
    #[inline(always)]
    pub fn aeven1(&mut self) -> Aeven1W<AbusallocSpec> {
        Aeven1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - A Bus Odd 0"]
    #[inline(always)]
    pub fn aodd0(&mut self) -> Aodd0W<AbusallocSpec> {
        Aodd0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - A Bus Odd 1"]
    #[inline(always)]
    pub fn aodd1(&mut self) -> Aodd1W<AbusallocSpec> {
        Aodd1W::new(self, 24)
    }
}
#[doc = "A Bus allocation\n\nYou can [`read`](crate::Reg::read) this register and get [`abusalloc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abusalloc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AbusallocSpec;
impl crate::RegisterSpec for AbusallocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`abusalloc::R`](R) reader structure"]
impl crate::Readable for AbusallocSpec {}
#[doc = "`write(|w| ..)` method takes [`abusalloc::W`](W) writer structure"]
impl crate::Writable for AbusallocSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ABUSALLOC to value 0"]
impl crate::Resettable for AbusallocSpec {}
