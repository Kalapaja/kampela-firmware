#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `INPUTCTRL` reader - Syncbusy for INPUTCTRL"]
pub type InputctrlR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Syncbusy for INPUTCTRL"]
    #[inline(always)]
    pub fn inputctrl(&self) -> InputctrlR {
        InputctrlR::new((self.bits & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
