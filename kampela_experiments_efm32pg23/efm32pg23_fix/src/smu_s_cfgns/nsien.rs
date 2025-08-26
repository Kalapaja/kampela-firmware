#[doc = "Register `NSIEN` reader"]
pub type R = crate::R<NsienSpec>;
#[doc = "Register `NSIEN` writer"]
pub type W = crate::W<NsienSpec>;
#[doc = "Field `PPUNSPRIV` reader - PPUNS Privilege Interrupt Enable"]
pub type PpunsprivR = crate::BitReader;
#[doc = "Field `PPUNSPRIV` writer - PPUNS Privilege Interrupt Enable"]
pub type PpunsprivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPUNSINST` reader - PPUNS Instruction Interrupt Enable"]
pub type PpunsinstR = crate::BitReader;
#[doc = "Field `PPUNSINST` writer - PPUNS Instruction Interrupt Enable"]
pub type PpunsinstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PPUNS Privilege Interrupt Enable"]
    #[inline(always)]
    pub fn ppunspriv(&self) -> PpunsprivR {
        PpunsprivR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PPUNS Instruction Interrupt Enable"]
    #[inline(always)]
    pub fn ppunsinst(&self) -> PpunsinstR {
        PpunsinstR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPUNS Privilege Interrupt Enable"]
    #[inline(always)]
    pub fn ppunspriv(&mut self) -> PpunsprivW<NsienSpec> {
        PpunsprivW::new(self, 0)
    }
    #[doc = "Bit 2 - PPUNS Instruction Interrupt Enable"]
    #[inline(always)]
    pub fn ppunsinst(&mut self) -> PpunsinstW<NsienSpec> {
        PpunsinstW::new(self, 2)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`nsien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NsienSpec;
impl crate::RegisterSpec for NsienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsien::R`](R) reader structure"]
impl crate::Readable for NsienSpec {}
#[doc = "`write(|w| ..)` method takes [`nsien::W`](W) writer structure"]
impl crate::Writable for NsienSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NSIEN to value 0"]
impl crate::Resettable for NsienSpec {}
