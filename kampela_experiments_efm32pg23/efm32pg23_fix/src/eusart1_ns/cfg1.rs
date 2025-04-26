#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Debug halt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbghalt {
    #[doc = "0: Continue normal EUSART operation even if core is halted"]
    Disable = 0,
    #[doc = "1: If core is halted, receive one frame and then halt reception by deactivating RTS. Next frame reception happens when the core is unhalted during single stepping."]
    Enable = 1,
}
impl From<Dbghalt> for bool {
    #[inline(always)]
    fn from(variant: Dbghalt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGHALT` reader - Debug halt"]
pub type DbghaltR = crate::BitReader<Dbghalt>;
impl DbghaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbghalt {
        match self.bits {
            false => Dbghalt::Disable,
            true => Dbghalt::Enable,
        }
    }
    #[doc = "Continue normal EUSART operation even if core is halted"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dbghalt::Disable
    }
    #[doc = "If core is halted, receive one frame and then halt reception by deactivating RTS. Next frame reception happens when the core is unhalted during single stepping."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dbghalt::Enable
    }
}
#[doc = "Field `DBGHALT` writer - Debug halt"]
pub type DbghaltW<'a, REG> = crate::BitWriter<'a, REG, Dbghalt>;
impl<'a, REG> DbghaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue normal EUSART operation even if core is halted"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dbghalt::Disable)
    }
    #[doc = "If core is halted, receive one frame and then halt reception by deactivating RTS. Next frame reception happens when the core is unhalted during single stepping."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dbghalt::Enable)
    }
}
#[doc = "Clear-to-send Invert Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsinv {
    #[doc = "0: The CTS pin is active low"]
    Disable = 0,
    #[doc = "1: The CTS pin is active high"]
    Enable = 1,
}
impl From<Ctsinv> for bool {
    #[inline(always)]
    fn from(variant: Ctsinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSINV` reader - Clear-to-send Invert Enable"]
pub type CtsinvR = crate::BitReader<Ctsinv>;
impl CtsinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsinv {
        match self.bits {
            false => Ctsinv::Disable,
            true => Ctsinv::Enable,
        }
    }
    #[doc = "The CTS pin is active low"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ctsinv::Disable
    }
    #[doc = "The CTS pin is active high"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ctsinv::Enable
    }
}
#[doc = "Field `CTSINV` writer - Clear-to-send Invert Enable"]
pub type CtsinvW<'a, REG> = crate::BitWriter<'a, REG, Ctsinv>;
impl<'a, REG> CtsinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CTS pin is active low"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsinv::Disable)
    }
    #[doc = "The CTS pin is active high"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsinv::Enable)
    }
}
#[doc = "Clear-to-send Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsen {
    #[doc = "0: Ignore CTS"]
    Disable = 0,
    #[doc = "1: Stop transmitting when CTS is inactive"]
    Enable = 1,
}
impl From<Ctsen> for bool {
    #[inline(always)]
    fn from(variant: Ctsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` reader - Clear-to-send Enable"]
pub type CtsenR = crate::BitReader<Ctsen>;
impl CtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsen {
        match self.bits {
            false => Ctsen::Disable,
            true => Ctsen::Enable,
        }
    }
    #[doc = "Ignore CTS"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ctsen::Disable
    }
    #[doc = "Stop transmitting when CTS is inactive"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ctsen::Enable
    }
}
#[doc = "Field `CTSEN` writer - Clear-to-send Enable"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG, Ctsen>;
impl<'a, REG> CtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore CTS"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Disable)
    }
    #[doc = "Stop transmitting when CTS is inactive"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Enable)
    }
}
#[doc = "Request-to-send Invert Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtsinv {
    #[doc = "0: The RTS pin is active low"]
    Disable = 0,
    #[doc = "1: The RTS pin is active high"]
    Enable = 1,
}
impl From<Rtsinv> for bool {
    #[inline(always)]
    fn from(variant: Rtsinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSINV` reader - Request-to-send Invert Enable"]
pub type RtsinvR = crate::BitReader<Rtsinv>;
impl RtsinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtsinv {
        match self.bits {
            false => Rtsinv::Disable,
            true => Rtsinv::Enable,
        }
    }
    #[doc = "The RTS pin is active low"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rtsinv::Disable
    }
    #[doc = "The RTS pin is active high"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtsinv::Enable
    }
}
#[doc = "Field `RTSINV` writer - Request-to-send Invert Enable"]
pub type RtsinvW<'a, REG> = crate::BitWriter<'a, REG, Rtsinv>;
impl<'a, REG> RtsinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RTS pin is active low"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsinv::Disable)
    }
    #[doc = "The RTS pin is active high"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsinv::Enable)
    }
}
#[doc = "RX Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxtimeout {
    #[doc = "0: DISABLED"]
    Disabled = 0,
    #[doc = "1: ONEFRAME"]
    Oneframe = 1,
    #[doc = "2: TWOFRAMES"]
    Twoframes = 2,
    #[doc = "3: THREEFRAMES"]
    Threeframes = 3,
    #[doc = "4: FOURFRAMES"]
    Fourframes = 4,
    #[doc = "5: FIVEFRAMES"]
    Fiveframes = 5,
    #[doc = "6: SIXFRAMES"]
    Sixframes = 6,
    #[doc = "7: SEVENFRAMES"]
    Sevenframes = 7,
}
impl From<Rxtimeout> for u8 {
    #[inline(always)]
    fn from(variant: Rxtimeout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxtimeout {
    type Ux = u8;
}
impl crate::IsEnum for Rxtimeout {}
#[doc = "Field `RXTIMEOUT` reader - RX Timeout"]
pub type RxtimeoutR = crate::FieldReader<Rxtimeout>;
impl RxtimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxtimeout {
        match self.bits {
            0 => Rxtimeout::Disabled,
            1 => Rxtimeout::Oneframe,
            2 => Rxtimeout::Twoframes,
            3 => Rxtimeout::Threeframes,
            4 => Rxtimeout::Fourframes,
            5 => Rxtimeout::Fiveframes,
            6 => Rxtimeout::Sixframes,
            7 => Rxtimeout::Sevenframes,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxtimeout::Disabled
    }
    #[doc = "ONEFRAME"]
    #[inline(always)]
    pub fn is_oneframe(&self) -> bool {
        *self == Rxtimeout::Oneframe
    }
    #[doc = "TWOFRAMES"]
    #[inline(always)]
    pub fn is_twoframes(&self) -> bool {
        *self == Rxtimeout::Twoframes
    }
    #[doc = "THREEFRAMES"]
    #[inline(always)]
    pub fn is_threeframes(&self) -> bool {
        *self == Rxtimeout::Threeframes
    }
    #[doc = "FOURFRAMES"]
    #[inline(always)]
    pub fn is_fourframes(&self) -> bool {
        *self == Rxtimeout::Fourframes
    }
    #[doc = "FIVEFRAMES"]
    #[inline(always)]
    pub fn is_fiveframes(&self) -> bool {
        *self == Rxtimeout::Fiveframes
    }
    #[doc = "SIXFRAMES"]
    #[inline(always)]
    pub fn is_sixframes(&self) -> bool {
        *self == Rxtimeout::Sixframes
    }
    #[doc = "SEVENFRAMES"]
    #[inline(always)]
    pub fn is_sevenframes(&self) -> bool {
        *self == Rxtimeout::Sevenframes
    }
}
#[doc = "Field `RXTIMEOUT` writer - RX Timeout"]
pub type RxtimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rxtimeout, crate::Safe>;
impl<'a, REG> RxtimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtimeout::Disabled)
    }
    #[doc = "ONEFRAME"]
    #[inline(always)]
    pub fn oneframe(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtimeout::Oneframe)
    }
    #[doc = "TWOFRAMES"]
    #[inline(always)]
    pub fn twoframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtimeout::Twoframes)
    }
    #[doc = "THREEFRAMES"]
    #[inline(always)]
    pub fn threeframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtimeout::Threeframes)
    }
    #[doc = "FOURFRAMES"]
    #[inline(always)]
    pub fn fourframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtimeout::Fourframes)
    }
    #[doc = "FIVEFRAMES"]
    #[inline(always)]
    pub fn fiveframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtimeout::Fiveframes)
    }
    #[doc = "SIXFRAMES"]
    #[inline(always)]
    pub fn sixframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtimeout::Sixframes)
    }
    #[doc = "SEVENFRAMES"]
    #[inline(always)]
    pub fn sevenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtimeout::Sevenframes)
    }
}
#[doc = "Field `SFUBRX` reader - Start Frame Unblock Receiver"]
pub type SfubrxR = crate::BitReader;
#[doc = "Field `SFUBRX` writer - Start Frame Unblock Receiver"]
pub type SfubrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPRSEN` reader - PRS RX Enable"]
pub type RxprsenR = crate::BitReader;
#[doc = "Field `RXPRSEN` writer - PRS RX Enable"]
pub type RxprsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TX FIFO Interrupt Watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txfiw {
    #[doc = "0: TXFL status flag and IF are set when the TX FIFO has space for at least one more frame."]
    Oneframe = 0,
    #[doc = "1: TXFL status flag and IF are set when the TX FIFO has space for at least two more frames."]
    Twoframes = 1,
    #[doc = "2: TXFL status flag and IF are set when the TX FIFO has space for at least three more frames."]
    Threeframes = 2,
    #[doc = "3: TXFL status flag and IF are set when the TX FIFO has space for at least four more frames."]
    Fourframes = 3,
    #[doc = "4: TXFL status flag and IF are set when the TX FIFO has space for at least five more frames."]
    Fiveframes = 4,
    #[doc = "5: TXFL status flag and IF are set when the TX FIFO has space for at least six more frames."]
    Sixframes = 5,
    #[doc = "6: TXFL status flag and IF are set when the TX FIFO has space for at least seven more frames."]
    Sevenframes = 6,
    #[doc = "7: TXFL status flag and IF are set when the TX FIFO has space for at least eight more frames."]
    Eightframes = 7,
    #[doc = "8: TXFL status flag and IF are set when the TX FIFO has space for at least nine more frames."]
    Nineframes = 8,
    #[doc = "9: TXFL status flag and IF are set when the TX FIFO has space for at least ten more frames."]
    Tenframes = 9,
    #[doc = "10: TXFL status flag and IF are set when the TX FIFO has space for at least eleven more frames."]
    Elevenframes = 10,
    #[doc = "11: TXFL status flag and IF are set when the TX FIFO has space for at least twelve more frames."]
    Twelveframes = 11,
    #[doc = "12: TXFL status flag and IF are set when the TX FIFO has space for at least thriteen more frames."]
    Thirteenframes = 12,
    #[doc = "13: TXFL status flag and IF are set when the TX FIFO has space for at least fourteen more frames."]
    Fourteenframes = 13,
    #[doc = "14: TXFL status flag and IF are set when the TX FIFO has space for at least fifteen more frames."]
    Fifteenframes = 14,
    #[doc = "15: TXFL status flag and IF are set when the TX FIFO has space for at least sixteen more frames."]
    Sixteenframes = 15,
}
impl From<Txfiw> for u8 {
    #[inline(always)]
    fn from(variant: Txfiw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txfiw {
    type Ux = u8;
}
impl crate::IsEnum for Txfiw {}
#[doc = "Field `TXFIW` reader - TX FIFO Interrupt Watermark"]
pub type TxfiwR = crate::FieldReader<Txfiw>;
impl TxfiwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txfiw {
        match self.bits {
            0 => Txfiw::Oneframe,
            1 => Txfiw::Twoframes,
            2 => Txfiw::Threeframes,
            3 => Txfiw::Fourframes,
            4 => Txfiw::Fiveframes,
            5 => Txfiw::Sixframes,
            6 => Txfiw::Sevenframes,
            7 => Txfiw::Eightframes,
            8 => Txfiw::Nineframes,
            9 => Txfiw::Tenframes,
            10 => Txfiw::Elevenframes,
            11 => Txfiw::Twelveframes,
            12 => Txfiw::Thirteenframes,
            13 => Txfiw::Fourteenframes,
            14 => Txfiw::Fifteenframes,
            15 => Txfiw::Sixteenframes,
            _ => unreachable!(),
        }
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least one more frame."]
    #[inline(always)]
    pub fn is_oneframe(&self) -> bool {
        *self == Txfiw::Oneframe
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least two more frames."]
    #[inline(always)]
    pub fn is_twoframes(&self) -> bool {
        *self == Txfiw::Twoframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least three more frames."]
    #[inline(always)]
    pub fn is_threeframes(&self) -> bool {
        *self == Txfiw::Threeframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least four more frames."]
    #[inline(always)]
    pub fn is_fourframes(&self) -> bool {
        *self == Txfiw::Fourframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least five more frames."]
    #[inline(always)]
    pub fn is_fiveframes(&self) -> bool {
        *self == Txfiw::Fiveframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least six more frames."]
    #[inline(always)]
    pub fn is_sixframes(&self) -> bool {
        *self == Txfiw::Sixframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least seven more frames."]
    #[inline(always)]
    pub fn is_sevenframes(&self) -> bool {
        *self == Txfiw::Sevenframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least eight more frames."]
    #[inline(always)]
    pub fn is_eightframes(&self) -> bool {
        *self == Txfiw::Eightframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least nine more frames."]
    #[inline(always)]
    pub fn is_nineframes(&self) -> bool {
        *self == Txfiw::Nineframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least ten more frames."]
    #[inline(always)]
    pub fn is_tenframes(&self) -> bool {
        *self == Txfiw::Tenframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least eleven more frames."]
    #[inline(always)]
    pub fn is_elevenframes(&self) -> bool {
        *self == Txfiw::Elevenframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least twelve more frames."]
    #[inline(always)]
    pub fn is_twelveframes(&self) -> bool {
        *self == Txfiw::Twelveframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least thriteen more frames."]
    #[inline(always)]
    pub fn is_thirteenframes(&self) -> bool {
        *self == Txfiw::Thirteenframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least fourteen more frames."]
    #[inline(always)]
    pub fn is_fourteenframes(&self) -> bool {
        *self == Txfiw::Fourteenframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least fifteen more frames."]
    #[inline(always)]
    pub fn is_fifteenframes(&self) -> bool {
        *self == Txfiw::Fifteenframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least sixteen more frames."]
    #[inline(always)]
    pub fn is_sixteenframes(&self) -> bool {
        *self == Txfiw::Sixteenframes
    }
}
#[doc = "Field `TXFIW` writer - TX FIFO Interrupt Watermark"]
pub type TxfiwW<'a, REG> = crate::FieldWriter<'a, REG, 4, Txfiw, crate::Safe>;
impl<'a, REG> TxfiwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least one more frame."]
    #[inline(always)]
    pub fn oneframe(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Oneframe)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least two more frames."]
    #[inline(always)]
    pub fn twoframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Twoframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least three more frames."]
    #[inline(always)]
    pub fn threeframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Threeframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least four more frames."]
    #[inline(always)]
    pub fn fourframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Fourframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least five more frames."]
    #[inline(always)]
    pub fn fiveframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Fiveframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least six more frames."]
    #[inline(always)]
    pub fn sixframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Sixframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least seven more frames."]
    #[inline(always)]
    pub fn sevenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Sevenframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least eight more frames."]
    #[inline(always)]
    pub fn eightframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Eightframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least nine more frames."]
    #[inline(always)]
    pub fn nineframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Nineframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least ten more frames."]
    #[inline(always)]
    pub fn tenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Tenframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least eleven more frames."]
    #[inline(always)]
    pub fn elevenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Elevenframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least twelve more frames."]
    #[inline(always)]
    pub fn twelveframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Twelveframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least thriteen more frames."]
    #[inline(always)]
    pub fn thirteenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Thirteenframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least fourteen more frames."]
    #[inline(always)]
    pub fn fourteenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Fourteenframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least fifteen more frames."]
    #[inline(always)]
    pub fn fifteenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Fifteenframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least sixteen more frames."]
    #[inline(always)]
    pub fn sixteenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Sixteenframes)
    }
}
#[doc = "Request-to-send RX FIFO Watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtsrxfw {
    #[doc = "0: RTS is set if there is space for at least one more frame in the RX FIFO."]
    Oneframe = 0,
    #[doc = "1: RTS is set if there is space for at least two more frames in the RX FIFO."]
    Twoframes = 1,
    #[doc = "2: RTS is set if there is space for at least three more frames in the RX FIFO."]
    Threeframes = 2,
    #[doc = "3: RTS is set if there is space for four more frames in the RX FIFO."]
    Fourframes = 3,
    #[doc = "4: RTS is set if there is space for five more frames in the RX FIFO."]
    Fiveframes = 4,
    #[doc = "5: RTS is set if there is space for six more frames in the RX FIFO."]
    Sixframes = 5,
    #[doc = "6: RTS is set if there is space for seven more frames in the RX FIFO."]
    Sevenframes = 6,
    #[doc = "7: RTS is set if there is space for eight more frames in the RX FIFO."]
    Eightframes = 7,
    #[doc = "8: RTS is set if there is space for nine more frames in the RX FIFO."]
    Nineframes = 8,
    #[doc = "9: RTS is set if there is space for ten more frames in the RX FIFO."]
    Tenframes = 9,
    #[doc = "10: RTS is set if there is space for eleven more frames in the RX FIFO."]
    Elevenframes = 10,
    #[doc = "11: RTS is set if there is space for twelve more frames in the RX FIFO."]
    Twelveframes = 11,
    #[doc = "12: RTS is set if there is space for thirteen more frames in the RX FIFO."]
    Thirteenframes = 12,
    #[doc = "13: RTS is set if there is space for fourteen more frames in the RX FIFO."]
    Fourteenframes = 13,
    #[doc = "14: RTS is set if there is space for fifteen more frames in the RX FIFO."]
    Fifteenframes = 14,
    #[doc = "15: RTS is set if there is space for sixteen more frames in the RX FIFO."]
    Sixteenframes = 15,
}
impl From<Rtsrxfw> for u8 {
    #[inline(always)]
    fn from(variant: Rtsrxfw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtsrxfw {
    type Ux = u8;
}
impl crate::IsEnum for Rtsrxfw {}
#[doc = "Field `RTSRXFW` reader - Request-to-send RX FIFO Watermark"]
pub type RtsrxfwR = crate::FieldReader<Rtsrxfw>;
impl RtsrxfwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtsrxfw {
        match self.bits {
            0 => Rtsrxfw::Oneframe,
            1 => Rtsrxfw::Twoframes,
            2 => Rtsrxfw::Threeframes,
            3 => Rtsrxfw::Fourframes,
            4 => Rtsrxfw::Fiveframes,
            5 => Rtsrxfw::Sixframes,
            6 => Rtsrxfw::Sevenframes,
            7 => Rtsrxfw::Eightframes,
            8 => Rtsrxfw::Nineframes,
            9 => Rtsrxfw::Tenframes,
            10 => Rtsrxfw::Elevenframes,
            11 => Rtsrxfw::Twelveframes,
            12 => Rtsrxfw::Thirteenframes,
            13 => Rtsrxfw::Fourteenframes,
            14 => Rtsrxfw::Fifteenframes,
            15 => Rtsrxfw::Sixteenframes,
            _ => unreachable!(),
        }
    }
    #[doc = "RTS is set if there is space for at least one more frame in the RX FIFO."]
    #[inline(always)]
    pub fn is_oneframe(&self) -> bool {
        *self == Rtsrxfw::Oneframe
    }
    #[doc = "RTS is set if there is space for at least two more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_twoframes(&self) -> bool {
        *self == Rtsrxfw::Twoframes
    }
    #[doc = "RTS is set if there is space for at least three more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_threeframes(&self) -> bool {
        *self == Rtsrxfw::Threeframes
    }
    #[doc = "RTS is set if there is space for four more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_fourframes(&self) -> bool {
        *self == Rtsrxfw::Fourframes
    }
    #[doc = "RTS is set if there is space for five more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_fiveframes(&self) -> bool {
        *self == Rtsrxfw::Fiveframes
    }
    #[doc = "RTS is set if there is space for six more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_sixframes(&self) -> bool {
        *self == Rtsrxfw::Sixframes
    }
    #[doc = "RTS is set if there is space for seven more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_sevenframes(&self) -> bool {
        *self == Rtsrxfw::Sevenframes
    }
    #[doc = "RTS is set if there is space for eight more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_eightframes(&self) -> bool {
        *self == Rtsrxfw::Eightframes
    }
    #[doc = "RTS is set if there is space for nine more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_nineframes(&self) -> bool {
        *self == Rtsrxfw::Nineframes
    }
    #[doc = "RTS is set if there is space for ten more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_tenframes(&self) -> bool {
        *self == Rtsrxfw::Tenframes
    }
    #[doc = "RTS is set if there is space for eleven more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_elevenframes(&self) -> bool {
        *self == Rtsrxfw::Elevenframes
    }
    #[doc = "RTS is set if there is space for twelve more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_twelveframes(&self) -> bool {
        *self == Rtsrxfw::Twelveframes
    }
    #[doc = "RTS is set if there is space for thirteen more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_thirteenframes(&self) -> bool {
        *self == Rtsrxfw::Thirteenframes
    }
    #[doc = "RTS is set if there is space for fourteen more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_fourteenframes(&self) -> bool {
        *self == Rtsrxfw::Fourteenframes
    }
    #[doc = "RTS is set if there is space for fifteen more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_fifteenframes(&self) -> bool {
        *self == Rtsrxfw::Fifteenframes
    }
    #[doc = "RTS is set if there is space for sixteen more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_sixteenframes(&self) -> bool {
        *self == Rtsrxfw::Sixteenframes
    }
}
#[doc = "Field `RTSRXFW` writer - Request-to-send RX FIFO Watermark"]
pub type RtsrxfwW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rtsrxfw, crate::Safe>;
impl<'a, REG> RtsrxfwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTS is set if there is space for at least one more frame in the RX FIFO."]
    #[inline(always)]
    pub fn oneframe(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Oneframe)
    }
    #[doc = "RTS is set if there is space for at least two more frames in the RX FIFO."]
    #[inline(always)]
    pub fn twoframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Twoframes)
    }
    #[doc = "RTS is set if there is space for at least three more frames in the RX FIFO."]
    #[inline(always)]
    pub fn threeframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Threeframes)
    }
    #[doc = "RTS is set if there is space for four more frames in the RX FIFO."]
    #[inline(always)]
    pub fn fourframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Fourframes)
    }
    #[doc = "RTS is set if there is space for five more frames in the RX FIFO."]
    #[inline(always)]
    pub fn fiveframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Fiveframes)
    }
    #[doc = "RTS is set if there is space for six more frames in the RX FIFO."]
    #[inline(always)]
    pub fn sixframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Sixframes)
    }
    #[doc = "RTS is set if there is space for seven more frames in the RX FIFO."]
    #[inline(always)]
    pub fn sevenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Sevenframes)
    }
    #[doc = "RTS is set if there is space for eight more frames in the RX FIFO."]
    #[inline(always)]
    pub fn eightframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Eightframes)
    }
    #[doc = "RTS is set if there is space for nine more frames in the RX FIFO."]
    #[inline(always)]
    pub fn nineframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Nineframes)
    }
    #[doc = "RTS is set if there is space for ten more frames in the RX FIFO."]
    #[inline(always)]
    pub fn tenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Tenframes)
    }
    #[doc = "RTS is set if there is space for eleven more frames in the RX FIFO."]
    #[inline(always)]
    pub fn elevenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Elevenframes)
    }
    #[doc = "RTS is set if there is space for twelve more frames in the RX FIFO."]
    #[inline(always)]
    pub fn twelveframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Twelveframes)
    }
    #[doc = "RTS is set if there is space for thirteen more frames in the RX FIFO."]
    #[inline(always)]
    pub fn thirteenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Thirteenframes)
    }
    #[doc = "RTS is set if there is space for fourteen more frames in the RX FIFO."]
    #[inline(always)]
    pub fn fourteenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Fourteenframes)
    }
    #[doc = "RTS is set if there is space for fifteen more frames in the RX FIFO."]
    #[inline(always)]
    pub fn fifteenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Fifteenframes)
    }
    #[doc = "RTS is set if there is space for sixteen more frames in the RX FIFO."]
    #[inline(always)]
    pub fn sixteenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Sixteenframes)
    }
}
#[doc = "RX FIFO Interrupt Watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxfiw {
    #[doc = "0: RXFL status flag and IF are set when the RX FIFO has at least one frame in it."]
    Oneframe = 0,
    #[doc = "1: RXFL status flag and IF are set when the RX FIFO has at least two frames in it."]
    Twoframes = 1,
    #[doc = "2: RXFL status flag and IF are set when the RX FIFO has at least three frames in it."]
    Threeframes = 2,
    #[doc = "3: RXFL status flag and IF are set when the RX FIFO has at least four frames in it."]
    Fourframes = 3,
    #[doc = "4: RXFL status flag and IF are set when the RX FIFO has at least five frames in it."]
    Fiveframes = 4,
    #[doc = "5: RXFL status flag and IF are set when the RX FIFO has at least six frames in it."]
    Sixframes = 5,
    #[doc = "6: RXFL status flag and IF are set when the RX FIFO has at least seven frames in it."]
    Sevenframes = 6,
    #[doc = "7: RXFL status flag and IF are set when the RX FIFO has at least eight frames in it."]
    Eightframes = 7,
    #[doc = "8: RXFL status flag and IF are set when the RX FIFO has at least nine frames in it."]
    Nineframes = 8,
    #[doc = "9: RXFL status flag and IF are set when the RX FIFO has at least ten frames in it."]
    Tenframes = 9,
    #[doc = "10: RXFL status flag and IF are set when the RX FIFO has at least eleven frames in it."]
    Elevenframes = 10,
    #[doc = "11: RXFL status flag and IF are set when the RX FIFO has at least twelve frames in it."]
    Twelveframes = 11,
    #[doc = "12: RXFL status flag and IF are set when the RX FIFO has at least thriteen frames in it."]
    Thirteenframes = 12,
    #[doc = "13: RXFL status flag and IF are set when the RX FIFO has at least fourteen frames in it."]
    Fourteenframes = 13,
    #[doc = "14: RXFL status flag and IF are set when the RX FIFO has at least fifteen frames in it."]
    Fifteenframes = 14,
    #[doc = "15: RXFL status flag and IF are set when the RX FIFO has at least sixteen frames in it."]
    Sixteenframes = 15,
}
impl From<Rxfiw> for u8 {
    #[inline(always)]
    fn from(variant: Rxfiw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxfiw {
    type Ux = u8;
}
impl crate::IsEnum for Rxfiw {}
#[doc = "Field `RXFIW` reader - RX FIFO Interrupt Watermark"]
pub type RxfiwR = crate::FieldReader<Rxfiw>;
impl RxfiwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfiw {
        match self.bits {
            0 => Rxfiw::Oneframe,
            1 => Rxfiw::Twoframes,
            2 => Rxfiw::Threeframes,
            3 => Rxfiw::Fourframes,
            4 => Rxfiw::Fiveframes,
            5 => Rxfiw::Sixframes,
            6 => Rxfiw::Sevenframes,
            7 => Rxfiw::Eightframes,
            8 => Rxfiw::Nineframes,
            9 => Rxfiw::Tenframes,
            10 => Rxfiw::Elevenframes,
            11 => Rxfiw::Twelveframes,
            12 => Rxfiw::Thirteenframes,
            13 => Rxfiw::Fourteenframes,
            14 => Rxfiw::Fifteenframes,
            15 => Rxfiw::Sixteenframes,
            _ => unreachable!(),
        }
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least one frame in it."]
    #[inline(always)]
    pub fn is_oneframe(&self) -> bool {
        *self == Rxfiw::Oneframe
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least two frames in it."]
    #[inline(always)]
    pub fn is_twoframes(&self) -> bool {
        *self == Rxfiw::Twoframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least three frames in it."]
    #[inline(always)]
    pub fn is_threeframes(&self) -> bool {
        *self == Rxfiw::Threeframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least four frames in it."]
    #[inline(always)]
    pub fn is_fourframes(&self) -> bool {
        *self == Rxfiw::Fourframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least five frames in it."]
    #[inline(always)]
    pub fn is_fiveframes(&self) -> bool {
        *self == Rxfiw::Fiveframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least six frames in it."]
    #[inline(always)]
    pub fn is_sixframes(&self) -> bool {
        *self == Rxfiw::Sixframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least seven frames in it."]
    #[inline(always)]
    pub fn is_sevenframes(&self) -> bool {
        *self == Rxfiw::Sevenframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least eight frames in it."]
    #[inline(always)]
    pub fn is_eightframes(&self) -> bool {
        *self == Rxfiw::Eightframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least nine frames in it."]
    #[inline(always)]
    pub fn is_nineframes(&self) -> bool {
        *self == Rxfiw::Nineframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least ten frames in it."]
    #[inline(always)]
    pub fn is_tenframes(&self) -> bool {
        *self == Rxfiw::Tenframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least eleven frames in it."]
    #[inline(always)]
    pub fn is_elevenframes(&self) -> bool {
        *self == Rxfiw::Elevenframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least twelve frames in it."]
    #[inline(always)]
    pub fn is_twelveframes(&self) -> bool {
        *self == Rxfiw::Twelveframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least thriteen frames in it."]
    #[inline(always)]
    pub fn is_thirteenframes(&self) -> bool {
        *self == Rxfiw::Thirteenframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least fourteen frames in it."]
    #[inline(always)]
    pub fn is_fourteenframes(&self) -> bool {
        *self == Rxfiw::Fourteenframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least fifteen frames in it."]
    #[inline(always)]
    pub fn is_fifteenframes(&self) -> bool {
        *self == Rxfiw::Fifteenframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least sixteen frames in it."]
    #[inline(always)]
    pub fn is_sixteenframes(&self) -> bool {
        *self == Rxfiw::Sixteenframes
    }
}
#[doc = "Field `RXFIW` writer - RX FIFO Interrupt Watermark"]
pub type RxfiwW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rxfiw, crate::Safe>;
impl<'a, REG> RxfiwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least one frame in it."]
    #[inline(always)]
    pub fn oneframe(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Oneframe)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least two frames in it."]
    #[inline(always)]
    pub fn twoframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Twoframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least three frames in it."]
    #[inline(always)]
    pub fn threeframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Threeframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least four frames in it."]
    #[inline(always)]
    pub fn fourframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Fourframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least five frames in it."]
    #[inline(always)]
    pub fn fiveframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Fiveframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least six frames in it."]
    #[inline(always)]
    pub fn sixframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Sixframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least seven frames in it."]
    #[inline(always)]
    pub fn sevenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Sevenframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least eight frames in it."]
    #[inline(always)]
    pub fn eightframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Eightframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least nine frames in it."]
    #[inline(always)]
    pub fn nineframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Nineframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least ten frames in it."]
    #[inline(always)]
    pub fn tenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Tenframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least eleven frames in it."]
    #[inline(always)]
    pub fn elevenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Elevenframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least twelve frames in it."]
    #[inline(always)]
    pub fn twelveframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Twelveframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least thriteen frames in it."]
    #[inline(always)]
    pub fn thirteenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Thirteenframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least fourteen frames in it."]
    #[inline(always)]
    pub fn fourteenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Fourteenframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least fifteen frames in it."]
    #[inline(always)]
    pub fn fifteenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Fifteenframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least sixteen frames in it."]
    #[inline(always)]
    pub fn sixteenframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Sixteenframes)
    }
}
impl R {
    #[doc = "Bit 0 - Debug halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DbghaltR {
        DbghaltR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear-to-send Invert Enable"]
    #[inline(always)]
    pub fn ctsinv(&self) -> CtsinvR {
        CtsinvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear-to-send Enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Request-to-send Invert Enable"]
    #[inline(always)]
    pub fn rtsinv(&self) -> RtsinvR {
        RtsinvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - RX Timeout"]
    #[inline(always)]
    pub fn rxtimeout(&self) -> RxtimeoutR {
        RxtimeoutR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11 - Start Frame Unblock Receiver"]
    #[inline(always)]
    pub fn sfubrx(&self) -> SfubrxR {
        SfubrxR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprsen(&self) -> RxprsenR {
        RxprsenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - TX FIFO Interrupt Watermark"]
    #[inline(always)]
    pub fn txfiw(&self) -> TxfiwR {
        TxfiwR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - Request-to-send RX FIFO Watermark"]
    #[inline(always)]
    pub fn rtsrxfw(&self) -> RtsrxfwR {
        RtsrxfwR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 27:30 - RX FIFO Interrupt Watermark"]
    #[inline(always)]
    pub fn rxfiw(&self) -> RxfiwR {
        RxfiwR::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Debug halt"]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DbghaltW<Cfg1Spec> {
        DbghaltW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear-to-send Invert Enable"]
    #[inline(always)]
    pub fn ctsinv(&mut self) -> CtsinvW<Cfg1Spec> {
        CtsinvW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear-to-send Enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CtsenW<Cfg1Spec> {
        CtsenW::new(self, 2)
    }
    #[doc = "Bit 3 - Request-to-send Invert Enable"]
    #[inline(always)]
    pub fn rtsinv(&mut self) -> RtsinvW<Cfg1Spec> {
        RtsinvW::new(self, 3)
    }
    #[doc = "Bits 4:6 - RX Timeout"]
    #[inline(always)]
    pub fn rxtimeout(&mut self) -> RxtimeoutW<Cfg1Spec> {
        RxtimeoutW::new(self, 4)
    }
    #[doc = "Bit 11 - Start Frame Unblock Receiver"]
    #[inline(always)]
    pub fn sfubrx(&mut self) -> SfubrxW<Cfg1Spec> {
        SfubrxW::new(self, 11)
    }
    #[doc = "Bit 15 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprsen(&mut self) -> RxprsenW<Cfg1Spec> {
        RxprsenW::new(self, 15)
    }
    #[doc = "Bits 16:19 - TX FIFO Interrupt Watermark"]
    #[inline(always)]
    pub fn txfiw(&mut self) -> TxfiwW<Cfg1Spec> {
        TxfiwW::new(self, 16)
    }
    #[doc = "Bits 22:25 - Request-to-send RX FIFO Watermark"]
    #[inline(always)]
    pub fn rtsrxfw(&mut self) -> RtsrxfwW<Cfg1Spec> {
        RtsrxfwW::new(self, 22)
    }
    #[doc = "Bits 27:30 - RX FIFO Interrupt Watermark"]
    #[inline(always)]
    pub fn rxfiw(&mut self) -> RxfiwW<Cfg1Spec> {
        RxfiwW::new(self, 27)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {}
