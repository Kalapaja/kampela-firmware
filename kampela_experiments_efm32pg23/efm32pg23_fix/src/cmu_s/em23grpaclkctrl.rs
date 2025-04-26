#[doc = "Register `EM23GRPACLKCTRL` reader"]
pub type R = crate::R<Em23grpaclkctrlSpec>;
#[doc = "Register `EM23GRPACLKCTRL` writer"]
pub type W = crate::W<Em23grpaclkctrlSpec>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "1: LFRCO is clocking EM23GRPACLK"]
    Lfrco = 1,
    #[doc = "2: LFXO is clocking EM23GRPACLK"]
    Lfxo = 2,
    #[doc = "3: ULFRCO is clocking EM23GRPACLK"]
    Ulfrco = 3,
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
            _ => None,
        }
    }
    #[doc = "LFRCO is clocking EM23GRPACLK"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Clksel::Lfrco
    }
    #[doc = "LFXO is clocking EM23GRPACLK"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Clksel::Lfxo
    }
    #[doc = "ULFRCO is clocking EM23GRPACLK"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Clksel::Ulfrco
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFRCO is clocking EM23GRPACLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lfrco)
    }
    #[doc = "LFXO is clocking EM23GRPACLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lfxo)
    }
    #[doc = "ULFRCO is clocking EM23GRPACLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Ulfrco)
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
    pub fn clksel(&mut self) -> ClkselW<Em23grpaclkctrlSpec> {
        ClkselW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em23grpaclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em23grpaclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em23grpaclkctrlSpec;
impl crate::RegisterSpec for Em23grpaclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em23grpaclkctrl::R`](R) reader structure"]
impl crate::Readable for Em23grpaclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`em23grpaclkctrl::W`](W) writer structure"]
impl crate::Writable for Em23grpaclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EM23GRPACLKCTRL to value 0x01"]
impl crate::Resettable for Em23grpaclkctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
