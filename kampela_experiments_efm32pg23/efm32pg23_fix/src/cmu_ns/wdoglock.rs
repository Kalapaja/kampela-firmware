#[doc = "Register `WDOGLOCK` writer"]
pub type W = crate::W<WdoglockSpec>;
#[doc = "Configuration Lock Key\n\nValue on reset: 21079"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Lockkey {
    #[doc = "37879: Write this value to unlock"]
    Unlock = 37879,
}
impl From<Lockkey> for u16 {
    #[inline(always)]
    fn from(variant: Lockkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lockkey {
    type Ux = u16;
}
impl crate::IsEnum for Lockkey {}
#[doc = "Field `LOCKKEY` writer - Configuration Lock Key"]
pub type LockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, Lockkey>;
impl<'a, REG> LockkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Write this value to unlock"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Lockkey::Unlock)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configuration Lock Key"]
    #[inline(always)]
    pub fn lockkey(&mut self) -> LockkeyW<WdoglockSpec> {
        LockkeyW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdoglock::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdoglockSpec;
impl crate::RegisterSpec for WdoglockSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wdoglock::W`](W) writer structure"]
impl crate::Writable for WdoglockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOGLOCK to value 0x5257"]
impl crate::Resettable for WdoglockSpec {
    const RESET_VALUE: u32 = 0x5257;
}
