#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Polynomial Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polysel {
    #[doc = "0: CRC-32 (0x04C11DB7) polynomial selected"]
    Crc32 = 0,
    #[doc = "1: 16-bit CRC programmable polynomial selected"]
    Crc16 = 1,
}
impl From<Polysel> for bool {
    #[inline(always)]
    fn from(variant: Polysel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLYSEL` reader - Polynomial Select"]
pub type PolyselR = crate::BitReader<Polysel>;
impl PolyselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polysel {
        match self.bits {
            false => Polysel::Crc32,
            true => Polysel::Crc16,
        }
    }
    #[doc = "CRC-32 (0x04C11DB7) polynomial selected"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == Polysel::Crc32
    }
    #[doc = "16-bit CRC programmable polynomial selected"]
    #[inline(always)]
    pub fn is_crc16(&self) -> bool {
        *self == Polysel::Crc16
    }
}
#[doc = "Field `POLYSEL` writer - Polynomial Select"]
pub type PolyselW<'a, REG> = crate::BitWriter<'a, REG, Polysel>;
impl<'a, REG> PolyselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC-32 (0x04C11DB7) polynomial selected"]
    #[inline(always)]
    pub fn crc32(self) -> &'a mut crate::W<REG> {
        self.variant(Polysel::Crc32)
    }
    #[doc = "16-bit CRC programmable polynomial selected"]
    #[inline(always)]
    pub fn crc16(self) -> &'a mut crate::W<REG> {
        self.variant(Polysel::Crc16)
    }
}
#[doc = "Field `BYTEMODE` reader - Byte Mode Enable"]
pub type BytemodeR = crate::BitReader;
#[doc = "Field `BYTEMODE` writer - Byte Mode Enable"]
pub type BytemodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Byte-level Bit Reverse Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bitreverse {
    #[doc = "0: No reverse"]
    Normal = 0,
    #[doc = "1: Reverse bit order in each byte"]
    Reversed = 1,
}
impl From<Bitreverse> for bool {
    #[inline(always)]
    fn from(variant: Bitreverse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BITREVERSE` reader - Byte-level Bit Reverse Enable"]
pub type BitreverseR = crate::BitReader<Bitreverse>;
impl BitreverseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bitreverse {
        match self.bits {
            false => Bitreverse::Normal,
            true => Bitreverse::Reversed,
        }
    }
    #[doc = "No reverse"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Bitreverse::Normal
    }
    #[doc = "Reverse bit order in each byte"]
    #[inline(always)]
    pub fn is_reversed(&self) -> bool {
        *self == Bitreverse::Reversed
    }
}
#[doc = "Field `BITREVERSE` writer - Byte-level Bit Reverse Enable"]
pub type BitreverseW<'a, REG> = crate::BitWriter<'a, REG, Bitreverse>;
impl<'a, REG> BitreverseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No reverse"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Bitreverse::Normal)
    }
    #[doc = "Reverse bit order in each byte"]
    #[inline(always)]
    pub fn reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Bitreverse::Reversed)
    }
}
#[doc = "Byte Reverse Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bytereverse {
    #[doc = "0: No reverse: B3, B2, B1, B0"]
    Normal = 0,
    #[doc = "1: Reverse byte order. For 32-bit: B0, B1, B2, B3; For 16-bit: 0, 0, B0, B1"]
    Reversed = 1,
}
impl From<Bytereverse> for bool {
    #[inline(always)]
    fn from(variant: Bytereverse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYTEREVERSE` reader - Byte Reverse Mode"]
pub type BytereverseR = crate::BitReader<Bytereverse>;
impl BytereverseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bytereverse {
        match self.bits {
            false => Bytereverse::Normal,
            true => Bytereverse::Reversed,
        }
    }
    #[doc = "No reverse: B3, B2, B1, B0"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Bytereverse::Normal
    }
    #[doc = "Reverse byte order. For 32-bit: B0, B1, B2, B3; For 16-bit: 0, 0, B0, B1"]
    #[inline(always)]
    pub fn is_reversed(&self) -> bool {
        *self == Bytereverse::Reversed
    }
}
#[doc = "Field `BYTEREVERSE` writer - Byte Reverse Mode"]
pub type BytereverseW<'a, REG> = crate::BitWriter<'a, REG, Bytereverse>;
impl<'a, REG> BytereverseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No reverse: B3, B2, B1, B0"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Bytereverse::Normal)
    }
    #[doc = "Reverse byte order. For 32-bit: B0, B1, B2, B3; For 16-bit: 0, 0, B0, B1"]
    #[inline(always)]
    pub fn reversed(self) -> &'a mut crate::W<REG> {
        self.variant(Bytereverse::Reversed)
    }
}
#[doc = "Field `AUTOINIT` reader - Auto Init Enable"]
pub type AutoinitR = crate::BitReader;
#[doc = "Field `AUTOINIT` writer - Auto Init Enable"]
pub type AutoinitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Polynomial Select"]
    #[inline(always)]
    pub fn polysel(&self) -> PolyselR {
        PolyselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Byte Mode Enable"]
    #[inline(always)]
    pub fn bytemode(&self) -> BytemodeR {
        BytemodeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Byte-level Bit Reverse Enable"]
    #[inline(always)]
    pub fn bitreverse(&self) -> BitreverseR {
        BitreverseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Byte Reverse Mode"]
    #[inline(always)]
    pub fn bytereverse(&self) -> BytereverseR {
        BytereverseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Auto Init Enable"]
    #[inline(always)]
    pub fn autoinit(&self) -> AutoinitR {
        AutoinitR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Polynomial Select"]
    #[inline(always)]
    pub fn polysel(&mut self) -> PolyselW<CtrlSpec> {
        PolyselW::new(self, 4)
    }
    #[doc = "Bit 8 - Byte Mode Enable"]
    #[inline(always)]
    pub fn bytemode(&mut self) -> BytemodeW<CtrlSpec> {
        BytemodeW::new(self, 8)
    }
    #[doc = "Bit 9 - Byte-level Bit Reverse Enable"]
    #[inline(always)]
    pub fn bitreverse(&mut self) -> BitreverseW<CtrlSpec> {
        BitreverseW::new(self, 9)
    }
    #[doc = "Bit 10 - Byte Reverse Mode"]
    #[inline(always)]
    pub fn bytereverse(&mut self) -> BytereverseW<CtrlSpec> {
        BytereverseW::new(self, 10)
    }
    #[doc = "Bit 13 - Auto Init Enable"]
    #[inline(always)]
    pub fn autoinit(&mut self) -> AutoinitW<CtrlSpec> {
        AutoinitW::new(self, 13)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
