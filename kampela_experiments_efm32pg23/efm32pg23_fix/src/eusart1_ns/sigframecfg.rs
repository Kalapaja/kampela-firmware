#[doc = "Register `SIGFRAMECFG` reader"]
pub type R = crate::R<SigframecfgSpec>;
#[doc = "Register `SIGFRAMECFG` writer"]
pub type W = crate::W<SigframecfgSpec>;
#[doc = "Field `SIGFRAME` reader - Signal Frame Value"]
pub type SigframeR = crate::FieldReader<u16>;
#[doc = "Field `SIGFRAME` writer - Signal Frame Value"]
pub type SigframeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Signal Frame Value"]
    #[inline(always)]
    pub fn sigframe(&self) -> SigframeR {
        SigframeR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Signal Frame Value"]
    #[inline(always)]
    pub fn sigframe(&mut self) -> SigframeW<SigframecfgSpec> {
        SigframeW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sigframecfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigframecfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SigframecfgSpec;
impl crate::RegisterSpec for SigframecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigframecfg::R`](R) reader structure"]
impl crate::Readable for SigframecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sigframecfg::W`](W) writer structure"]
impl crate::Writable for SigframecfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIGFRAMECFG to value 0"]
impl crate::Resettable for SigframecfgSpec {}
