#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Soft Reset the internal state registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corerst {
    #[doc = "0: No change to internal state registers"]
    Disable = 0,
    #[doc = "1: Reset the internal state registers"]
    Enable = 1,
}
impl From<Corerst> for bool {
    #[inline(always)]
    fn from(variant: Corerst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORERST` reader - Soft Reset the internal state registers"]
pub type CorerstR = crate::BitReader<Corerst>;
impl CorerstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Corerst {
        match self.bits {
            false => Corerst::Disable,
            true => Corerst::Enable,
        }
    }
    #[doc = "No change to internal state registers"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Corerst::Disable
    }
    #[doc = "Reset the internal state registers"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Corerst::Enable
    }
}
#[doc = "Field `CORERST` writer - Soft Reset the internal state registers"]
pub type CorerstW<'a, REG> = crate::BitWriter<'a, REG, Corerst>;
impl<'a, REG> CorerstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change to internal state registers"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Corerst::Disable)
    }
    #[doc = "Reset the internal state registers"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Corerst::Enable)
    }
}
#[doc = "Addressable as Follower\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slave {
    #[doc = "0: All addresses will be responded to with a NACK"]
    Disable = 0,
    #[doc = "1: Addresses matching the programmed follower address or the general call address (if enabled) require a response from software. Other addresses are automatically responded to with a NACK."]
    Enable = 1,
}
impl From<Slave> for bool {
    #[inline(always)]
    fn from(variant: Slave) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE` reader - Addressable as Follower"]
pub type SlaveR = crate::BitReader<Slave>;
impl SlaveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slave {
        match self.bits {
            false => Slave::Disable,
            true => Slave::Enable,
        }
    }
    #[doc = "All addresses will be responded to with a NACK"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Slave::Disable
    }
    #[doc = "Addresses matching the programmed follower address or the general call address (if enabled) require a response from software. Other addresses are automatically responded to with a NACK."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Slave::Enable
    }
}
#[doc = "Field `SLAVE` writer - Addressable as Follower"]
pub type SlaveW<'a, REG> = crate::BitWriter<'a, REG, Slave>;
impl<'a, REG> SlaveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All addresses will be responded to with a NACK"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Slave::Disable)
    }
    #[doc = "Addresses matching the programmed follower address or the general call address (if enabled) require a response from software. Other addresses are automatically responded to with a NACK."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Slave::Enable)
    }
}
#[doc = "Automatic Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autoack {
    #[doc = "0: Software must give one ACK command for each ACK transmitted on the I2C bus."]
    Disable = 0,
    #[doc = "1: Addresses that are not automatically NACK'ed, and all data is automatically acknowledged."]
    Enable = 1,
}
impl From<Autoack> for bool {
    #[inline(always)]
    fn from(variant: Autoack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOACK` reader - Automatic Acknowledge"]
pub type AutoackR = crate::BitReader<Autoack>;
impl AutoackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autoack {
        match self.bits {
            false => Autoack::Disable,
            true => Autoack::Enable,
        }
    }
    #[doc = "Software must give one ACK command for each ACK transmitted on the I2C bus."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Autoack::Disable
    }
    #[doc = "Addresses that are not automatically NACK'ed, and all data is automatically acknowledged."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Autoack::Enable
    }
}
#[doc = "Field `AUTOACK` writer - Automatic Acknowledge"]
pub type AutoackW<'a, REG> = crate::BitWriter<'a, REG, Autoack>;
impl<'a, REG> AutoackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software must give one ACK command for each ACK transmitted on the I2C bus."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Autoack::Disable)
    }
    #[doc = "Addresses that are not automatically NACK'ed, and all data is automatically acknowledged."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Autoack::Enable)
    }
}
#[doc = "Automatic STOP when Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autose {
    #[doc = "0: A stop must be sent manually when no more data is to be transmitted."]
    Disable = 0,
    #[doc = "1: The leader automatically sends a STOP when no more data is available for transmission."]
    Enable = 1,
}
impl From<Autose> for bool {
    #[inline(always)]
    fn from(variant: Autose) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOSE` reader - Automatic STOP when Empty"]
pub type AutoseR = crate::BitReader<Autose>;
impl AutoseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autose {
        match self.bits {
            false => Autose::Disable,
            true => Autose::Enable,
        }
    }
    #[doc = "A stop must be sent manually when no more data is to be transmitted."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Autose::Disable
    }
    #[doc = "The leader automatically sends a STOP when no more data is available for transmission."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Autose::Enable
    }
}
#[doc = "Field `AUTOSE` writer - Automatic STOP when Empty"]
pub type AutoseW<'a, REG> = crate::BitWriter<'a, REG, Autose>;
impl<'a, REG> AutoseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A stop must be sent manually when no more data is to be transmitted."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Autose::Disable)
    }
    #[doc = "The leader automatically sends a STOP when no more data is available for transmission."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Autose::Enable)
    }
}
#[doc = "Automatic STOP on NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autosn {
    #[doc = "0: Stop is not automatically sent if a NACK is received from a follower."]
    Disable = 0,
    #[doc = "1: The leader automatically sends a STOP if a NACK is received from a follower."]
    Enable = 1,
}
impl From<Autosn> for bool {
    #[inline(always)]
    fn from(variant: Autosn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOSN` reader - Automatic STOP on NACK"]
pub type AutosnR = crate::BitReader<Autosn>;
impl AutosnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autosn {
        match self.bits {
            false => Autosn::Disable,
            true => Autosn::Enable,
        }
    }
    #[doc = "Stop is not automatically sent if a NACK is received from a follower."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Autosn::Disable
    }
    #[doc = "The leader automatically sends a STOP if a NACK is received from a follower."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Autosn::Enable
    }
}
#[doc = "Field `AUTOSN` writer - Automatic STOP on NACK"]
pub type AutosnW<'a, REG> = crate::BitWriter<'a, REG, Autosn>;
impl<'a, REG> AutosnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop is not automatically sent if a NACK is received from a follower."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Autosn::Disable)
    }
    #[doc = "The leader automatically sends a STOP if a NACK is received from a follower."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Autosn::Enable)
    }
}
#[doc = "Arbitration Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arbdis {
    #[doc = "0: When a device loses arbitration, the ARBIF interrupt flag is set and the bus is released."]
    Disable = 0,
    #[doc = "1: When a device loses arbitration, the ARBIF interrupt flag is set, but communication proceeds."]
    Enable = 1,
}
impl From<Arbdis> for bool {
    #[inline(always)]
    fn from(variant: Arbdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBDIS` reader - Arbitration Disable"]
pub type ArbdisR = crate::BitReader<Arbdis>;
impl ArbdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arbdis {
        match self.bits {
            false => Arbdis::Disable,
            true => Arbdis::Enable,
        }
    }
    #[doc = "When a device loses arbitration, the ARBIF interrupt flag is set and the bus is released."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Arbdis::Disable
    }
    #[doc = "When a device loses arbitration, the ARBIF interrupt flag is set, but communication proceeds."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Arbdis::Enable
    }
}
#[doc = "Field `ARBDIS` writer - Arbitration Disable"]
pub type ArbdisW<'a, REG> = crate::BitWriter<'a, REG, Arbdis>;
impl<'a, REG> ArbdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When a device loses arbitration, the ARBIF interrupt flag is set and the bus is released."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Arbdis::Disable)
    }
    #[doc = "When a device loses arbitration, the ARBIF interrupt flag is set, but communication proceeds."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Arbdis::Enable)
    }
}
#[doc = "General Call Address Match Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gcamen {
    #[doc = "0: General call address will be NACK'ed if it is not included by the follower address and address mask."]
    Disable = 0,
    #[doc = "1: When a general call address is received, a software response is required"]
    Enable = 1,
}
impl From<Gcamen> for bool {
    #[inline(always)]
    fn from(variant: Gcamen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCAMEN` reader - General Call Address Match Enable"]
pub type GcamenR = crate::BitReader<Gcamen>;
impl GcamenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gcamen {
        match self.bits {
            false => Gcamen::Disable,
            true => Gcamen::Enable,
        }
    }
    #[doc = "General call address will be NACK'ed if it is not included by the follower address and address mask."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Gcamen::Disable
    }
    #[doc = "When a general call address is received, a software response is required"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Gcamen::Enable
    }
}
#[doc = "Field `GCAMEN` writer - General Call Address Match Enable"]
pub type GcamenW<'a, REG> = crate::BitWriter<'a, REG, Gcamen>;
impl<'a, REG> GcamenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "General call address will be NACK'ed if it is not included by the follower address and address mask."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Gcamen::Disable)
    }
    #[doc = "When a general call address is received, a software response is required"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Gcamen::Enable)
    }
}
#[doc = "TX Buffer Interrupt Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txbil {
    #[doc = "0: TXBL status and the TXBL interrupt flag are set when the transmit buffer becomes empty. TXBL is cleared when the buffer becomes nonempty."]
    Empty = 0,
    #[doc = "1: TXBL status and the TXBL interrupt flag are set when the transmit buffer goes from full to half-full or empty. TXBL is cleared when the buffer becomes full"]
    HalfFull = 1,
}
impl From<Txbil> for bool {
    #[inline(always)]
    fn from(variant: Txbil) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXBIL` reader - TX Buffer Interrupt Level"]
pub type TxbilR = crate::BitReader<Txbil>;
impl TxbilR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txbil {
        match self.bits {
            false => Txbil::Empty,
            true => Txbil::HalfFull,
        }
    }
    #[doc = "TXBL status and the TXBL interrupt flag are set when the transmit buffer becomes empty. TXBL is cleared when the buffer becomes nonempty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Txbil::Empty
    }
    #[doc = "TXBL status and the TXBL interrupt flag are set when the transmit buffer goes from full to half-full or empty. TXBL is cleared when the buffer becomes full"]
    #[inline(always)]
    pub fn is_half_full(&self) -> bool {
        *self == Txbil::HalfFull
    }
}
#[doc = "Field `TXBIL` writer - TX Buffer Interrupt Level"]
pub type TxbilW<'a, REG> = crate::BitWriter<'a, REG, Txbil>;
impl<'a, REG> TxbilW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXBL status and the TXBL interrupt flag are set when the transmit buffer becomes empty. TXBL is cleared when the buffer becomes nonempty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Txbil::Empty)
    }
    #[doc = "TXBL status and the TXBL interrupt flag are set when the transmit buffer goes from full to half-full or empty. TXBL is cleared when the buffer becomes full"]
    #[inline(always)]
    pub fn half_full(self) -> &'a mut crate::W<REG> {
        self.variant(Txbil::HalfFull)
    }
}
#[doc = "Clock Low High Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clhr {
    #[doc = "0: Nlow=4 and Nhigh=4, and the Nlow:Nhigh ratio is 4:4"]
    Standard = 0,
    #[doc = "1: Nlow=6 and Nhigh=3, and the Nlow:Nhigh ratio is 6:3"]
    Asymmetric = 1,
    #[doc = "2: Nlow=11 and Nhigh=6, and the Nlow:Nhigh ratio is 11:6"]
    Fast = 2,
}
impl From<Clhr> for u8 {
    #[inline(always)]
    fn from(variant: Clhr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clhr {
    type Ux = u8;
}
impl crate::IsEnum for Clhr {}
#[doc = "Field `CLHR` reader - Clock Low High Ratio"]
pub type ClhrR = crate::FieldReader<Clhr>;
impl ClhrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clhr> {
        match self.bits {
            0 => Some(Clhr::Standard),
            1 => Some(Clhr::Asymmetric),
            2 => Some(Clhr::Fast),
            _ => None,
        }
    }
    #[doc = "Nlow=4 and Nhigh=4, and the Nlow:Nhigh ratio is 4:4"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Clhr::Standard
    }
    #[doc = "Nlow=6 and Nhigh=3, and the Nlow:Nhigh ratio is 6:3"]
    #[inline(always)]
    pub fn is_asymmetric(&self) -> bool {
        *self == Clhr::Asymmetric
    }
    #[doc = "Nlow=11 and Nhigh=6, and the Nlow:Nhigh ratio is 11:6"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Clhr::Fast
    }
}
#[doc = "Field `CLHR` writer - Clock Low High Ratio"]
pub type ClhrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clhr>;
impl<'a, REG> ClhrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Nlow=4 and Nhigh=4, and the Nlow:Nhigh ratio is 4:4"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Clhr::Standard)
    }
    #[doc = "Nlow=6 and Nhigh=3, and the Nlow:Nhigh ratio is 6:3"]
    #[inline(always)]
    pub fn asymmetric(self) -> &'a mut crate::W<REG> {
        self.variant(Clhr::Asymmetric)
    }
    #[doc = "Nlow=11 and Nhigh=6, and the Nlow:Nhigh ratio is 11:6"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(Clhr::Fast)
    }
}
#[doc = "Bus Idle Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bito {
    #[doc = "0: Timeout disabled"]
    Off = 0,
    #[doc = "1: Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    I2c40pcc = 1,
    #[doc = "2: Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    I2c80pcc = 2,
    #[doc = "3: Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    I2c160pcc = 3,
}
impl From<Bito> for u8 {
    #[inline(always)]
    fn from(variant: Bito) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bito {
    type Ux = u8;
}
impl crate::IsEnum for Bito {}
#[doc = "Field `BITO` reader - Bus Idle Timeout"]
pub type BitoR = crate::FieldReader<Bito>;
impl BitoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bito {
        match self.bits {
            0 => Bito::Off,
            1 => Bito::I2c40pcc,
            2 => Bito::I2c80pcc,
            3 => Bito::I2c160pcc,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Bito::Off
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn is_i2c40pcc(&self) -> bool {
        *self == Bito::I2c40pcc
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn is_i2c80pcc(&self) -> bool {
        *self == Bito::I2c80pcc
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn is_i2c160pcc(&self) -> bool {
        *self == Bito::I2c160pcc
    }
}
#[doc = "Field `BITO` writer - Bus Idle Timeout"]
pub type BitoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bito, crate::Safe>;
impl<'a, REG> BitoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Bito::Off)
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn i2c40pcc(self) -> &'a mut crate::W<REG> {
        self.variant(Bito::I2c40pcc)
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn i2c80pcc(self) -> &'a mut crate::W<REG> {
        self.variant(Bito::I2c80pcc)
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn i2c160pcc(self) -> &'a mut crate::W<REG> {
        self.variant(Bito::I2c160pcc)
    }
}
#[doc = "Go Idle on Bus Idle Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gibito {
    #[doc = "0: A bus idle timeout has no effect on the bus state."]
    Disable = 0,
    #[doc = "1: A bus idle timeout tells the I2C module that the bus is idle, allowing new transfers to be initiated."]
    Enable = 1,
}
impl From<Gibito> for bool {
    #[inline(always)]
    fn from(variant: Gibito) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIBITO` reader - Go Idle on Bus Idle Timeout"]
pub type GibitoR = crate::BitReader<Gibito>;
impl GibitoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gibito {
        match self.bits {
            false => Gibito::Disable,
            true => Gibito::Enable,
        }
    }
    #[doc = "A bus idle timeout has no effect on the bus state."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Gibito::Disable
    }
    #[doc = "A bus idle timeout tells the I2C module that the bus is idle, allowing new transfers to be initiated."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Gibito::Enable
    }
}
#[doc = "Field `GIBITO` writer - Go Idle on Bus Idle Timeout"]
pub type GibitoW<'a, REG> = crate::BitWriter<'a, REG, Gibito>;
impl<'a, REG> GibitoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A bus idle timeout has no effect on the bus state."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Gibito::Disable)
    }
    #[doc = "A bus idle timeout tells the I2C module that the bus is idle, allowing new transfers to be initiated."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Gibito::Enable)
    }
}
#[doc = "Clock Low Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clto {
    #[doc = "0: Timeout disabled"]
    Off = 0,
    #[doc = "1: Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    I2c40pcc = 1,
    #[doc = "2: Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    I2c80pcc = 2,
    #[doc = "3: Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    I2c160pcc = 3,
    #[doc = "4: Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    I2c320pcc = 4,
    #[doc = "5: Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    I2c1024pcc = 5,
}
impl From<Clto> for u8 {
    #[inline(always)]
    fn from(variant: Clto) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clto {
    type Ux = u8;
}
impl crate::IsEnum for Clto {}
#[doc = "Field `CLTO` reader - Clock Low Timeout"]
pub type CltoR = crate::FieldReader<Clto>;
impl CltoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clto> {
        match self.bits {
            0 => Some(Clto::Off),
            1 => Some(Clto::I2c40pcc),
            2 => Some(Clto::I2c80pcc),
            3 => Some(Clto::I2c160pcc),
            4 => Some(Clto::I2c320pcc),
            5 => Some(Clto::I2c1024pcc),
            _ => None,
        }
    }
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Clto::Off
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn is_i2c40pcc(&self) -> bool {
        *self == Clto::I2c40pcc
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn is_i2c80pcc(&self) -> bool {
        *self == Clto::I2c80pcc
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn is_i2c160pcc(&self) -> bool {
        *self == Clto::I2c160pcc
    }
    #[doc = "Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    #[inline(always)]
    pub fn is_i2c320pcc(&self) -> bool {
        *self == Clto::I2c320pcc
    }
    #[doc = "Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    #[inline(always)]
    pub fn is_i2c1024pcc(&self) -> bool {
        *self == Clto::I2c1024pcc
    }
}
#[doc = "Field `CLTO` writer - Clock Low Timeout"]
pub type CltoW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clto>;
impl<'a, REG> CltoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Clto::Off)
    }
    #[doc = "Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout."]
    #[inline(always)]
    pub fn i2c40pcc(self) -> &'a mut crate::W<REG> {
        self.variant(Clto::I2c40pcc)
    }
    #[doc = "Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout."]
    #[inline(always)]
    pub fn i2c80pcc(self) -> &'a mut crate::W<REG> {
        self.variant(Clto::I2c80pcc)
    }
    #[doc = "Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout."]
    #[inline(always)]
    pub fn i2c160pcc(self) -> &'a mut crate::W<REG> {
        self.variant(Clto::I2c160pcc)
    }
    #[doc = "Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout."]
    #[inline(always)]
    pub fn i2c320pcc(self) -> &'a mut crate::W<REG> {
        self.variant(Clto::I2c320pcc)
    }
    #[doc = "Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout."]
    #[inline(always)]
    pub fn i2c1024pcc(self) -> &'a mut crate::W<REG> {
        self.variant(Clto::I2c1024pcc)
    }
}
#[doc = "SCL Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sclmonen {
    #[doc = "0: Disable SCL monitor"]
    Disable = 0,
    #[doc = "1: Enable SCL monitor"]
    Enable = 1,
}
impl From<Sclmonen> for bool {
    #[inline(always)]
    fn from(variant: Sclmonen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCLMONEN` reader - SCL Monitor Enable"]
pub type SclmonenR = crate::BitReader<Sclmonen>;
impl SclmonenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sclmonen {
        match self.bits {
            false => Sclmonen::Disable,
            true => Sclmonen::Enable,
        }
    }
    #[doc = "Disable SCL monitor"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sclmonen::Disable
    }
    #[doc = "Enable SCL monitor"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sclmonen::Enable
    }
}
#[doc = "Field `SCLMONEN` writer - SCL Monitor Enable"]
pub type SclmonenW<'a, REG> = crate::BitWriter<'a, REG, Sclmonen>;
impl<'a, REG> SclmonenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable SCL monitor"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sclmonen::Disable)
    }
    #[doc = "Enable SCL monitor"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sclmonen::Enable)
    }
}
#[doc = "SDA Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdamonen {
    #[doc = "0: Disable SDA Monitor"]
    Disable = 0,
    #[doc = "1: Enable SDA Monitor"]
    Enable = 1,
}
impl From<Sdamonen> for bool {
    #[inline(always)]
    fn from(variant: Sdamonen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDAMONEN` reader - SDA Monitor Enable"]
pub type SdamonenR = crate::BitReader<Sdamonen>;
impl SdamonenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdamonen {
        match self.bits {
            false => Sdamonen::Disable,
            true => Sdamonen::Enable,
        }
    }
    #[doc = "Disable SDA Monitor"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sdamonen::Disable
    }
    #[doc = "Enable SDA Monitor"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sdamonen::Enable
    }
}
#[doc = "Field `SDAMONEN` writer - SDA Monitor Enable"]
pub type SdamonenW<'a, REG> = crate::BitWriter<'a, REG, Sdamonen>;
impl<'a, REG> SdamonenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable SDA Monitor"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sdamonen::Disable)
    }
    #[doc = "Enable SDA Monitor"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sdamonen::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Soft Reset the internal state registers"]
    #[inline(always)]
    pub fn corerst(&self) -> CorerstR {
        CorerstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Addressable as Follower"]
    #[inline(always)]
    pub fn slave(&self) -> SlaveR {
        SlaveR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Automatic Acknowledge"]
    #[inline(always)]
    pub fn autoack(&self) -> AutoackR {
        AutoackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Automatic STOP when Empty"]
    #[inline(always)]
    pub fn autose(&self) -> AutoseR {
        AutoseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic STOP on NACK"]
    #[inline(always)]
    pub fn autosn(&self) -> AutosnR {
        AutosnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Arbitration Disable"]
    #[inline(always)]
    pub fn arbdis(&self) -> ArbdisR {
        ArbdisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General Call Address Match Enable"]
    #[inline(always)]
    pub fn gcamen(&self) -> GcamenR {
        GcamenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX Buffer Interrupt Level"]
    #[inline(always)]
    pub fn txbil(&self) -> TxbilR {
        TxbilR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock Low High Ratio"]
    #[inline(always)]
    pub fn clhr(&self) -> ClhrR {
        ClhrR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Bus Idle Timeout"]
    #[inline(always)]
    pub fn bito(&self) -> BitoR {
        BitoR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Go Idle on Bus Idle Timeout"]
    #[inline(always)]
    pub fn gibito(&self) -> GibitoR {
        GibitoR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Clock Low Timeout"]
    #[inline(always)]
    pub fn clto(&self) -> CltoR {
        CltoR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - SCL Monitor Enable"]
    #[inline(always)]
    pub fn sclmonen(&self) -> SclmonenR {
        SclmonenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SDA Monitor Enable"]
    #[inline(always)]
    pub fn sdamonen(&self) -> SdamonenR {
        SdamonenR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Soft Reset the internal state registers"]
    #[inline(always)]
    pub fn corerst(&mut self) -> CorerstW<CtrlSpec> {
        CorerstW::new(self, 0)
    }
    #[doc = "Bit 1 - Addressable as Follower"]
    #[inline(always)]
    pub fn slave(&mut self) -> SlaveW<CtrlSpec> {
        SlaveW::new(self, 1)
    }
    #[doc = "Bit 2 - Automatic Acknowledge"]
    #[inline(always)]
    pub fn autoack(&mut self) -> AutoackW<CtrlSpec> {
        AutoackW::new(self, 2)
    }
    #[doc = "Bit 3 - Automatic STOP when Empty"]
    #[inline(always)]
    pub fn autose(&mut self) -> AutoseW<CtrlSpec> {
        AutoseW::new(self, 3)
    }
    #[doc = "Bit 4 - Automatic STOP on NACK"]
    #[inline(always)]
    pub fn autosn(&mut self) -> AutosnW<CtrlSpec> {
        AutosnW::new(self, 4)
    }
    #[doc = "Bit 5 - Arbitration Disable"]
    #[inline(always)]
    pub fn arbdis(&mut self) -> ArbdisW<CtrlSpec> {
        ArbdisW::new(self, 5)
    }
    #[doc = "Bit 6 - General Call Address Match Enable"]
    #[inline(always)]
    pub fn gcamen(&mut self) -> GcamenW<CtrlSpec> {
        GcamenW::new(self, 6)
    }
    #[doc = "Bit 7 - TX Buffer Interrupt Level"]
    #[inline(always)]
    pub fn txbil(&mut self) -> TxbilW<CtrlSpec> {
        TxbilW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Clock Low High Ratio"]
    #[inline(always)]
    pub fn clhr(&mut self) -> ClhrW<CtrlSpec> {
        ClhrW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Bus Idle Timeout"]
    #[inline(always)]
    pub fn bito(&mut self) -> BitoW<CtrlSpec> {
        BitoW::new(self, 12)
    }
    #[doc = "Bit 15 - Go Idle on Bus Idle Timeout"]
    #[inline(always)]
    pub fn gibito(&mut self) -> GibitoW<CtrlSpec> {
        GibitoW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Clock Low Timeout"]
    #[inline(always)]
    pub fn clto(&mut self) -> CltoW<CtrlSpec> {
        CltoW::new(self, 16)
    }
    #[doc = "Bit 20 - SCL Monitor Enable"]
    #[inline(always)]
    pub fn sclmonen(&mut self) -> SclmonenW<CtrlSpec> {
        SclmonenW::new(self, 20)
    }
    #[doc = "Bit 21 - SDA Monitor Enable"]
    #[inline(always)]
    pub fn sdamonen(&mut self) -> SdamonenW<CtrlSpec> {
        SdamonenW::new(self, 21)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
