#[doc = "Register `BMPUSATD0` reader"]
pub type R = crate::R<Bmpusatd0Spec>;
#[doc = "Register `BMPUSATD0` writer"]
pub type W = crate::W<Bmpusatd0Spec>;
#[doc = "Field `LDMA` reader - MCU LDMA secure mode"]
pub type LdmaR = crate::BitReader;
#[doc = "Field `LDMA` writer - MCU LDMA secure mode"]
pub type LdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEEXTDMA` reader - SEEXTDMA secure mode"]
pub type SeextdmaR = crate::BitReader;
#[doc = "Field `SEEXTDMA` writer - SEEXTDMA secure mode"]
pub type SeextdmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - MCU LDMA secure mode"]
    #[inline(always)]
    pub fn ldma(&self) -> LdmaR {
        LdmaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - SEEXTDMA secure mode"]
    #[inline(always)]
    pub fn seextdma(&self) -> SeextdmaR {
        SeextdmaR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - MCU LDMA secure mode"]
    #[inline(always)]
    pub fn ldma(&mut self) -> LdmaW<Bmpusatd0Spec> {
        LdmaW::new(self, 2)
    }
    #[doc = "Bit 5 - SEEXTDMA secure mode"]
    #[inline(always)]
    pub fn seextdma(&mut self) -> SeextdmaW<Bmpusatd0Spec> {
        SeextdmaW::new(self, 5)
    }
}
#[doc = "Set master bits to 1 to mark as a secure master\n\nYou can [`read`](crate::Reg::read) this register and get [`bmpusatd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmpusatd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bmpusatd0Spec;
impl crate::RegisterSpec for Bmpusatd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmpusatd0::R`](R) reader structure"]
impl crate::Readable for Bmpusatd0Spec {}
#[doc = "`write(|w| ..)` method takes [`bmpusatd0::W`](W) writer structure"]
impl crate::Writable for Bmpusatd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMPUSATD0 to value 0x3f"]
impl crate::Resettable for Bmpusatd0Spec {
    const RESET_VALUE: u32 = 0x3f;
}
