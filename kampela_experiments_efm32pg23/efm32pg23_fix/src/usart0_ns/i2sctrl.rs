#[doc = "Register `I2SCTRL` reader"]
pub type R = crate::R<I2sctrlSpec>;
#[doc = "Register `I2SCTRL` writer"]
pub type W = crate::W<I2sctrlSpec>;
#[doc = "Field `EN` reader - Enable I2S Mode"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable I2S Mode"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONO` reader - Stero or Mono"]
pub type MonoR = crate::BitReader;
#[doc = "Field `MONO` writer - Stero or Mono"]
pub type MonoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Justification of I2S Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Justify {
    #[doc = "0: Data is left-justified"]
    Left = 0,
    #[doc = "1: Data is right-justified"]
    Right = 1,
}
impl From<Justify> for bool {
    #[inline(always)]
    fn from(variant: Justify) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JUSTIFY` reader - Justification of I2S Data"]
pub type JustifyR = crate::BitReader<Justify>;
impl JustifyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Justify {
        match self.bits {
            false => Justify::Left,
            true => Justify::Right,
        }
    }
    #[doc = "Data is left-justified"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == Justify::Left
    }
    #[doc = "Data is right-justified"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == Justify::Right
    }
}
#[doc = "Field `JUSTIFY` writer - Justification of I2S Data"]
pub type JustifyW<'a, REG> = crate::BitWriter<'a, REG, Justify>;
impl<'a, REG> JustifyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is left-justified"]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(Justify::Left)
    }
    #[doc = "Data is right-justified"]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(Justify::Right)
    }
}
#[doc = "Field `DMASPLIT` reader - Separate DMA Request For Left/Right Data"]
pub type DmasplitR = crate::BitReader;
#[doc = "Field `DMASPLIT` writer - Separate DMA Request For Left/Right Data"]
pub type DmasplitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELAY` reader - Delay on I2S data"]
pub type DelayR = crate::BitReader;
#[doc = "Field `DELAY` writer - Delay on I2S data"]
pub type DelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "I2S Word Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Format {
    #[doc = "0: 32-bit word, 32-bit data"]
    W32d32 = 0,
    #[doc = "1: 32-bit word, 32-bit data with 8 lsb masked"]
    W32d24m = 1,
    #[doc = "2: 32-bit word, 24-bit data"]
    W32d24 = 2,
    #[doc = "3: 32-bit word, 16-bit data"]
    W32d16 = 3,
    #[doc = "4: 32-bit word, 8-bit data"]
    W32d8 = 4,
    #[doc = "5: 16-bit word, 16-bit data"]
    W16d16 = 5,
    #[doc = "6: 16-bit word, 8-bit data"]
    W16d8 = 6,
    #[doc = "7: 8-bit word, 8-bit data"]
    W8d8 = 7,
}
impl From<Format> for u8 {
    #[inline(always)]
    fn from(variant: Format) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Format {
    type Ux = u8;
}
impl crate::IsEnum for Format {}
#[doc = "Field `FORMAT` reader - I2S Word Format"]
pub type FormatR = crate::FieldReader<Format>;
impl FormatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Format {
        match self.bits {
            0 => Format::W32d32,
            1 => Format::W32d24m,
            2 => Format::W32d24,
            3 => Format::W32d16,
            4 => Format::W32d8,
            5 => Format::W16d16,
            6 => Format::W16d8,
            7 => Format::W8d8,
            _ => unreachable!(),
        }
    }
    #[doc = "32-bit word, 32-bit data"]
    #[inline(always)]
    pub fn is_w32d32(&self) -> bool {
        *self == Format::W32d32
    }
    #[doc = "32-bit word, 32-bit data with 8 lsb masked"]
    #[inline(always)]
    pub fn is_w32d24m(&self) -> bool {
        *self == Format::W32d24m
    }
    #[doc = "32-bit word, 24-bit data"]
    #[inline(always)]
    pub fn is_w32d24(&self) -> bool {
        *self == Format::W32d24
    }
    #[doc = "32-bit word, 16-bit data"]
    #[inline(always)]
    pub fn is_w32d16(&self) -> bool {
        *self == Format::W32d16
    }
    #[doc = "32-bit word, 8-bit data"]
    #[inline(always)]
    pub fn is_w32d8(&self) -> bool {
        *self == Format::W32d8
    }
    #[doc = "16-bit word, 16-bit data"]
    #[inline(always)]
    pub fn is_w16d16(&self) -> bool {
        *self == Format::W16d16
    }
    #[doc = "16-bit word, 8-bit data"]
    #[inline(always)]
    pub fn is_w16d8(&self) -> bool {
        *self == Format::W16d8
    }
    #[doc = "8-bit word, 8-bit data"]
    #[inline(always)]
    pub fn is_w8d8(&self) -> bool {
        *self == Format::W8d8
    }
}
#[doc = "Field `FORMAT` writer - I2S Word Format"]
pub type FormatW<'a, REG> = crate::FieldWriter<'a, REG, 3, Format, crate::Safe>;
impl<'a, REG> FormatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32-bit word, 32-bit data"]
    #[inline(always)]
    pub fn w32d32(self) -> &'a mut crate::W<REG> {
        self.variant(Format::W32d32)
    }
    #[doc = "32-bit word, 32-bit data with 8 lsb masked"]
    #[inline(always)]
    pub fn w32d24m(self) -> &'a mut crate::W<REG> {
        self.variant(Format::W32d24m)
    }
    #[doc = "32-bit word, 24-bit data"]
    #[inline(always)]
    pub fn w32d24(self) -> &'a mut crate::W<REG> {
        self.variant(Format::W32d24)
    }
    #[doc = "32-bit word, 16-bit data"]
    #[inline(always)]
    pub fn w32d16(self) -> &'a mut crate::W<REG> {
        self.variant(Format::W32d16)
    }
    #[doc = "32-bit word, 8-bit data"]
    #[inline(always)]
    pub fn w32d8(self) -> &'a mut crate::W<REG> {
        self.variant(Format::W32d8)
    }
    #[doc = "16-bit word, 16-bit data"]
    #[inline(always)]
    pub fn w16d16(self) -> &'a mut crate::W<REG> {
        self.variant(Format::W16d16)
    }
    #[doc = "16-bit word, 8-bit data"]
    #[inline(always)]
    pub fn w16d8(self) -> &'a mut crate::W<REG> {
        self.variant(Format::W16d8)
    }
    #[doc = "8-bit word, 8-bit data"]
    #[inline(always)]
    pub fn w8d8(self) -> &'a mut crate::W<REG> {
        self.variant(Format::W8d8)
    }
}
impl R {
    #[doc = "Bit 0 - Enable I2S Mode"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stero or Mono"]
    #[inline(always)]
    pub fn mono(&self) -> MonoR {
        MonoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Justification of I2S Data"]
    #[inline(always)]
    pub fn justify(&self) -> JustifyR {
        JustifyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Separate DMA Request For Left/Right Data"]
    #[inline(always)]
    pub fn dmasplit(&self) -> DmasplitR {
        DmasplitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Delay on I2S data"]
    #[inline(always)]
    pub fn delay(&self) -> DelayR {
        DelayR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - I2S Word Format"]
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable I2S Mode"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<I2sctrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Stero or Mono"]
    #[inline(always)]
    pub fn mono(&mut self) -> MonoW<I2sctrlSpec> {
        MonoW::new(self, 1)
    }
    #[doc = "Bit 2 - Justification of I2S Data"]
    #[inline(always)]
    pub fn justify(&mut self) -> JustifyW<I2sctrlSpec> {
        JustifyW::new(self, 2)
    }
    #[doc = "Bit 3 - Separate DMA Request For Left/Right Data"]
    #[inline(always)]
    pub fn dmasplit(&mut self) -> DmasplitW<I2sctrlSpec> {
        DmasplitW::new(self, 3)
    }
    #[doc = "Bit 4 - Delay on I2S data"]
    #[inline(always)]
    pub fn delay(&mut self) -> DelayW<I2sctrlSpec> {
        DelayW::new(self, 4)
    }
    #[doc = "Bits 8:10 - I2S Word Format"]
    #[inline(always)]
    pub fn format(&mut self) -> FormatW<I2sctrlSpec> {
        FormatW::new(self, 8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sctrlSpec;
impl crate::RegisterSpec for I2sctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sctrl::R`](R) reader structure"]
impl crate::Readable for I2sctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`i2sctrl::W`](W) writer structure"]
impl crate::Writable for I2sctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2SCTRL to value 0"]
impl crate::Resettable for I2sctrlSpec {}
