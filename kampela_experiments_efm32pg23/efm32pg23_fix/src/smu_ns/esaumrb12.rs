#[doc = "Register `ESAUMRB12` reader"]
pub type R = crate::R<Esaumrb12Spec>;
#[doc = "Register `ESAUMRB12` writer"]
pub type W = crate::W<Esaumrb12Spec>;
#[doc = "Field `ESAUMRB12` reader - Moveable Region Boundary"]
pub type Esaumrb12R = crate::FieldReader<u16>;
#[doc = "Field `ESAUMRB12` writer - Moveable Region Boundary"]
pub type Esaumrb12W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 12:27 - Moveable Region Boundary"]
    #[inline(always)]
    pub fn esaumrb12(&self) -> Esaumrb12R {
        Esaumrb12R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:27 - Moveable Region Boundary"]
    #[inline(always)]
    pub fn esaumrb12(&mut self) -> Esaumrb12W<Esaumrb12Spec> {
        Esaumrb12W::new(self, 12)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`esaumrb12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esaumrb12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esaumrb12Spec;
impl crate::RegisterSpec for Esaumrb12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esaumrb12::R`](R) reader structure"]
impl crate::Readable for Esaumrb12Spec {}
#[doc = "`write(|w| ..)` method takes [`esaumrb12::W`](W) writer structure"]
impl crate::Writable for Esaumrb12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ESAUMRB12 to value 0x0c00_0000"]
impl crate::Resettable for Esaumrb12Spec {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
