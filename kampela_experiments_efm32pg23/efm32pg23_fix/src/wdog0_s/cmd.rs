#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "WDOG Timer Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clear {
    #[doc = "0: WDOG timer is unchanged."]
    Unchanged = 0,
    #[doc = "1: WDOG timer is cleared to 0."]
    Cleared = 1,
}
impl From<Clear> for bool {
    #[inline(always)]
    fn from(variant: Clear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLEAR` writer - WDOG Timer Clear"]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG, Clear>;
impl<'a, REG> ClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDOG timer is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut crate::W<REG> {
        self.variant(Clear::Unchanged)
    }
    #[doc = "WDOG timer is cleared to 0."]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut crate::W<REG> {
        self.variant(Clear::Cleared)
    }
}
impl W {
    #[doc = "Bit 0 - WDOG Timer Clear"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<CmdSpec> {
        ClearW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {}
