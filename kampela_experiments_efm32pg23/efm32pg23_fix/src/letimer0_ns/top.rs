#[doc = "Register `TOP` reader"]
pub type R = crate::R<TopSpec>;
#[doc = "Register `TOP` writer"]
pub type W = crate::W<TopSpec>;
#[doc = "Field `TOP` reader - Counter TOP Value"]
pub type TopR = crate::FieldReader<u32>;
#[doc = "Field `TOP` writer - Counter TOP Value"]
pub type TopW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Counter TOP Value"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Counter TOP Value"]
    #[inline(always)]
    pub fn top(&mut self) -> TopW<TopSpec> {
        TopW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`top::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TopSpec;
impl crate::RegisterSpec for TopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`top::R`](R) reader structure"]
impl crate::Readable for TopSpec {}
#[doc = "`write(|w| ..)` method takes [`top::W`](W) writer structure"]
impl crate::Writable for TopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOP to value 0"]
impl crate::Resettable for TopSpec {}
