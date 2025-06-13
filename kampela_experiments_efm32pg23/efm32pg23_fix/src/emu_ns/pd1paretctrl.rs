#[doc = "Register `PD1PARETCTRL` reader"]
pub type R = crate::R<Pd1paretctrlSpec>;
#[doc = "Register `PD1PARETCTRL` writer"]
pub type W = crate::W<Pd1paretctrlSpec>;
#[doc = "Disable PD1 Partial Retention\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Pd1paretdis {
    #[doc = "1: Retain associated registers when in EM2/3"]
    Periphnoretain = 1,
}
impl From<Pd1paretdis> for u16 {
    #[inline(always)]
    fn from(variant: Pd1paretdis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pd1paretdis {
    type Ux = u16;
}
impl crate::IsEnum for Pd1paretdis {}
#[doc = "Field `PD1PARETDIS` reader - Disable PD1 Partial Retention"]
pub type Pd1paretdisR = crate::FieldReader<Pd1paretdis>;
impl Pd1paretdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pd1paretdis> {
        match self.bits {
            1 => Some(Pd1paretdis::Periphnoretain),
            _ => None,
        }
    }
    #[doc = "Retain associated registers when in EM2/3"]
    #[inline(always)]
    pub fn is_periphnoretain(&self) -> bool {
        *self == Pd1paretdis::Periphnoretain
    }
}
#[doc = "Field `PD1PARETDIS` writer - Disable PD1 Partial Retention"]
pub type Pd1paretdisW<'a, REG> = crate::FieldWriter<'a, REG, 16, Pd1paretdis>;
impl<'a, REG> Pd1paretdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Retain associated registers when in EM2/3"]
    #[inline(always)]
    pub fn periphnoretain(self) -> &'a mut crate::W<REG> {
        self.variant(Pd1paretdis::Periphnoretain)
    }
}
impl R {
    #[doc = "Bits 0:15 - Disable PD1 Partial Retention"]
    #[inline(always)]
    pub fn pd1paretdis(&self) -> Pd1paretdisR {
        Pd1paretdisR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Disable PD1 Partial Retention"]
    #[inline(always)]
    pub fn pd1paretdis(&mut self) -> Pd1paretdisW<Pd1paretctrlSpec> {
        Pd1paretdisW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pd1paretctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd1paretctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pd1paretctrlSpec;
impl crate::RegisterSpec for Pd1paretctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd1paretctrl::R`](R) reader structure"]
impl crate::Readable for Pd1paretctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pd1paretctrl::W`](W) writer structure"]
impl crate::Writable for Pd1paretctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD1PARETCTRL to value 0"]
impl crate::Resettable for Pd1paretctrlSpec {}
