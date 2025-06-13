#[doc = "Register `HFRCOEM23CAL6` reader"]
pub type R = crate::R<Hfrcoem23cal6Spec>;
#[doc = "Field `TUNING` reader - No Description"]
pub type TuningR = crate::FieldReader;
#[doc = "Field `FINETUNING` reader - No Description"]
pub type FinetuningR = crate::FieldReader;
#[doc = "Field `LDOHP` reader - No Description"]
pub type LdohpR = crate::BitReader;
#[doc = "Field `FREQRANGE` reader - No Description"]
pub type FreqrangeR = crate::FieldReader;
#[doc = "Field `CMPBIAS` reader - No Description"]
pub type CmpbiasR = crate::FieldReader;
#[doc = "Field `CLKDIV` reader - No Description"]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CMPSEL` reader - No Description"]
pub type CmpselR = crate::FieldReader;
#[doc = "Field `IREFTC` reader - No Description"]
pub type IreftcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - No Description"]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - No Description"]
    #[inline(always)]
    pub fn finetuning(&self) -> FinetuningR {
        FinetuningR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - No Description"]
    #[inline(always)]
    pub fn ldohp(&self) -> LdohpR {
        LdohpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - No Description"]
    #[inline(always)]
    pub fn freqrange(&self) -> FreqrangeR {
        FreqrangeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - No Description"]
    #[inline(always)]
    pub fn cmpbias(&self) -> CmpbiasR {
        CmpbiasR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:25 - No Description"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - No Description"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CmpselR {
        CmpselR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - No Description"]
    #[inline(always)]
    pub fn ireftc(&self) -> IreftcR {
        IreftcR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfrcoem23cal6Spec;
impl crate::RegisterSpec for Hfrcoem23cal6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfrcoem23cal6::R`](R) reader structure"]
impl crate::Readable for Hfrcoem23cal6Spec {}
#[doc = "`reset()` method sets HFRCOEM23CAL6 to value 0"]
impl crate::Resettable for Hfrcoem23cal6Spec {}
