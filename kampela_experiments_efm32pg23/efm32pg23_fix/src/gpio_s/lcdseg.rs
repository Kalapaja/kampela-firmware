#[doc = "Register `LCDSEG` reader"]
pub type R = crate::R<LcdsegSpec>;
#[doc = "Register `LCDSEG` writer"]
pub type W = crate::W<LcdsegSpec>;
#[doc = "Field `LCDSEGALLOC` reader - LCD Segment Allocation"]
pub type LcdsegallocR = crate::FieldReader<u32>;
#[doc = "Field `LCDSEGALLOC` writer - LCD Segment Allocation"]
pub type LcdsegallocW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - LCD Segment Allocation"]
    #[inline(always)]
    pub fn lcdsegalloc(&self) -> LcdsegallocR {
        LcdsegallocR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - LCD Segment Allocation"]
    #[inline(always)]
    pub fn lcdsegalloc(&mut self) -> LcdsegallocW<LcdsegSpec> {
        LcdsegallocW::new(self, 0)
    }
}
#[doc = "LCD Segment Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdseg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdseg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdsegSpec;
impl crate::RegisterSpec for LcdsegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdseg::R`](R) reader structure"]
impl crate::Readable for LcdsegSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdseg::W`](W) writer structure"]
impl crate::Writable for LcdsegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDSEG to value 0"]
impl crate::Resettable for LcdsegSpec {}
