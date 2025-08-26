#[doc = "Register `CAL` reader"]
pub type R = crate::R<CalSpec>;
#[doc = "Register `CAL` writer"]
pub type W = crate::W<CalSpec>;
#[doc = "Field `FREQTRIM` reader - Frequency Trim"]
pub type FreqtrimR = crate::FieldReader;
#[doc = "Field `FREQTRIM` writer - Frequency Trim"]
pub type FreqtrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frequency Trim"]
    #[inline(always)]
    pub fn freqtrim(&self) -> FreqtrimR {
        FreqtrimR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frequency Trim"]
    #[inline(always)]
    pub fn freqtrim(&mut self) -> FreqtrimW<CalSpec> {
        FreqtrimW::new(self, 0)
    }
}
#[doc = "Calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalSpec;
impl crate::RegisterSpec for CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CalSpec {}
#[doc = "`write(|w| ..)` method takes [`cal::W`](W) writer structure"]
impl crate::Writable for CalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAL to value 0xa5"]
impl crate::Resettable for CalSpec {
    const RESET_VALUE: u32 = 0xa5;
}
