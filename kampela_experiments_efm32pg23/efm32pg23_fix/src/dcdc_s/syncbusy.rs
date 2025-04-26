#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `CTRL` reader - CTRL Sync Busy Status"]
pub type CtrlR = crate::BitReader;
#[doc = "Field `EM01CTRL0` reader - EM01CTRL0 Sync Busy Status"]
pub type Em01ctrl0R = crate::BitReader;
#[doc = "Field `EM01CTRL1` reader - EM01CTRL1 Sync Bust Status"]
pub type Em01ctrl1R = crate::BitReader;
#[doc = "Field `EM23CTRL0` reader - EM23CTRL0 Sync Busy Status"]
pub type Em23ctrl0R = crate::BitReader;
#[doc = "Field `PFMXCTRL` reader - PFMXCTRL Sync Busy Status"]
pub type PfmxctrlR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CTRL Sync Busy Status"]
    #[inline(always)]
    pub fn ctrl(&self) -> CtrlR {
        CtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EM01CTRL0 Sync Busy Status"]
    #[inline(always)]
    pub fn em01ctrl0(&self) -> Em01ctrl0R {
        Em01ctrl0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EM01CTRL1 Sync Bust Status"]
    #[inline(always)]
    pub fn em01ctrl1(&self) -> Em01ctrl1R {
        Em01ctrl1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EM23CTRL0 Sync Busy Status"]
    #[inline(always)]
    pub fn em23ctrl0(&self) -> Em23ctrl0R {
        Em23ctrl0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - PFMXCTRL Sync Busy Status"]
    #[inline(always)]
    pub fn pfmxctrl(&self) -> PfmxctrlR {
        PfmxctrlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Syncbusy Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
