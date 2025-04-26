#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `LOAD` writer - Load command"]
pub type LoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR` writer - Clear command"]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Load command"]
    #[inline(always)]
    pub fn load(&mut self) -> LoadW<CmdSpec> {
        LoadW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear command"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<CmdSpec> {
        ClearW::new(self, 1)
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
