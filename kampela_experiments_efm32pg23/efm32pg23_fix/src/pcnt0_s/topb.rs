#[doc = "Register `TOPB` reader"]
pub type R = crate::R<TopbSpec>;
#[doc = "Register `TOPB` writer"]
pub type W = crate::W<TopbSpec>;
#[doc = "Field `TOPB` reader - Counter Top Buffer Register"]
pub type TopbR = crate::FieldReader<u16>;
#[doc = "Field `TOPB` writer - Counter Top Buffer Register"]
pub type TopbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Top Buffer Register"]
    #[inline(always)]
    pub fn topb(&self) -> TopbR {
        TopbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Top Buffer Register"]
    #[inline(always)]
    pub fn topb(&mut self) -> TopbW<TopbSpec> {
        TopbW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`topb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`topb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TopbSpec;
impl crate::RegisterSpec for TopbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`topb::R`](R) reader structure"]
impl crate::Readable for TopbSpec {}
#[doc = "`write(|w| ..)` method takes [`topb::W`](W) writer structure"]
impl crate::Writable for TopbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOPB to value 0xff"]
impl crate::Resettable for TopbSpec {
    const RESET_VALUE: u32 = 0xff;
}
