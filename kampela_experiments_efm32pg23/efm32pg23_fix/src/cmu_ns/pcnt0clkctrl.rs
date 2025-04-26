#[doc = "Register `PCNT0CLKCTRL` reader"]
pub type R = crate::R<Pcnt0clkctrlSpec>;
#[doc = "Register `PCNT0CLKCTRL` writer"]
pub type W = crate::W<Pcnt0clkctrlSpec>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: PCNT0 is not clocked"]
    Disabled = 0,
    #[doc = "1: EM23GRPACLK is clocking PCNT0"]
    Em23grpaclk = 1,
    #[doc = "2: External pin PCNT_S0 is clocking PCNT0"]
    Pcnts0 = 2,
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
            0 => Some(Clksel::Disabled),
            1 => Some(Clksel::Em23grpaclk),
            2 => Some(Clksel::Pcnts0),
            _ => None,
        }
    }
    #[doc = "PCNT0 is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clksel::Disabled
    }
    #[doc = "EM23GRPACLK is clocking PCNT0"]
    #[inline(always)]
    pub fn is_em23grpaclk(&self) -> bool {
        *self == Clksel::Em23grpaclk
    }
    #[doc = "External pin PCNT_S0 is clocking PCNT0"]
    #[inline(always)]
    pub fn is_pcnts0(&self) -> bool {
        *self == Clksel::Pcnts0
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCNT0 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Disabled)
    }
    #[doc = "EM23GRPACLK is clocking PCNT0"]
    #[inline(always)]
    pub fn em23grpaclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Em23grpaclk)
    }
    #[doc = "External pin PCNT_S0 is clocking PCNT0"]
    #[inline(always)]
    pub fn pcnts0(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Pcnts0)
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
    pub fn clksel(&mut self) -> ClkselW<Pcnt0clkctrlSpec> {
        ClkselW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt0clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt0clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcnt0clkctrlSpec;
impl crate::RegisterSpec for Pcnt0clkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcnt0clkctrl::R`](R) reader structure"]
impl crate::Readable for Pcnt0clkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcnt0clkctrl::W`](W) writer structure"]
impl crate::Writable for Pcnt0clkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCNT0CLKCTRL to value 0x01"]
impl crate::Resettable for Pcnt0clkctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
