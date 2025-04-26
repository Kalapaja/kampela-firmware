#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `CTRL` reader - CTRL Register Busy"]
pub type CtrlR = crate::BitReader;
#[doc = "Field `CMD` reader - CMD Register Busy"]
pub type CmdR = crate::BitReader;
#[doc = "Field `TOP` reader - TOP Register Busy"]
pub type TopR = crate::BitReader;
#[doc = "Field `TOPB` reader - TOPB Register Busy"]
pub type TopbR = crate::BitReader;
#[doc = "Field `OVSCTRL` reader - OVSCTRL Register Busy"]
pub type OvsctrlR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CTRL Register Busy"]
    #[inline(always)]
    pub fn ctrl(&self) -> CtrlR {
        CtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMD Register Busy"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TOP Register Busy"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TOPB Register Busy"]
    #[inline(always)]
    pub fn topb(&self) -> TopbR {
        TopbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OVSCTRL Register Busy"]
    #[inline(always)]
    pub fn ovsctrl(&self) -> OvsctrlR {
        OvsctrlR::new(((self.bits >> 4) & 1) != 0)
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
