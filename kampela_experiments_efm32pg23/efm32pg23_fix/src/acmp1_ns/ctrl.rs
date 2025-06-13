#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Not Ready Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Notrdyval {
    #[doc = "0: ACMP output is 0 when the ACMP is not ready."]
    Low = 0,
    #[doc = "1: ACMP output is 1 when the ACMP is not ready."]
    High = 1,
}
impl From<Notrdyval> for bool {
    #[inline(always)]
    fn from(variant: Notrdyval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTRDYVAL` reader - Not Ready Value"]
pub type NotrdyvalR = crate::BitReader<Notrdyval>;
impl NotrdyvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Notrdyval {
        match self.bits {
            false => Notrdyval::Low,
            true => Notrdyval::High,
        }
    }
    #[doc = "ACMP output is 0 when the ACMP is not ready."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Notrdyval::Low
    }
    #[doc = "ACMP output is 1 when the ACMP is not ready."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Notrdyval::High
    }
}
#[doc = "Field `NOTRDYVAL` writer - Not Ready Value"]
pub type NotrdyvalW<'a, REG> = crate::BitWriter<'a, REG, Notrdyval>;
impl<'a, REG> NotrdyvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ACMP output is 0 when the ACMP is not ready."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Notrdyval::Low)
    }
    #[doc = "ACMP output is 1 when the ACMP is not ready."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Notrdyval::High)
    }
}
#[doc = "Comparator GPIO Output Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpioinv {
    #[doc = "0: The comparator output to GPIO is not inverted"]
    Notinv = 0,
    #[doc = "1: The comparator output to GPIO is inverted"]
    Inv = 1,
}
impl From<Gpioinv> for bool {
    #[inline(always)]
    fn from(variant: Gpioinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOINV` reader - Comparator GPIO Output Invert"]
pub type GpioinvR = crate::BitReader<Gpioinv>;
impl GpioinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpioinv {
        match self.bits {
            false => Gpioinv::Notinv,
            true => Gpioinv::Inv,
        }
    }
    #[doc = "The comparator output to GPIO is not inverted"]
    #[inline(always)]
    pub fn is_notinv(&self) -> bool {
        *self == Gpioinv::Notinv
    }
    #[doc = "The comparator output to GPIO is inverted"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == Gpioinv::Inv
    }
}
#[doc = "Field `GPIOINV` writer - Comparator GPIO Output Invert"]
pub type GpioinvW<'a, REG> = crate::BitWriter<'a, REG, Gpioinv>;
impl<'a, REG> GpioinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The comparator output to GPIO is not inverted"]
    #[inline(always)]
    pub fn notinv(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioinv::Notinv)
    }
    #[doc = "The comparator output to GPIO is inverted"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut crate::W<REG> {
        self.variant(Gpioinv::Inv)
    }
}
impl R {
    #[doc = "Bit 0 - Not Ready Value"]
    #[inline(always)]
    pub fn notrdyval(&self) -> NotrdyvalR {
        NotrdyvalR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator GPIO Output Invert"]
    #[inline(always)]
    pub fn gpioinv(&self) -> GpioinvR {
        GpioinvR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Not Ready Value"]
    #[inline(always)]
    pub fn notrdyval(&mut self) -> NotrdyvalW<CtrlSpec> {
        NotrdyvalW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator GPIO Output Invert"]
    #[inline(always)]
    pub fn gpioinv(&mut self) -> GpioinvW<CtrlSpec> {
        GpioinvW::new(self, 1)
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
