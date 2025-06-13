#[doc = "Register `NSLOCK` writer"]
pub type W = crate::W<NslockSpec>;
#[doc = "No Description\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Smunslockkey {
    #[doc = "11325013: Unlocks Registers"]
    Unlock = 11325013,
}
impl From<Smunslockkey> for u32 {
    #[inline(always)]
    fn from(variant: Smunslockkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smunslockkey {
    type Ux = u32;
}
impl crate::IsEnum for Smunslockkey {}
#[doc = "Field `SMUNSLOCKKEY` writer - No Description"]
pub type SmunslockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, Smunslockkey>;
impl<'a, REG> SmunslockkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Unlocks Registers"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Smunslockkey::Unlock)
    }
}
impl W {
    #[doc = "Bits 0:23 - No Description"]
    #[inline(always)]
    pub fn smunslockkey(&mut self) -> SmunslockkeyW<NslockSpec> {
        SmunslockkeyW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nslock::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NslockSpec;
impl crate::RegisterSpec for NslockSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nslock::W`](W) writer structure"]
impl crate::Writable for NslockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NSLOCK to value 0"]
impl crate::Resettable for NslockSpec {}
