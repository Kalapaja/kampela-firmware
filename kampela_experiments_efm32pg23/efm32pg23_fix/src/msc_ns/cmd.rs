#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `PWRUP` writer - Flash Power Up Command"]
pub type PwrupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWROFF` writer - Flash power off/sleep command"]
pub type PwroffW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Flash Power Up Command"]
    #[inline(always)]
    pub fn pwrup(&mut self) -> PwrupW<CmdSpec> {
        PwrupW::new(self, 0)
    }
    #[doc = "Bit 4 - Flash power off/sleep command"]
    #[inline(always)]
    pub fn pwroff(&mut self) -> PwroffW<CmdSpec> {
        PwroffW::new(self, 4)
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
