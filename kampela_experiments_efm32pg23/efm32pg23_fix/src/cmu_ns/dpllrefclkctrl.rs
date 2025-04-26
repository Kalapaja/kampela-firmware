#[doc = "Register `DPLLREFCLKCTRL` reader"]
pub type R = crate::R<DpllrefclkctrlSpec>;
#[doc = "Register `DPLLREFCLKCTRL` writer"]
pub type W = crate::W<DpllrefclkctrlSpec>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: DPLLREFCLK is not clocked"]
    Disabled = 0,
    #[doc = "1: HFXO is clocking DPLLREFCLK"]
    Hfxo = 1,
    #[doc = "2: LFXO is clocking DPLLREFCLK"]
    Lfxo = 2,
    #[doc = "3: CLKIN0 is clocking DPLLREFCLK"]
    Clkin0 = 3,
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
    pub const fn variant(&self) -> Clksel {
        match self.bits {
            0 => Clksel::Disabled,
            1 => Clksel::Hfxo,
            2 => Clksel::Lfxo,
            3 => Clksel::Clkin0,
            _ => unreachable!(),
        }
    }
    #[doc = "DPLLREFCLK is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clksel::Disabled
    }
    #[doc = "HFXO is clocking DPLLREFCLK"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Clksel::Hfxo
    }
    #[doc = "LFXO is clocking DPLLREFCLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Clksel::Lfxo
    }
    #[doc = "CLKIN0 is clocking DPLLREFCLK"]
    #[inline(always)]
    pub fn is_clkin0(&self) -> bool {
        *self == Clksel::Clkin0
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel, crate::Safe>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DPLLREFCLK is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Disabled)
    }
    #[doc = "HFXO is clocking DPLLREFCLK"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfxo)
    }
    #[doc = "LFXO is clocking DPLLREFCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lfxo)
    }
    #[doc = "CLKIN0 is clocking DPLLREFCLK"]
    #[inline(always)]
    pub fn clkin0(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Clkin0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<DpllrefclkctrlSpec> {
        ClkselW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllrefclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllrefclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpllrefclkctrlSpec;
impl crate::RegisterSpec for DpllrefclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpllrefclkctrl::R`](R) reader structure"]
impl crate::Readable for DpllrefclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dpllrefclkctrl::W`](W) writer structure"]
impl crate::Writable for DpllrefclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPLLREFCLKCTRL to value 0"]
impl crate::Resettable for DpllrefclkctrlSpec {}
