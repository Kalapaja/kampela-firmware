#[doc = "Register `ESAUMRB01` reader"]
pub type R = crate::R<Esaumrb01Spec>;
#[doc = "Register `ESAUMRB01` writer"]
pub type W = crate::W<Esaumrb01Spec>;
#[doc = "Field `ESAUMRB01` reader - Moveable Region Boundary"]
pub type Esaumrb01R = crate::FieldReader<u16>;
#[doc = "Field `ESAUMRB01` writer - Moveable Region Boundary"]
pub type Esaumrb01W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - Moveable Region Boundary"]
    #[inline(always)]
    pub fn esaumrb01(&self) -> Esaumrb01R {
        Esaumrb01R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - Moveable Region Boundary"]
    #[inline(always)]
    pub fn esaumrb01(&mut self) -> Esaumrb01W<Esaumrb01Spec> {
        Esaumrb01W::new(self, 12)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`esaumrb01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esaumrb01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esaumrb01Spec;
impl crate::RegisterSpec for Esaumrb01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esaumrb01::R`](R) reader structure"]
impl crate::Readable for Esaumrb01Spec {}
#[doc = "`write(|w| ..)` method takes [`esaumrb01::W`](W) writer structure"]
impl crate::Writable for Esaumrb01Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ESAUMRB01 to value 0x0a00_0000"]
impl crate::Resettable for Esaumrb01Spec {
    const RESET_VALUE: u32 = 0x0a00_0000;
}
