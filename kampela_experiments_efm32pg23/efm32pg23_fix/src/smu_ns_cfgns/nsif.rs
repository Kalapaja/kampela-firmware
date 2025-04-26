#[doc = "Register `NSIF` reader"]
pub type R = crate::R<NsifSpec>;
#[doc = "Register `NSIF` writer"]
pub type W = crate::W<NsifSpec>;
#[doc = "Field `PPUNSPRIV` reader - PPUNS Privilege Interrupt Flag"]
pub type PpunsprivR = crate::BitReader;
#[doc = "Field `PPUNSPRIV` writer - PPUNS Privilege Interrupt Flag"]
pub type PpunsprivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPUNSINST` reader - PPUNS Instruction Interrupt Flag"]
pub type PpunsinstR = crate::BitReader;
#[doc = "Field `PPUNSINST` writer - PPUNS Instruction Interrupt Flag"]
pub type PpunsinstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PPUNS Privilege Interrupt Flag"]
    #[inline(always)]
    pub fn ppunspriv(&self) -> PpunsprivR {
        PpunsprivR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PPUNS Instruction Interrupt Flag"]
    #[inline(always)]
    pub fn ppunsinst(&self) -> PpunsinstR {
        PpunsinstR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPUNS Privilege Interrupt Flag"]
    #[inline(always)]
    pub fn ppunspriv(&mut self) -> PpunsprivW<NsifSpec> {
        PpunsprivW::new(self, 0)
    }
    #[doc = "Bit 2 - PPUNS Instruction Interrupt Flag"]
    #[inline(always)]
    pub fn ppunsinst(&mut self) -> PpunsinstW<NsifSpec> {
        PpunsinstW::new(self, 2)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`nsif::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsif::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NsifSpec;
impl crate::RegisterSpec for NsifSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsif::R`](R) reader structure"]
impl crate::Readable for NsifSpec {}
#[doc = "`write(|w| ..)` method takes [`nsif::W`](W) writer structure"]
impl crate::Writable for NsifSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NSIF to value 0"]
impl crate::Resettable for NsifSpec {}
