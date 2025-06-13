#[doc = "Register `SCALE1` reader"]
pub type R = crate::R<Scale1Spec>;
#[doc = "Register `SCALE1` writer"]
pub type W = crate::W<Scale1Spec>;
#[doc = "Field `OFFSET` reader - Offset"]
pub type OffsetR = crate::FieldReader<u32>;
#[doc = "Field `OFFSET` writer - Offset"]
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `GAIN13LSB` reader - Gain 13 LSBs"]
pub type Gain13lsbR = crate::FieldReader<u16>;
#[doc = "Field `GAIN13LSB` writer - Gain 13 LSBs"]
pub type Gain13lsbW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Gain 3 MSBs\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gain3msb {
    #[doc = "0: Upper 3 bits of gain = 011 (0.75x)"]
    Gain011 = 0,
    #[doc = "1: Upper 3 bits of gain = 100 (1.00x)"]
    Gain100 = 1,
}
impl From<Gain3msb> for bool {
    #[inline(always)]
    fn from(variant: Gain3msb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GAIN3MSB` reader - Gain 3 MSBs"]
pub type Gain3msbR = crate::BitReader<Gain3msb>;
impl Gain3msbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gain3msb {
        match self.bits {
            false => Gain3msb::Gain011,
            true => Gain3msb::Gain100,
        }
    }
    #[doc = "Upper 3 bits of gain = 011 (0.75x)"]
    #[inline(always)]
    pub fn is_gain011(&self) -> bool {
        *self == Gain3msb::Gain011
    }
    #[doc = "Upper 3 bits of gain = 100 (1.00x)"]
    #[inline(always)]
    pub fn is_gain100(&self) -> bool {
        *self == Gain3msb::Gain100
    }
}
#[doc = "Field `GAIN3MSB` writer - Gain 3 MSBs"]
pub type Gain3msbW<'a, REG> = crate::BitWriter<'a, REG, Gain3msb>;
impl<'a, REG> Gain3msbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Upper 3 bits of gain = 011 (0.75x)"]
    #[inline(always)]
    pub fn gain011(self) -> &'a mut crate::W<REG> {
        self.variant(Gain3msb::Gain011)
    }
    #[doc = "Upper 3 bits of gain = 100 (1.00x)"]
    #[inline(always)]
    pub fn gain100(self) -> &'a mut crate::W<REG> {
        self.variant(Gain3msb::Gain100)
    }
}
impl R {
    #[doc = "Bits 0:17 - Offset"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:30 - Gain 13 LSBs"]
    #[inline(always)]
    pub fn gain13lsb(&self) -> Gain13lsbR {
        Gain13lsbR::new(((self.bits >> 18) & 0x1fff) as u16)
    }
    #[doc = "Bit 31 - Gain 3 MSBs"]
    #[inline(always)]
    pub fn gain3msb(&self) -> Gain3msbR {
        Gain3msbR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - Offset"]
    #[inline(always)]
    pub fn offset(&mut self) -> OffsetW<Scale1Spec> {
        OffsetW::new(self, 0)
    }
    #[doc = "Bits 18:30 - Gain 13 LSBs"]
    #[inline(always)]
    pub fn gain13lsb(&mut self) -> Gain13lsbW<Scale1Spec> {
        Gain13lsbW::new(self, 18)
    }
    #[doc = "Bit 31 - Gain 3 MSBs"]
    #[inline(always)]
    pub fn gain3msb(&mut self) -> Gain3msbW<Scale1Spec> {
        Gain3msbW::new(self, 31)
    }
}
#[doc = "Scale\n\nYou can [`read`](crate::Reg::read) this register and get [`scale1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scale1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scale1Spec;
impl crate::RegisterSpec for Scale1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scale1::R`](R) reader structure"]
impl crate::Readable for Scale1Spec {}
#[doc = "`write(|w| ..)` method takes [`scale1::W`](W) writer structure"]
impl crate::Writable for Scale1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCALE1 to value 0x8002_c000"]
impl crate::Resettable for Scale1Spec {
    const RESET_VALUE: u32 = 0x8002_c000;
}
