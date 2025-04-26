#[doc = "Register `SYNCSTATUS` reader"]
pub type R = crate::R<SyncstatusSpec>;
#[doc = "Field `SYNCTRIG` reader - sync trig status"]
pub type SynctrigR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - sync trig status"]
    #[inline(always)]
    pub fn synctrig(&self) -> SynctrigR {
        SynctrigR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`syncstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncstatusSpec;
impl crate::RegisterSpec for SyncstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncstatus::R`](R) reader structure"]
impl crate::Readable for SyncstatusSpec {}
#[doc = "`reset()` method sets SYNCSTATUS to value 0"]
impl crate::Resettable for SyncstatusSpec {}
