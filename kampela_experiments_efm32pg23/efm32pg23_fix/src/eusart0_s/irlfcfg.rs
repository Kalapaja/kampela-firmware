#[doc = "Register `IRLFCFG` reader"]
pub type R = crate::R<IrlfcfgSpec>;
#[doc = "Register `IRLFCFG` writer"]
pub type W = crate::W<IrlfcfgSpec>;
#[doc = "Field `IRLFEN` reader - Pulse Generator/Extender Enable"]
pub type IrlfenR = crate::BitReader;
#[doc = "Field `IRLFEN` writer - Pulse Generator/Extender Enable"]
pub type IrlfenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    pub fn irlfen(&self) -> IrlfenR {
        IrlfenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    pub fn irlfen(&mut self) -> IrlfenW<IrlfcfgSpec> {
        IrlfenW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`irlfcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irlfcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrlfcfgSpec;
impl crate::RegisterSpec for IrlfcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irlfcfg::R`](R) reader structure"]
impl crate::Readable for IrlfcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`irlfcfg::W`](W) writer structure"]
impl crate::Writable for IrlfcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRLFCFG to value 0"]
impl crate::Resettable for IrlfcfgSpec {}
