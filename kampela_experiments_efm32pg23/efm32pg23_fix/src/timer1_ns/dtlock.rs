#[doc = "Register `DTLOCK` writer"]
pub type W = crate::W<DtlockSpec>;
#[doc = "DTI Lock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Dtilockkey {
    #[doc = "52864: Write to unlock TIMER DTI registers"]
    Unlock = 52864,
}
impl From<Dtilockkey> for u16 {
    #[inline(always)]
    fn from(variant: Dtilockkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtilockkey {
    type Ux = u16;
}
impl crate::IsEnum for Dtilockkey {}
#[doc = "Field `DTILOCKKEY` writer - DTI Lock Key"]
pub type DtilockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, Dtilockkey>;
impl<'a, REG> DtilockkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Write to unlock TIMER DTI registers"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Dtilockkey::Unlock)
    }
}
impl W {
    #[doc = "Bits 0:15 - DTI Lock Key"]
    #[inline(always)]
    pub fn dtilockkey(&mut self) -> DtilockkeyW<DtlockSpec> {
        DtilockkeyW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtlock::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtlockSpec;
impl crate::RegisterSpec for DtlockSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dtlock::W`](W) writer structure"]
impl crate::Writable for DtlockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTLOCK to value 0"]
impl crate::Resettable for DtlockSpec {}
