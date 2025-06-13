#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `KEYSCANSTART` writer - Keyscan Start"]
pub type KeyscanstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYSCANSTOP` writer - Keyscan Stop"]
pub type KeyscanstopW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Keyscan Start"]
    #[inline(always)]
    pub fn keyscanstart(&mut self) -> KeyscanstartW<CmdSpec> {
        KeyscanstartW::new(self, 0)
    }
    #[doc = "Bit 1 - Keyscan Stop"]
    #[inline(always)]
    pub fn keyscanstop(&mut self) -> KeyscanstopW<CmdSpec> {
        KeyscanstopW::new(self, 1)
    }
}
#[doc = "Command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
