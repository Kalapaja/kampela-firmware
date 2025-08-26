#[doc = "Register `CONFIGURATION` reader"]
pub type R = crate::R<ConfigurationSpec>;
#[doc = "Register `CONFIGURATION` writer"]
pub type W = crate::W<ConfigurationSpec>;
#[doc = "Field `TXINTEN` reader - TXINTEN"]
pub type TxintenR = crate::BitReader;
#[doc = "Field `TXINTEN` writer - TXINTEN"]
pub type TxintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINTEN` reader - RXINTEN"]
pub type RxintenR = crate::BitReader;
#[doc = "Field `RXINTEN` writer - RXINTEN"]
pub type RxintenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TXINTEN"]
    #[inline(always)]
    pub fn txinten(&self) -> TxintenR {
        TxintenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXINTEN"]
    #[inline(always)]
    pub fn rxinten(&self) -> RxintenR {
        RxintenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXINTEN"]
    #[inline(always)]
    pub fn txinten(&mut self) -> TxintenW<ConfigurationSpec> {
        TxintenW::new(self, 0)
    }
    #[doc = "Bit 1 - RXINTEN"]
    #[inline(always)]
    pub fn rxinten(&mut self) -> RxintenW<ConfigurationSpec> {
        RxintenW::new(self, 1)
    }
}
#[doc = "Configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`configuration::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`configuration::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigurationSpec;
impl crate::RegisterSpec for ConfigurationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`configuration::R`](R) reader structure"]
impl crate::Readable for ConfigurationSpec {}
#[doc = "`write(|w| ..)` method takes [`configuration::W`](W) writer structure"]
impl crate::Writable for ConfigurationSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIGURATION to value 0"]
impl crate::Resettable for ConfigurationSpec {}
