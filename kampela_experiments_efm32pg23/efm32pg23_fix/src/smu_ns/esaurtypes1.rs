#[doc = "Register `ESAURTYPES1` reader"]
pub type R = crate::R<Esaurtypes1Spec>;
#[doc = "Register `ESAURTYPES1` writer"]
pub type W = crate::W<Esaurtypes1Spec>;
#[doc = "Field `ESAUR11NS` reader - Region 11 Non-Secure"]
pub type Esaur11nsR = crate::BitReader;
#[doc = "Field `ESAUR11NS` writer - Region 11 Non-Secure"]
pub type Esaur11nsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - Region 11 Non-Secure"]
    #[inline(always)]
    pub fn esaur11ns(&self) -> Esaur11nsR {
        Esaur11nsR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Region 11 Non-Secure"]
    #[inline(always)]
    pub fn esaur11ns(&mut self) -> Esaur11nsW<Esaurtypes1Spec> {
        Esaur11nsW::new(self, 12)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`esaurtypes1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esaurtypes1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esaurtypes1Spec;
impl crate::RegisterSpec for Esaurtypes1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esaurtypes1::R`](R) reader structure"]
impl crate::Readable for Esaurtypes1Spec {}
#[doc = "`write(|w| ..)` method takes [`esaurtypes1::W`](W) writer structure"]
impl crate::Writable for Esaurtypes1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ESAURTYPES1 to value 0"]
impl crate::Resettable for Esaurtypes1Spec {}
