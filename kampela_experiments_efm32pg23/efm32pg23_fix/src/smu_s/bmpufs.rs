#[doc = "Register `BMPUFS` reader"]
pub type R = crate::R<BmpufsSpec>;
#[doc = "Field `BMPUFSMASTERID` reader - Bus Manager ID"]
pub type BmpufsmasteridR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bus Manager ID"]
    #[inline(always)]
    pub fn bmpufsmasterid(&self) -> BmpufsmasteridR {
        BmpufsmasteridR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bmpufs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmpufsSpec;
impl crate::RegisterSpec for BmpufsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmpufs::R`](R) reader structure"]
impl crate::Readable for BmpufsSpec {}
#[doc = "`reset()` method sets BMPUFS to value 0"]
impl crate::Resettable for BmpufsSpec {}
