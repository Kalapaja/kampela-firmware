#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "WDOG Clear Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clrsrc {
    #[doc = "0: A write to the clear bit will clear the WDOG counter"]
    Sw = 0,
    #[doc = "1: A rising edge on the PRS Source 0 will clear the WDOG counter"]
    Prssrc0 = 1,
}
impl From<Clrsrc> for bool {
    #[inline(always)]
    fn from(variant: Clrsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRSRC` reader - WDOG Clear Source"]
pub type ClrsrcR = crate::BitReader<Clrsrc>;
impl ClrsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clrsrc {
        match self.bits {
            false => Clrsrc::Sw,
            true => Clrsrc::Prssrc0,
        }
    }
    #[doc = "A write to the clear bit will clear the WDOG counter"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == Clrsrc::Sw
    }
    #[doc = "A rising edge on the PRS Source 0 will clear the WDOG counter"]
    #[inline(always)]
    pub fn is_prssrc0(&self) -> bool {
        *self == Clrsrc::Prssrc0
    }
}
#[doc = "Field `CLRSRC` writer - WDOG Clear Source"]
pub type ClrsrcW<'a, REG> = crate::BitWriter<'a, REG, Clrsrc>;
impl<'a, REG> ClrsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A write to the clear bit will clear the WDOG counter"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(Clrsrc::Sw)
    }
    #[doc = "A rising edge on the PRS Source 0 will clear the WDOG counter"]
    #[inline(always)]
    pub fn prssrc0(self) -> &'a mut crate::W<REG> {
        self.variant(Clrsrc::Prssrc0)
    }
}
#[doc = "EM1 Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em1run {
    #[doc = "0: WDOG timer is frozen in EM1."]
    Disable = 0,
    #[doc = "1: WDOG timer is running in EM1."]
    Enable = 1,
}
impl From<Em1run> for bool {
    #[inline(always)]
    fn from(variant: Em1run) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM1RUN` reader - EM1 Run"]
pub type Em1runR = crate::BitReader<Em1run>;
impl Em1runR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em1run {
        match self.bits {
            false => Em1run::Disable,
            true => Em1run::Enable,
        }
    }
    #[doc = "WDOG timer is frozen in EM1."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Em1run::Disable
    }
    #[doc = "WDOG timer is running in EM1."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Em1run::Enable
    }
}
#[doc = "Field `EM1RUN` writer - EM1 Run"]
pub type Em1runW<'a, REG> = crate::BitWriter<'a, REG, Em1run>;
impl<'a, REG> Em1runW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDOG timer is frozen in EM1."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Em1run::Disable)
    }
    #[doc = "WDOG timer is running in EM1."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Em1run::Enable)
    }
}
#[doc = "EM2 Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em2run {
    #[doc = "0: WDOG timer is frozen in EM2."]
    Disable = 0,
    #[doc = "1: WDOG timer is running in EM2."]
    Enable = 1,
}
impl From<Em2run> for bool {
    #[inline(always)]
    fn from(variant: Em2run) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM2RUN` reader - EM2 Run"]
pub type Em2runR = crate::BitReader<Em2run>;
impl Em2runR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em2run {
        match self.bits {
            false => Em2run::Disable,
            true => Em2run::Enable,
        }
    }
    #[doc = "WDOG timer is frozen in EM2."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Em2run::Disable
    }
    #[doc = "WDOG timer is running in EM2."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Em2run::Enable
    }
}
#[doc = "Field `EM2RUN` writer - EM2 Run"]
pub type Em2runW<'a, REG> = crate::BitWriter<'a, REG, Em2run>;
impl<'a, REG> Em2runW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDOG timer is frozen in EM2."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Em2run::Disable)
    }
    #[doc = "WDOG timer is running in EM2."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Em2run::Enable)
    }
}
#[doc = "EM3 Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em3run {
    #[doc = "0: WDOG timer is frozen in EM3."]
    Disable = 0,
    #[doc = "1: WDOG timer is running in EM3."]
    Enable = 1,
}
impl From<Em3run> for bool {
    #[inline(always)]
    fn from(variant: Em3run) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM3RUN` reader - EM3 Run"]
pub type Em3runR = crate::BitReader<Em3run>;
impl Em3runR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em3run {
        match self.bits {
            false => Em3run::Disable,
            true => Em3run::Enable,
        }
    }
    #[doc = "WDOG timer is frozen in EM3."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Em3run::Disable
    }
    #[doc = "WDOG timer is running in EM3."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Em3run::Enable
    }
}
#[doc = "Field `EM3RUN` writer - EM3 Run"]
pub type Em3runW<'a, REG> = crate::BitWriter<'a, REG, Em3run>;
impl<'a, REG> Em3runW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDOG timer is frozen in EM3."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Em3run::Disable)
    }
    #[doc = "WDOG timer is running in EM3."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Em3run::Enable)
    }
}
#[doc = "EM4 Block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em4block {
    #[doc = "0: EM4 can be entered by software. See EMU for detailed description."]
    Disable = 0,
    #[doc = "1: EM4 cannot be entered by software."]
    Enable = 1,
}
impl From<Em4block> for bool {
    #[inline(always)]
    fn from(variant: Em4block) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM4BLOCK` reader - EM4 Block"]
pub type Em4blockR = crate::BitReader<Em4block>;
impl Em4blockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em4block {
        match self.bits {
            false => Em4block::Disable,
            true => Em4block::Enable,
        }
    }
    #[doc = "EM4 can be entered by software. See EMU for detailed description."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Em4block::Disable
    }
    #[doc = "EM4 cannot be entered by software."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Em4block::Enable
    }
}
#[doc = "Field `EM4BLOCK` writer - EM4 Block"]
pub type Em4blockW<'a, REG> = crate::BitWriter<'a, REG, Em4block>;
impl<'a, REG> Em4blockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EM4 can be entered by software. See EMU for detailed description."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Em4block::Disable)
    }
    #[doc = "EM4 cannot be entered by software."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Em4block::Enable)
    }
}
#[doc = "Debug Mode Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debugrun {
    #[doc = "0: WDOG timer is frozen in debug mode"]
    Disable = 0,
    #[doc = "1: WDOG timer is running in debug mode"]
    Enable = 1,
}
impl From<Debugrun> for bool {
    #[inline(always)]
    fn from(variant: Debugrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run"]
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
    #[doc = "WDOG timer is frozen in debug mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Debugrun::Disable
    }
    #[doc = "WDOG timer is running in debug mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Debugrun::Enable
    }
}
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run"]
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG, Debugrun>;
impl<'a, REG> DebugrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDOG timer is frozen in debug mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Debugrun::Disable)
    }
    #[doc = "WDOG timer is running in debug mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Debugrun::Enable)
    }
}
#[doc = "WDOG Reset Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdogrstdis {
    #[doc = "0: A timeout will cause a WDOG reset"]
    En = 0,
    #[doc = "1: A timeout will not cause a WDOG reset"]
    Dis = 1,
}
impl From<Wdogrstdis> for bool {
    #[inline(always)]
    fn from(variant: Wdogrstdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDOGRSTDIS` reader - WDOG Reset Disable"]
pub type WdogrstdisR = crate::BitReader<Wdogrstdis>;
impl WdogrstdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdogrstdis {
        match self.bits {
            false => Wdogrstdis::En,
            true => Wdogrstdis::Dis,
        }
    }
    #[doc = "A timeout will cause a WDOG reset"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wdogrstdis::En
    }
    #[doc = "A timeout will not cause a WDOG reset"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wdogrstdis::Dis
    }
}
#[doc = "Field `WDOGRSTDIS` writer - WDOG Reset Disable"]
pub type WdogrstdisW<'a, REG> = crate::BitWriter<'a, REG, Wdogrstdis>;
impl<'a, REG> WdogrstdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A timeout will cause a WDOG reset"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Wdogrstdis::En)
    }
    #[doc = "A timeout will not cause a WDOG reset"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Wdogrstdis::Dis)
    }
}
#[doc = "Field `PRS0MISSRSTEN` reader - PRS Src0 Missing Event WDOG Reset"]
pub type Prs0missrstenR = crate::BitReader;
#[doc = "Field `PRS0MISSRSTEN` writer - PRS Src0 Missing Event WDOG Reset"]
pub type Prs0missrstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRS1MISSRSTEN` reader - PRS Src1 Missing Event WDOG Reset"]
pub type Prs1missrstenR = crate::BitReader;
#[doc = "Field `PRS1MISSRSTEN` writer - PRS Src1 Missing Event WDOG Reset"]
pub type Prs1missrstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "WDOG Timeout Period Select\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Persel {
    #[doc = "0: Timeout period of 9 wdog cycles"]
    Sel0 = 0,
    #[doc = "1: Timeout period of 17 wdog cycles"]
    Sel1 = 1,
    #[doc = "2: Timeout period of 33 wdog cycles"]
    Sel2 = 2,
    #[doc = "3: Timeout period of 65 wdog cycles"]
    Sel3 = 3,
    #[doc = "4: Timeout period of 129 wdog cycles"]
    Sel4 = 4,
    #[doc = "5: Timeout period of 257 wdog cycles"]
    Sel5 = 5,
    #[doc = "6: Timeout period of 513 wdog cycles"]
    Sel6 = 6,
    #[doc = "7: Timeout period of 1k wdog cycles"]
    Sel7 = 7,
    #[doc = "8: Timeout period of 2k wdog cycles"]
    Sel8 = 8,
    #[doc = "9: Timeout period of 4k wdog cycles"]
    Sel9 = 9,
    #[doc = "10: Timeout period of 8k wdog cycles"]
    Sel10 = 10,
    #[doc = "11: Timeout period of 16k wdog cycles"]
    Sel11 = 11,
    #[doc = "12: Timeout period of 32k wdog cycles"]
    Sel12 = 12,
    #[doc = "13: Timeout period of 64k wdog cycles"]
    Sel13 = 13,
    #[doc = "14: Timeout period of 128k wdog cycles"]
    Sel14 = 14,
    #[doc = "15: Timeout period of 256k wdog cycles"]
    Sel15 = 15,
}
impl From<Persel> for u8 {
    #[inline(always)]
    fn from(variant: Persel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Persel {
    type Ux = u8;
}
impl crate::IsEnum for Persel {}
#[doc = "Field `PERSEL` reader - WDOG Timeout Period Select"]
pub type PerselR = crate::FieldReader<Persel>;
impl PerselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Persel {
        match self.bits {
            0 => Persel::Sel0,
            1 => Persel::Sel1,
            2 => Persel::Sel2,
            3 => Persel::Sel3,
            4 => Persel::Sel4,
            5 => Persel::Sel5,
            6 => Persel::Sel6,
            7 => Persel::Sel7,
            8 => Persel::Sel8,
            9 => Persel::Sel9,
            10 => Persel::Sel10,
            11 => Persel::Sel11,
            12 => Persel::Sel12,
            13 => Persel::Sel13,
            14 => Persel::Sel14,
            15 => Persel::Sel15,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout period of 9 wdog cycles"]
    #[inline(always)]
    pub fn is_sel0(&self) -> bool {
        *self == Persel::Sel0
    }
    #[doc = "Timeout period of 17 wdog cycles"]
    #[inline(always)]
    pub fn is_sel1(&self) -> bool {
        *self == Persel::Sel1
    }
    #[doc = "Timeout period of 33 wdog cycles"]
    #[inline(always)]
    pub fn is_sel2(&self) -> bool {
        *self == Persel::Sel2
    }
    #[doc = "Timeout period of 65 wdog cycles"]
    #[inline(always)]
    pub fn is_sel3(&self) -> bool {
        *self == Persel::Sel3
    }
    #[doc = "Timeout period of 129 wdog cycles"]
    #[inline(always)]
    pub fn is_sel4(&self) -> bool {
        *self == Persel::Sel4
    }
    #[doc = "Timeout period of 257 wdog cycles"]
    #[inline(always)]
    pub fn is_sel5(&self) -> bool {
        *self == Persel::Sel5
    }
    #[doc = "Timeout period of 513 wdog cycles"]
    #[inline(always)]
    pub fn is_sel6(&self) -> bool {
        *self == Persel::Sel6
    }
    #[doc = "Timeout period of 1k wdog cycles"]
    #[inline(always)]
    pub fn is_sel7(&self) -> bool {
        *self == Persel::Sel7
    }
    #[doc = "Timeout period of 2k wdog cycles"]
    #[inline(always)]
    pub fn is_sel8(&self) -> bool {
        *self == Persel::Sel8
    }
    #[doc = "Timeout period of 4k wdog cycles"]
    #[inline(always)]
    pub fn is_sel9(&self) -> bool {
        *self == Persel::Sel9
    }
    #[doc = "Timeout period of 8k wdog cycles"]
    #[inline(always)]
    pub fn is_sel10(&self) -> bool {
        *self == Persel::Sel10
    }
    #[doc = "Timeout period of 16k wdog cycles"]
    #[inline(always)]
    pub fn is_sel11(&self) -> bool {
        *self == Persel::Sel11
    }
    #[doc = "Timeout period of 32k wdog cycles"]
    #[inline(always)]
    pub fn is_sel12(&self) -> bool {
        *self == Persel::Sel12
    }
    #[doc = "Timeout period of 64k wdog cycles"]
    #[inline(always)]
    pub fn is_sel13(&self) -> bool {
        *self == Persel::Sel13
    }
    #[doc = "Timeout period of 128k wdog cycles"]
    #[inline(always)]
    pub fn is_sel14(&self) -> bool {
        *self == Persel::Sel14
    }
    #[doc = "Timeout period of 256k wdog cycles"]
    #[inline(always)]
    pub fn is_sel15(&self) -> bool {
        *self == Persel::Sel15
    }
}
#[doc = "Field `PERSEL` writer - WDOG Timeout Period Select"]
pub type PerselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Persel, crate::Safe>;
impl<'a, REG> PerselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period of 9 wdog cycles"]
    #[inline(always)]
    pub fn sel0(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel0)
    }
    #[doc = "Timeout period of 17 wdog cycles"]
    #[inline(always)]
    pub fn sel1(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel1)
    }
    #[doc = "Timeout period of 33 wdog cycles"]
    #[inline(always)]
    pub fn sel2(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel2)
    }
    #[doc = "Timeout period of 65 wdog cycles"]
    #[inline(always)]
    pub fn sel3(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel3)
    }
    #[doc = "Timeout period of 129 wdog cycles"]
    #[inline(always)]
    pub fn sel4(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel4)
    }
    #[doc = "Timeout period of 257 wdog cycles"]
    #[inline(always)]
    pub fn sel5(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel5)
    }
    #[doc = "Timeout period of 513 wdog cycles"]
    #[inline(always)]
    pub fn sel6(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel6)
    }
    #[doc = "Timeout period of 1k wdog cycles"]
    #[inline(always)]
    pub fn sel7(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel7)
    }
    #[doc = "Timeout period of 2k wdog cycles"]
    #[inline(always)]
    pub fn sel8(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel8)
    }
    #[doc = "Timeout period of 4k wdog cycles"]
    #[inline(always)]
    pub fn sel9(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel9)
    }
    #[doc = "Timeout period of 8k wdog cycles"]
    #[inline(always)]
    pub fn sel10(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel10)
    }
    #[doc = "Timeout period of 16k wdog cycles"]
    #[inline(always)]
    pub fn sel11(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel11)
    }
    #[doc = "Timeout period of 32k wdog cycles"]
    #[inline(always)]
    pub fn sel12(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel12)
    }
    #[doc = "Timeout period of 64k wdog cycles"]
    #[inline(always)]
    pub fn sel13(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel13)
    }
    #[doc = "Timeout period of 128k wdog cycles"]
    #[inline(always)]
    pub fn sel14(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel14)
    }
    #[doc = "Timeout period of 256k wdog cycles"]
    #[inline(always)]
    pub fn sel15(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Sel15)
    }
}
#[doc = "WDOG Warning Period Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Warnsel {
    #[doc = "0: Disable"]
    Dis = 0,
    #[doc = "1: Warning timeout is 25% of the Timeout."]
    Sel1 = 1,
    #[doc = "2: Warning timeout is 50% of the Timeout."]
    Sel2 = 2,
    #[doc = "3: Warning timeout is 75% of the Timeout."]
    Sel3 = 3,
}
impl From<Warnsel> for u8 {
    #[inline(always)]
    fn from(variant: Warnsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Warnsel {
    type Ux = u8;
}
impl crate::IsEnum for Warnsel {}
#[doc = "Field `WARNSEL` reader - WDOG Warning Period Select"]
pub type WarnselR = crate::FieldReader<Warnsel>;
impl WarnselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Warnsel {
        match self.bits {
            0 => Warnsel::Dis,
            1 => Warnsel::Sel1,
            2 => Warnsel::Sel2,
            3 => Warnsel::Sel3,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Warnsel::Dis
    }
    #[doc = "Warning timeout is 25% of the Timeout."]
    #[inline(always)]
    pub fn is_sel1(&self) -> bool {
        *self == Warnsel::Sel1
    }
    #[doc = "Warning timeout is 50% of the Timeout."]
    #[inline(always)]
    pub fn is_sel2(&self) -> bool {
        *self == Warnsel::Sel2
    }
    #[doc = "Warning timeout is 75% of the Timeout."]
    #[inline(always)]
    pub fn is_sel3(&self) -> bool {
        *self == Warnsel::Sel3
    }
}
#[doc = "Field `WARNSEL` writer - WDOG Warning Period Select"]
pub type WarnselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Warnsel, crate::Safe>;
impl<'a, REG> WarnselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Warnsel::Dis)
    }
    #[doc = "Warning timeout is 25% of the Timeout."]
    #[inline(always)]
    pub fn sel1(self) -> &'a mut crate::W<REG> {
        self.variant(Warnsel::Sel1)
    }
    #[doc = "Warning timeout is 50% of the Timeout."]
    #[inline(always)]
    pub fn sel2(self) -> &'a mut crate::W<REG> {
        self.variant(Warnsel::Sel2)
    }
    #[doc = "Warning timeout is 75% of the Timeout."]
    #[inline(always)]
    pub fn sel3(self) -> &'a mut crate::W<REG> {
        self.variant(Warnsel::Sel3)
    }
}
#[doc = "WDOG Illegal Window Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Winsel {
    #[doc = "0: Disabled."]
    Dis = 0,
    #[doc = "1: Window timeout is 12.5% of the Timeout."]
    Sel1 = 1,
    #[doc = "2: Window timeout is 25% of the Timeout."]
    Sel2 = 2,
    #[doc = "3: Window timeout is 37.5% of the Timeout."]
    Sel3 = 3,
    #[doc = "4: Window timeout is 50% of the Timeout."]
    Sel4 = 4,
    #[doc = "5: Window timeout is 62.5% of the Timeout."]
    Sel5 = 5,
    #[doc = "6: Window timeout is 75.5% of the Timeout."]
    Sel6 = 6,
    #[doc = "7: Window timeout is 87.5% of the Timeout."]
    Sel7 = 7,
}
impl From<Winsel> for u8 {
    #[inline(always)]
    fn from(variant: Winsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Winsel {
    type Ux = u8;
}
impl crate::IsEnum for Winsel {}
#[doc = "Field `WINSEL` reader - WDOG Illegal Window Select"]
pub type WinselR = crate::FieldReader<Winsel>;
impl WinselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Winsel {
        match self.bits {
            0 => Winsel::Dis,
            1 => Winsel::Sel1,
            2 => Winsel::Sel2,
            3 => Winsel::Sel3,
            4 => Winsel::Sel4,
            5 => Winsel::Sel5,
            6 => Winsel::Sel6,
            7 => Winsel::Sel7,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Winsel::Dis
    }
    #[doc = "Window timeout is 12.5% of the Timeout."]
    #[inline(always)]
    pub fn is_sel1(&self) -> bool {
        *self == Winsel::Sel1
    }
    #[doc = "Window timeout is 25% of the Timeout."]
    #[inline(always)]
    pub fn is_sel2(&self) -> bool {
        *self == Winsel::Sel2
    }
    #[doc = "Window timeout is 37.5% of the Timeout."]
    #[inline(always)]
    pub fn is_sel3(&self) -> bool {
        *self == Winsel::Sel3
    }
    #[doc = "Window timeout is 50% of the Timeout."]
    #[inline(always)]
    pub fn is_sel4(&self) -> bool {
        *self == Winsel::Sel4
    }
    #[doc = "Window timeout is 62.5% of the Timeout."]
    #[inline(always)]
    pub fn is_sel5(&self) -> bool {
        *self == Winsel::Sel5
    }
    #[doc = "Window timeout is 75.5% of the Timeout."]
    #[inline(always)]
    pub fn is_sel6(&self) -> bool {
        *self == Winsel::Sel6
    }
    #[doc = "Window timeout is 87.5% of the Timeout."]
    #[inline(always)]
    pub fn is_sel7(&self) -> bool {
        *self == Winsel::Sel7
    }
}
#[doc = "Field `WINSEL` writer - WDOG Illegal Window Select"]
pub type WinselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Winsel, crate::Safe>;
impl<'a, REG> WinselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Winsel::Dis)
    }
    #[doc = "Window timeout is 12.5% of the Timeout."]
    #[inline(always)]
    pub fn sel1(self) -> &'a mut crate::W<REG> {
        self.variant(Winsel::Sel1)
    }
    #[doc = "Window timeout is 25% of the Timeout."]
    #[inline(always)]
    pub fn sel2(self) -> &'a mut crate::W<REG> {
        self.variant(Winsel::Sel2)
    }
    #[doc = "Window timeout is 37.5% of the Timeout."]
    #[inline(always)]
    pub fn sel3(self) -> &'a mut crate::W<REG> {
        self.variant(Winsel::Sel3)
    }
    #[doc = "Window timeout is 50% of the Timeout."]
    #[inline(always)]
    pub fn sel4(self) -> &'a mut crate::W<REG> {
        self.variant(Winsel::Sel4)
    }
    #[doc = "Window timeout is 62.5% of the Timeout."]
    #[inline(always)]
    pub fn sel5(self) -> &'a mut crate::W<REG> {
        self.variant(Winsel::Sel5)
    }
    #[doc = "Window timeout is 75.5% of the Timeout."]
    #[inline(always)]
    pub fn sel6(self) -> &'a mut crate::W<REG> {
        self.variant(Winsel::Sel6)
    }
    #[doc = "Window timeout is 87.5% of the Timeout."]
    #[inline(always)]
    pub fn sel7(self) -> &'a mut crate::W<REG> {
        self.variant(Winsel::Sel7)
    }
}
impl R {
    #[doc = "Bit 0 - WDOG Clear Source"]
    #[inline(always)]
    pub fn clrsrc(&self) -> ClrsrcR {
        ClrsrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EM1 Run"]
    #[inline(always)]
    pub fn em1run(&self) -> Em1runR {
        Em1runR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EM2 Run"]
    #[inline(always)]
    pub fn em2run(&self) -> Em2runR {
        Em2runR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EM3 Run"]
    #[inline(always)]
    pub fn em3run(&self) -> Em3runR {
        Em3runR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EM4 Block"]
    #[inline(always)]
    pub fn em4block(&self) -> Em4blockR {
        Em4blockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Debug Mode Run"]
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - WDOG Reset Disable"]
    #[inline(always)]
    pub fn wdogrstdis(&self) -> WdogrstdisR {
        WdogrstdisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PRS Src0 Missing Event WDOG Reset"]
    #[inline(always)]
    pub fn prs0missrsten(&self) -> Prs0missrstenR {
        Prs0missrstenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PRS Src1 Missing Event WDOG Reset"]
    #[inline(always)]
    pub fn prs1missrsten(&self) -> Prs1missrstenR {
        Prs1missrstenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:19 - WDOG Timeout Period Select"]
    #[inline(always)]
    pub fn persel(&self) -> PerselR {
        PerselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - WDOG Warning Period Select"]
    #[inline(always)]
    pub fn warnsel(&self) -> WarnselR {
        WarnselR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:30 - WDOG Illegal Window Select"]
    #[inline(always)]
    pub fn winsel(&self) -> WinselR {
        WinselR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - WDOG Clear Source"]
    #[inline(always)]
    pub fn clrsrc(&mut self) -> ClrsrcW<CfgSpec> {
        ClrsrcW::new(self, 0)
    }
    #[doc = "Bit 1 - EM1 Run"]
    #[inline(always)]
    pub fn em1run(&mut self) -> Em1runW<CfgSpec> {
        Em1runW::new(self, 1)
    }
    #[doc = "Bit 2 - EM2 Run"]
    #[inline(always)]
    pub fn em2run(&mut self) -> Em2runW<CfgSpec> {
        Em2runW::new(self, 2)
    }
    #[doc = "Bit 3 - EM3 Run"]
    #[inline(always)]
    pub fn em3run(&mut self) -> Em3runW<CfgSpec> {
        Em3runW::new(self, 3)
    }
    #[doc = "Bit 4 - EM4 Block"]
    #[inline(always)]
    pub fn em4block(&mut self) -> Em4blockW<CfgSpec> {
        Em4blockW::new(self, 4)
    }
    #[doc = "Bit 5 - Debug Mode Run"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DebugrunW<CfgSpec> {
        DebugrunW::new(self, 5)
    }
    #[doc = "Bit 8 - WDOG Reset Disable"]
    #[inline(always)]
    pub fn wdogrstdis(&mut self) -> WdogrstdisW<CfgSpec> {
        WdogrstdisW::new(self, 8)
    }
    #[doc = "Bit 9 - PRS Src0 Missing Event WDOG Reset"]
    #[inline(always)]
    pub fn prs0missrsten(&mut self) -> Prs0missrstenW<CfgSpec> {
        Prs0missrstenW::new(self, 9)
    }
    #[doc = "Bit 10 - PRS Src1 Missing Event WDOG Reset"]
    #[inline(always)]
    pub fn prs1missrsten(&mut self) -> Prs1missrstenW<CfgSpec> {
        Prs1missrstenW::new(self, 10)
    }
    #[doc = "Bits 16:19 - WDOG Timeout Period Select"]
    #[inline(always)]
    pub fn persel(&mut self) -> PerselW<CfgSpec> {
        PerselW::new(self, 16)
    }
    #[doc = "Bits 24:25 - WDOG Warning Period Select"]
    #[inline(always)]
    pub fn warnsel(&mut self) -> WarnselW<CfgSpec> {
        WarnselW::new(self, 24)
    }
    #[doc = "Bits 28:30 - WDOG Illegal Window Select"]
    #[inline(always)]
    pub fn winsel(&mut self) -> WinselW<CfgSpec> {
        WinselW::new(self, 28)
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
#[doc = "`reset()` method sets CFG to value 0x000f_0000"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x000f_0000;
}
