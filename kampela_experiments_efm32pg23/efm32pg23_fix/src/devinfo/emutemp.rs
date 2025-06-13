#[doc = "Register `EMUTEMP` reader"]
pub type R = crate::R<EmutempSpec>;
#[doc = "Field `EMUTEMPROOM` reader - Emu Room Temperature"]
pub type EmutemproomR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 2:10 - Emu Room Temperature"]
    #[inline(always)]
    pub fn emutemproom(&self) -> EmutemproomR {
        EmutemproomR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
}
#[doc = "EMU Temperature Sensor Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`emutemp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmutempSpec;
impl crate::RegisterSpec for EmutempSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emutemp::R`](R) reader structure"]
impl crate::Readable for EmutempSpec {}
#[doc = "`reset()` method sets EMUTEMP to value 0"]
impl crate::Resettable for EmutempSpec {}
