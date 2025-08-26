#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "USART Synchronous Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sync {
    #[doc = "0: The USART operates in asynchronous mode"]
    Disable = 0,
    #[doc = "1: The USART operates in synchronous mode"]
    Enable = 1,
}
impl From<Sync> for bool {
    #[inline(always)]
    fn from(variant: Sync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` reader - USART Synchronous Mode"]
pub type SyncR = crate::BitReader<Sync>;
impl SyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync {
        match self.bits {
            false => Sync::Disable,
            true => Sync::Enable,
        }
    }
    #[doc = "The USART operates in asynchronous mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sync::Disable
    }
    #[doc = "The USART operates in synchronous mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sync::Enable
    }
}
#[doc = "Field `SYNC` writer - USART Synchronous Mode"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG, Sync>;
impl<'a, REG> SyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The USART operates in asynchronous mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::Disable)
    }
    #[doc = "The USART operates in synchronous mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::Enable)
    }
}
#[doc = "Loopback Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loopbk {
    #[doc = "0: The receiver is connected to and receives data from U(S)n_RX"]
    Disable = 0,
    #[doc = "1: The receiver is connected to and receives data from U(S)n_TX"]
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
    #[doc = "The receiver is connected to and receives data from U(S)n_RX"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Loopbk::Disable
    }
    #[doc = "The receiver is connected to and receives data from U(S)n_TX"]
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
    #[doc = "The receiver is connected to and receives data from U(S)n_RX"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Loopbk::Disable)
    }
    #[doc = "The receiver is connected to and receives data from U(S)n_TX"]
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
    #[doc = "1: An incoming frame with the 9th bit equal to MPAB will be loaded into the receive buffer regardless of RXBLOCK and will result in the MPAB interrupt flag being set"]
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
    #[doc = "An incoming frame with the 9th bit equal to MPAB will be loaded into the receive buffer regardless of RXBLOCK and will result in the MPAB interrupt flag being set"]
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
    #[doc = "An incoming frame with the 9th bit equal to MPAB will be loaded into the receive buffer regardless of RXBLOCK and will result in the MPAB interrupt flag being set"]
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
    #[doc = "0: Regular UART mode with 16X oversampling in asynchronous mode"]
    X16 = 0,
    #[doc = "1: Double speed with 8X oversampling in asynchronous mode"]
    X8 = 1,
    #[doc = "2: 6X oversampling in asynchronous mode"]
    X6 = 2,
    #[doc = "3: Quadruple speed with 4X oversampling in asynchronous mode"]
    X4 = 3,
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
    pub const fn variant(&self) -> Ovs {
        match self.bits {
            0 => Ovs::X16,
            1 => Ovs::X8,
            2 => Ovs::X6,
            3 => Ovs::X4,
            _ => unreachable!(),
        }
    }
    #[doc = "Regular UART mode with 16X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == Ovs::X16
    }
    #[doc = "Double speed with 8X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == Ovs::X8
    }
    #[doc = "6X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn is_x6(&self) -> bool {
        *self == Ovs::X6
    }
    #[doc = "Quadruple speed with 4X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == Ovs::X4
    }
}
#[doc = "Field `OVS` writer - Oversampling"]
pub type OvsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ovs, crate::Safe>;
impl<'a, REG> OvsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Regular UART mode with 16X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut crate::W<REG> {
        self.variant(Ovs::X16)
    }
    #[doc = "Double speed with 8X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut crate::W<REG> {
        self.variant(Ovs::X8)
    }
    #[doc = "6X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x6(self) -> &'a mut crate::W<REG> {
        self.variant(Ovs::X6)
    }
    #[doc = "Quadruple speed with 4X oversampling in asynchronous mode"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ovs::X4)
    }
}
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkpol {
    #[doc = "0: The bus clock used in synchronous mode has a low base value"]
    Idlelow = 0,
    #[doc = "1: The bus clock used in synchronous mode has a high base value"]
    Idlehigh = 1,
}
impl From<Clkpol> for bool {
    #[inline(always)]
    fn from(variant: Clkpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKPOL` reader - Clock Polarity"]
pub type ClkpolR = crate::BitReader<Clkpol>;
impl ClkpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkpol {
        match self.bits {
            false => Clkpol::Idlelow,
            true => Clkpol::Idlehigh,
        }
    }
    #[doc = "The bus clock used in synchronous mode has a low base value"]
    #[inline(always)]
    pub fn is_idlelow(&self) -> bool {
        *self == Clkpol::Idlelow
    }
    #[doc = "The bus clock used in synchronous mode has a high base value"]
    #[inline(always)]
    pub fn is_idlehigh(&self) -> bool {
        *self == Clkpol::Idlehigh
    }
}
#[doc = "Field `CLKPOL` writer - Clock Polarity"]
pub type ClkpolW<'a, REG> = crate::BitWriter<'a, REG, Clkpol>;
impl<'a, REG> ClkpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The bus clock used in synchronous mode has a low base value"]
    #[inline(always)]
    pub fn idlelow(self) -> &'a mut crate::W<REG> {
        self.variant(Clkpol::Idlelow)
    }
    #[doc = "The bus clock used in synchronous mode has a high base value"]
    #[inline(always)]
    pub fn idlehigh(self) -> &'a mut crate::W<REG> {
        self.variant(Clkpol::Idlehigh)
    }
}
#[doc = "Clock Edge For Setup/Sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkpha {
    #[doc = "0: Data is sampled on the leading edge and set-up on the trailing edge of the bus clock in synchronous mode"]
    Sampleleading = 0,
    #[doc = "1: Data is set-up on the leading edge and sampled on the trailing edge of the bus clock in synchronous mode"]
    Sampletrailing = 1,
}
impl From<Clkpha> for bool {
    #[inline(always)]
    fn from(variant: Clkpha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKPHA` reader - Clock Edge For Setup/Sample"]
pub type ClkphaR = crate::BitReader<Clkpha>;
impl ClkphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkpha {
        match self.bits {
            false => Clkpha::Sampleleading,
            true => Clkpha::Sampletrailing,
        }
    }
    #[doc = "Data is sampled on the leading edge and set-up on the trailing edge of the bus clock in synchronous mode"]
    #[inline(always)]
    pub fn is_sampleleading(&self) -> bool {
        *self == Clkpha::Sampleleading
    }
    #[doc = "Data is set-up on the leading edge and sampled on the trailing edge of the bus clock in synchronous mode"]
    #[inline(always)]
    pub fn is_sampletrailing(&self) -> bool {
        *self == Clkpha::Sampletrailing
    }
}
#[doc = "Field `CLKPHA` writer - Clock Edge For Setup/Sample"]
pub type ClkphaW<'a, REG> = crate::BitWriter<'a, REG, Clkpha>;
impl<'a, REG> ClkphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is sampled on the leading edge and set-up on the trailing edge of the bus clock in synchronous mode"]
    #[inline(always)]
    pub fn sampleleading(self) -> &'a mut crate::W<REG> {
        self.variant(Clkpha::Sampleleading)
    }
    #[doc = "Data is set-up on the leading edge and sampled on the trailing edge of the bus clock in synchronous mode"]
    #[inline(always)]
    pub fn sampletrailing(self) -> &'a mut crate::W<REG> {
        self.variant(Clkpha::Sampletrailing)
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
#[doc = "Action On Chip Select In Main Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csma {
    #[doc = "0: No action taken"]
    Noaction = 0,
    #[doc = "1: Go to secondary mode"]
    Gotoslavemode = 1,
}
impl From<Csma> for bool {
    #[inline(always)]
    fn from(variant: Csma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSMA` reader - Action On Chip Select In Main Mode"]
pub type CsmaR = crate::BitReader<Csma>;
impl CsmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csma {
        match self.bits {
            false => Csma::Noaction,
            true => Csma::Gotoslavemode,
        }
    }
    #[doc = "No action taken"]
    #[inline(always)]
    pub fn is_noaction(&self) -> bool {
        *self == Csma::Noaction
    }
    #[doc = "Go to secondary mode"]
    #[inline(always)]
    pub fn is_gotoslavemode(&self) -> bool {
        *self == Csma::Gotoslavemode
    }
}
#[doc = "Field `CSMA` writer - Action On Chip Select In Main Mode"]
pub type CsmaW<'a, REG> = crate::BitWriter<'a, REG, Csma>;
impl<'a, REG> CsmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action taken"]
    #[inline(always)]
    pub fn noaction(self) -> &'a mut crate::W<REG> {
        self.variant(Csma::Noaction)
    }
    #[doc = "Go to secondary mode"]
    #[inline(always)]
    pub fn gotoslavemode(self) -> &'a mut crate::W<REG> {
        self.variant(Csma::Gotoslavemode)
    }
}
#[doc = "TX Buffer Interrupt Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txbil {
    #[doc = "0: TXBL and the TXBL interrupt flag are set when the transmit buffer becomes empty. TXBL is cleared when the buffer becomes nonempty."]
    Empty = 0,
    #[doc = "1: TXBL and TXBLIF are set when the transmit buffer goes from full to half-full or empty. TXBL is cleared when the buffer becomes full."]
    Halffull = 1,
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
            true => Txbil::Halffull,
        }
    }
    #[doc = "TXBL and the TXBL interrupt flag are set when the transmit buffer becomes empty. TXBL is cleared when the buffer becomes nonempty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Txbil::Empty
    }
    #[doc = "TXBL and TXBLIF are set when the transmit buffer goes from full to half-full or empty. TXBL is cleared when the buffer becomes full."]
    #[inline(always)]
    pub fn is_halffull(&self) -> bool {
        *self == Txbil::Halffull
    }
}
#[doc = "Field `TXBIL` writer - TX Buffer Interrupt Level"]
pub type TxbilW<'a, REG> = crate::BitWriter<'a, REG, Txbil>;
impl<'a, REG> TxbilW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXBL and the TXBL interrupt flag are set when the transmit buffer becomes empty. TXBL is cleared when the buffer becomes nonempty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Txbil::Empty)
    }
    #[doc = "TXBL and TXBLIF are set when the transmit buffer goes from full to half-full or empty. TXBL is cleared when the buffer becomes full."]
    #[inline(always)]
    pub fn halffull(self) -> &'a mut crate::W<REG> {
        self.variant(Txbil::Halffull)
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
    #[doc = "0: Output from the transmitter is passed unchanged to U(S)n_TX"]
    Disable = 0,
    #[doc = "1: Output from the transmitter is inverted before it is passed to U(S)n_TX"]
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
    #[doc = "Output from the transmitter is passed unchanged to U(S)n_TX"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Txinv::Disable
    }
    #[doc = "Output from the transmitter is inverted before it is passed to U(S)n_TX"]
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
    #[doc = "Output from the transmitter is passed unchanged to U(S)n_TX"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::Disable)
    }
    #[doc = "Output from the transmitter is inverted before it is passed to U(S)n_TX"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::Enable)
    }
}
#[doc = "Chip Select Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csinv {
    #[doc = "0: Chip select is active low"]
    Disable = 0,
    #[doc = "1: Chip select is active high"]
    Enable = 1,
}
impl From<Csinv> for bool {
    #[inline(always)]
    fn from(variant: Csinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSINV` reader - Chip Select Invert"]
pub type CsinvR = crate::BitReader<Csinv>;
impl CsinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csinv {
        match self.bits {
            false => Csinv::Disable,
            true => Csinv::Enable,
        }
    }
    #[doc = "Chip select is active low"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Csinv::Disable
    }
    #[doc = "Chip select is active high"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Csinv::Enable
    }
}
#[doc = "Field `CSINV` writer - Chip Select Invert"]
pub type CsinvW<'a, REG> = crate::BitWriter<'a, REG, Csinv>;
impl<'a, REG> CsinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Chip select is active low"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Csinv::Disable)
    }
    #[doc = "Chip select is active high"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Csinv::Enable)
    }
}
#[doc = "Field `AUTOCS` reader - Automatic Chip Select"]
pub type AutocsR = crate::BitReader;
#[doc = "Field `AUTOCS` writer - Automatic Chip Select"]
pub type AutocsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Automatic TX Tristate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autotri {
    #[doc = "0: The output on U(S)n_TX when the transmitter is idle is defined by TXINV"]
    Disable = 0,
    #[doc = "1: U(S)n_TX is tristated whenever the transmitter is idle"]
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
    #[doc = "The output on U(S)n_TX when the transmitter is idle is defined by TXINV"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Autotri::Disable
    }
    #[doc = "U(S)n_TX is tristated whenever the transmitter is idle"]
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
    #[doc = "The output on U(S)n_TX when the transmitter is idle is defined by TXINV"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Autotri::Disable)
    }
    #[doc = "U(S)n_TX is tristated whenever the transmitter is idle"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Autotri::Enable)
    }
}
#[doc = "Field `SCMODE` reader - SmartCard Mode"]
pub type ScmodeR = crate::BitReader;
#[doc = "Field `SCMODE` writer - SmartCard Mode"]
pub type ScmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCRETRANS` reader - SmartCard Retransmit"]
pub type ScretransR = crate::BitReader;
#[doc = "Field `SCRETRANS` writer - SmartCard Retransmit"]
pub type ScretransW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SKIPPERRF` reader - Skip Parity Error Frames"]
pub type SkipperrfR = crate::BitReader;
#[doc = "Field `SKIPPERRF` writer - Skip Parity Error Frames"]
pub type SkipperrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT8DV` reader - Bit 8 Default Value"]
pub type Bit8dvR = crate::BitReader;
#[doc = "Field `BIT8DV` writer - Bit 8 Default Value"]
pub type Bit8dvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Halt DMA On Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errsdma {
    #[doc = "0: Framing and parity errors have no effect on DMA requests from the USART"]
    Disable = 0,
    #[doc = "1: DMA requests from the USART are blocked while the PERR or FERR interrupt flags are set"]
    Enable = 1,
}
impl From<Errsdma> for bool {
    #[inline(always)]
    fn from(variant: Errsdma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSDMA` reader - Halt DMA On Error"]
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
    #[doc = "Framing and parity errors have no effect on DMA requests from the USART"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errsdma::Disable
    }
    #[doc = "DMA requests from the USART are blocked while the PERR or FERR interrupt flags are set"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errsdma::Enable
    }
}
#[doc = "Field `ERRSDMA` writer - Halt DMA On Error"]
pub type ErrsdmaW<'a, REG> = crate::BitWriter<'a, REG, Errsdma>;
impl<'a, REG> ErrsdmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Framing and parity errors have no effect on DMA requests from the USART"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errsdma::Disable)
    }
    #[doc = "DMA requests from the USART are blocked while the PERR or FERR interrupt flags are set"]
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
#[doc = "Field `SSSEARLY` reader - Synchronous Secondary Setup Early"]
pub type SssearlyR = crate::BitReader;
#[doc = "Field `SSSEARLY` writer - Synchronous Secondary Setup Early"]
pub type SssearlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Byteswap In Double Accesses\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Byteswap {
    #[doc = "0: Normal byte order"]
    Disable = 0,
    #[doc = "1: Byte order swapped"]
    Enable = 1,
}
impl From<Byteswap> for bool {
    #[inline(always)]
    fn from(variant: Byteswap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYTESWAP` reader - Byteswap In Double Accesses"]
pub type ByteswapR = crate::BitReader<Byteswap>;
impl ByteswapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Byteswap {
        match self.bits {
            false => Byteswap::Disable,
            true => Byteswap::Enable,
        }
    }
    #[doc = "Normal byte order"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Byteswap::Disable
    }
    #[doc = "Byte order swapped"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Byteswap::Enable
    }
}
#[doc = "Field `BYTESWAP` writer - Byteswap In Double Accesses"]
pub type ByteswapW<'a, REG> = crate::BitWriter<'a, REG, Byteswap>;
impl<'a, REG> ByteswapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal byte order"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Byteswap::Disable)
    }
    #[doc = "Byte order swapped"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Byteswap::Enable)
    }
}
#[doc = "Field `AUTOTX` reader - Always Transmit When RX Not Full"]
pub type AutotxR = crate::BitReader;
#[doc = "Field `AUTOTX` writer - Always Transmit When RX Not Full"]
pub type AutotxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MVDIS` reader - Majority Vote Disable"]
pub type MvdisR = crate::BitReader;
#[doc = "Field `MVDIS` writer - Majority Vote Disable"]
pub type MvdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSDELAY` reader - Synchronous Main Sample Delay"]
pub type SmsdelayR = crate::BitReader;
#[doc = "Field `SMSDELAY` writer - Synchronous Main Sample Delay"]
pub type SmsdelayW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USART Synchronous Mode"]
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
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline(always)]
    pub fn ovs(&self) -> OvsR {
        OvsR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> ClkpolR {
        ClkpolR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock Edge For Setup/Sample"]
    #[inline(always)]
    pub fn clkpha(&self) -> ClkphaR {
        ClkphaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MsbfR {
        MsbfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Action On Chip Select In Main Mode"]
    #[inline(always)]
    pub fn csma(&self) -> CsmaR {
        CsmaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline(always)]
    pub fn txbil(&self) -> TxbilR {
        TxbilR::new(((self.bits >> 12) & 1) != 0)
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
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline(always)]
    pub fn csinv(&self) -> CsinvR {
        CsinvR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline(always)]
    pub fn autocs(&self) -> AutocsR {
        AutocsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    pub fn autotri(&self) -> AutotriR {
        AutotriR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline(always)]
    pub fn scmode(&self) -> ScmodeR {
        ScmodeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline(always)]
    pub fn scretrans(&self) -> ScretransR {
        ScretransR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    pub fn skipperrf(&self) -> SkipperrfR {
        SkipperrfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&self) -> Bit8dvR {
        Bit8dvR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Halt DMA On Error"]
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
    #[doc = "Bit 25 - Synchronous Secondary Setup Early"]
    #[inline(always)]
    pub fn sssearly(&self) -> SssearlyR {
        SssearlyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Byteswap In Double Accesses"]
    #[inline(always)]
    pub fn byteswap(&self) -> ByteswapR {
        ByteswapR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline(always)]
    pub fn autotx(&self) -> AutotxR {
        AutotxR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    pub fn mvdis(&self) -> MvdisR {
        MvdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Synchronous Main Sample Delay"]
    #[inline(always)]
    pub fn smsdelay(&self) -> SmsdelayR {
        SmsdelayR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&mut self) -> SyncW<CtrlSpec> {
        SyncW::new(self, 0)
    }
    #[doc = "Bit 1 - Loopback Enable"]
    #[inline(always)]
    pub fn loopbk(&mut self) -> LoopbkW<CtrlSpec> {
        LoopbkW::new(self, 1)
    }
    #[doc = "Bit 2 - Collision Check Enable"]
    #[inline(always)]
    pub fn ccen(&mut self) -> CcenW<CtrlSpec> {
        CcenW::new(self, 2)
    }
    #[doc = "Bit 3 - Multi-Processor Mode"]
    #[inline(always)]
    pub fn mpm(&mut self) -> MpmW<CtrlSpec> {
        MpmW::new(self, 3)
    }
    #[doc = "Bit 4 - Multi-Processor Address-Bit"]
    #[inline(always)]
    pub fn mpab(&mut self) -> MpabW<CtrlSpec> {
        MpabW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Oversampling"]
    #[inline(always)]
    pub fn ovs(&mut self) -> OvsW<CtrlSpec> {
        OvsW::new(self, 5)
    }
    #[doc = "Bit 8 - Clock Polarity"]
    #[inline(always)]
    pub fn clkpol(&mut self) -> ClkpolW<CtrlSpec> {
        ClkpolW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock Edge For Setup/Sample"]
    #[inline(always)]
    pub fn clkpha(&mut self) -> ClkphaW<CtrlSpec> {
        ClkphaW::new(self, 9)
    }
    #[doc = "Bit 10 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MsbfW<CtrlSpec> {
        MsbfW::new(self, 10)
    }
    #[doc = "Bit 11 - Action On Chip Select In Main Mode"]
    #[inline(always)]
    pub fn csma(&mut self) -> CsmaW<CtrlSpec> {
        CsmaW::new(self, 11)
    }
    #[doc = "Bit 12 - TX Buffer Interrupt Level"]
    #[inline(always)]
    pub fn txbil(&mut self) -> TxbilW<CtrlSpec> {
        TxbilW::new(self, 12)
    }
    #[doc = "Bit 13 - Receiver Input Invert"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RxinvW<CtrlSpec> {
        RxinvW::new(self, 13)
    }
    #[doc = "Bit 14 - Transmitter output Invert"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TxinvW<CtrlSpec> {
        TxinvW::new(self, 14)
    }
    #[doc = "Bit 15 - Chip Select Invert"]
    #[inline(always)]
    pub fn csinv(&mut self) -> CsinvW<CtrlSpec> {
        CsinvW::new(self, 15)
    }
    #[doc = "Bit 16 - Automatic Chip Select"]
    #[inline(always)]
    pub fn autocs(&mut self) -> AutocsW<CtrlSpec> {
        AutocsW::new(self, 16)
    }
    #[doc = "Bit 17 - Automatic TX Tristate"]
    #[inline(always)]
    pub fn autotri(&mut self) -> AutotriW<CtrlSpec> {
        AutotriW::new(self, 17)
    }
    #[doc = "Bit 18 - SmartCard Mode"]
    #[inline(always)]
    pub fn scmode(&mut self) -> ScmodeW<CtrlSpec> {
        ScmodeW::new(self, 18)
    }
    #[doc = "Bit 19 - SmartCard Retransmit"]
    #[inline(always)]
    pub fn scretrans(&mut self) -> ScretransW<CtrlSpec> {
        ScretransW::new(self, 19)
    }
    #[doc = "Bit 20 - Skip Parity Error Frames"]
    #[inline(always)]
    pub fn skipperrf(&mut self) -> SkipperrfW<CtrlSpec> {
        SkipperrfW::new(self, 20)
    }
    #[doc = "Bit 21 - Bit 8 Default Value"]
    #[inline(always)]
    pub fn bit8dv(&mut self) -> Bit8dvW<CtrlSpec> {
        Bit8dvW::new(self, 21)
    }
    #[doc = "Bit 22 - Halt DMA On Error"]
    #[inline(always)]
    pub fn errsdma(&mut self) -> ErrsdmaW<CtrlSpec> {
        ErrsdmaW::new(self, 22)
    }
    #[doc = "Bit 23 - Disable RX On Error"]
    #[inline(always)]
    pub fn errsrx(&mut self) -> ErrsrxW<CtrlSpec> {
        ErrsrxW::new(self, 23)
    }
    #[doc = "Bit 24 - Disable TX On Error"]
    #[inline(always)]
    pub fn errstx(&mut self) -> ErrstxW<CtrlSpec> {
        ErrstxW::new(self, 24)
    }
    #[doc = "Bit 25 - Synchronous Secondary Setup Early"]
    #[inline(always)]
    pub fn sssearly(&mut self) -> SssearlyW<CtrlSpec> {
        SssearlyW::new(self, 25)
    }
    #[doc = "Bit 28 - Byteswap In Double Accesses"]
    #[inline(always)]
    pub fn byteswap(&mut self) -> ByteswapW<CtrlSpec> {
        ByteswapW::new(self, 28)
    }
    #[doc = "Bit 29 - Always Transmit When RX Not Full"]
    #[inline(always)]
    pub fn autotx(&mut self) -> AutotxW<CtrlSpec> {
        AutotxW::new(self, 29)
    }
    #[doc = "Bit 30 - Majority Vote Disable"]
    #[inline(always)]
    pub fn mvdis(&mut self) -> MvdisW<CtrlSpec> {
        MvdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Synchronous Main Sample Delay"]
    #[inline(always)]
    pub fn smsdelay(&mut self) -> SmsdelayW<CtrlSpec> {
        SmsdelayW::new(self, 31)
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
