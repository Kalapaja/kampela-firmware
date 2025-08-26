#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Differential Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diff {
    #[doc = "0: Single ended output"]
    Singleended = 0,
    #[doc = "1: Differential output"]
    Differential = 1,
}
impl From<Diff> for bool {
    #[inline(always)]
    fn from(variant: Diff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFF` reader - Differential Mode"]
pub type DiffR = crate::BitReader<Diff>;
impl DiffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diff {
        match self.bits {
            false => Diff::Singleended,
            true => Diff::Differential,
        }
    }
    #[doc = "Single ended output"]
    #[inline(always)]
    pub fn is_singleended(&self) -> bool {
        *self == Diff::Singleended
    }
    #[doc = "Differential output"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == Diff::Differential
    }
}
#[doc = "Field `DIFF` writer - Differential Mode"]
pub type DiffW<'a, REG> = crate::BitWriter<'a, REG, Diff>;
impl<'a, REG> DiffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single ended output"]
    #[inline(always)]
    pub fn singleended(self) -> &'a mut crate::W<REG> {
        self.variant(Diff::Singleended)
    }
    #[doc = "Differential output"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut crate::W<REG> {
        self.variant(Diff::Differential)
    }
}
#[doc = "Sine Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sinemode {
    #[doc = "0: Sine mode disabled. Sine reset to 0 degrees"]
    Dissinemode = 0,
    #[doc = "1: Sine mode enabled"]
    Ensinemode = 1,
}
impl From<Sinemode> for bool {
    #[inline(always)]
    fn from(variant: Sinemode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINEMODE` reader - Sine Mode"]
pub type SinemodeR = crate::BitReader<Sinemode>;
impl SinemodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sinemode {
        match self.bits {
            false => Sinemode::Dissinemode,
            true => Sinemode::Ensinemode,
        }
    }
    #[doc = "Sine mode disabled. Sine reset to 0 degrees"]
    #[inline(always)]
    pub fn is_dissinemode(&self) -> bool {
        *self == Sinemode::Dissinemode
    }
    #[doc = "Sine mode enabled"]
    #[inline(always)]
    pub fn is_ensinemode(&self) -> bool {
        *self == Sinemode::Ensinemode
    }
}
#[doc = "Field `SINEMODE` writer - Sine Mode"]
pub type SinemodeW<'a, REG> = crate::BitWriter<'a, REG, Sinemode>;
impl<'a, REG> SinemodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sine mode disabled. Sine reset to 0 degrees"]
    #[inline(always)]
    pub fn dissinemode(self) -> &'a mut crate::W<REG> {
        self.variant(Sinemode::Dissinemode)
    }
    #[doc = "Sine mode enabled"]
    #[inline(always)]
    pub fn ensinemode(self) -> &'a mut crate::W<REG> {
        self.variant(Sinemode::Ensinemode)
    }
}
#[doc = "Field `SINERESET` reader - Sine Wave Reset When inactive"]
pub type SineresetR = crate::BitReader;
#[doc = "Field `SINERESET` writer - Sine Wave Reset When inactive"]
pub type SineresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel 0 Start Reset Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0prescrst {
    #[doc = "0: Prescaler not reset on channel 0 start"]
    Noresetpresc = 0,
    #[doc = "1: Prescaler reset on channel 0 start"]
    Resetpresc = 1,
}
impl From<Ch0prescrst> for bool {
    #[inline(always)]
    fn from(variant: Ch0prescrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0PRESCRST` reader - Channel 0 Start Reset Prescaler"]
pub type Ch0prescrstR = crate::BitReader<Ch0prescrst>;
impl Ch0prescrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0prescrst {
        match self.bits {
            false => Ch0prescrst::Noresetpresc,
            true => Ch0prescrst::Resetpresc,
        }
    }
    #[doc = "Prescaler not reset on channel 0 start"]
    #[inline(always)]
    pub fn is_noresetpresc(&self) -> bool {
        *self == Ch0prescrst::Noresetpresc
    }
    #[doc = "Prescaler reset on channel 0 start"]
    #[inline(always)]
    pub fn is_resetpresc(&self) -> bool {
        *self == Ch0prescrst::Resetpresc
    }
}
#[doc = "Field `CH0PRESCRST` writer - Channel 0 Start Reset Prescaler"]
pub type Ch0prescrstW<'a, REG> = crate::BitWriter<'a, REG, Ch0prescrst>;
impl<'a, REG> Ch0prescrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prescaler not reset on channel 0 start"]
    #[inline(always)]
    pub fn noresetpresc(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0prescrst::Noresetpresc)
    }
    #[doc = "Prescaler reset on channel 0 start"]
    #[inline(always)]
    pub fn resetpresc(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0prescrst::Resetpresc)
    }
}
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refrsel {
    #[doc = "0: Internal 1.25 V bandgap reference"]
    V125 = 0,
    #[doc = "1: Internal 2.5 V bandgap reference"]
    V25 = 1,
    #[doc = "2: AVDD reference"]
    Vdd = 2,
    #[doc = "3: External pin reference"]
    Ext = 3,
}
impl From<Refrsel> for u8 {
    #[inline(always)]
    fn from(variant: Refrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refrsel {
    type Ux = u8;
}
impl crate::IsEnum for Refrsel {}
#[doc = "Field `REFRSEL` reader - Reference Selection"]
pub type RefrselR = crate::FieldReader<Refrsel>;
impl RefrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refrsel {
        match self.bits {
            0 => Refrsel::V125,
            1 => Refrsel::V25,
            2 => Refrsel::Vdd,
            3 => Refrsel::Ext,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal 1.25 V bandgap reference"]
    #[inline(always)]
    pub fn is_v125(&self) -> bool {
        *self == Refrsel::V125
    }
    #[doc = "Internal 2.5 V bandgap reference"]
    #[inline(always)]
    pub fn is_v25(&self) -> bool {
        *self == Refrsel::V25
    }
    #[doc = "AVDD reference"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == Refrsel::Vdd
    }
    #[doc = "External pin reference"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == Refrsel::Ext
    }
}
#[doc = "Field `REFRSEL` writer - Reference Selection"]
pub type RefrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refrsel, crate::Safe>;
impl<'a, REG> RefrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal 1.25 V bandgap reference"]
    #[inline(always)]
    pub fn v125(self) -> &'a mut crate::W<REG> {
        self.variant(Refrsel::V125)
    }
    #[doc = "Internal 2.5 V bandgap reference"]
    #[inline(always)]
    pub fn v25(self) -> &'a mut crate::W<REG> {
        self.variant(Refrsel::V25)
    }
    #[doc = "AVDD reference"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Refrsel::Vdd)
    }
    #[doc = "External pin reference"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut crate::W<REG> {
        self.variant(Refrsel::Ext)
    }
}
#[doc = "Field `PRESC` reader - Prescaler Setting for DAC clock"]
pub type PrescR = crate::FieldReader;
#[doc = "Field `PRESC` writer - Prescaler Setting for DAC clock"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Internal Timer Overflow Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timerovrflowperiod {
    #[doc = "0: The Timer overflows every 2 Prescaled CLK_DAC cycles"]
    Cycles2 = 0,
    #[doc = "1: The Timer overflows every 4 Prescaled CLK_DAC cycles"]
    Cycles4 = 1,
    #[doc = "2: The Timer overflows every 8 Prescaled CLK_DAC cycles"]
    Cycles8 = 2,
    #[doc = "3: The Timer overflows every 16 Prescaled CLK_DAC cycles"]
    Cycles16 = 3,
    #[doc = "4: The Timer overflows every 32 Prescaled CLK_DAC cycles"]
    Cycles32 = 4,
    #[doc = "5: The Timer overflows every 64 Prescaled CLK_DAC cycles"]
    Cycles64 = 5,
}
impl From<Timerovrflowperiod> for u8 {
    #[inline(always)]
    fn from(variant: Timerovrflowperiod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timerovrflowperiod {
    type Ux = u8;
}
impl crate::IsEnum for Timerovrflowperiod {}
#[doc = "Field `TIMEROVRFLOWPERIOD` reader - Internal Timer Overflow Period"]
pub type TimerovrflowperiodR = crate::FieldReader<Timerovrflowperiod>;
impl TimerovrflowperiodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Timerovrflowperiod> {
        match self.bits {
            0 => Some(Timerovrflowperiod::Cycles2),
            1 => Some(Timerovrflowperiod::Cycles4),
            2 => Some(Timerovrflowperiod::Cycles8),
            3 => Some(Timerovrflowperiod::Cycles16),
            4 => Some(Timerovrflowperiod::Cycles32),
            5 => Some(Timerovrflowperiod::Cycles64),
            _ => None,
        }
    }
    #[doc = "The Timer overflows every 2 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        *self == Timerovrflowperiod::Cycles2
    }
    #[doc = "The Timer overflows every 4 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn is_cycles4(&self) -> bool {
        *self == Timerovrflowperiod::Cycles4
    }
    #[doc = "The Timer overflows every 8 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn is_cycles8(&self) -> bool {
        *self == Timerovrflowperiod::Cycles8
    }
    #[doc = "The Timer overflows every 16 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn is_cycles16(&self) -> bool {
        *self == Timerovrflowperiod::Cycles16
    }
    #[doc = "The Timer overflows every 32 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn is_cycles32(&self) -> bool {
        *self == Timerovrflowperiod::Cycles32
    }
    #[doc = "The Timer overflows every 64 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn is_cycles64(&self) -> bool {
        *self == Timerovrflowperiod::Cycles64
    }
}
#[doc = "Field `TIMEROVRFLOWPERIOD` writer - Internal Timer Overflow Period"]
pub type TimerovrflowperiodW<'a, REG> = crate::FieldWriter<'a, REG, 3, Timerovrflowperiod>;
impl<'a, REG> TimerovrflowperiodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The Timer overflows every 2 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut crate::W<REG> {
        self.variant(Timerovrflowperiod::Cycles2)
    }
    #[doc = "The Timer overflows every 4 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn cycles4(self) -> &'a mut crate::W<REG> {
        self.variant(Timerovrflowperiod::Cycles4)
    }
    #[doc = "The Timer overflows every 8 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn cycles8(self) -> &'a mut crate::W<REG> {
        self.variant(Timerovrflowperiod::Cycles8)
    }
    #[doc = "The Timer overflows every 16 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn cycles16(self) -> &'a mut crate::W<REG> {
        self.variant(Timerovrflowperiod::Cycles16)
    }
    #[doc = "The Timer overflows every 32 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn cycles32(self) -> &'a mut crate::W<REG> {
        self.variant(Timerovrflowperiod::Cycles32)
    }
    #[doc = "The Timer overflows every 64 Prescaled CLK_DAC cycles"]
    #[inline(always)]
    pub fn cycles64(self) -> &'a mut crate::W<REG> {
        self.variant(Timerovrflowperiod::Cycles64)
    }
}
#[doc = "Refresh Timer Overflow Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refreshperiod {
    #[doc = "0: All channels with enabled refresh are refreshed every 2 CLK_REFRESH cycles"]
    Cycles2 = 0,
    #[doc = "1: All channels with enabled refresh are refreshed every 4 CLK_REFRESH cycles"]
    Cycles4 = 1,
    #[doc = "2: All channels with enabled refresh are refreshed every 8 CLK_REFRESH cycles"]
    Cycles8 = 2,
    #[doc = "3: All channels with enabled refresh are refreshed every 16 CLK_REFRESH cycles"]
    Cycles16 = 3,
    #[doc = "4: All channels with enabled refresh are refreshed every 32 CLK_REFRESH cycles"]
    Cycles32 = 4,
    #[doc = "5: All channels with enabled refresh are refreshed every 64 CLK_REFRESH cycles"]
    Cycles64 = 5,
    #[doc = "6: All channels with enabled refresh are refreshed every 128 CLK_REFRESH cycles"]
    Cycles128 = 6,
    #[doc = "7: All channels with enabled refresh are refreshed every 256 CLK_REFRESH cycles"]
    Cycles256 = 7,
}
impl From<Refreshperiod> for u8 {
    #[inline(always)]
    fn from(variant: Refreshperiod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refreshperiod {
    type Ux = u8;
}
impl crate::IsEnum for Refreshperiod {}
#[doc = "Field `REFRESHPERIOD` reader - Refresh Timer Overflow Period"]
pub type RefreshperiodR = crate::FieldReader<Refreshperiod>;
impl RefreshperiodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refreshperiod {
        match self.bits {
            0 => Refreshperiod::Cycles2,
            1 => Refreshperiod::Cycles4,
            2 => Refreshperiod::Cycles8,
            3 => Refreshperiod::Cycles16,
            4 => Refreshperiod::Cycles32,
            5 => Refreshperiod::Cycles64,
            6 => Refreshperiod::Cycles128,
            7 => Refreshperiod::Cycles256,
            _ => unreachable!(),
        }
    }
    #[doc = "All channels with enabled refresh are refreshed every 2 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        *self == Refreshperiod::Cycles2
    }
    #[doc = "All channels with enabled refresh are refreshed every 4 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn is_cycles4(&self) -> bool {
        *self == Refreshperiod::Cycles4
    }
    #[doc = "All channels with enabled refresh are refreshed every 8 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn is_cycles8(&self) -> bool {
        *self == Refreshperiod::Cycles8
    }
    #[doc = "All channels with enabled refresh are refreshed every 16 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn is_cycles16(&self) -> bool {
        *self == Refreshperiod::Cycles16
    }
    #[doc = "All channels with enabled refresh are refreshed every 32 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn is_cycles32(&self) -> bool {
        *self == Refreshperiod::Cycles32
    }
    #[doc = "All channels with enabled refresh are refreshed every 64 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn is_cycles64(&self) -> bool {
        *self == Refreshperiod::Cycles64
    }
    #[doc = "All channels with enabled refresh are refreshed every 128 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn is_cycles128(&self) -> bool {
        *self == Refreshperiod::Cycles128
    }
    #[doc = "All channels with enabled refresh are refreshed every 256 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn is_cycles256(&self) -> bool {
        *self == Refreshperiod::Cycles256
    }
}
#[doc = "Field `REFRESHPERIOD` writer - Refresh Timer Overflow Period"]
pub type RefreshperiodW<'a, REG> = crate::FieldWriter<'a, REG, 3, Refreshperiod, crate::Safe>;
impl<'a, REG> RefreshperiodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All channels with enabled refresh are refreshed every 2 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshperiod::Cycles2)
    }
    #[doc = "All channels with enabled refresh are refreshed every 4 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles4(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshperiod::Cycles4)
    }
    #[doc = "All channels with enabled refresh are refreshed every 8 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles8(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshperiod::Cycles8)
    }
    #[doc = "All channels with enabled refresh are refreshed every 16 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles16(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshperiod::Cycles16)
    }
    #[doc = "All channels with enabled refresh are refreshed every 32 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles32(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshperiod::Cycles32)
    }
    #[doc = "All channels with enabled refresh are refreshed every 64 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles64(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshperiod::Cycles64)
    }
    #[doc = "All channels with enabled refresh are refreshed every 128 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles128(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshperiod::Cycles128)
    }
    #[doc = "All channels with enabled refresh are refreshed every 256 CLK_REFRESH cycles"]
    #[inline(always)]
    pub fn cycles256(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshperiod::Cycles256)
    }
}
#[doc = "Field `BIASKEEPWARM` reader - Bias Keepwarm Mode Enable"]
pub type BiaskeepwarmR = crate::BitReader;
#[doc = "Field `BIASKEEPWARM` writer - Bias Keepwarm Mode Enable"]
pub type BiaskeepwarmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAWU` reader - VDAC DMA Wakeup"]
pub type DmawuR = crate::BitReader;
#[doc = "Field `DMAWU` writer - VDAC DMA Wakeup"]
pub type DmawuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMANDCLK` reader - Always allow clk_dac"]
pub type OndemandclkR = crate::BitReader;
#[doc = "Field `ONDEMANDCLK` writer - Always allow clk_dac"]
pub type OndemandclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Debug Halt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbghalt {
    #[doc = "0: Continue operation as normal during debug mode"]
    Normal = 0,
    #[doc = "1: Complete the current conversion and then halt during debug mode"]
    Halt = 1,
}
impl From<Dbghalt> for bool {
    #[inline(always)]
    fn from(variant: Dbghalt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGHALT` reader - Debug Halt"]
pub type DbghaltR = crate::BitReader<Dbghalt>;
impl DbghaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbghalt {
        match self.bits {
            false => Dbghalt::Normal,
            true => Dbghalt::Halt,
        }
    }
    #[doc = "Continue operation as normal during debug mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Dbghalt::Normal
    }
    #[doc = "Complete the current conversion and then halt during debug mode"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == Dbghalt::Halt
    }
}
#[doc = "Field `DBGHALT` writer - Debug Halt"]
pub type DbghaltW<'a, REG> = crate::BitWriter<'a, REG, Dbghalt>;
impl<'a, REG> DbghaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue operation as normal during debug mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Dbghalt::Normal)
    }
    #[doc = "Complete the current conversion and then halt during debug mode"]
    #[inline(always)]
    pub fn halt(self) -> &'a mut crate::W<REG> {
        self.variant(Dbghalt::Halt)
    }
}
#[doc = "Field `WARMUPTIME` reader - DAC Warmup Time"]
pub type WarmuptimeR = crate::FieldReader;
#[doc = "Field `WARMUPTIME` writer - DAC Warmup Time"]
pub type WarmuptimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sine Mode"]
    #[inline(always)]
    pub fn sinemode(&self) -> SinemodeR {
        SinemodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sine Wave Reset When inactive"]
    #[inline(always)]
    pub fn sinereset(&self) -> SineresetR {
        SineresetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 Start Reset Prescaler"]
    #[inline(always)]
    pub fn ch0prescrst(&self) -> Ch0prescrstR {
        Ch0prescrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reference Selection"]
    #[inline(always)]
    pub fn refrsel(&self) -> RefrselR {
        RefrselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 7:13 - Prescaler Setting for DAC clock"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - Internal Timer Overflow Period"]
    #[inline(always)]
    pub fn timerovrflowperiod(&self) -> TimerovrflowperiodR {
        TimerovrflowperiodR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Refresh Timer Overflow Period"]
    #[inline(always)]
    pub fn refreshperiod(&self) -> RefreshperiodR {
        RefreshperiodR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Bias Keepwarm Mode Enable"]
    #[inline(always)]
    pub fn biaskeepwarm(&self) -> BiaskeepwarmR {
        BiaskeepwarmR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VDAC DMA Wakeup"]
    #[inline(always)]
    pub fn dmawu(&self) -> DmawuR {
        DmawuR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Always allow clk_dac"]
    #[inline(always)]
    pub fn ondemandclk(&self) -> OndemandclkR {
        OndemandclkR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DbghaltR {
        DbghaltR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - DAC Warmup Time"]
    #[inline(always)]
    pub fn warmuptime(&self) -> WarmuptimeR {
        WarmuptimeR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DiffW<CfgSpec> {
        DiffW::new(self, 0)
    }
    #[doc = "Bit 1 - Sine Mode"]
    #[inline(always)]
    pub fn sinemode(&mut self) -> SinemodeW<CfgSpec> {
        SinemodeW::new(self, 1)
    }
    #[doc = "Bit 2 - Sine Wave Reset When inactive"]
    #[inline(always)]
    pub fn sinereset(&mut self) -> SineresetW<CfgSpec> {
        SineresetW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 0 Start Reset Prescaler"]
    #[inline(always)]
    pub fn ch0prescrst(&mut self) -> Ch0prescrstW<CfgSpec> {
        Ch0prescrstW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Reference Selection"]
    #[inline(always)]
    pub fn refrsel(&mut self) -> RefrselW<CfgSpec> {
        RefrselW::new(self, 4)
    }
    #[doc = "Bits 7:13 - Prescaler Setting for DAC clock"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<CfgSpec> {
        PrescW::new(self, 7)
    }
    #[doc = "Bits 16:18 - Internal Timer Overflow Period"]
    #[inline(always)]
    pub fn timerovrflowperiod(&mut self) -> TimerovrflowperiodW<CfgSpec> {
        TimerovrflowperiodW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Refresh Timer Overflow Period"]
    #[inline(always)]
    pub fn refreshperiod(&mut self) -> RefreshperiodW<CfgSpec> {
        RefreshperiodW::new(self, 20)
    }
    #[doc = "Bit 24 - Bias Keepwarm Mode Enable"]
    #[inline(always)]
    pub fn biaskeepwarm(&mut self) -> BiaskeepwarmW<CfgSpec> {
        BiaskeepwarmW::new(self, 24)
    }
    #[doc = "Bit 25 - VDAC DMA Wakeup"]
    #[inline(always)]
    pub fn dmawu(&mut self) -> DmawuW<CfgSpec> {
        DmawuW::new(self, 25)
    }
    #[doc = "Bit 26 - Always allow clk_dac"]
    #[inline(always)]
    pub fn ondemandclk(&mut self) -> OndemandclkW<CfgSpec> {
        OndemandclkW::new(self, 26)
    }
    #[doc = "Bit 27 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DbghaltW<CfgSpec> {
        DbghaltW::new(self, 27)
    }
    #[doc = "Bits 28:30 - DAC Warmup Time"]
    #[inline(always)]
    pub fn warmuptime(&mut self) -> WarmuptimeW<CfgSpec> {
        WarmuptimeW::new(self, 28)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0x2000_0000"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
