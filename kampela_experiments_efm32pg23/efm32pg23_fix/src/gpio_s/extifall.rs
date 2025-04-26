#[doc = "Register `EXTIFALL` reader"]
pub type R = crate::R<ExtifallSpec>;
#[doc = "Register `EXTIFALL` writer"]
pub type W = crate::W<ExtifallSpec>;
#[doc = "Field `EXTIFALL` reader - EXT Int FALL"]
pub type ExtifallR = crate::FieldReader<u16>;
#[doc = "Field `EXTIFALL` writer - EXT Int FALL"]
pub type ExtifallW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - EXT Int FALL"]
    #[inline(always)]
    pub fn extifall(&self) -> ExtifallR {
        ExtifallR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - EXT Int FALL"]
    #[inline(always)]
    pub fn extifall(&mut self) -> ExtifallW<ExtifallSpec> {
        ExtifallW::new(self, 0)
    }
}
#[doc = "External Interrupt Falling Edge Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`extifall::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extifall::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtifallSpec;
impl crate::RegisterSpec for ExtifallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extifall::R`](R) reader structure"]
impl crate::Readable for ExtifallSpec {}
#[doc = "`write(|w| ..)` method takes [`extifall::W`](W) writer structure"]
impl crate::Writable for ExtifallSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTIFALL to value 0"]
impl crate::Resettable for ExtifallSpec {}
