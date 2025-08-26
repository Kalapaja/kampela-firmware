#[doc = "Register `EXTIPINSELH` reader"]
pub type R = crate::R<ExtipinselhSpec>;
#[doc = "Register `EXTIPINSELH` writer"]
pub type W = crate::W<ExtipinselhSpec>;
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel0 {
    #[doc = "0: OFFSET=8"]
    Pin8 = 0,
    #[doc = "1: OFFSET=9"]
    Pin9 = 1,
    #[doc = "2: OFFSET=10"]
    Pin10 = 2,
    #[doc = "3: OFFSET=11"]
    Pin11 = 3,
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
            0 => Extipinsel0::Pin8,
            1 => Extipinsel0::Pin9,
            2 => Extipinsel0::Pin10,
            3 => Extipinsel0::Pin11,
            _ => unreachable!(),
        }
    }
    #[doc = "OFFSET=8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == Extipinsel0::Pin8
    }
    #[doc = "OFFSET=9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == Extipinsel0::Pin9
    }
    #[doc = "OFFSET=10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == Extipinsel0::Pin10
    }
    #[doc = "OFFSET=11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == Extipinsel0::Pin11
    }
}
#[doc = "Field `EXTIPINSEL0` writer - External Interrupt Pin select"]
pub type Extipinsel0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel0, crate::Safe>;
impl<'a, REG> Extipinsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFFSET=8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel0::Pin8)
    }
    #[doc = "OFFSET=9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel0::Pin9)
    }
    #[doc = "OFFSET=10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel0::Pin10)
    }
    #[doc = "OFFSET=11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel0::Pin11)
    }
}
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel1 {
    #[doc = "0: OFFSET=8"]
    Pin8 = 0,
    #[doc = "1: OFFSET=9"]
    Pin9 = 1,
    #[doc = "2: OFFSET=10"]
    Pin10 = 2,
    #[doc = "3: OFFSET=11"]
    Pin11 = 3,
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
            0 => Extipinsel1::Pin8,
            1 => Extipinsel1::Pin9,
            2 => Extipinsel1::Pin10,
            3 => Extipinsel1::Pin11,
            _ => unreachable!(),
        }
    }
    #[doc = "OFFSET=8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == Extipinsel1::Pin8
    }
    #[doc = "OFFSET=9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == Extipinsel1::Pin9
    }
    #[doc = "OFFSET=10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == Extipinsel1::Pin10
    }
    #[doc = "OFFSET=11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == Extipinsel1::Pin11
    }
}
#[doc = "Field `EXTIPINSEL1` writer - External Interrupt Pin select"]
pub type Extipinsel1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel1, crate::Safe>;
impl<'a, REG> Extipinsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFFSET=8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel1::Pin8)
    }
    #[doc = "OFFSET=9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel1::Pin9)
    }
    #[doc = "OFFSET=10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel1::Pin10)
    }
    #[doc = "OFFSET=11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel1::Pin11)
    }
}
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel2 {
    #[doc = "0: OFFSET=8"]
    Pin8 = 0,
    #[doc = "1: OFFSET=9"]
    Pin9 = 1,
    #[doc = "2: OFFSET=10"]
    Pin10 = 2,
    #[doc = "3: OFFSET=11"]
    Pin11 = 3,
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
            0 => Extipinsel2::Pin8,
            1 => Extipinsel2::Pin9,
            2 => Extipinsel2::Pin10,
            3 => Extipinsel2::Pin11,
            _ => unreachable!(),
        }
    }
    #[doc = "OFFSET=8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == Extipinsel2::Pin8
    }
    #[doc = "OFFSET=9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == Extipinsel2::Pin9
    }
    #[doc = "OFFSET=10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == Extipinsel2::Pin10
    }
    #[doc = "OFFSET=11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == Extipinsel2::Pin11
    }
}
#[doc = "Field `EXTIPINSEL2` writer - External Interrupt Pin select"]
pub type Extipinsel2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel2, crate::Safe>;
impl<'a, REG> Extipinsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFFSET=8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel2::Pin8)
    }
    #[doc = "OFFSET=9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel2::Pin9)
    }
    #[doc = "OFFSET=10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel2::Pin10)
    }
    #[doc = "OFFSET=11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel2::Pin11)
    }
}
#[doc = "External Interrupt Pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipinsel3 {
    #[doc = "0: OFFSET=8"]
    Pin8 = 0,
    #[doc = "1: OFFSET=9"]
    Pin9 = 1,
    #[doc = "2: OFFSET=10"]
    Pin10 = 2,
    #[doc = "3: OFFSET=11"]
    Pin11 = 3,
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
            0 => Extipinsel3::Pin8,
            1 => Extipinsel3::Pin9,
            2 => Extipinsel3::Pin10,
            3 => Extipinsel3::Pin11,
            _ => unreachable!(),
        }
    }
    #[doc = "OFFSET=8"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == Extipinsel3::Pin8
    }
    #[doc = "OFFSET=9"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == Extipinsel3::Pin9
    }
    #[doc = "OFFSET=10"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == Extipinsel3::Pin10
    }
    #[doc = "OFFSET=11"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == Extipinsel3::Pin11
    }
}
#[doc = "Field `EXTIPINSEL3` writer - External Interrupt Pin select"]
pub type Extipinsel3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipinsel3, crate::Safe>;
impl<'a, REG> Extipinsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFFSET=8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel3::Pin8)
    }
    #[doc = "OFFSET=9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel3::Pin9)
    }
    #[doc = "OFFSET=10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel3::Pin10)
    }
    #[doc = "OFFSET=11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut crate::W<REG> {
        self.variant(Extipinsel3::Pin11)
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
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel0(&mut self) -> Extipinsel0W<ExtipinselhSpec> {
        Extipinsel0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel1(&mut self) -> Extipinsel1W<ExtipinselhSpec> {
        Extipinsel1W::new(self, 4)
    }
    #[doc = "Bits 8:9 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel2(&mut self) -> Extipinsel2W<ExtipinselhSpec> {
        Extipinsel2W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External Interrupt Pin select"]
    #[inline(always)]
    pub fn extipinsel3(&mut self) -> Extipinsel3W<ExtipinselhSpec> {
        Extipinsel3W::new(self, 12)
    }
}
#[doc = "External Interrupt Pin Select High\n\nYou can [`read`](crate::Reg::read) this register and get [`extipinselh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipinselh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtipinselhSpec;
impl crate::RegisterSpec for ExtipinselhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extipinselh::R`](R) reader structure"]
impl crate::Readable for ExtipinselhSpec {}
#[doc = "`write(|w| ..)` method takes [`extipinselh::W`](W) writer structure"]
impl crate::Writable for ExtipinselhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTIPINSELH to value 0"]
impl crate::Resettable for ExtipinselhSpec {}
