#[doc = "Register `SWCAPA0` reader"]
pub type R = crate::R<Swcapa0Spec>;
#[doc = "Zigbee Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Zigbee {
    #[doc = "0: ZigBee stack capability not available"]
    Level0 = 0,
    #[doc = "1: GreenPower only"]
    Level1 = 1,
    #[doc = "2: ZigBee and GreenPower"]
    Level2 = 2,
    #[doc = "3: ZigBee Only"]
    Level3 = 3,
}
impl From<Zigbee> for u8 {
    #[inline(always)]
    fn from(variant: Zigbee) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Zigbee {
    type Ux = u8;
}
impl crate::IsEnum for Zigbee {}
#[doc = "Field `ZIGBEE` reader - Zigbee Capability"]
pub type ZigbeeR = crate::FieldReader<Zigbee>;
impl ZigbeeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Zigbee {
        match self.bits {
            0 => Zigbee::Level0,
            1 => Zigbee::Level1,
            2 => Zigbee::Level2,
            3 => Zigbee::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "ZigBee stack capability not available"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == Zigbee::Level0
    }
    #[doc = "GreenPower only"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == Zigbee::Level1
    }
    #[doc = "ZigBee and GreenPower"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == Zigbee::Level2
    }
    #[doc = "ZigBee Only"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == Zigbee::Level3
    }
}
#[doc = "Thread Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Thread {
    #[doc = "0: RF4CE stack capability not available"]
    Level0 = 0,
    #[doc = "1: RF4CE stack enabled"]
    Level1 = 1,
    #[doc = "2: N/A"]
    Level2 = 2,
    #[doc = "3: N/A"]
    Level3 = 3,
}
impl From<Thread> for u8 {
    #[inline(always)]
    fn from(variant: Thread) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Thread {
    type Ux = u8;
}
impl crate::IsEnum for Thread {}
#[doc = "Field `THREAD` reader - Thread Capability"]
pub type ThreadR = crate::FieldReader<Thread>;
impl ThreadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Thread {
        match self.bits {
            0 => Thread::Level0,
            1 => Thread::Level1,
            2 => Thread::Level2,
            3 => Thread::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "RF4CE stack capability not available"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == Thread::Level0
    }
    #[doc = "RF4CE stack enabled"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == Thread::Level1
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == Thread::Level2
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == Thread::Level3
    }
}
#[doc = "RF4CE Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rf4ce {
    #[doc = "0: Thread stack capability not available"]
    Level0 = 0,
    #[doc = "1: Thread stack enabled"]
    Level1 = 1,
    #[doc = "2: N/A"]
    Level2 = 2,
    #[doc = "3: N/A"]
    Level3 = 3,
}
impl From<Rf4ce> for u8 {
    #[inline(always)]
    fn from(variant: Rf4ce) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rf4ce {
    type Ux = u8;
}
impl crate::IsEnum for Rf4ce {}
#[doc = "Field `RF4CE` reader - RF4CE Capability"]
pub type Rf4ceR = crate::FieldReader<Rf4ce>;
impl Rf4ceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rf4ce {
        match self.bits {
            0 => Rf4ce::Level0,
            1 => Rf4ce::Level1,
            2 => Rf4ce::Level2,
            3 => Rf4ce::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "Thread stack capability not available"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == Rf4ce::Level0
    }
    #[doc = "Thread stack enabled"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == Rf4ce::Level1
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == Rf4ce::Level2
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == Rf4ce::Level3
    }
}
#[doc = "Bluetooth Smart Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Btsmart {
    #[doc = "0: Bluetooth SMART stack capability not available"]
    Level0 = 0,
    #[doc = "1: Bluetooth SMART enabled"]
    Level1 = 1,
    #[doc = "2: N/A"]
    Level2 = 2,
    #[doc = "3: N/A"]
    Level3 = 3,
}
impl From<Btsmart> for u8 {
    #[inline(always)]
    fn from(variant: Btsmart) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Btsmart {
    type Ux = u8;
}
impl crate::IsEnum for Btsmart {}
#[doc = "Field `BTSMART` reader - Bluetooth Smart Capability"]
pub type BtsmartR = crate::FieldReader<Btsmart>;
impl BtsmartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Btsmart {
        match self.bits {
            0 => Btsmart::Level0,
            1 => Btsmart::Level1,
            2 => Btsmart::Level2,
            3 => Btsmart::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "Bluetooth SMART stack capability not available"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == Btsmart::Level0
    }
    #[doc = "Bluetooth SMART enabled"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == Btsmart::Level1
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == Btsmart::Level2
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == Btsmart::Level3
    }
}
#[doc = "Connect Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Connect {
    #[doc = "0: Connect stack capability not available"]
    Level0 = 0,
    #[doc = "1: Connect enabled"]
    Level1 = 1,
    #[doc = "2: N/A"]
    Level2 = 2,
    #[doc = "3: N/A"]
    Level3 = 3,
}
impl From<Connect> for u8 {
    #[inline(always)]
    fn from(variant: Connect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Connect {
    type Ux = u8;
}
impl crate::IsEnum for Connect {}
#[doc = "Field `CONNECT` reader - Connect Capability"]
pub type ConnectR = crate::FieldReader<Connect>;
impl ConnectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connect {
        match self.bits {
            0 => Connect::Level0,
            1 => Connect::Level1,
            2 => Connect::Level2,
            3 => Connect::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "Connect stack capability not available"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == Connect::Level0
    }
    #[doc = "Connect enabled"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == Connect::Level1
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == Connect::Level2
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == Connect::Level3
    }
}
#[doc = "RAIL Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sri {
    #[doc = "0: RAIL capability not available"]
    Level0 = 0,
    #[doc = "1: RAIL enabled"]
    Level1 = 1,
    #[doc = "2: N/A"]
    Level2 = 2,
    #[doc = "3: N/A"]
    Level3 = 3,
}
impl From<Sri> for u8 {
    #[inline(always)]
    fn from(variant: Sri) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sri {
    type Ux = u8;
}
impl crate::IsEnum for Sri {}
#[doc = "Field `SRI` reader - RAIL Capability"]
pub type SriR = crate::FieldReader<Sri>;
impl SriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sri {
        match self.bits {
            0 => Sri::Level0,
            1 => Sri::Level1,
            2 => Sri::Level2,
            3 => Sri::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "RAIL capability not available"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == Sri::Level0
    }
    #[doc = "RAIL enabled"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == Sri::Level1
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == Sri::Level2
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == Sri::Level3
    }
}
#[doc = "Z-Wave Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Zwave {
    #[doc = "0: Z-Wave stack capability not available"]
    Level0 = 0,
    #[doc = "1: Z-Wave Gateway"]
    Level1 = 1,
    #[doc = "2: Z-Wave End Device"]
    Level2 = 2,
    #[doc = "3: Z-Wave Sensor"]
    Level3 = 3,
    #[doc = "4: Z-Wave Lighting"]
    Level4 = 4,
}
impl From<Zwave> for u8 {
    #[inline(always)]
    fn from(variant: Zwave) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Zwave {
    type Ux = u8;
}
impl crate::IsEnum for Zwave {}
#[doc = "Field `ZWAVE` reader - Z-Wave Capability"]
pub type ZwaveR = crate::FieldReader<Zwave>;
impl ZwaveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Zwave> {
        match self.bits {
            0 => Some(Zwave::Level0),
            1 => Some(Zwave::Level1),
            2 => Some(Zwave::Level2),
            3 => Some(Zwave::Level3),
            4 => Some(Zwave::Level4),
            _ => None,
        }
    }
    #[doc = "Z-Wave stack capability not available"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == Zwave::Level0
    }
    #[doc = "Z-Wave Gateway"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == Zwave::Level1
    }
    #[doc = "Z-Wave End Device"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == Zwave::Level2
    }
    #[doc = "Z-Wave Sensor"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == Zwave::Level3
    }
    #[doc = "Z-Wave Lighting"]
    #[inline(always)]
    pub fn is_level4(&self) -> bool {
        *self == Zwave::Level4
    }
}
impl R {
    #[doc = "Bits 0:1 - Zigbee Capability"]
    #[inline(always)]
    pub fn zigbee(&self) -> ZigbeeR {
        ZigbeeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Thread Capability"]
    #[inline(always)]
    pub fn thread(&self) -> ThreadR {
        ThreadR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - RF4CE Capability"]
    #[inline(always)]
    pub fn rf4ce(&self) -> Rf4ceR {
        Rf4ceR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Bluetooth Smart Capability"]
    #[inline(always)]
    pub fn btsmart(&self) -> BtsmartR {
        BtsmartR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Connect Capability"]
    #[inline(always)]
    pub fn connect(&self) -> ConnectR {
        ConnectR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - RAIL Capability"]
    #[inline(always)]
    pub fn sri(&self) -> SriR {
        SriR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Z-Wave Capability"]
    #[inline(always)]
    pub fn zwave(&self) -> ZwaveR {
        ZwaveR::new(((self.bits >> 24) & 7) as u8)
    }
}
#[doc = "Software Capability Vector 0\n\nYou can [`read`](crate::Reg::read) this register and get [`swcapa0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swcapa0Spec;
impl crate::RegisterSpec for Swcapa0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swcapa0::R`](R) reader structure"]
impl crate::Readable for Swcapa0Spec {}
#[doc = "`reset()` method sets SWCAPA0 to value 0"]
impl crate::Resettable for Swcapa0Spec {}
