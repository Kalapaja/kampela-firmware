#[doc = "Register `SWCAPA1` reader"]
pub type R = crate::R<Swcapa1Spec>;
#[doc = "Field `RFMCUEN` reader - RF-MCU"]
pub type RfmcuenR = crate::BitReader;
#[doc = "Field `NCPEN` reader - NCP"]
pub type NcpenR = crate::BitReader;
#[doc = "Field `GWEN` reader - Gateway"]
pub type GwenR = crate::BitReader;
#[doc = "Field `XOUT` reader - XOUT"]
pub type XoutR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RF-MCU"]
    #[inline(always)]
    pub fn rfmcuen(&self) -> RfmcuenR {
        RfmcuenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NCP"]
    #[inline(always)]
    pub fn ncpen(&self) -> NcpenR {
        NcpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gateway"]
    #[inline(always)]
    pub fn gwen(&self) -> GwenR {
        GwenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XOUT"]
    #[inline(always)]
    pub fn xout(&self) -> XoutR {
        XoutR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Software Capability Vector 1\n\nYou can [`read`](crate::Reg::read) this register and get [`swcapa1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swcapa1Spec;
impl crate::RegisterSpec for Swcapa1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swcapa1::R`](R) reader structure"]
impl crate::Readable for Swcapa1Spec {}
#[doc = "`reset()` method sets SWCAPA1 to value 0"]
impl crate::Resettable for Swcapa1Spec {}
