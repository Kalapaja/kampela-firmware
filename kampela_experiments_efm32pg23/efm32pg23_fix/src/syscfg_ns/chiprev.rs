#[doc = "Register `CHIPREV` reader"]
pub type R = crate::R<ChiprevSpec>;
#[doc = "Register `CHIPREV` writer"]
pub type W = crate::W<ChiprevSpec>;
#[doc = "Field `MAJOR` reader - Chip Revision Major value"]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - Chip Revision Major value"]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Chip Family value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Family {
    #[doc = "26: Product is in PG23 family"]
    Pg23 = 26,
    #[doc = "56: Product is in FG23 family"]
    Fg23 = 56,
    #[doc = "57: Product is in ZG23 family"]
    Zg23 = 57,
    #[doc = "58: Product is in SG23 family"]
    Sg23 = 58,
}
impl From<Family> for u8 {
    #[inline(always)]
    fn from(variant: Family) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Family {
    type Ux = u8;
}
impl crate::IsEnum for Family {}
#[doc = "Field `FAMILY` reader - Chip Family value"]
pub type FamilyR = crate::FieldReader<Family>;
impl FamilyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Family> {
        match self.bits {
            26 => Some(Family::Pg23),
            56 => Some(Family::Fg23),
            57 => Some(Family::Zg23),
            58 => Some(Family::Sg23),
            _ => None,
        }
    }
    #[doc = "Product is in PG23 family"]
    #[inline(always)]
    pub fn is_pg23(&self) -> bool {
        *self == Family::Pg23
    }
    #[doc = "Product is in FG23 family"]
    #[inline(always)]
    pub fn is_fg23(&self) -> bool {
        *self == Family::Fg23
    }
    #[doc = "Product is in ZG23 family"]
    #[inline(always)]
    pub fn is_zg23(&self) -> bool {
        *self == Family::Zg23
    }
    #[doc = "Product is in SG23 family"]
    #[inline(always)]
    pub fn is_sg23(&self) -> bool {
        *self == Family::Sg23
    }
}
#[doc = "Field `FAMILY` writer - Chip Family value"]
pub type FamilyW<'a, REG> = crate::FieldWriter<'a, REG, 6, Family>;
impl<'a, REG> FamilyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Product is in PG23 family"]
    #[inline(always)]
    pub fn pg23(self) -> &'a mut crate::W<REG> {
        self.variant(Family::Pg23)
    }
    #[doc = "Product is in FG23 family"]
    #[inline(always)]
    pub fn fg23(self) -> &'a mut crate::W<REG> {
        self.variant(Family::Fg23)
    }
    #[doc = "Product is in ZG23 family"]
    #[inline(always)]
    pub fn zg23(self) -> &'a mut crate::W<REG> {
        self.variant(Family::Zg23)
    }
    #[doc = "Product is in SG23 family"]
    #[inline(always)]
    pub fn sg23(self) -> &'a mut crate::W<REG> {
        self.variant(Family::Sg23)
    }
}
#[doc = "Field `MINOR` reader - Chip Revision Minor value"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - Chip Revision Minor value"]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - Chip Revision Major value"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Chip Family value"]
    #[inline(always)]
    pub fn family(&self) -> FamilyR {
        FamilyR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - Chip Revision Minor value"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Chip Revision Major value"]
    #[inline(always)]
    pub fn major(&mut self) -> MajorW<ChiprevSpec> {
        MajorW::new(self, 0)
    }
    #[doc = "Bits 6:11 - Chip Family value"]
    #[inline(always)]
    pub fn family(&mut self) -> FamilyW<ChiprevSpec> {
        FamilyW::new(self, 6)
    }
    #[doc = "Bits 12:19 - Chip Revision Minor value"]
    #[inline(always)]
    pub fn minor(&mut self) -> MinorW<ChiprevSpec> {
        MinorW::new(self, 12)
    }
}
#[doc = "Read to get the chip revision programmed by feature configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`chiprev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chiprev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChiprevSpec;
impl crate::RegisterSpec for ChiprevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chiprev::R`](R) reader structure"]
impl crate::Readable for ChiprevSpec {}
#[doc = "`write(|w| ..)` method takes [`chiprev::W`](W) writer structure"]
impl crate::Writable for ChiprevSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHIPREV to value 0"]
impl crate::Resettable for ChiprevSpec {}
