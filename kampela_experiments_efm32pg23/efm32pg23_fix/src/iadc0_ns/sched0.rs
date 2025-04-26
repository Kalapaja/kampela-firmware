#[doc = "Register `SCHED0` reader"]
pub type R = crate::R<Sched0Spec>;
#[doc = "Register `SCHED0` writer"]
pub type W = crate::W<Sched0Spec>;
#[doc = "Field `PRESCALE` reader - Prescale"]
pub type PrescaleR = crate::FieldReader<u16>;
#[doc = "Field `PRESCALE` writer - Prescale"]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Prescale"]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Prescale"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PrescaleW<Sched0Spec> {
        PrescaleW::new(self, 0)
    }
}
#[doc = "Scheduling\n\nYou can [`read`](crate::Reg::read) this register and get [`sched0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sched0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sched0Spec;
impl crate::RegisterSpec for Sched0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sched0::R`](R) reader structure"]
impl crate::Readable for Sched0Spec {}
#[doc = "`write(|w| ..)` method takes [`sched0::W`](W) writer structure"]
impl crate::Writable for Sched0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCHED0 to value 0"]
impl crate::Resettable for Sched0Spec {}
