#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `CLEARECCADDR0` writer - Clear ECCERRADDR0"]
pub type Cleareccaddr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARECCADDR1` writer - Clear ECCERRADDR1"]
pub type Cleareccaddr1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear ECCERRADDR0"]
    #[inline(always)]
    pub fn cleareccaddr0(&mut self) -> Cleareccaddr0W<CmdSpec> {
        Cleareccaddr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear ECCERRADDR1"]
    #[inline(always)]
    pub fn cleareccaddr1(&mut self) -> Cleareccaddr1W<CmdSpec> {
        Cleareccaddr1W::new(self, 1)
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
