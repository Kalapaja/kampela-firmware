#[doc = "Register `EM01GRPACLKCTRL` reader"]
pub type R = crate::R<Em01grpaclkctrlSpec>;
#[doc = "Register `EM01GRPACLKCTRL` writer"]
pub type W = crate::W<Em01grpaclkctrlSpec>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "1: HFRCODPLL is clocking EM01GRPACLK"]
    Hfrcodpll = 1,
    #[doc = "2: HFXO is clocking EM01GRPACLK"]
    Hfxo = 2,
    #[doc = "3: FSRCO is clocking EM01GRPACLK"]
    Fsrco = 3,
    #[doc = "4: HFRCOEM23 is clocking EM01GRPACLK"]
    Hfrcoem23 = 4,
    #[doc = "5: HFRCODPLL (retimed) is clocking EM01GRPACLK. Check with datasheet for frequency limitation when using retiming with voltage scaling."]
    Hfrcodpllrt = 5,
    #[doc = "6: HFXO (retimed) is clocking EM01GRPACLK. Check with datasheet for frequency limitation when using retiming with voltage scaling."]
    Hfxort = 6,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clksel> {
        match self.bits {
            1 => Some(Clksel::Hfrcodpll),
            2 => Some(Clksel::Hfxo),
            3 => Some(Clksel::Fsrco),
            4 => Some(Clksel::Hfrcoem23),
            5 => Some(Clksel::Hfrcodpllrt),
            6 => Some(Clksel::Hfxort),
            _ => None,
        }
    }
    #[doc = "HFRCODPLL is clocking EM01GRPACLK"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == Clksel::Hfrcodpll
    }
    #[doc = "HFXO is clocking EM01GRPACLK"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Clksel::Hfxo
    }
    #[doc = "FSRCO is clocking EM01GRPACLK"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == Clksel::Fsrco
    }
    #[doc = "HFRCOEM23 is clocking EM01GRPACLK"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == Clksel::Hfrcoem23
    }
    #[doc = "HFRCODPLL (retimed) is clocking EM01GRPACLK. Check with datasheet for frequency limitation when using retiming with voltage scaling."]
    #[inline(always)]
    pub fn is_hfrcodpllrt(&self) -> bool {
        *self == Clksel::Hfrcodpllrt
    }
    #[doc = "HFXO (retimed) is clocking EM01GRPACLK. Check with datasheet for frequency limitation when using retiming with voltage scaling."]
    #[inline(always)]
    pub fn is_hfxort(&self) -> bool {
        *self == Clksel::Hfxort
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HFRCODPLL is clocking EM01GRPACLK"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfrcodpll)
    }
    #[doc = "HFXO is clocking EM01GRPACLK"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfxo)
    }
    #[doc = "FSRCO is clocking EM01GRPACLK"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Fsrco)
    }
    #[doc = "HFRCOEM23 is clocking EM01GRPACLK"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfrcoem23)
    }
    #[doc = "HFRCODPLL (retimed) is clocking EM01GRPACLK. Check with datasheet for frequency limitation when using retiming with voltage scaling."]
    #[inline(always)]
    pub fn hfrcodpllrt(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfrcodpllrt)
    }
    #[doc = "HFXO (retimed) is clocking EM01GRPACLK. Check with datasheet for frequency limitation when using retiming with voltage scaling."]
    #[inline(always)]
    pub fn hfxort(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfxort)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<Em01grpaclkctrlSpec> {
        ClkselW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em01grpaclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em01grpaclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em01grpaclkctrlSpec;
impl crate::RegisterSpec for Em01grpaclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em01grpaclkctrl::R`](R) reader structure"]
impl crate::Readable for Em01grpaclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`em01grpaclkctrl::W`](W) writer structure"]
impl crate::Writable for Em01grpaclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EM01GRPACLKCTRL to value 0x01"]
impl crate::Resettable for Em01grpaclkctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
