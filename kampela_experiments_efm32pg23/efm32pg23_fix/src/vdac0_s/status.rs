#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `CH0ENS` reader - Channel 0 Enabled Status"]
pub type Ch0ensR = crate::BitReader;
#[doc = "Field `CH1ENS` reader - Channel 1 Enabled Status"]
pub type Ch1ensR = crate::BitReader;
#[doc = "Field `CH0WARM` reader - Channel 0 Warmed Status"]
pub type Ch0warmR = crate::BitReader;
#[doc = "Field `CH1WARM` reader - Channel 1 Warmed Status"]
pub type Ch1warmR = crate::BitReader;
#[doc = "Field `CH0FIFOFULL` reader - Channel 0 FIFO Full Status"]
pub type Ch0fifofullR = crate::BitReader;
#[doc = "Field `CH1FIFOFULL` reader - Channel 1 FIFO Full Status"]
pub type Ch1fifofullR = crate::BitReader;
#[doc = "Field `CH0FIFOCNT` reader - Channel 0 FIFO Valid Count"]
pub type Ch0fifocntR = crate::FieldReader;
#[doc = "Field `CH1FIFOCNT` reader - Channel 1 FIFO Valid Count"]
pub type Ch1fifocntR = crate::FieldReader;
#[doc = "Field `CH0CURRENTSTATE` reader - Channel 0 Current Status"]
pub type Ch0currentstateR = crate::BitReader;
#[doc = "Field `CH1CURRENTSTATE` reader - Channel 1 Current Status"]
pub type Ch1currentstateR = crate::BitReader;
#[doc = "Field `CH0FIFOEMPTY` reader - Channel 0 FIFO Empty Status"]
pub type Ch0fifoemptyR = crate::BitReader;
#[doc = "Field `CH1FIFOEMPTY` reader - Channel 1 FIFO Empty Status"]
pub type Ch1fifoemptyR = crate::BitReader;
#[doc = "Field `CH0FIFOFLBUSY` reader - CH0 FIFO Flush Sync Busy"]
pub type Ch0fifoflbusyR = crate::BitReader;
#[doc = "Field `CH1FIFOFLBUSY` reader - CH1 FIFO Flush Sync Busy"]
pub type Ch1fifoflbusyR = crate::BitReader;
#[doc = "Field `ABUSINPUTCONFLICT` reader - ABUS Input Conflict Status"]
pub type AbusinputconflictR = crate::BitReader;
#[doc = "Field `SINEACTIVE` reader - Sine Wave Output Status on Channel"]
pub type SineactiveR = crate::BitReader;
#[doc = "Field `ABUSALLOCERR` reader - ABUS Allocation Error Status"]
pub type AbusallocerrR = crate::BitReader;
#[doc = "Field `SYNCBUSY` reader - Sync Busy Combined"]
pub type SyncbusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 Enabled Status"]
    #[inline(always)]
    pub fn ch0ens(&self) -> Ch0ensR {
        Ch0ensR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Enabled Status"]
    #[inline(always)]
    pub fn ch1ens(&self) -> Ch1ensR {
        Ch1ensR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Warmed Status"]
    #[inline(always)]
    pub fn ch0warm(&self) -> Ch0warmR {
        Ch0warmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Warmed Status"]
    #[inline(always)]
    pub fn ch1warm(&self) -> Ch1warmR {
        Ch1warmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 FIFO Full Status"]
    #[inline(always)]
    pub fn ch0fifofull(&self) -> Ch0fifofullR {
        Ch0fifofullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 FIFO Full Status"]
    #[inline(always)]
    pub fn ch1fifofull(&self) -> Ch1fifofullR {
        Ch1fifofullR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Channel 0 FIFO Valid Count"]
    #[inline(always)]
    pub fn ch0fifocnt(&self) -> Ch0fifocntR {
        Ch0fifocntR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 1 FIFO Valid Count"]
    #[inline(always)]
    pub fn ch1fifocnt(&self) -> Ch1fifocntR {
        Ch1fifocntR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bit 19 - Channel 0 Current Status"]
    #[inline(always)]
    pub fn ch0currentstate(&self) -> Ch0currentstateR {
        Ch0currentstateR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 1 Current Status"]
    #[inline(always)]
    pub fn ch1currentstate(&self) -> Ch1currentstateR {
        Ch1currentstateR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel 0 FIFO Empty Status"]
    #[inline(always)]
    pub fn ch0fifoempty(&self) -> Ch0fifoemptyR {
        Ch0fifoemptyR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 1 FIFO Empty Status"]
    #[inline(always)]
    pub fn ch1fifoempty(&self) -> Ch1fifoemptyR {
        Ch1fifoemptyR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - CH0 FIFO Flush Sync Busy"]
    #[inline(always)]
    pub fn ch0fifoflbusy(&self) -> Ch0fifoflbusyR {
        Ch0fifoflbusyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CH1 FIFO Flush Sync Busy"]
    #[inline(always)]
    pub fn ch1fifoflbusy(&self) -> Ch1fifoflbusyR {
        Ch1fifoflbusyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ABUS Input Conflict Status"]
    #[inline(always)]
    pub fn abusinputconflict(&self) -> AbusinputconflictR {
        AbusinputconflictR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Sine Wave Output Status on Channel"]
    #[inline(always)]
    pub fn sineactive(&self) -> SineactiveR {
        SineactiveR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ABUS Allocation Error Status"]
    #[inline(always)]
    pub fn abusallocerr(&self) -> AbusallocerrR {
        AbusallocerrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Sync Busy Combined"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SyncbusyR {
        SyncbusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
