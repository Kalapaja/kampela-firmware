#[doc = "Register `LCDCOM` reader"]
pub type R = crate::R<LcdcomSpec>;
#[doc = "Register `LCDCOM` writer"]
pub type W = crate::W<LcdcomSpec>;
#[doc = "Field `LCDCOMALLOC` reader - LCD Common Allocation"]
pub type LcdcomallocR = crate::FieldReader;
#[doc = "Field `LCDCOMALLOC` writer - LCD Common Allocation"]
pub type LcdcomallocW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - LCD Common Allocation"]
    #[inline(always)]
    pub fn lcdcomalloc(&self) -> LcdcomallocR {
        LcdcomallocR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - LCD Common Allocation"]
    #[inline(always)]
    pub fn lcdcomalloc(&mut self) -> LcdcomallocW<LcdcomSpec> {
        LcdcomallocW::new(self, 0)
    }
}
#[doc = "LCD Common Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcomSpec;
impl crate::RegisterSpec for LcdcomSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdcom::R`](R) reader structure"]
impl crate::Readable for LcdcomSpec {}
#[doc = "`write(|w| ..)` method takes [`lcdcom::W`](W) writer structure"]
impl crate::Writable for LcdcomSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDCOM to value 0"]
impl crate::Resettable for LcdcomSpec {}
