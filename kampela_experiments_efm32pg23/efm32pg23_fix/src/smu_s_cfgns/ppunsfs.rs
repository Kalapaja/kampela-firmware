#[doc = "Register `PPUNSFS` reader"]
pub type R = crate::R<PpunsfsSpec>;
#[doc = "Field `PPUFSPERIPHID` reader - Peripheral I"]
pub type PpufsperiphidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Peripheral I"]
    #[inline(always)]
    pub fn ppufsperiphid(&self) -> PpufsperiphidR {
        PpufsperiphidR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ppunsfs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpunsfsSpec;
impl crate::RegisterSpec for PpunsfsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppunsfs::R`](R) reader structure"]
impl crate::Readable for PpunsfsSpec {}
#[doc = "`reset()` method sets PPUNSFS to value 0"]
impl crate::Resettable for PpunsfsSpec {}
