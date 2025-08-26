#[doc = "Register `BBUSALLOC` reader"]
pub type R = crate::R<BbusallocSpec>;
#[doc = "Register `BBUSALLOC` writer"]
pub type W = crate::W<BbusallocSpec>;
#[doc = "B Bus Even 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Beven0 {
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
impl From<Beven0> for u8 {
    #[inline(always)]
    fn from(variant: Beven0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Beven0 {
    type Ux = u8;
}
impl crate::IsEnum for Beven0 {}
#[doc = "Field `BEVEN0` reader - B Bus Even 0"]
pub type Beven0R = crate::FieldReader<Beven0>;
impl Beven0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Beven0> {
        match self.bits {
            0 => Some(Beven0::Tristate),
            1 => Some(Beven0::Adc0),
            2 => Some(Beven0::Acmp0),
            3 => Some(Beven0::Acmp1),
            4 => Some(Beven0::Vdac0ch0),
            _ => None,
        }
    }
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == Beven0::Tristate
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Beven0::Adc0
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Beven0::Acmp0
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Beven0::Acmp1
    }
    #[doc = "The bus is allocated to VDAC0 CH0"]
    #[inline(always)]
    pub fn is_vdac0ch0(&self) -> bool {
        *self == Beven0::Vdac0ch0
    }
}
#[doc = "Field `BEVEN0` writer - B Bus Even 0"]
pub type Beven0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Beven0>;
impl<'a, REG> Beven0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(Beven0::Tristate)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Beven0::Adc0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Beven0::Acmp0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Beven0::Acmp1)
    }
    #[doc = "The bus is allocated to VDAC0 CH0"]
    #[inline(always)]
    pub fn vdac0ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Beven0::Vdac0ch0)
    }
}
#[doc = "B Bus Even 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Beven1 {
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
impl From<Beven1> for u8 {
    #[inline(always)]
    fn from(variant: Beven1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Beven1 {
    type Ux = u8;
}
impl crate::IsEnum for Beven1 {}
#[doc = "Field `BEVEN1` reader - B Bus Even 1"]
pub type Beven1R = crate::FieldReader<Beven1>;
impl Beven1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Beven1> {
        match self.bits {
            0 => Some(Beven1::Tristate),
            1 => Some(Beven1::Adc0),
            2 => Some(Beven1::Acmp0),
            3 => Some(Beven1::Acmp1),
            4 => Some(Beven1::Vdac0ch1),
            _ => None,
        }
    }
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == Beven1::Tristate
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Beven1::Adc0
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Beven1::Acmp0
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Beven1::Acmp1
    }
    #[doc = "The bus is allocated to VDAC0 CH1"]
    #[inline(always)]
    pub fn is_vdac0ch1(&self) -> bool {
        *self == Beven1::Vdac0ch1
    }
}
#[doc = "Field `BEVEN1` writer - B Bus Even 1"]
pub type Beven1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Beven1>;
impl<'a, REG> Beven1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(Beven1::Tristate)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Beven1::Adc0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Beven1::Acmp0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Beven1::Acmp1)
    }
    #[doc = "The bus is allocated to VDAC0 CH1"]
    #[inline(always)]
    pub fn vdac0ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Beven1::Vdac0ch1)
    }
}
#[doc = "B Bus Odd 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bodd0 {
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
impl From<Bodd0> for u8 {
    #[inline(always)]
    fn from(variant: Bodd0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bodd0 {
    type Ux = u8;
}
impl crate::IsEnum for Bodd0 {}
#[doc = "Field `BODD0` reader - B Bus Odd 0"]
pub type Bodd0R = crate::FieldReader<Bodd0>;
impl Bodd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bodd0> {
        match self.bits {
            0 => Some(Bodd0::Tristate),
            1 => Some(Bodd0::Adc0),
            2 => Some(Bodd0::Acmp0),
            3 => Some(Bodd0::Acmp1),
            4 => Some(Bodd0::Vdac0ch0),
            _ => None,
        }
    }
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == Bodd0::Tristate
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Bodd0::Adc0
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Bodd0::Acmp0
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Bodd0::Acmp1
    }
    #[doc = "The bus is allocated to VDAC0 CH0"]
    #[inline(always)]
    pub fn is_vdac0ch0(&self) -> bool {
        *self == Bodd0::Vdac0ch0
    }
}
#[doc = "Field `BODD0` writer - B Bus Odd 0"]
pub type Bodd0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Bodd0>;
impl<'a, REG> Bodd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(Bodd0::Tristate)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Bodd0::Adc0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Bodd0::Acmp0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Bodd0::Acmp1)
    }
    #[doc = "The bus is allocated to VDAC0 CH0"]
    #[inline(always)]
    pub fn vdac0ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Bodd0::Vdac0ch0)
    }
}
#[doc = "B Bus Odd 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bodd1 {
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
impl From<Bodd1> for u8 {
    #[inline(always)]
    fn from(variant: Bodd1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bodd1 {
    type Ux = u8;
}
impl crate::IsEnum for Bodd1 {}
#[doc = "Field `BODD1` reader - B Bus Odd 1"]
pub type Bodd1R = crate::FieldReader<Bodd1>;
impl Bodd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bodd1> {
        match self.bits {
            0 => Some(Bodd1::Tristate),
            1 => Some(Bodd1::Adc0),
            2 => Some(Bodd1::Acmp0),
            3 => Some(Bodd1::Acmp1),
            4 => Some(Bodd1::Vdac0ch1),
            _ => None,
        }
    }
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == Bodd1::Tristate
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == Bodd1::Adc0
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == Bodd1::Acmp0
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == Bodd1::Acmp1
    }
    #[doc = "The bus is allocated to VDAC0 CH1"]
    #[inline(always)]
    pub fn is_vdac0ch1(&self) -> bool {
        *self == Bodd1::Vdac0ch1
    }
}
#[doc = "Field `BODD1` writer - B Bus Odd 1"]
pub type Bodd1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Bodd1>;
impl<'a, REG> Bodd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The bus is not allocated"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(Bodd1::Tristate)
    }
    #[doc = "The bus is allocated to ADC0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Bodd1::Adc0)
    }
    #[doc = "The bus is allocated to ACMP0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Bodd1::Acmp0)
    }
    #[doc = "The bus is allocated to ACMP1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Bodd1::Acmp1)
    }
    #[doc = "The bus is allocated to VDAC0 CH1"]
    #[inline(always)]
    pub fn vdac0ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Bodd1::Vdac0ch1)
    }
}
impl R {
    #[doc = "Bits 0:3 - B Bus Even 0"]
    #[inline(always)]
    pub fn beven0(&self) -> Beven0R {
        Beven0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - B Bus Even 1"]
    #[inline(always)]
    pub fn beven1(&self) -> Beven1R {
        Beven1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - B Bus Odd 0"]
    #[inline(always)]
    pub fn bodd0(&self) -> Bodd0R {
        Bodd0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - B Bus Odd 1"]
    #[inline(always)]
    pub fn bodd1(&self) -> Bodd1R {
        Bodd1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - B Bus Even 0"]
    #[inline(always)]
    pub fn beven0(&mut self) -> Beven0W<BbusallocSpec> {
        Beven0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - B Bus Even 1"]
    #[inline(always)]
    pub fn beven1(&mut self) -> Beven1W<BbusallocSpec> {
        Beven1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - B Bus Odd 0"]
    #[inline(always)]
    pub fn bodd0(&mut self) -> Bodd0W<BbusallocSpec> {
        Bodd0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - B Bus Odd 1"]
    #[inline(always)]
    pub fn bodd1(&mut self) -> Bodd1W<BbusallocSpec> {
        Bodd1W::new(self, 24)
    }
}
#[doc = "B Bus allocation\n\nYou can [`read`](crate::Reg::read) this register and get [`bbusalloc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bbusalloc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BbusallocSpec;
impl crate::RegisterSpec for BbusallocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bbusalloc::R`](R) reader structure"]
impl crate::Readable for BbusallocSpec {}
#[doc = "`write(|w| ..)` method takes [`bbusalloc::W`](W) writer structure"]
impl crate::Writable for BbusallocSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BBUSALLOC to value 0"]
impl crate::Resettable for BbusallocSpec {}
