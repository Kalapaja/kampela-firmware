#[doc = "Register `ESAUMRB56` reader"]
pub type R = crate::R<Esaumrb56Spec>;
#[doc = "Register `ESAUMRB56` writer"]
pub type W = crate::W<Esaumrb56Spec>;
#[doc = "Field `ESAUMRB56` reader - Moveable Region Boundary"]
pub type Esaumrb56R = crate::FieldReader<u16>;
#[doc = "Field `ESAUMRB56` writer - Moveable Region Boundary"]
pub type Esaumrb56W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - Moveable Region Boundary"]
    #[inline(always)]
    pub fn esaumrb56(&self) -> Esaumrb56R {
        Esaumrb56R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - Moveable Region Boundary"]
    #[inline(always)]
    pub fn esaumrb56(&mut self) -> Esaumrb56W<Esaumrb56Spec> {
        Esaumrb56W::new(self, 12)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`esaumrb56::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esaumrb56::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esaumrb56Spec;
impl crate::RegisterSpec for Esaumrb56Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esaumrb56::R`](R) reader structure"]
impl crate::Readable for Esaumrb56Spec {}
#[doc = "`write(|w| ..)` method takes [`esaumrb56::W`](W) writer structure"]
impl crate::Writable for Esaumrb56Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ESAUMRB56 to value 0x0400_0000"]
impl crate::Resettable for Esaumrb56Spec {
    const RESET_VALUE: u32 = 0x0400_0000;
}
