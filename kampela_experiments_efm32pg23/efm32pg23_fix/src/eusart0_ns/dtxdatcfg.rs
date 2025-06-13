#[doc = "Register `DTXDATCFG` reader"]
pub type R = crate::R<DtxdatcfgSpec>;
#[doc = "Register `DTXDATCFG` writer"]
pub type W = crate::W<DtxdatcfgSpec>;
#[doc = "Field `DTXDAT` reader - Default TX DATA"]
pub type DtxdatR = crate::FieldReader<u16>;
#[doc = "Field `DTXDAT` writer - Default TX DATA"]
pub type DtxdatW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Default TX DATA"]
    #[inline(always)]
    pub fn dtxdat(&self) -> DtxdatR {
        DtxdatR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Default TX DATA"]
    #[inline(always)]
    pub fn dtxdat(&mut self) -> DtxdatW<DtxdatcfgSpec> {
        DtxdatW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxdatcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtxdatcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtxdatcfgSpec;
impl crate::RegisterSpec for DtxdatcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxdatcfg::R`](R) reader structure"]
impl crate::Readable for DtxdatcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dtxdatcfg::W`](W) writer structure"]
impl crate::Writable for DtxdatcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTXDATCFG to value 0"]
impl crate::Resettable for DtxdatcfgSpec {}
