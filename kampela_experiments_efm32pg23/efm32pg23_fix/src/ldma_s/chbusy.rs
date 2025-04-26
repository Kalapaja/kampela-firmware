#[doc = "Register `CHBUSY` reader"]
pub type R = crate::R<ChbusySpec>;
#[doc = "Field `BUSY` reader - Channels Busy"]
pub type BusyR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Channels Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`chbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChbusySpec;
impl crate::RegisterSpec for ChbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chbusy::R`](R) reader structure"]
impl crate::Readable for ChbusySpec {}
#[doc = "`reset()` method sets CHBUSY to value 0"]
impl crate::Resettable for ChbusySpec {}
