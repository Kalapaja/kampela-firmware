#[doc = "Register `SWFIX` reader"]
pub type R = crate::R<SwfixSpec>;
#[doc = "Field `RSV` reader - Reserved"]
pub type RsvR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved"]
    #[inline(always)]
    pub fn rsv(&self) -> RsvR {
        RsvR::new(self.bits)
    }
}
#[doc = "Used to track s/w workaround info\n\nYou can [`read`](crate::Reg::read) this register and get [`swfix::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwfixSpec;
impl crate::RegisterSpec for SwfixSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swfix::R`](R) reader structure"]
impl crate::Readable for SwfixSpec {}
#[doc = "`reset()` method sets SWFIX to value 0xffff_ffff"]
impl crate::Resettable for SwfixSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
