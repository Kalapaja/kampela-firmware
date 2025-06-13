#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Repeat Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Repmode {
    #[doc = "0: When started, the LETIMER counts down until it is stopped by software"]
    Free = 0,
    #[doc = "1: The counter counts REP0 times. When REP0 reaches zero, the counter stops"]
    Oneshot = 1,
    #[doc = "2: The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero, otherwise the counter stops"]
    Buffered = 2,
    #[doc = "3: Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    Double = 3,
}
impl From<Repmode> for u8 {
    #[inline(always)]
    fn from(variant: Repmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Repmode {
    type Ux = u8;
}
impl crate::IsEnum for Repmode {}
#[doc = "Field `REPMODE` reader - Repeat Mode"]
pub type RepmodeR = crate::FieldReader<Repmode>;
impl RepmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Repmode {
        match self.bits {
            0 => Repmode::Free,
            1 => Repmode::Oneshot,
            2 => Repmode::Buffered,
            3 => Repmode::Double,
            _ => unreachable!(),
        }
    }
    #[doc = "When started, the LETIMER counts down until it is stopped by software"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == Repmode::Free
    }
    #[doc = "The counter counts REP0 times. When REP0 reaches zero, the counter stops"]
    #[inline(always)]
    pub fn is_oneshot(&self) -> bool {
        *self == Repmode::Oneshot
    }
    #[doc = "The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero, otherwise the counter stops"]
    #[inline(always)]
    pub fn is_buffered(&self) -> bool {
        *self == Repmode::Buffered
    }
    #[doc = "Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == Repmode::Double
    }
}
#[doc = "Field `REPMODE` writer - Repeat Mode"]
pub type RepmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Repmode, crate::Safe>;
impl<'a, REG> RepmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When started, the LETIMER counts down until it is stopped by software"]
    #[inline(always)]
    pub fn free(self) -> &'a mut crate::W<REG> {
        self.variant(Repmode::Free)
    }
    #[doc = "The counter counts REP0 times. When REP0 reaches zero, the counter stops"]
    #[inline(always)]
    pub fn oneshot(self) -> &'a mut crate::W<REG> {
        self.variant(Repmode::Oneshot)
    }
    #[doc = "The counter counts REP0 times. If REP1 has been written, it is loaded into REP0 when REP0 reaches zero, otherwise the counter stops"]
    #[inline(always)]
    pub fn buffered(self) -> &'a mut crate::W<REG> {
        self.variant(Repmode::Buffered)
    }
    #[doc = "Both REP0 and REP1 are decremented when the LETIMER wraps around. The LETIMER counts until both REP0 and REP1 are zero"]
    #[inline(always)]
    pub fn double(self) -> &'a mut crate::W<REG> {
        self.variant(Repmode::Double)
    }
}
#[doc = "Underflow Output Action 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ufoa0 {
    #[doc = "0: LETIMERn_OUT0 is held at its idle value as defined by OPOL0"]
    None = 0,
    #[doc = "1: LETIMERn_OUT0 is toggled on CNT underflow"]
    Toggle = 1,
    #[doc = "2: LETIMERn_OUT0 is held active for one LETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0"]
    Pulse = 2,
    #[doc = "3: LETIMERn_OUT0 is set idle on CNT underflow, and active on compare match with COMP1"]
    Pwm = 3,
}
impl From<Ufoa0> for u8 {
    #[inline(always)]
    fn from(variant: Ufoa0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ufoa0 {
    type Ux = u8;
}
impl crate::IsEnum for Ufoa0 {}
#[doc = "Field `UFOA0` reader - Underflow Output Action 0"]
pub type Ufoa0R = crate::FieldReader<Ufoa0>;
impl Ufoa0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ufoa0 {
        match self.bits {
            0 => Ufoa0::None,
            1 => Ufoa0::Toggle,
            2 => Ufoa0::Pulse,
            3 => Ufoa0::Pwm,
            _ => unreachable!(),
        }
    }
    #[doc = "LETIMERn_OUT0 is held at its idle value as defined by OPOL0"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ufoa0::None
    }
    #[doc = "LETIMERn_OUT0 is toggled on CNT underflow"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Ufoa0::Toggle
    }
    #[doc = "LETIMERn_OUT0 is held active for one LETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == Ufoa0::Pulse
    }
    #[doc = "LETIMERn_OUT0 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Ufoa0::Pwm
    }
}
#[doc = "Field `UFOA0` writer - Underflow Output Action 0"]
pub type Ufoa0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ufoa0, crate::Safe>;
impl<'a, REG> Ufoa0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LETIMERn_OUT0 is held at its idle value as defined by OPOL0"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ufoa0::None)
    }
    #[doc = "LETIMERn_OUT0 is toggled on CNT underflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ufoa0::Toggle)
    }
    #[doc = "LETIMERn_OUT0 is held active for one LETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL0"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Ufoa0::Pulse)
    }
    #[doc = "LETIMERn_OUT0 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Ufoa0::Pwm)
    }
}
#[doc = "Underflow Output Action 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ufoa1 {
    #[doc = "0: LETIMERn_OUT1 is held at its idle value as defined by OPOL1"]
    None = 0,
    #[doc = "1: LETIMERn_OUT1 is toggled on CNT underflow"]
    Toggle = 1,
    #[doc = "2: LETIMERn_OUT1 is held active for one LETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1"]
    Pulse = 2,
    #[doc = "3: LETIMERn_OUT1 is set idle on CNT underflow, and active on compare match with COMP1"]
    Pwm = 3,
}
impl From<Ufoa1> for u8 {
    #[inline(always)]
    fn from(variant: Ufoa1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ufoa1 {
    type Ux = u8;
}
impl crate::IsEnum for Ufoa1 {}
#[doc = "Field `UFOA1` reader - Underflow Output Action 1"]
pub type Ufoa1R = crate::FieldReader<Ufoa1>;
impl Ufoa1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ufoa1 {
        match self.bits {
            0 => Ufoa1::None,
            1 => Ufoa1::Toggle,
            2 => Ufoa1::Pulse,
            3 => Ufoa1::Pwm,
            _ => unreachable!(),
        }
    }
    #[doc = "LETIMERn_OUT1 is held at its idle value as defined by OPOL1"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ufoa1::None
    }
    #[doc = "LETIMERn_OUT1 is toggled on CNT underflow"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Ufoa1::Toggle
    }
    #[doc = "LETIMERn_OUT1 is held active for one LETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == Ufoa1::Pulse
    }
    #[doc = "LETIMERn_OUT1 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Ufoa1::Pwm
    }
}
#[doc = "Field `UFOA1` writer - Underflow Output Action 1"]
pub type Ufoa1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ufoa1, crate::Safe>;
impl<'a, REG> Ufoa1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LETIMERn_OUT1 is held at its idle value as defined by OPOL1"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ufoa1::None)
    }
    #[doc = "LETIMERn_OUT1 is toggled on CNT underflow"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ufoa1::Toggle)
    }
    #[doc = "LETIMERn_OUT1 is held active for one LETIMER0 clock cycle on CNT underflow. The output then returns to its idle value as defined by OPOL1"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Ufoa1::Pulse)
    }
    #[doc = "LETIMERn_OUT1 is set idle on CNT underflow, and active on compare match with COMP1"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Ufoa1::Pwm)
    }
}
#[doc = "Field `OPOL0` reader - Output 0 Polarity"]
pub type Opol0R = crate::BitReader;
#[doc = "Field `OPOL0` writer - Output 0 Polarity"]
pub type Opol0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPOL1` reader - Output 1 Polarity"]
pub type Opol1R = crate::BitReader;
#[doc = "Field `OPOL1` writer - Output 1 Polarity"]
pub type Opol1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Buffered Top\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Buftop {
    #[doc = "0: COMP0 is only written by software"]
    Disable = 0,
    #[doc = "1: COMP0 is set to COMP1 when REP0 reaches 0"]
    Enable = 1,
}
impl From<Buftop> for bool {
    #[inline(always)]
    fn from(variant: Buftop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFTOP` reader - Buffered Top"]
pub type BuftopR = crate::BitReader<Buftop>;
impl BuftopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Buftop {
        match self.bits {
            false => Buftop::Disable,
            true => Buftop::Enable,
        }
    }
    #[doc = "COMP0 is only written by software"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Buftop::Disable
    }
    #[doc = "COMP0 is set to COMP1 when REP0 reaches 0"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Buftop::Enable
    }
}
#[doc = "Field `BUFTOP` writer - Buffered Top"]
pub type BuftopW<'a, REG> = crate::BitWriter<'a, REG, Buftop>;
impl<'a, REG> BuftopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP0 is only written by software"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Buftop::Disable)
    }
    #[doc = "COMP0 is set to COMP1 when REP0 reaches 0"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Buftop::Enable)
    }
}
#[doc = "Compare Value 0 Is Top Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cnttopen {
    #[doc = "0: The top value of the LETIMER is 65535 (0xFFFF)"]
    Disable = 0,
    #[doc = "1: The top value of the LETIMER is given by COMP0"]
    Enable = 1,
}
impl From<Cnttopen> for bool {
    #[inline(always)]
    fn from(variant: Cnttopen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTTOPEN` reader - Compare Value 0 Is Top Value"]
pub type CnttopenR = crate::BitReader<Cnttopen>;
impl CnttopenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cnttopen {
        match self.bits {
            false => Cnttopen::Disable,
            true => Cnttopen::Enable,
        }
    }
    #[doc = "The top value of the LETIMER is 65535 (0xFFFF)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cnttopen::Disable
    }
    #[doc = "The top value of the LETIMER is given by COMP0"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cnttopen::Enable
    }
}
#[doc = "Field `CNTTOPEN` writer - Compare Value 0 Is Top Value"]
pub type CnttopenW<'a, REG> = crate::BitWriter<'a, REG, Cnttopen>;
impl<'a, REG> CnttopenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The top value of the LETIMER is 65535 (0xFFFF)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cnttopen::Disable)
    }
    #[doc = "The top value of the LETIMER is given by COMP0"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cnttopen::Enable)
    }
}
#[doc = "Debug Mode Run Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debugrun {
    #[doc = "0: LETIMER is frozen in debug mode"]
    Disable = 0,
    #[doc = "1: LETIMER is running in debug mode"]
    Enable = 1,
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
            false => Debugrun::Disable,
            true => Debugrun::Enable,
        }
    }
    #[doc = "LETIMER is frozen in debug mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Debugrun::Disable
    }
    #[doc = "LETIMER is running in debug mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Debugrun::Enable
    }
}
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG, Debugrun>;
impl<'a, REG> DebugrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LETIMER is frozen in debug mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Debugrun::Disable)
    }
    #[doc = "LETIMER is running in debug mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Debugrun::Enable)
    }
}
#[doc = "Counter prescaler value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cntpresc {
    #[doc = "0: CLK_CNT = (LETIMER LF CLK)/1"]
    Div1 = 0,
    #[doc = "1: CLK_CNT = (LETIMER LF CLK)/2"]
    Div2 = 1,
    #[doc = "2: CLK_CNT = (LETIMER LF CLK)/4"]
    Div4 = 2,
    #[doc = "3: CLK_CNT = (LETIMER LF CLK)/8"]
    Div8 = 3,
    #[doc = "4: CLK_CNT = (LETIMER LF CLK)/16"]
    Div16 = 4,
    #[doc = "5: CLK_CNT = (LETIMER LF CLK)/32"]
    Div32 = 5,
    #[doc = "6: CLK_CNT = (LETIMER LF CLK)/64"]
    Div64 = 6,
    #[doc = "7: CLK_CNT = (LETIMER LF CLK)/128"]
    Div128 = 7,
    #[doc = "8: CLK_CNT = (LETIMER LF CLK)/256"]
    Div256 = 8,
}
impl From<Cntpresc> for u8 {
    #[inline(always)]
    fn from(variant: Cntpresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cntpresc {
    type Ux = u8;
}
impl crate::IsEnum for Cntpresc {}
#[doc = "Field `CNTPRESC` reader - Counter prescaler value"]
pub type CntprescR = crate::FieldReader<Cntpresc>;
impl CntprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cntpresc> {
        match self.bits {
            0 => Some(Cntpresc::Div1),
            1 => Some(Cntpresc::Div2),
            2 => Some(Cntpresc::Div4),
            3 => Some(Cntpresc::Div8),
            4 => Some(Cntpresc::Div16),
            5 => Some(Cntpresc::Div32),
            6 => Some(Cntpresc::Div64),
            7 => Some(Cntpresc::Div128),
            8 => Some(Cntpresc::Div256),
            _ => None,
        }
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Cntpresc::Div1
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Cntpresc::Div2
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Cntpresc::Div4
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Cntpresc::Div8
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Cntpresc::Div16
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Cntpresc::Div32
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Cntpresc::Div64
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Cntpresc::Div128
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Cntpresc::Div256
    }
}
#[doc = "Field `CNTPRESC` writer - Counter prescaler value"]
pub type CntprescW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cntpresc>;
impl<'a, REG> CntprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_CNT = (LETIMER LF CLK)/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div1)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div2)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div4)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div8)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div16)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div32)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div64)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div128)
    }
    #[doc = "CLK_CNT = (LETIMER LF CLK)/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div256)
    }
}
impl R {
    #[doc = "Bits 0:1 - Repeat Mode"]
    #[inline(always)]
    pub fn repmode(&self) -> RepmodeR {
        RepmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Underflow Output Action 0"]
    #[inline(always)]
    pub fn ufoa0(&self) -> Ufoa0R {
        Ufoa0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Underflow Output Action 1"]
    #[inline(always)]
    pub fn ufoa1(&self) -> Ufoa1R {
        Ufoa1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Output 0 Polarity"]
    #[inline(always)]
    pub fn opol0(&self) -> Opol0R {
        Opol0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output 1 Polarity"]
    #[inline(always)]
    pub fn opol1(&self) -> Opol1R {
        Opol1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Buffered Top"]
    #[inline(always)]
    pub fn buftop(&self) -> BuftopR {
        BuftopR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare Value 0 Is Top Value"]
    #[inline(always)]
    pub fn cnttopen(&self) -> CnttopenR {
        CnttopenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Counter prescaler value"]
    #[inline(always)]
    pub fn cntpresc(&self) -> CntprescR {
        CntprescR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Repeat Mode"]
    #[inline(always)]
    pub fn repmode(&mut self) -> RepmodeW<CtrlSpec> {
        RepmodeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Underflow Output Action 0"]
    #[inline(always)]
    pub fn ufoa0(&mut self) -> Ufoa0W<CtrlSpec> {
        Ufoa0W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Underflow Output Action 1"]
    #[inline(always)]
    pub fn ufoa1(&mut self) -> Ufoa1W<CtrlSpec> {
        Ufoa1W::new(self, 4)
    }
    #[doc = "Bit 6 - Output 0 Polarity"]
    #[inline(always)]
    pub fn opol0(&mut self) -> Opol0W<CtrlSpec> {
        Opol0W::new(self, 6)
    }
    #[doc = "Bit 7 - Output 1 Polarity"]
    #[inline(always)]
    pub fn opol1(&mut self) -> Opol1W<CtrlSpec> {
        Opol1W::new(self, 7)
    }
    #[doc = "Bit 8 - Buffered Top"]
    #[inline(always)]
    pub fn buftop(&mut self) -> BuftopW<CtrlSpec> {
        BuftopW::new(self, 8)
    }
    #[doc = "Bit 9 - Compare Value 0 Is Top Value"]
    #[inline(always)]
    pub fn cnttopen(&mut self) -> CnttopenW<CtrlSpec> {
        CnttopenW::new(self, 9)
    }
    #[doc = "Bit 12 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DebugrunW<CtrlSpec> {
        DebugrunW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Counter prescaler value"]
    #[inline(always)]
    pub fn cntpresc(&mut self) -> CntprescW<CtrlSpec> {
        CntprescW::new(self, 16)
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
