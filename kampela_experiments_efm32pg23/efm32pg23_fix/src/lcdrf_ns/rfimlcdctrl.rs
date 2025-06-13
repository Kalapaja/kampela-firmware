#[doc = "Register `RFIMLCDCTRL` reader"]
pub type R = crate::R<RfimlcdctrlSpec>;
#[doc = "Register `RFIMLCDCTRL` writer"]
pub type W = crate::W<RfimlcdctrlSpec>;
#[doc = "Field `LCDCPXOEN` reader - LCD Charge Pump XO Clock Enable"]
pub type LcdcpxoenR = crate::BitReader;
#[doc = "Field `LCDCPXOEN` writer - LCD Charge Pump XO Clock Enable"]
pub type LcdcpxoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "LCD Charge Pump XO Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdcpxosel {
    #[doc = "0: Internal LCD CP 10Mhz RC oscillator"]
    Intrco = 0,
    #[doc = "1: HFXO divided 4 clock"]
    Hfxodiv = 1,
}
impl From<Lcdcpxosel> for bool {
    #[inline(always)]
    fn from(variant: Lcdcpxosel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDCPXOSEL` reader - LCD Charge Pump XO Select"]
pub type LcdcpxoselR = crate::BitReader<Lcdcpxosel>;
impl LcdcpxoselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdcpxosel {
        match self.bits {
            false => Lcdcpxosel::Intrco,
            true => Lcdcpxosel::Hfxodiv,
        }
    }
    #[doc = "Internal LCD CP 10Mhz RC oscillator"]
    #[inline(always)]
    pub fn is_intrco(&self) -> bool {
        *self == Lcdcpxosel::Intrco
    }
    #[doc = "HFXO divided 4 clock"]
    #[inline(always)]
    pub fn is_hfxodiv(&self) -> bool {
        *self == Lcdcpxosel::Hfxodiv
    }
}
#[doc = "Field `LCDCPXOSEL` writer - LCD Charge Pump XO Select"]
pub type LcdcpxoselW<'a, REG> = crate::BitWriter<'a, REG, Lcdcpxosel>;
impl<'a, REG> LcdcpxoselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal LCD CP 10Mhz RC oscillator"]
    #[inline(always)]
    pub fn intrco(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpxosel::Intrco)
    }
    #[doc = "HFXO divided 4 clock"]
    #[inline(always)]
    pub fn hfxodiv(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdcpxosel::Hfxodiv)
    }
}
#[doc = "Field `LCDCPXORETIMEEN` reader - LCD Charge Pump XO Retime Enable"]
pub type LcdcpxoretimeenR = crate::BitReader;
#[doc = "Field `LCDCPXORETIMEEN` writer - LCD Charge Pump XO Retime Enable"]
pub type LcdcpxoretimeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "LCD Low Noise\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdlownoise {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: slows down slew rate to reduce RF interference at a cost of additional power consumption"]
    Slow = 1,
}
impl From<Lcdlownoise> for bool {
    #[inline(always)]
    fn from(variant: Lcdlownoise) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDLOWNOISE` reader - LCD Low Noise"]
pub type LcdlownoiseR = crate::BitReader<Lcdlownoise>;
impl LcdlownoiseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdlownoise {
        match self.bits {
            false => Lcdlownoise::Normal,
            true => Lcdlownoise::Slow,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Lcdlownoise::Normal
    }
    #[doc = "slows down slew rate to reduce RF interference at a cost of additional power consumption"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == Lcdlownoise::Slow
    }
}
#[doc = "Field `LCDLOWNOISE` writer - LCD Low Noise"]
pub type LcdlownoiseW<'a, REG> = crate::BitWriter<'a, REG, Lcdlownoise>;
impl<'a, REG> LcdlownoiseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdlownoise::Normal)
    }
    #[doc = "slows down slew rate to reduce RF interference at a cost of additional power consumption"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdlownoise::Slow)
    }
}
#[doc = "Field `LCDCMPDOUT` reader - LCD Comparator Dout"]
pub type LcdcmpdoutR = crate::BitReader;
#[doc = "Field `LCDCMPDOUT` writer - LCD Comparator Dout"]
pub type LcdcmpdoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCD Charge Pump XO Clock Enable"]
    #[inline(always)]
    pub fn lcdcpxoen(&self) -> LcdcpxoenR {
        LcdcpxoenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD Charge Pump XO Select"]
    #[inline(always)]
    pub fn lcdcpxosel(&self) -> LcdcpxoselR {
        LcdcpxoselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD Charge Pump XO Retime Enable"]
    #[inline(always)]
    pub fn lcdcpxoretimeen(&self) -> LcdcpxoretimeenR {
        LcdcpxoretimeenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD Low Noise"]
    #[inline(always)]
    pub fn lcdlownoise(&self) -> LcdlownoiseR {
        LcdlownoiseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD Comparator Dout"]
    #[inline(always)]
    pub fn lcdcmpdout(&self) -> LcdcmpdoutR {
        LcdcmpdoutR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Charge Pump XO Clock Enable"]
    #[inline(always)]
    pub fn lcdcpxoen(&mut self) -> LcdcpxoenW<RfimlcdctrlSpec> {
        LcdcpxoenW::new(self, 0)
    }
    #[doc = "Bit 1 - LCD Charge Pump XO Select"]
    #[inline(always)]
    pub fn lcdcpxosel(&mut self) -> LcdcpxoselW<RfimlcdctrlSpec> {
        LcdcpxoselW::new(self, 1)
    }
    #[doc = "Bit 2 - LCD Charge Pump XO Retime Enable"]
    #[inline(always)]
    pub fn lcdcpxoretimeen(&mut self) -> LcdcpxoretimeenW<RfimlcdctrlSpec> {
        LcdcpxoretimeenW::new(self, 2)
    }
    #[doc = "Bit 3 - LCD Low Noise"]
    #[inline(always)]
    pub fn lcdlownoise(&mut self) -> LcdlownoiseW<RfimlcdctrlSpec> {
        LcdlownoiseW::new(self, 3)
    }
    #[doc = "Bit 4 - LCD Comparator Dout"]
    #[inline(always)]
    pub fn lcdcmpdout(&mut self) -> LcdcmpdoutW<RfimlcdctrlSpec> {
        LcdcmpdoutW::new(self, 4)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rfimlcdctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfimlcdctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfimlcdctrlSpec;
impl crate::RegisterSpec for RfimlcdctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfimlcdctrl::R`](R) reader structure"]
impl crate::Readable for RfimlcdctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rfimlcdctrl::W`](W) writer structure"]
impl crate::Writable for RfimlcdctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFIMLCDCTRL to value 0"]
impl crate::Resettable for RfimlcdctrlSpec {}
