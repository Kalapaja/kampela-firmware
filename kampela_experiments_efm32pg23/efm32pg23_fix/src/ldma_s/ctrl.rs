#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `NUMFIXED` reader - Number of Fixed Priority Channels"]
pub type NumfixedR = crate::FieldReader;
#[doc = "Field `NUMFIXED` writer - Number of Fixed Priority Channels"]
pub type NumfixedW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CORERST` reader - Reset DMA controller"]
pub type CorerstR = crate::BitReader;
#[doc = "Field `CORERST` writer - Reset DMA controller"]
pub type CorerstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 24:28 - Number of Fixed Priority Channels"]
    #[inline(always)]
    pub fn numfixed(&self) -> NumfixedR {
        NumfixedR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Reset DMA controller"]
    #[inline(always)]
    pub fn corerst(&self) -> CorerstR {
        CorerstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:28 - Number of Fixed Priority Channels"]
    #[inline(always)]
    pub fn numfixed(&mut self) -> NumfixedW<CtrlSpec> {
        NumfixedW::new(self, 24)
    }
    #[doc = "Bit 31 - Reset DMA controller"]
    #[inline(always)]
    pub fn corerst(&mut self) -> CorerstW<CtrlSpec> {
        CorerstW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x1e00_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x1e00_0000;
}
