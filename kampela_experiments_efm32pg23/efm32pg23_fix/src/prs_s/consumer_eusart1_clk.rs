#[doc = "Register `CONSUMER_EUSART1_CLK` reader"]
pub type R = crate::R<ConsumerEusart1ClkSpec>;
#[doc = "Register `CONSUMER_EUSART1_CLK` writer"]
pub type W = crate::W<ConsumerEusart1ClkSpec>;
#[doc = "Field `PRSSEL` reader - CLK async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - CLK async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CLK async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CLK async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerEusart1ClkSpec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "CLK consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_eusart1_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_eusart1_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerEusart1ClkSpec;
impl crate::RegisterSpec for ConsumerEusart1ClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_eusart1_clk::R`](R) reader structure"]
impl crate::Readable for ConsumerEusart1ClkSpec {}
#[doc = "`write(|w| ..)` method takes [`consumer_eusart1_clk::W`](W) writer structure"]
impl crate::Writable for ConsumerEusart1ClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_EUSART1_CLK to value 0"]
impl crate::Resettable for ConsumerEusart1ClkSpec {}
