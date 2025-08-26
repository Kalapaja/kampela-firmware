#[doc = "Register `TRACECLKCTRL` reader"]
pub type R = crate::R<TraceclkctrlSpec>;
#[doc = "Register `TRACECLKCTRL` writer"]
pub type W = crate::W<TraceclkctrlSpec>;
#[doc = "TRACECLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Presc {
    #[doc = "0: TRACECLK is SYSCLK divided by 1"]
    Div1 = 0,
    #[doc = "1: TRACECLK is SYSCLK divided by 2"]
    Div2 = 1,
    #[doc = "3: TRACECLK is SYSCLK divided by 4"]
    Div4 = 3,
}
impl From<Presc> for u8 {
    #[inline(always)]
    fn from(variant: Presc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Presc {
    type Ux = u8;
}
impl crate::IsEnum for Presc {}
#[doc = "Field `PRESC` reader - TRACECLK Prescaler"]
pub type PrescR = crate::FieldReader<Presc>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Presc> {
        match self.bits {
            0 => Some(Presc::Div1),
            1 => Some(Presc::Div2),
            3 => Some(Presc::Div4),
            _ => None,
        }
    }
    #[doc = "TRACECLK is SYSCLK divided by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Presc::Div1
    }
    #[doc = "TRACECLK is SYSCLK divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Presc::Div2
    }
    #[doc = "TRACECLK is SYSCLK divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Presc::Div4
    }
}
#[doc = "Field `PRESC` writer - TRACECLK Prescaler"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 2, Presc>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TRACECLK is SYSCLK divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div1)
    }
    #[doc = "TRACECLK is SYSCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div2)
    }
    #[doc = "TRACECLK is SYSCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div4)
    }
}
impl R {
    #[doc = "Bits 4:5 - TRACECLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - TRACECLK Prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<TraceclkctrlSpec> {
        PrescW::new(self, 4)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`traceclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TraceclkctrlSpec;
impl crate::RegisterSpec for TraceclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`traceclkctrl::R`](R) reader structure"]
impl crate::Readable for TraceclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`traceclkctrl::W`](W) writer structure"]
impl crate::Writable for TraceclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRACECLKCTRL to value 0"]
impl crate::Resettable for TraceclkctrlSpec {}
