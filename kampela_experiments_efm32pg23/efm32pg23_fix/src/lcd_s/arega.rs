#[doc = "Register `AREGA` reader"]
pub type R = crate::R<AregaSpec>;
#[doc = "Register `AREGA` writer"]
pub type W = crate::W<AregaSpec>;
#[doc = "Field `AREGA` reader - Animation Register A Data"]
pub type AregaR = crate::FieldReader;
#[doc = "Field `AREGA` writer - Animation Register A Data"]
pub type AregaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Animation Register A Data"]
    #[inline(always)]
    pub fn arega(&self) -> AregaR {
        AregaR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Animation Register A Data"]
    #[inline(always)]
    pub fn arega(&mut self) -> AregaW<AregaSpec> {
        AregaW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`arega::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arega::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AregaSpec;
impl crate::RegisterSpec for AregaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arega::R`](R) reader structure"]
impl crate::Readable for AregaSpec {}
#[doc = "`write(|w| ..)` method takes [`arega::W`](W) writer structure"]
impl crate::Writable for AregaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AREGA to value 0"]
impl crate::Resettable for AregaSpec {}
