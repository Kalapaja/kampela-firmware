#[doc = "Register `WDOG0CLKCTRL` reader"]
pub type R = crate::R<Wdog0clkctrlSpec>;
#[doc = "Register `WDOG0CLKCTRL` writer"]
pub type W = crate::W<Wdog0clkctrlSpec>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "1: LFRCO is clocking WDOG0CLK"]
    Lfrco = 1,
    #[doc = "2: LFXO is clocking WDOG0CLK"]
    Lfxo = 2,
    #[doc = "3: ULFRCO is clocking WDOG0CLK"]
    Ulfrco = 3,
    #[doc = "4: HCLKDIV1024 is clocking WDOG0CLK"]
    Hclkdiv1024 = 4,
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
            1 => Some(Clksel::Lfrco),
            2 => Some(Clksel::Lfxo),
            3 => Some(Clksel::Ulfrco),
            4 => Some(Clksel::Hclkdiv1024),
            _ => None,
        }
    }
    #[doc = "LFRCO is clocking WDOG0CLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Clksel::Lfrco
    }
    #[doc = "LFXO is clocking WDOG0CLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Clksel::Lfxo
    }
    #[doc = "ULFRCO is clocking WDOG0CLK"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Clksel::Ulfrco
    }
    #[doc = "HCLKDIV1024 is clocking WDOG0CLK"]
    #[inline(always)]
    pub fn is_hclkdiv1024(&self) -> bool {
        *self == Clksel::Hclkdiv1024
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFRCO is clocking WDOG0CLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lfrco)
    }
    #[doc = "LFXO is clocking WDOG0CLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lfxo)
    }
    #[doc = "ULFRCO is clocking WDOG0CLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Ulfrco)
    }
    #[doc = "HCLKDIV1024 is clocking WDOG0CLK"]
    #[inline(always)]
    pub fn hclkdiv1024(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hclkdiv1024)
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
    pub fn clksel(&mut self) -> ClkselW<Wdog0clkctrlSpec> {
        ClkselW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog0clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog0clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdog0clkctrlSpec;
impl crate::RegisterSpec for Wdog0clkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdog0clkctrl::R`](R) reader structure"]
impl crate::Readable for Wdog0clkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wdog0clkctrl::W`](W) writer structure"]
impl crate::Writable for Wdog0clkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOG0CLKCTRL to value 0x01"]
impl crate::Resettable for Wdog0clkctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
