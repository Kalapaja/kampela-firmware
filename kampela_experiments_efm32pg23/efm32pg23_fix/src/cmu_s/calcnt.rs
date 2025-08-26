#[doc = "Register `CALCNT` reader"]
pub type R = crate::R<CalcntSpec>;
#[doc = "Field `CALCNT` reader - Calibration Result Counter Value"]
pub type CalcntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Calibration Result Counter Value"]
    #[inline(always)]
    pub fn calcnt(&self) -> CalcntR {
        CalcntR::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`calcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalcntSpec;
impl crate::RegisterSpec for CalcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calcnt::R`](R) reader structure"]
impl crate::Readable for CalcntSpec {}
#[doc = "`reset()` method sets CALCNT to value 0"]
impl crate::Resettable for CalcntSpec {}
