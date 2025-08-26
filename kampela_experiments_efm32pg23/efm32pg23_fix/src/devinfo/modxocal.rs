#[doc = "Register `MODXOCAL` reader"]
pub type R = crate::R<ModxocalSpec>;
#[doc = "Field `HFXOCTUNEXIANA` reader - No Description"]
pub type HfxoctunexianaR = crate::FieldReader;
#[doc = "Field `HFXOCTUNEXOANA` reader - No Description"]
pub type HfxoctunexoanaR = crate::FieldReader;
#[doc = "Field `LFXOCAPTUNE` reader - No Description"]
pub type LfxocaptuneR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - No Description"]
    #[inline(always)]
    pub fn hfxoctunexiana(&self) -> HfxoctunexianaR {
        HfxoctunexianaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - No Description"]
    #[inline(always)]
    pub fn hfxoctunexoana(&self) -> HfxoctunexoanaR {
        HfxoctunexoanaR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - No Description"]
    #[inline(always)]
    pub fn lfxocaptune(&self) -> LfxocaptuneR {
        LfxocaptuneR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Module Crystal Oscillator Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`modxocal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModxocalSpec;
impl crate::RegisterSpec for ModxocalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modxocal::R`](R) reader structure"]
impl crate::Readable for ModxocalSpec {}
#[doc = "`reset()` method sets MODXOCAL to value 0x007f_ffff"]
impl crate::Resettable for ModxocalSpec {
    const RESET_VALUE: u32 = 0x007f_ffff;
}
