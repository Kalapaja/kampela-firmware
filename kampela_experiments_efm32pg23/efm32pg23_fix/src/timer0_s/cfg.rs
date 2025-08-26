#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Up-count mode"]
    Up = 0,
    #[doc = "1: Down-count mode"]
    Down = 1,
    #[doc = "2: Up/down-count mode"]
    Updown = 2,
    #[doc = "3: Quadrature decoder mode"]
    Qdec = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Timer Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Up,
            1 => Mode::Down,
            2 => Mode::Updown,
            3 => Mode::Qdec,
            _ => unreachable!(),
        }
    }
    #[doc = "Up-count mode"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Mode::Up
    }
    #[doc = "Down-count mode"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Mode::Down
    }
    #[doc = "Up/down-count mode"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == Mode::Updown
    }
    #[doc = "Quadrature decoder mode"]
    #[inline(always)]
    pub fn is_qdec(&self) -> bool {
        *self == Mode::Qdec
    }
}
#[doc = "Field `MODE` writer - Timer Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Up-count mode"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Up)
    }
    #[doc = "Down-count mode"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Down)
    }
    #[doc = "Up/down-count mode"]
    #[inline(always)]
    pub fn updown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Updown)
    }
    #[doc = "Quadrature decoder mode"]
    #[inline(always)]
    pub fn qdec(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Qdec)
    }
}
#[doc = "Timer Start/Stop/Reload Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sync {
    #[doc = "0: Timer operation is unaffected by other timers."]
    Disable = 0,
    #[doc = "1: Timer may be started, stopped and re-loaded from other timer instances."]
    Enable = 1,
}
impl From<Sync> for bool {
    #[inline(always)]
    fn from(variant: Sync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` reader - Timer Start/Stop/Reload Synchronization"]
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
    #[doc = "Timer operation is unaffected by other timers."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sync::Disable
    }
    #[doc = "Timer may be started, stopped and re-loaded from other timer instances."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sync::Enable
    }
}
#[doc = "Field `SYNC` writer - Timer Start/Stop/Reload Synchronization"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG, Sync>;
impl<'a, REG> SyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer operation is unaffected by other timers."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::Disable)
    }
    #[doc = "Timer may be started, stopped and re-loaded from other timer instances."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::Enable)
    }
}
#[doc = "Field `OSMEN` reader - One-shot Mode Enable"]
pub type OsmenR = crate::BitReader;
#[doc = "Field `OSMEN` writer - One-shot Mode Enable"]
pub type OsmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Quadrature Decoder Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qdm {
    #[doc = "0: X2 mode selected"]
    X2 = 0,
    #[doc = "1: X4 mode selected"]
    X4 = 1,
}
impl From<Qdm> for bool {
    #[inline(always)]
    fn from(variant: Qdm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QDM` reader - Quadrature Decoder Mode Selection"]
pub type QdmR = crate::BitReader<Qdm>;
impl QdmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qdm {
        match self.bits {
            false => Qdm::X2,
            true => Qdm::X4,
        }
    }
    #[doc = "X2 mode selected"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == Qdm::X2
    }
    #[doc = "X4 mode selected"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == Qdm::X4
    }
}
#[doc = "Field `QDM` writer - Quadrature Decoder Mode Selection"]
pub type QdmW<'a, REG> = crate::BitWriter<'a, REG, Qdm>;
impl<'a, REG> QdmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "X2 mode selected"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut crate::W<REG> {
        self.variant(Qdm::X2)
    }
    #[doc = "X4 mode selected"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(Qdm::X4)
    }
}
#[doc = "Debug Mode Run Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debugrun {
    #[doc = "0: Timer is halted in debug mode"]
    Halt = 0,
    #[doc = "1: Timer is running in debug mode"]
    Run = 1,
}
impl From<Debugrun> for bool {
    #[inline(always)]
    fn from(variant: Debugrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DebugrunR = crate::BitReader<Debugrun>;
impl DebugrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Debugrun {
        match self.bits {
            false => Debugrun::Halt,
            true => Debugrun::Run,
        }
    }
    #[doc = "Timer is halted in debug mode"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == Debugrun::Halt
    }
    #[doc = "Timer is running in debug mode"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Debugrun::Run
    }
}
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG, Debugrun>;
impl<'a, REG> DebugrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer is halted in debug mode"]
    #[inline(always)]
    pub fn halt(self) -> &'a mut crate::W<REG> {
        self.variant(Debugrun::Halt)
    }
    #[doc = "Timer is running in debug mode"]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Debugrun::Run)
    }
}
#[doc = "Field `DMACLRACT` reader - DMA Request Clear on Active"]
pub type DmaclractR = crate::BitReader;
#[doc = "Field `DMACLRACT` writer - DMA Request Clear on Active"]
pub type DmaclractW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: Prescaled EM01GRPACLK"]
    Prescem01grpaclk = 0,
    #[doc = "1: Compare/Capture Channel 1 Input"]
    Cc1 = 1,
    #[doc = "2: Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    Timerouf = 2,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Clock Source Select"]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clksel> {
        match self.bits {
            0 => Some(Clksel::Prescem01grpaclk),
            1 => Some(Clksel::Cc1),
            2 => Some(Clksel::Timerouf),
            _ => None,
        }
    }
    #[doc = "Prescaled EM01GRPACLK"]
    #[inline(always)]
    pub fn is_prescem01grpaclk(&self) -> bool {
        *self == Clksel::Prescem01grpaclk
    }
    #[doc = "Compare/Capture Channel 1 Input"]
    #[inline(always)]
    pub fn is_cc1(&self) -> bool {
        *self == Clksel::Cc1
    }
    #[doc = "Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    #[inline(always)]
    pub fn is_timerouf(&self) -> bool {
        *self == Clksel::Timerouf
    }
}
#[doc = "Field `CLKSEL` writer - Clock Source Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaled EM01GRPACLK"]
    #[inline(always)]
    pub fn prescem01grpaclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Prescem01grpaclk)
    }
    #[doc = "Compare/Capture Channel 1 Input"]
    #[inline(always)]
    pub fn cc1(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Cc1)
    }
    #[doc = "Timer is clocked by underflow(down-count) or overflow(up-count) in the lower numbered neighbor Timer"]
    #[inline(always)]
    pub fn timerouf(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Timerouf)
    }
}
#[doc = "PWM output retimed enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Retimeen {
    #[doc = "0: PWM outputs are not re-timed."]
    Disable = 0,
    #[doc = "1: PWM outputs are re-timed."]
    Enable = 1,
}
impl From<Retimeen> for bool {
    #[inline(always)]
    fn from(variant: Retimeen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RETIMEEN` reader - PWM output retimed enable"]
pub type RetimeenR = crate::BitReader<Retimeen>;
impl RetimeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Retimeen {
        match self.bits {
            false => Retimeen::Disable,
            true => Retimeen::Enable,
        }
    }
    #[doc = "PWM outputs are not re-timed."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Retimeen::Disable
    }
    #[doc = "PWM outputs are re-timed."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Retimeen::Enable
    }
}
#[doc = "Field `RETIMEEN` writer - PWM output retimed enable"]
pub type RetimeenW<'a, REG> = crate::BitWriter<'a, REG, Retimeen>;
impl<'a, REG> RetimeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PWM outputs are not re-timed."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Retimeen::Disable)
    }
    #[doc = "PWM outputs are re-timed."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Retimeen::Enable)
    }
}
#[doc = "Disable Timer Start/Stop/Reload output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dissyncout {
    #[doc = "0: Timer can start/stop/reload other timers with SYNC bit set"]
    En = 0,
    #[doc = "1: Timer cannot start/stop/reload other timers with SYNC bit set"]
    Dis = 1,
}
impl From<Dissyncout> for bool {
    #[inline(always)]
    fn from(variant: Dissyncout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISSYNCOUT` reader - Disable Timer Start/Stop/Reload output"]
pub type DissyncoutR = crate::BitReader<Dissyncout>;
impl DissyncoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dissyncout {
        match self.bits {
            false => Dissyncout::En,
            true => Dissyncout::Dis,
        }
    }
    #[doc = "Timer can start/stop/reload other timers with SYNC bit set"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dissyncout::En
    }
    #[doc = "Timer cannot start/stop/reload other timers with SYNC bit set"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dissyncout::Dis
    }
}
#[doc = "Field `DISSYNCOUT` writer - Disable Timer Start/Stop/Reload output"]
pub type DissyncoutW<'a, REG> = crate::BitWriter<'a, REG, Dissyncout>;
impl<'a, REG> DissyncoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer can start/stop/reload other timers with SYNC bit set"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dissyncout::En)
    }
    #[doc = "Timer cannot start/stop/reload other timers with SYNC bit set"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dissyncout::Dis)
    }
}
#[doc = "Field `ATI` reader - Always Track Inputs"]
pub type AtiR = crate::BitReader;
#[doc = "Field `ATI` writer - Always Track Inputs"]
pub type AtiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSSCOIST` reader - Reload-Start Sets COIST"]
pub type RsscoistR = crate::BitReader;
#[doc = "Field `RSSCOIST` writer - Reload-Start Sets COIST"]
pub type RsscoistW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Presc {
    #[doc = "0: No prescaling"]
    Div1 = 0,
    #[doc = "1: Prescale by 2"]
    Div2 = 1,
    #[doc = "3: Prescale by 4"]
    Div4 = 3,
    #[doc = "7: Prescale by 8"]
    Div8 = 7,
    #[doc = "15: Prescale by 16"]
    Div16 = 15,
    #[doc = "31: Prescale by 32"]
    Div32 = 31,
    #[doc = "63: Prescale by 64"]
    Div64 = 63,
    #[doc = "127: Prescale by 128"]
    Div128 = 127,
    #[doc = "255: Prescale by 256"]
    Div256 = 255,
    #[doc = "511: Prescale by 512"]
    Div512 = 511,
    #[doc = "1023: Prescale by 1024"]
    Div1024 = 1023,
}
impl From<Presc> for u16 {
    #[inline(always)]
    fn from(variant: Presc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Presc {
    type Ux = u16;
}
impl crate::IsEnum for Presc {}
#[doc = "Field `PRESC` reader - Prescaler Setting"]
pub type PrescR = crate::FieldReader<Presc>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Presc> {
        match self.bits {
            0 => Some(Presc::Div1),
            1 => Some(Presc::Div2),
            3 => Some(Presc::Div4),
            7 => Some(Presc::Div8),
            15 => Some(Presc::Div16),
            31 => Some(Presc::Div32),
            63 => Some(Presc::Div64),
            127 => Some(Presc::Div128),
            255 => Some(Presc::Div256),
            511 => Some(Presc::Div512),
            1023 => Some(Presc::Div1024),
            _ => None,
        }
    }
    #[doc = "No prescaling"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Presc::Div1
    }
    #[doc = "Prescale by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Presc::Div2
    }
    #[doc = "Prescale by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Presc::Div4
    }
    #[doc = "Prescale by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Presc::Div8
    }
    #[doc = "Prescale by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Presc::Div16
    }
    #[doc = "Prescale by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Presc::Div32
    }
    #[doc = "Prescale by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Presc::Div64
    }
    #[doc = "Prescale by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Presc::Div128
    }
    #[doc = "Prescale by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Presc::Div256
    }
    #[doc = "Prescale by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Presc::Div512
    }
    #[doc = "Prescale by 1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Presc::Div1024
    }
}
#[doc = "Field `PRESC` writer - Prescaler Setting"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 10, Presc>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No prescaling"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div1)
    }
    #[doc = "Prescale by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div2)
    }
    #[doc = "Prescale by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div4)
    }
    #[doc = "Prescale by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div8)
    }
    #[doc = "Prescale by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div16)
    }
    #[doc = "Prescale by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div32)
    }
    #[doc = "Prescale by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div64)
    }
    #[doc = "Prescale by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div128)
    }
    #[doc = "Prescale by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div256)
    }
    #[doc = "Prescale by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div512)
    }
    #[doc = "Prescale by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::Div1024)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Timer Start/Stop/Reload Synchronization"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - One-shot Mode Enable"]
    #[inline(always)]
    pub fn osmen(&self) -> OsmenR {
        OsmenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quadrature Decoder Mode Selection"]
    #[inline(always)]
    pub fn qdm(&self) -> QdmR {
        QdmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Request Clear on Active"]
    #[inline(always)]
    pub fn dmaclract(&self) -> DmaclractR {
        DmaclractR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock Source Select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - PWM output retimed enable"]
    #[inline(always)]
    pub fn retimeen(&self) -> RetimeenR {
        RetimeenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Disable Timer Start/Stop/Reload output"]
    #[inline(always)]
    pub fn dissyncout(&self) -> DissyncoutR {
        DissyncoutR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Always Track Inputs"]
    #[inline(always)]
    pub fn ati(&self) -> AtiR {
        AtiR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reload-Start Sets COIST"]
    #[inline(always)]
    pub fn rsscoist(&self) -> RsscoistR {
        RsscoistR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:27 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 18) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<CfgSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 3 - Timer Start/Stop/Reload Synchronization"]
    #[inline(always)]
    pub fn sync(&mut self) -> SyncW<CfgSpec> {
        SyncW::new(self, 3)
    }
    #[doc = "Bit 4 - One-shot Mode Enable"]
    #[inline(always)]
    pub fn osmen(&mut self) -> OsmenW<CfgSpec> {
        OsmenW::new(self, 4)
    }
    #[doc = "Bit 5 - Quadrature Decoder Mode Selection"]
    #[inline(always)]
    pub fn qdm(&mut self) -> QdmW<CfgSpec> {
        QdmW::new(self, 5)
    }
    #[doc = "Bit 6 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DebugrunW<CfgSpec> {
        DebugrunW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Request Clear on Active"]
    #[inline(always)]
    pub fn dmaclract(&mut self) -> DmaclractW<CfgSpec> {
        DmaclractW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Clock Source Select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<CfgSpec> {
        ClkselW::new(self, 8)
    }
    #[doc = "Bit 10 - PWM output retimed enable"]
    #[inline(always)]
    pub fn retimeen(&mut self) -> RetimeenW<CfgSpec> {
        RetimeenW::new(self, 10)
    }
    #[doc = "Bit 11 - Disable Timer Start/Stop/Reload output"]
    #[inline(always)]
    pub fn dissyncout(&mut self) -> DissyncoutW<CfgSpec> {
        DissyncoutW::new(self, 11)
    }
    #[doc = "Bit 16 - Always Track Inputs"]
    #[inline(always)]
    pub fn ati(&mut self) -> AtiW<CfgSpec> {
        AtiW::new(self, 16)
    }
    #[doc = "Bit 17 - Reload-Start Sets COIST"]
    #[inline(always)]
    pub fn rsscoist(&mut self) -> RsscoistW<CfgSpec> {
        RsscoistW::new(self, 17)
    }
    #[doc = "Bits 18:27 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<CfgSpec> {
        PrescW::new(self, 18)
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
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}
