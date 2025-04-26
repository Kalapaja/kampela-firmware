#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CACHEDIS` reader - Cache Disable"]
pub type CachedisR = crate::BitReader;
#[doc = "Field `CACHEDIS` writer - Cache Disable"]
pub type CachedisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USEMPU` reader - Use MPU"]
pub type UsempuR = crate::BitReader;
#[doc = "Field `USEMPU` writer - Use MPU"]
pub type UsempuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOFLUSHDIS` reader - Automatic Flushing Disable"]
pub type AutoflushdisR = crate::BitReader;
#[doc = "Field `AUTOFLUSHDIS` writer - Automatic Flushing Disable"]
pub type AutoflushdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cache Disable"]
    #[inline(always)]
    pub fn cachedis(&self) -> CachedisR {
        CachedisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Use MPU"]
    #[inline(always)]
    pub fn usempu(&self) -> UsempuR {
        UsempuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic Flushing Disable"]
    #[inline(always)]
    pub fn autoflushdis(&self) -> AutoflushdisR {
        AutoflushdisR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Disable"]
    #[inline(always)]
    pub fn cachedis(&mut self) -> CachedisW<CtrlSpec> {
        CachedisW::new(self, 0)
    }
    #[doc = "Bit 1 - Use MPU"]
    #[inline(always)]
    pub fn usempu(&mut self) -> UsempuW<CtrlSpec> {
        UsempuW::new(self, 1)
    }
    #[doc = "Bit 2 - Automatic Flushing Disable"]
    #[inline(always)]
    pub fn autoflushdis(&mut self) -> AutoflushdisW<CtrlSpec> {
        AutoflushdisW::new(self, 2)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
