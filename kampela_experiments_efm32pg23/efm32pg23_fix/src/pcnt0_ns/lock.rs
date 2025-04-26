#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Configuration Lock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Pcntlockkey {
    #[doc = "42976: Write to unock PCNT lockable registers"]
    Unlock = 42976,
}
impl From<Pcntlockkey> for u16 {
    #[inline(always)]
    fn from(variant: Pcntlockkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcntlockkey {
    type Ux = u16;
}
impl crate::IsEnum for Pcntlockkey {}
#[doc = "Field `PCNTLOCKKEY` writer - Configuration Lock Key"]
pub type PcntlockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, Pcntlockkey>;
impl<'a, REG> PcntlockkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Write to unock PCNT lockable registers"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Pcntlockkey::Unlock)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configuration Lock Key"]
    #[inline(always)]
    pub fn pcntlockkey(&mut self) -> PcntlockkeyW<LockSpec> {
        PcntlockkeyW::new(self, 0)
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
