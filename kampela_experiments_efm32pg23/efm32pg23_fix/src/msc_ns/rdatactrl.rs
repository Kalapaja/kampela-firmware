#[doc = "Register `RDATACTRL` reader"]
pub type R = crate::R<RdatactrlSpec>;
#[doc = "Register `RDATACTRL` writer"]
pub type W = crate::W<RdatactrlSpec>;
#[doc = "Field `AFDIS` reader - Automatic Invalidate Disable"]
pub type AfdisR = crate::BitReader;
#[doc = "Field `AFDIS` writer - Automatic Invalidate Disable"]
pub type AfdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTBUFEN` reader - Flash dout pipeline buffer enable"]
pub type DoutbufenR = crate::BitReader;
#[doc = "Field `DOUTBUFEN` writer - Flash dout pipeline buffer enable"]
pub type DoutbufenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Automatic Invalidate Disable"]
    #[inline(always)]
    pub fn afdis(&self) -> AfdisR {
        AfdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 12 - Flash dout pipeline buffer enable"]
    #[inline(always)]
    pub fn doutbufen(&self) -> DoutbufenR {
        DoutbufenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Automatic Invalidate Disable"]
    #[inline(always)]
    pub fn afdis(&mut self) -> AfdisW<RdatactrlSpec> {
        AfdisW::new(self, 1)
    }
    #[doc = "Bit 12 - Flash dout pipeline buffer enable"]
    #[inline(always)]
    pub fn doutbufen(&mut self) -> DoutbufenW<RdatactrlSpec> {
        DoutbufenW::new(self, 12)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rdatactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdatactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdatactrlSpec;
impl crate::RegisterSpec for RdatactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdatactrl::R`](R) reader structure"]
impl crate::Readable for RdatactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rdatactrl::W`](W) writer structure"]
impl crate::Writable for RdatactrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RDATACTRL to value 0x1000"]
impl crate::Resettable for RdatactrlSpec {
    const RESET_VALUE: u32 = 0x1000;
}
