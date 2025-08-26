#[doc = "Register `EUSART0CLKCTRL` reader"]
pub type R = crate::R<Eusart0clkctrlSpec>;
#[doc = "Register `EUSART0CLKCTRL` writer"]
pub type W = crate::W<Eusart0clkctrlSpec>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: EUSART0 is not clocked"]
    Disabled = 0,
    #[doc = "1: EM01GRPCCLK is clocking EUSART0"]
    Em01grpcclk = 1,
    #[doc = "2: HFRCOEM23 is clocking EUSART0"]
    Hfrcoem23 = 2,
    #[doc = "3: LFRCO is clocking EUSART0"]
    Lfrco = 3,
    #[doc = "4: LFXO is clocking EUSART0"]
    Lfxo = 4,
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
            1 => Some(Clksel::Em01grpcclk),
            2 => Some(Clksel::Hfrcoem23),
            3 => Some(Clksel::Lfrco),
            4 => Some(Clksel::Lfxo),
            _ => None,
        }
    }
    #[doc = "EUSART0 is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clksel::Disabled
    }
    #[doc = "EM01GRPCCLK is clocking EUSART0"]
    #[inline(always)]
    pub fn is_em01grpcclk(&self) -> bool {
        *self == Clksel::Em01grpcclk
    }
    #[doc = "HFRCOEM23 is clocking EUSART0"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == Clksel::Hfrcoem23
    }
    #[doc = "LFRCO is clocking EUSART0"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Clksel::Lfrco
    }
    #[doc = "LFXO is clocking EUSART0"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Clksel::Lfxo
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EUSART0 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Disabled)
    }
    #[doc = "EM01GRPCCLK is clocking EUSART0"]
    #[inline(always)]
    pub fn em01grpcclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Em01grpcclk)
    }
    #[doc = "HFRCOEM23 is clocking EUSART0"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfrcoem23)
    }
    #[doc = "LFRCO is clocking EUSART0"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lfrco)
    }
    #[doc = "LFXO is clocking EUSART0"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lfxo)
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
    pub fn clksel(&mut self) -> ClkselW<Eusart0clkctrlSpec> {
        ClkselW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eusart0clkctrlSpec;
impl crate::RegisterSpec for Eusart0clkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eusart0clkctrl::R`](R) reader structure"]
impl crate::Readable for Eusart0clkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`eusart0clkctrl::W`](W) writer structure"]
impl crate::Writable for Eusart0clkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EUSART0CLKCTRL to value 0x01"]
impl crate::Resettable for Eusart0clkctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
