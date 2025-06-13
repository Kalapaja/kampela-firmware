#[doc = "Register `SREG1` reader"]
pub type R = crate::R<Sreg1Spec>;
#[doc = "Register `SREG1` writer"]
pub type W = crate::W<Sreg1Spec>;
#[doc = "Field `SCRATCH` reader - Scratch Register"]
pub type ScratchR = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH` writer - Scratch Register"]
pub type ScratchW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Scratch Register"]
    #[inline(always)]
    pub fn scratch(&self) -> ScratchR {
        ScratchR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scratch Register"]
    #[inline(always)]
    pub fn scratch(&mut self) -> ScratchW<Sreg1Spec> {
        ScratchW::new(self, 0)
    }
}
#[doc = "Used for SIMCTRL Data Access in Verification Environment\n\nYou can [`read`](crate::Reg::read) this register and get [`sreg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sreg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sreg1Spec;
impl crate::RegisterSpec for Sreg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sreg1::R`](R) reader structure"]
impl crate::Readable for Sreg1Spec {}
#[doc = "`write(|w| ..)` method takes [`sreg1::W`](W) writer structure"]
impl crate::Writable for Sreg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SREG1 to value 0"]
impl crate::Resettable for Sreg1Spec {}
