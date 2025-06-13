#[doc = "Register `IADCCLKCTRL` reader"]
pub type R = crate::R<IadcclkctrlSpec>;
#[doc = "Register `IADCCLKCTRL` writer"]
pub type W = crate::W<IadcclkctrlSpec>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "1: EM01GRPACLK is clocking IADCCLK"]
    Em01grpaclk = 1,
    #[doc = "2: FSRCO is clocking IADCCLK"]
    Fsrco = 2,
    #[doc = "3: HFRCOEM23 is clocking IADCCLK"]
    Hfrcoem23 = 3,
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
            1 => Some(Clksel::Em01grpaclk),
            2 => Some(Clksel::Fsrco),
            3 => Some(Clksel::Hfrcoem23),
            _ => None,
        }
    }
    #[doc = "EM01GRPACLK is clocking IADCCLK"]
    #[inline(always)]
    pub fn is_em01grpaclk(&self) -> bool {
        *self == Clksel::Em01grpaclk
    }
    #[doc = "FSRCO is clocking IADCCLK"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == Clksel::Fsrco
    }
    #[doc = "HFRCOEM23 is clocking IADCCLK"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == Clksel::Hfrcoem23
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EM01GRPACLK is clocking IADCCLK"]
    #[inline(always)]
    pub fn em01grpaclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Em01grpaclk)
    }
    #[doc = "FSRCO is clocking IADCCLK"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Fsrco)
    }
    #[doc = "HFRCOEM23 is clocking IADCCLK"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfrcoem23)
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
    pub fn clksel(&mut self) -> ClkselW<IadcclkctrlSpec> {
        ClkselW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`iadcclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iadcclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IadcclkctrlSpec;
impl crate::RegisterSpec for IadcclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iadcclkctrl::R`](R) reader structure"]
impl crate::Readable for IadcclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`iadcclkctrl::W`](W) writer structure"]
impl crate::Writable for IadcclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IADCCLKCTRL to value 0x01"]
impl crate::Resettable for IadcclkctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
