#[doc = "Register `PPUFS` reader"]
pub type R = crate::R<PpufsSpec>;
#[doc = "Field `PPUFSPERIPHID` reader - Peripheral ID"]
pub type PpufsperiphidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Peripheral ID"]
    #[inline(always)]
    pub fn ppufsperiphid(&self) -> PpufsperiphidR {
        PpufsperiphidR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ppufs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpufsSpec;
impl crate::RegisterSpec for PpufsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppufs::R`](R) reader structure"]
impl crate::Readable for PpufsSpec {}
#[doc = "`reset()` method sets PPUFS to value 0"]
impl crate::Resettable for PpufsSpec {}
