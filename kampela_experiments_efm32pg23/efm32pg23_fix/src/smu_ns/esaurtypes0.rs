#[doc = "Register `ESAURTYPES0` reader"]
pub type R = crate::R<Esaurtypes0Spec>;
#[doc = "Register `ESAURTYPES0` writer"]
pub type W = crate::W<Esaurtypes0Spec>;
#[doc = "Field `ESAUR3NS` reader - Region 3 Non-Secure"]
pub type Esaur3nsR = crate::BitReader;
#[doc = "Field `ESAUR3NS` writer - Region 3 Non-Secure"]
pub type Esaur3nsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - Region 3 Non-Secure"]
    #[inline(always)]
    pub fn esaur3ns(&self) -> Esaur3nsR {
        Esaur3nsR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Region 3 Non-Secure"]
    #[inline(always)]
    pub fn esaur3ns(&mut self) -> Esaur3nsW<Esaurtypes0Spec> {
        Esaur3nsW::new(self, 12)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`esaurtypes0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esaurtypes0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esaurtypes0Spec;
impl crate::RegisterSpec for Esaurtypes0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esaurtypes0::R`](R) reader structure"]
impl crate::Readable for Esaurtypes0Spec {}
#[doc = "`write(|w| ..)` method takes [`esaurtypes0::W`](W) writer structure"]
impl crate::Writable for Esaurtypes0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ESAURTYPES0 to value 0"]
impl crate::Resettable for Esaurtypes0Spec {}
