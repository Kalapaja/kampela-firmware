#[doc = "Register `USERDATASIZE` reader"]
pub type R = crate::R<UserdatasizeSpec>;
#[doc = "Field `USERDATASIZE` reader - User Data Size"]
pub type UserdatasizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - User Data Size"]
    #[inline(always)]
    pub fn userdatasize(&self) -> UserdatasizeR {
        UserdatasizeR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`userdatasize::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserdatasizeSpec;
impl crate::RegisterSpec for UserdatasizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`userdatasize::R`](R) reader structure"]
impl crate::Readable for UserdatasizeSpec {}
#[doc = "`reset()` method sets USERDATASIZE to value 0x04"]
impl crate::Resettable for UserdatasizeSpec {
    const RESET_VALUE: u32 = 0x04;
}
