#[doc = "Register `POLY` reader"]
pub type R = crate::R<PolySpec>;
#[doc = "Register `POLY` writer"]
pub type W = crate::W<PolySpec>;
#[doc = "Field `POLY` reader - CRC Polynomial Value"]
pub type PolyR = crate::FieldReader<u16>;
#[doc = "Field `POLY` writer - CRC Polynomial Value"]
pub type PolyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC Polynomial Value"]
    #[inline(always)]
    pub fn poly(&self) -> PolyR {
        PolyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC Polynomial Value"]
    #[inline(always)]
    pub fn poly(&mut self) -> PolyW<PolySpec> {
        PolyW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`poly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PolySpec;
impl crate::RegisterSpec for PolySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`poly::R`](R) reader structure"]
impl crate::Readable for PolySpec {}
#[doc = "`write(|w| ..)` method takes [`poly::W`](W) writer structure"]
impl crate::Writable for PolySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POLY to value 0"]
impl crate::Resettable for PolySpec {}
