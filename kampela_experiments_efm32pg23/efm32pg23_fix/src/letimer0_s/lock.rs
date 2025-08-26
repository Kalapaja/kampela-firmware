#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Configuration Lock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Letimerlockkey {
    #[doc = "52476: Write to unock LETIMER lockable registers"]
    Unlock = 52476,
}
impl From<Letimerlockkey> for u16 {
    #[inline(always)]
    fn from(variant: Letimerlockkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Letimerlockkey {
    type Ux = u16;
}
impl crate::IsEnum for Letimerlockkey {}
#[doc = "Field `LETIMERLOCKKEY` writer - Configuration Lock Key"]
pub type LetimerlockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, Letimerlockkey>;
impl<'a, REG> LetimerlockkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Write to unock LETIMER lockable registers"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Letimerlockkey::Unlock)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configuration Lock Key"]
    #[inline(always)]
    pub fn letimerlockkey(&mut self) -> LetimerlockkeyW<LockSpec> {
        LetimerlockkeyW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LockSpec {}
