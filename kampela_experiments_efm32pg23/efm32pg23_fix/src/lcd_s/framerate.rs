#[doc = "Register `FRAMERATE` reader"]
pub type R = crate::R<FramerateSpec>;
#[doc = "Register `FRAMERATE` writer"]
pub type W = crate::W<FramerateSpec>;
#[doc = "Field `FRDIV` reader - Frame Rate Divider"]
pub type FrdivR = crate::FieldReader<u16>;
#[doc = "Field `FRDIV` writer - Frame Rate Divider"]
pub type FrdivW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Frame Rate Divider"]
    #[inline(always)]
    pub fn frdiv(&self) -> FrdivR {
        FrdivR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Frame Rate Divider"]
    #[inline(always)]
    pub fn frdiv(&mut self) -> FrdivW<FramerateSpec> {
        FrdivW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`framerate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framerate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FramerateSpec;
impl crate::RegisterSpec for FramerateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framerate::R`](R) reader structure"]
impl crate::Readable for FramerateSpec {}
#[doc = "`write(|w| ..)` method takes [`framerate::W`](W) writer structure"]
impl crate::Writable for FramerateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMERATE to value 0"]
impl crate::Resettable for FramerateSpec {}
