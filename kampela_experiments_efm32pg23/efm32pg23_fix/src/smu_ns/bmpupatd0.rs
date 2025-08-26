#[doc = "Register `BMPUPATD0` reader"]
pub type R = crate::R<Bmpupatd0Spec>;
#[doc = "Register `BMPUPATD0` writer"]
pub type W = crate::W<Bmpupatd0Spec>;
#[doc = "Field `LDMA` reader - MCU LDMA privileged mode"]
pub type LdmaR = crate::BitReader;
#[doc = "Field `LDMA` writer - MCU LDMA privileged mode"]
pub type LdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEEXTDMA` reader - SEEXTDMA privileged mode"]
pub type SeextdmaR = crate::BitReader;
#[doc = "Field `SEEXTDMA` writer - SEEXTDMA privileged mode"]
pub type SeextdmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - MCU LDMA privileged mode"]
    #[inline(always)]
    pub fn ldma(&self) -> LdmaR {
        LdmaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - SEEXTDMA privileged mode"]
    #[inline(always)]
    pub fn seextdma(&self) -> SeextdmaR {
        SeextdmaR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - MCU LDMA privileged mode"]
    #[inline(always)]
    pub fn ldma(&mut self) -> LdmaW<Bmpupatd0Spec> {
        LdmaW::new(self, 2)
    }
    #[doc = "Bit 5 - SEEXTDMA privileged mode"]
    #[inline(always)]
    pub fn seextdma(&mut self) -> SeextdmaW<Bmpupatd0Spec> {
        SeextdmaW::new(self, 5)
    }
}
#[doc = "Set master bits to 1 to mark as a privileged master\n\nYou can [`read`](crate::Reg::read) this register and get [`bmpupatd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmpupatd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bmpupatd0Spec;
impl crate::RegisterSpec for Bmpupatd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmpupatd0::R`](R) reader structure"]
impl crate::Readable for Bmpupatd0Spec {}
#[doc = "`write(|w| ..)` method takes [`bmpupatd0::W`](W) writer structure"]
impl crate::Writable for Bmpupatd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMPUPATD0 to value 0x3f"]
impl crate::Resettable for Bmpupatd0Spec {
    const RESET_VALUE: u32 = 0x3f;
}
