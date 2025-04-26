#[doc = "Register `CALTEMP` reader"]
pub type R = crate::R<CaltempSpec>;
#[doc = "Field `TEMP` reader - Cal Temp"]
pub type TempR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Cal Temp"]
    #[inline(always)]
    pub fn temp(&self) -> TempR {
        TempR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Calibration Temperature Information\n\nYou can [`read`](crate::Reg::read) this register and get [`caltemp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaltempSpec;
impl crate::RegisterSpec for CaltempSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`caltemp::R`](R) reader structure"]
impl crate::Readable for CaltempSpec {}
#[doc = "`reset()` method sets CALTEMP to value 0"]
impl crate::Resettable for CaltempSpec {}
