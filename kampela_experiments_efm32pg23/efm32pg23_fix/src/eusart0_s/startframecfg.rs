#[doc = "Register `STARTFRAMECFG` reader"]
pub type R = crate::R<StartframecfgSpec>;
#[doc = "Register `STARTFRAMECFG` writer"]
pub type W = crate::W<StartframecfgSpec>;
#[doc = "Field `STARTFRAME` reader - Start Frame"]
pub type StartframeR = crate::FieldReader<u16>;
#[doc = "Field `STARTFRAME` writer - Start Frame"]
pub type StartframeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Start Frame"]
    #[inline(always)]
    pub fn startframe(&self) -> StartframeR {
        StartframeR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Start Frame"]
    #[inline(always)]
    pub fn startframe(&mut self) -> StartframeW<StartframecfgSpec> {
        StartframeW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`startframecfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`startframecfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartframecfgSpec;
impl crate::RegisterSpec for StartframecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`startframecfg::R`](R) reader structure"]
impl crate::Readable for StartframecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`startframecfg::W`](W) writer structure"]
impl crate::Writable for StartframecfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STARTFRAMECFG to value 0"]
impl crate::Resettable for StartframecfgSpec {}
