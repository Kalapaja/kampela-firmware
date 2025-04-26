#[doc = "Register `CFG0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<Cfg0Spec>;
#[doc = "Synchronous Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sync {
    #[doc = "0: The EUSART operates in asynchronous mode"]
    Async = 0,
    #[doc = "1: The EUSART operates in synchronous mode"]
    Sync = 1,
}
impl From<Sync> for bool {
    #[inline(always)]
    fn from(variant: Sync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` reader - Synchronous Mode"]
pub type SyncR = crate::BitReader<Sync>;
impl SyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync {
        match self.bits {
            false => Sync::Async,
            true => Sync::Sync,
        }
    }
    #[doc = "The EUSART operates in asynchronous mode"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == Sync::Async
    }
    #[doc = "The EUSART operates in synchronous mode"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Sync::Sync
    }
}
#[doc = "Field `SYNC` writer - Synchronous Mode"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG, Sync>;
impl<'a, REG> SyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The EUSART operates in asynchronous mode"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::Async)
    }
    #[doc = "The EUSART operates in synchronous mode"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::Sync)
    }
}
#[doc = "Loopback Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loopbk {
    #[doc = "0: The receiver is connected to and receives data from UARTn_RX"]
    Disable = 0,
    #[doc = "1: The receiver is connected to and receives data from UARTn_TX"]
    Enable = 1,
}
impl From<Loopbk> for bool {
    #[inline(always)]
    fn from(variant: Loopbk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPBK` reader - Loopback Enable"]
pub type LoopbkR = crate::BitReader<Loopbk>;
impl LoopbkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loopbk {
        match self.bits {
            false => Loopbk::Disable,
            true => Loopbk::Enable,
        }
    }
    #[doc = "The receiver is connected to and receives data from UARTn_RX"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Loopbk::Disable
    }
    #[doc = "The receiver is connected to and receives data from UARTn_TX"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Loopbk::Enable
    }
}
#[doc = "Field `LOOPBK` writer - Loopback Enable"]
pub type LoopbkW<'a, REG> = crate::BitWriter<'a, REG, Loopbk>;
impl<'a, REG> LoopbkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The receiver is connected to and receives data from UARTn_RX"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Loopbk::Disable)
    }
    #[doc = "The receiver is connected to and receives data from UARTn_TX"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Loopbk::Enable)
    }
}
#[doc = "Collision Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccen {
    #[doc = "0: Collision check is disabled"]
    Disable = 0,
    #[doc = "1: Collision check is enabled. The receiver must be enabled for the check to be performed"]
    Enable = 1,
}
impl From<Ccen> for bool {
    #[inline(always)]
    fn from(variant: Ccen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCEN` reader - Collision Check Enable"]
pub type CcenR = crate::BitReader<Ccen>;
impl CcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccen {
        match self.bits {
            false => Ccen::Disable,
            true => Ccen::Enable,
        }
    }
    #[doc = "Collision check is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ccen::Disable
    }
    #[doc = "Collision check is enabled. The receiver must be enabled for the check to be performed"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ccen::Enable
    }
}
#[doc = "Field `CCEN` writer - Collision Check Enable"]
pub type CcenW<'a, REG> = crate::BitWriter<'a, REG, Ccen>;
impl<'a, REG> CcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Collision check is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ccen::Disable)
    }
    #[doc = "Collision check is enabled. The receiver must be enabled for the check to be performed"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ccen::Enable)
    }
}
#[doc = "Multi-Processor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpm {
    #[doc = "0: The 9th bit of incoming frames has no special function"]
    Disable = 0,
    #[doc = "1: An incoming frame with the 9th bit equal to MPAB will be loaded into the RX FIFO regardless of RXBLOCK and will result in the MPAB interrupt flag being set"]
    Enable = 1,
}
impl From<Mpm> for bool {
    #[inline(always)]
    fn from(variant: Mpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPM` reader - Multi-Processor Mode"]
pub type MpmR = crate::BitReader<Mpm>;
impl MpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpm {
        match self.bits {
            false => Mpm::Disable,
            true => Mpm::Enable,
        }
    }
    #[doc = "The 9th bit of incoming frames has no special function"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpm::Disable
    }
    #[doc = "An incoming frame with the 9th bit equal to MPAB will be loaded into the RX FIFO regardless of RXBLOCK and will result in the MPAB interrupt flag being set"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpm::Enable
    }
}
#[doc = "Field `MPM` writer - Multi-Processor Mode"]
pub type MpmW<'a, REG> = crate::BitWriter<'a, REG, Mpm>;
impl<'a, REG> MpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The 9th bit of incoming frames has no special function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpm::Disable)
    }
    #[doc = "An incoming frame with the 9th bit equal to MPAB will be loaded into the RX FIFO regardless of RXBLOCK and will result in the MPAB interrupt flag being set"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpm::Enable)
    }
}
#[doc = "Field `MPAB` reader - Multi-Processor Address-Bit"]
pub type MpabR = crate::BitReader;
#[doc = "Field `MPAB` writer - Multi-Processor Address-Bit"]
pub type MpabW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ovs {
    #[doc = "0: 16X oversampling"]
    X16 = 0,
    #[doc = "1: 8X oversampling"]
    X8 = 1,
    #[doc = "2: 6X oversampling"]
    X6 = 2,
    #[doc = "3: 4X oversampling"]
    X4 = 3,
    #[doc = "4: Disable oversampling (for LF operation)"]
    Disable = 4,
}
impl From<Ovs> for u8 {
    #[inline(always)]
    fn from(variant: Ovs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ovs {
    type Ux = u8;
}
impl crate::IsEnum for Ovs {}
#[doc = "Field `OVS` reader - Oversampling"]
pub type OvsR = crate::FieldReader<Ovs>;
impl OvsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ovs> {
        match self.bits {
            0 => Some(Ovs::X16),
            1 => Some(Ovs::X8),
            2 => Some(Ovs::X6),
            3 => Some(Ovs::X4),
            4 => Some(Ovs::Disable),
            _ => None,
        }
    }
    #[doc = "16X oversampling"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == Ovs::X16
    }
    #[doc = "8X oversampling"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == Ovs::X8
    }
    #[doc = "6X oversampling"]
    #[inline(always)]
    pub fn is_x6(&self) -> bool {
        *self == Ovs::X6
    }
    #[doc = "4X oversampling"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == Ovs::X4
    }
    #[doc = "Disable oversampling (for LF operation)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ovs::Disable
    }
}
#[doc = "Field `OVS` writer - Oversampling"]
pub type OvsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ovs>;
impl<'a, REG> OvsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16X oversampling"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut crate::W<REG> {
        self.variant(Ovs::X16)
    }
    #[doc = "8X oversampling"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut crate::W<REG> {
        self.variant(Ovs::X8)
    }
    #[doc = "6X oversampling"]
    #[inline(always)]
    pub fn x6(self) -> &'a mut crate::W<REG> {
        self.variant(Ovs::X6)
    }
    #[doc = "4X oversampling"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ovs::X4)
    }
    #[doc = "Disable oversampling (for LF operation)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ovs::Disable)
    }
}
#[doc = "Most Significant Bit First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msbf {
    #[doc = "0: Data is sent with the least significant bit first"]
    Disable = 0,
    #[doc = "1: Data is sent with the most significant bit first"]
    Enable = 1,
}
impl From<Msbf> for bool {
    #[inline(always)]
    fn from(variant: Msbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MsbfR = crate::BitReader<Msbf>;
impl MsbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msbf {
        match self.bits {
            false => Msbf::Disable,
            true => Msbf::Enable,
        }
    }
    #[doc = "Data is sent with the least significant bit first"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Msbf::Disable
    }
    #[doc = "Data is sent with the most significant bit first"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Msbf::Enable
    }
}
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MsbfW<'a, REG> = crate::BitWriter<'a, REG, Msbf>;
impl<'a, REG> MsbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is sent with the least significant bit first"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Msbf::Disable)
    }
    #[doc = "Data is sent with the most significant bit first"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Msbf::Enable)
    }
}
#[doc = "Receiver Input Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxinv {
    #[doc = "0: Input is passed directly to the receiver"]
    Disable = 0,
    #[doc = "1: Input is inverted before it is passed to the receiver"]
    Enable = 1,
}
impl From<Rxinv> for bool {
    #[inline(always)]
    fn from(variant: Rxinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINV` reader - Receiver Input Invert"]
pub type RxinvR = crate::BitReader<Rxinv>;
impl RxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxinv {
        match self.bits {
            false => Rxinv::Disable,
            true => Rxinv::Enable,
        }
    }
    #[doc = "Input is passed directly to the receiver"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rxinv::Disable
    }
    #[doc = "Input is inverted before it is passed to the receiver"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rxinv::Enable
    }
}
#[doc = "Field `RXINV` writer - Receiver Input Invert"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG, Rxinv>;
impl<'a, REG> RxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input is passed directly to the receiver"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinv::Disable)
    }
    #[doc = "Input is inverted before it is passed to the receiver"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinv::Enable)
    }
}
#[doc = "Transmitter output Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txinv {
    #[doc = "0: Output from the transmitter is passed unchanged to UARTn_TX"]
    Disable = 0,
    #[doc = "1: Output from the transmitter is inverted before it is passed to UARTn_TX"]
    Enable = 1,
}
impl From<Txinv> for bool {
    #[inline(always)]
    fn from(variant: Txinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINV` reader - Transmitter output Invert"]
pub type TxinvR = crate::BitReader<Txinv>;
impl TxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txinv {
        match self.bits {
            false => Txinv::Disable,
            true => Txinv::Enable,
        }
    }
    #[doc = "Output from the transmitter is passed unchanged to UARTn_TX"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Txinv::Disable
    }
    #[doc = "Output from the transmitter is inverted before it is passed to UARTn_TX"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Txinv::Enable
    }
}
#[doc = "Field `TXINV` writer - Transmitter output Invert"]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG, Txinv>;
impl<'a, REG> TxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output from the transmitter is passed unchanged to UARTn_TX"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::Disable)
    }
    #[doc = "Output from the transmitter is inverted before it is passed to UARTn_TX"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::Enable)
    }
}
#[doc = "Automatic TX Tristate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autotri {
    #[doc = "0: The output on UARTn_TX when the transmitter is idle is defined by TXINV"]
    Disable = 0,
    #[doc = "1: UARTn_TX is tristated whenever the transmitter is idle"]
    Enable = 1,
}
impl From<Autotri> for bool {
    #[inline(always)]
    fn from(variant: Autotri) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOTRI` reader - Automatic TX Tristate"]
pub type AutotriR = crate::BitReader<Autotri>;
impl AutotriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autotri {
        match self.bits {
            false => Autotri::Disable,
            true => Autotri::Enable,
        }
    }
    #[doc = "The output on UARTn_TX when the transmitter is idle is defined by TXINV"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Autotri::Disable
    }
    #[doc = "UARTn_TX is tristated whenever the transmitter is idle"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Autotri::Enable
    }
}
#[doc = "Field `AUTOTRI` writer - Automatic TX Tristate"]
pub type AutotriW<'a, REG> = crate::BitWriter<'a, REG, Autotri>;
impl<'a, REG> AutotriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The output on UARTn_TX when the transmitter is idle is defined by TXINV"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Autotri::Disable)
    }
    #[doc = "UARTn_TX is tristated whenever the transmitter is idle"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Autotri::Enable)
    }
}
#[doc = "Field `SKIPPERRF` reader - Skip Parity Error Frames"]
pub type SkipperrfR = crate::BitReader;
#[doc = "Field `SKIPPERRF` writer - Skip Parity Error Frames"]
pub type SkipperrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Halt DMA Read On Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errsdma {
    #[doc = "0: Framing and parity errors have no effect on DMA requests from the EUSART"]
    Disable = 0,
    #[doc = "1: DMA requests from the EUSART are blocked while the PERR or FERR interrupt flags are set"]
    Enable = 1,
}
impl From<Errsdma> for bool {
    #[inline(always)]
    fn from(variant: Errsdma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSDMA` reader - Halt DMA Read On Error"]
pub type ErrsdmaR = crate::BitReader<Errsdma>;
impl ErrsdmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errsdma {
        match self.bits {
            false => Errsdma::Disable,
            true => Errsdma::Enable,
        }
    }
    #[doc = "Framing and parity errors have no effect on DMA requests from the EUSART"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errsdma::Disable
    }
    #[doc = "DMA requests from the EUSART are blocked while the PERR or FERR interrupt flags are set"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errsdma::Enable
    }
}
#[doc = "Field `ERRSDMA` writer - Halt DMA Read On Error"]
pub type ErrsdmaW<'a, REG> = crate::BitWriter<'a, REG, Errsdma>;
impl<'a, REG> ErrsdmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Framing and parity errors have no effect on DMA requests from the EUSART"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsdma::Disable)
    }
    #[doc = "DMA requests from the EUSART are blocked while the PERR or FERR interrupt flags are set"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsdma::Enable)
    }
}
#[doc = "Disable RX On Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errsrx {
    #[doc = "0: Framing and parity errors have no effect on receiver"]
    Disable = 0,
    #[doc = "1: Framing and parity errors disable the receiver"]
    Enable = 1,
}
impl From<Errsrx> for bool {
    #[inline(always)]
    fn from(variant: Errsrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSRX` reader - Disable RX On Error"]
pub type ErrsrxR = crate::BitReader<Errsrx>;
impl ErrsrxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errsrx {
        match self.bits {
            false => Errsrx::Disable,
            true => Errsrx::Enable,
        }
    }
    #[doc = "Framing and parity errors have no effect on receiver"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errsrx::Disable
    }
    #[doc = "Framing and parity errors disable the receiver"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errsrx::Enable
    }
}
#[doc = "Field `ERRSRX` writer - Disable RX On Error"]
pub type ErrsrxW<'a, REG> = crate::BitWriter<'a, REG, Errsrx>;
impl<'a, REG> ErrsrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Framing and parity errors have no effect on receiver"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsrx::Disable)
    }
    #[doc = "Framing and parity errors disable the receiver"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsrx::Enable)
    }
}
#[doc = "Disable TX On Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errstx {
    #[doc = "0: Received framing and parity errors have no effect on transmitter"]
    Disable = 0,
    #[doc = "1: Received framing and parity errors disable the transmitter"]
    Enable = 1,
}
impl From<Errstx> for bool {
    #[inline(always)]
    fn from(variant: Errstx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSTX` reader - Disable TX On Error"]
pub type ErrstxR = crate::BitReader<Errstx>;
impl ErrstxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errstx {
        match self.bits {
            false => Errstx::Disable,
            true => Errstx::Enable,
        }
    }
    #[doc = "Received framing and parity errors have no effect on transmitter"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errstx::Disable
    }
    #[doc = "Received framing and parity errors disable the transmitter"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errstx::Enable
    }
}
#[doc = "Field `ERRSTX` writer - Disable TX On Error"]
pub type ErrstxW<'a, REG> = crate::BitWriter<'a, REG, Errstx>;
impl<'a, REG> ErrstxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Received framing and parity errors have no effect on transmitter"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errstx::Disable)
    }
    #[doc = "Received framing and parity errors disable the transmitter"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errstx::Enable)
    }
}
#[doc = "Field `MVDIS` reader - Majority Vote Disable"]
pub type MvdisR = crate::BitReader;
#[doc = "Field `MVDIS` writer - Majority Vote Disable"]
pub type MvdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOBAUDEN` reader - AUTOBAUD detection enable"]
pub type AutobaudenR = crate::BitReader;
#[doc = "Field `AUTOBAUDEN` writer - AUTOBAUD detection enable"]
pub type AutobaudenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&self) -> LoopbkR {
        LoopbkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline(always)]
    pub fn ccen(&self) -> CcenR {
        CcenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&self) -> MpmR {
        MpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&self) -> MpabR {
        MpabR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Oversampling"]
    #[inline(always)]
    pub fn ovs(&self) -> OvsR {
        OvsR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MsbfR {
        MsbfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmitter output Invert"]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    pub fn autotri(&self) -> AutotriR {
        AutotriR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    pub fn skipperrf(&self) -> SkipperrfR {
        SkipperrfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Halt DMA Read On Error"]
    #[inline(always)]
    pub fn errsdma(&self) -> ErrsdmaR {
        ErrsdmaR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Disable RX On Error"]
    #[inline(always)]
    pub fn errsrx(&self) -> ErrsrxR {
        ErrsrxR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable TX On Error"]
    #[inline(always)]
    pub fn errstx(&self) -> ErrstxR {
        ErrstxR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    pub fn mvdis(&self) -> MvdisR {
        MvdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AUTOBAUD detection enable"]
    #[inline(always)]
    pub fn autobauden(&self) -> AutobaudenR {
        AutobaudenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&mut self) -> SyncW<Cfg0Spec> {
        SyncW::new(self, 0)
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&mut self) -> LoopbkW<Cfg0Spec> {
        LoopbkW::new(self, 1)
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline(always)]
    pub fn ccen(&mut self) -> CcenW<Cfg0Spec> {
        CcenW::new(self, 2)
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&mut self) -> MpmW<Cfg0Spec> {
        MpmW::new(self, 3)
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&mut self) -> MpabW<Cfg0Spec> {
        MpabW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Oversampling"]
    #[inline(always)]
    pub fn ovs(&mut self) -> OvsW<Cfg0Spec> {
        OvsW::new(self, 5)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MsbfW<Cfg0Spec> {
        MsbfW::new(self, 10)
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RxinvW<Cfg0Spec> {
        RxinvW::new(self, 13)
    }
    #[doc = "Bit 14 - Transmitter output Invert"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TxinvW<Cfg0Spec> {
        TxinvW::new(self, 14)
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    pub fn autotri(&mut self) -> AutotriW<Cfg0Spec> {
        AutotriW::new(self, 17)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    pub fn skipperrf(&mut self) -> SkipperrfW<Cfg0Spec> {
        SkipperrfW::new(self, 20)
    }
    #[doc = "Bit 22 - Halt DMA Read On Error"]
    #[inline(always)]
    pub fn errsdma(&mut self) -> ErrsdmaW<Cfg0Spec> {
        ErrsdmaW::new(self, 22)
    }
    #[doc = "Bit 23 - Disable RX On Error"]
    #[inline(always)]
    pub fn errsrx(&mut self) -> ErrsrxW<Cfg0Spec> {
        ErrsrxW::new(self, 23)
    }
    #[doc = "Bit 24 - Disable TX On Error"]
    #[inline(always)]
    pub fn errstx(&mut self) -> ErrstxW<Cfg0Spec> {
        ErrstxW::new(self, 24)
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    pub fn mvdis(&mut self) -> MvdisW<Cfg0Spec> {
        MvdisW::new(self, 30)
    }
    #[doc = "Bit 31 - AUTOBAUD detection enable"]
    #[inline(always)]
    pub fn autobauden(&mut self) -> AutobaudenW<Cfg0Spec> {
        AutobaudenW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spec;
impl crate::RegisterSpec for Cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for Cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for Cfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for Cfg0Spec {}
