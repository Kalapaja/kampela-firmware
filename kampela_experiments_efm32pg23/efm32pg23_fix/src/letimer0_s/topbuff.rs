#[doc = "Register `TOPBUFF` reader"]
pub type R = crate::R<TopbuffSpec>;
#[doc = "Register `TOPBUFF` writer"]
pub type W = crate::W<TopbuffSpec>;
#[doc = "Field `TOPBUFF` reader - Buffered Counter TOP Value"]
pub type TopbuffR = crate::FieldReader<u32>;
#[doc = "Field `TOPBUFF` writer - Buffered Counter TOP Value"]
pub type TopbuffW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Buffered Counter TOP Value"]
    #[inline(always)]
    pub fn topbuff(&self) -> TopbuffR {
        TopbuffR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Buffered Counter TOP Value"]
    #[inline(always)]
    pub fn topbuff(&mut self) -> TopbuffW<TopbuffSpec> {
        TopbuffW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`topbuff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`topbuff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TopbuffSpec;
impl crate::RegisterSpec for TopbuffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`topbuff::R`](R) reader structure"]
impl crate::Readable for TopbuffSpec {}
#[doc = "`write(|w| ..)` method takes [`topbuff::W`](W) writer structure"]
impl crate::Writable for TopbuffSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOPBUFF to value 0"]
impl crate::Resettable for TopbuffSpec {}
