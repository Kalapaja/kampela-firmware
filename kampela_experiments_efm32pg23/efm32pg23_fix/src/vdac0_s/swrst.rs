#[doc = "Register `SWRST` reader"]
pub type R = crate::R<SwrstSpec>;
#[doc = "Register `SWRST` writer"]
pub type W = crate::W<SwrstSpec>;
#[doc = "Field `SWRST` writer - Software reset command"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETTING` reader - Software reset busy status"]
pub type ResettingR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Software reset busy status"]
    #[inline(always)]
    pub fn resetting(&self) -> ResettingR {
        ResettingR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset command"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<SwrstSpec> {
        SwrstW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrstSpec;
impl crate::RegisterSpec for SwrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swrst::R`](R) reader structure"]
impl crate::Readable for SwrstSpec {}
#[doc = "`write(|w| ..)` method takes [`swrst::W`](W) writer structure"]
impl crate::Writable for SwrstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWRST to value 0"]
impl crate::Resettable for SwrstSpec {}
