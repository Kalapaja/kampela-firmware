#[doc = "Register `TAMPERRSTCAUSE` reader"]
pub type R = crate::R<TamperrstcauseSpec>;
#[doc = "Field `TAMPERRST` reader - Tamper reset vector"]
pub type TamperrstR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Tamper reset vector"]
    #[inline(always)]
    pub fn tamperrst(&self) -> TamperrstR {
        TamperrstR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`tamperrstcause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TamperrstcauseSpec;
impl crate::RegisterSpec for TamperrstcauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamperrstcause::R`](R) reader structure"]
impl crate::Readable for TamperrstcauseSpec {}
#[doc = "`reset()` method sets TAMPERRSTCAUSE to value 0"]
impl crate::Resettable for TamperrstcauseSpec {}
