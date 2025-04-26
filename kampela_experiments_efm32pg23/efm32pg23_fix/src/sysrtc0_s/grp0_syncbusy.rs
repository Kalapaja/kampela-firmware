#[doc = "Register `GRP0_SYNCBUSY` reader"]
pub type R = crate::R<Grp0SyncbusySpec>;
#[doc = "Field `CTRL` reader - Sync busy for CTRL register"]
pub type CtrlR = crate::BitReader;
#[doc = "Field `CMP0VALUE` reader - Sync busy for CMP0VALUE register"]
pub type Cmp0valueR = crate::BitReader;
#[doc = "Field `CMP1VALUE` reader - Sync busy for CMP1VALUE register"]
pub type Cmp1valueR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Sync busy for CTRL register"]
    #[inline(always)]
    pub fn ctrl(&self) -> CtrlR {
        CtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sync busy for CMP0VALUE register"]
    #[inline(always)]
    pub fn cmp0value(&self) -> Cmp0valueR {
        Cmp0valueR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sync busy for CMP1VALUE register"]
    #[inline(always)]
    pub fn cmp1value(&self) -> Cmp1valueR {
        Cmp1valueR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grp0_syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Grp0SyncbusySpec;
impl crate::RegisterSpec for Grp0SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp0_syncbusy::R`](R) reader structure"]
impl crate::Readable for Grp0SyncbusySpec {}
#[doc = "`reset()` method sets GRP0_SYNCBUSY to value 0"]
impl crate::Resettable for Grp0SyncbusySpec {}
