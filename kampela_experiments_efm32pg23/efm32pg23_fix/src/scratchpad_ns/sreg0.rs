#[doc = "Register `SREG0` reader"]
pub type R = crate::R<Sreg0Spec>;
#[doc = "Register `SREG0` writer"]
pub type W = crate::W<Sreg0Spec>;
#[doc = "Field `SCRATCH` reader - Scratch Pad Register"]
pub type ScratchR = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH` writer - Scratch Pad Register"]
pub type ScratchW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Scratch Pad Register"]
    #[inline(always)]
    pub fn scratch(&self) -> ScratchR {
        ScratchR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scratch Pad Register"]
    #[inline(always)]
    pub fn scratch(&mut self) -> ScratchW<Sreg0Spec> {
        ScratchW::new(self, 0)
    }
}
#[doc = "Used for SIMCTRL Pointer in Verification Environment\n\nYou can [`read`](crate::Reg::read) this register and get [`sreg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sreg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sreg0Spec;
impl crate::RegisterSpec for Sreg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sreg0::R`](R) reader structure"]
impl crate::Readable for Sreg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sreg0::W`](W) writer structure"]
impl crate::Writable for Sreg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SREG0 to value 0"]
impl crate::Resettable for Sreg0Spec {}
