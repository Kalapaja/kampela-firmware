#[doc = "Register `RSTCTRL` reader"]
pub type R = crate::R<RstctrlSpec>;
#[doc = "Register `RSTCTRL` writer"]
pub type W = crate::W<RstctrlSpec>;
#[doc = "Enable WDOG0 reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdog0rmode {
    #[doc = "0: Reset request is blocked"]
    Disabled = 0,
    #[doc = "1: The entire device is reset except some EMU registers"]
    Enabled = 1,
}
impl From<Wdog0rmode> for bool {
    #[inline(always)]
    fn from(variant: Wdog0rmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDOG0RMODE` reader - Enable WDOG0 reset"]
pub type Wdog0rmodeR = crate::BitReader<Wdog0rmode>;
impl Wdog0rmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdog0rmode {
        match self.bits {
            false => Wdog0rmode::Disabled,
            true => Wdog0rmode::Enabled,
        }
    }
    #[doc = "Reset request is blocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wdog0rmode::Disabled
    }
    #[doc = "The entire device is reset except some EMU registers"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wdog0rmode::Enabled
    }
}
#[doc = "Field `WDOG0RMODE` writer - Enable WDOG0 reset"]
pub type Wdog0rmodeW<'a, REG> = crate::BitWriter<'a, REG, Wdog0rmode>;
impl<'a, REG> Wdog0rmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset request is blocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wdog0rmode::Disabled)
    }
    #[doc = "The entire device is reset except some EMU registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wdog0rmode::Enabled)
    }
}
#[doc = "Enable M33 System reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysrmode {
    #[doc = "0: Reset request is blocked"]
    Disabled = 0,
    #[doc = "1: Device is reset except some EMU registers"]
    Enabled = 1,
}
impl From<Sysrmode> for bool {
    #[inline(always)]
    fn from(variant: Sysrmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRMODE` reader - Enable M33 System reset"]
pub type SysrmodeR = crate::BitReader<Sysrmode>;
impl SysrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysrmode {
        match self.bits {
            false => Sysrmode::Disabled,
            true => Sysrmode::Enabled,
        }
    }
    #[doc = "Reset request is blocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sysrmode::Disabled
    }
    #[doc = "Device is reset except some EMU registers"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sysrmode::Enabled
    }
}
#[doc = "Field `SYSRMODE` writer - Enable M33 System reset"]
pub type SysrmodeW<'a, REG> = crate::BitWriter<'a, REG, Sysrmode>;
impl<'a, REG> SysrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset request is blocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrmode::Disabled)
    }
    #[doc = "Device is reset except some EMU registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrmode::Enabled)
    }
}
#[doc = "Enable M33 Lockup reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockuprmode {
    #[doc = "0: Reset Request is Block"]
    Disabled = 0,
    #[doc = "1: The entire device is reset except some EMU registers"]
    Enabled = 1,
}
impl From<Lockuprmode> for bool {
    #[inline(always)]
    fn from(variant: Lockuprmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKUPRMODE` reader - Enable M33 Lockup reset"]
pub type LockuprmodeR = crate::BitReader<Lockuprmode>;
impl LockuprmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockuprmode {
        match self.bits {
            false => Lockuprmode::Disabled,
            true => Lockuprmode::Enabled,
        }
    }
    #[doc = "Reset Request is Block"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lockuprmode::Disabled
    }
    #[doc = "The entire device is reset except some EMU registers"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lockuprmode::Enabled
    }
}
#[doc = "Field `LOCKUPRMODE` writer - Enable M33 Lockup reset"]
pub type LockuprmodeW<'a, REG> = crate::BitWriter<'a, REG, Lockuprmode>;
impl<'a, REG> LockuprmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset Request is Block"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lockuprmode::Disabled)
    }
    #[doc = "The entire device is reset except some EMU registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lockuprmode::Enabled)
    }
}
#[doc = "Enable AVDD BOD reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Avddbodrmode {
    #[doc = "0: Reset Request is block"]
    Disabled = 0,
    #[doc = "1: The entire device is reset except some EMU registers"]
    Enabled = 1,
}
impl From<Avddbodrmode> for bool {
    #[inline(always)]
    fn from(variant: Avddbodrmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVDDBODRMODE` reader - Enable AVDD BOD reset"]
pub type AvddbodrmodeR = crate::BitReader<Avddbodrmode>;
impl AvddbodrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avddbodrmode {
        match self.bits {
            false => Avddbodrmode::Disabled,
            true => Avddbodrmode::Enabled,
        }
    }
    #[doc = "Reset Request is block"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Avddbodrmode::Disabled
    }
    #[doc = "The entire device is reset except some EMU registers"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Avddbodrmode::Enabled
    }
}
#[doc = "Field `AVDDBODRMODE` writer - Enable AVDD BOD reset"]
pub type AvddbodrmodeW<'a, REG> = crate::BitWriter<'a, REG, Avddbodrmode>;
impl<'a, REG> AvddbodrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset Request is block"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Avddbodrmode::Disabled)
    }
    #[doc = "The entire device is reset except some EMU registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Avddbodrmode::Enabled)
    }
}
#[doc = "Enable VDDIO0 BOD reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iovdd0bodrmode {
    #[doc = "0: Reset request is blocked"]
    Disabled = 0,
    #[doc = "1: The entire device is reset except some EMU registers"]
    Enabled = 1,
}
impl From<Iovdd0bodrmode> for bool {
    #[inline(always)]
    fn from(variant: Iovdd0bodrmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOVDD0BODRMODE` reader - Enable VDDIO0 BOD reset"]
pub type Iovdd0bodrmodeR = crate::BitReader<Iovdd0bodrmode>;
impl Iovdd0bodrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iovdd0bodrmode {
        match self.bits {
            false => Iovdd0bodrmode::Disabled,
            true => Iovdd0bodrmode::Enabled,
        }
    }
    #[doc = "Reset request is blocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Iovdd0bodrmode::Disabled
    }
    #[doc = "The entire device is reset except some EMU registers"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Iovdd0bodrmode::Enabled
    }
}
#[doc = "Field `IOVDD0BODRMODE` writer - Enable VDDIO0 BOD reset"]
pub type Iovdd0bodrmodeW<'a, REG> = crate::BitWriter<'a, REG, Iovdd0bodrmode>;
impl<'a, REG> Iovdd0bodrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset request is blocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Iovdd0bodrmode::Disabled)
    }
    #[doc = "The entire device is reset except some EMU registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Iovdd0bodrmode::Enabled)
    }
}
#[doc = "Enable DECBOD reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decbodrmode {
    #[doc = "0: Reset request is blocked"]
    Disabled = 0,
    #[doc = "1: The entire device is reset"]
    Enabled = 1,
}
impl From<Decbodrmode> for bool {
    #[inline(always)]
    fn from(variant: Decbodrmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DECBODRMODE` reader - Enable DECBOD reset"]
pub type DecbodrmodeR = crate::BitReader<Decbodrmode>;
impl DecbodrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Decbodrmode {
        match self.bits {
            false => Decbodrmode::Disabled,
            true => Decbodrmode::Enabled,
        }
    }
    #[doc = "Reset request is blocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Decbodrmode::Disabled
    }
    #[doc = "The entire device is reset"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Decbodrmode::Enabled
    }
}
#[doc = "Field `DECBODRMODE` writer - Enable DECBOD reset"]
pub type DecbodrmodeW<'a, REG> = crate::BitWriter<'a, REG, Decbodrmode>;
impl<'a, REG> DecbodrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset request is blocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Decbodrmode::Disabled)
    }
    #[doc = "The entire device is reset"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Decbodrmode::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable WDOG0 reset"]
    #[inline(always)]
    pub fn wdog0rmode(&self) -> Wdog0rmodeR {
        Wdog0rmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Enable M33 System reset"]
    #[inline(always)]
    pub fn sysrmode(&self) -> SysrmodeR {
        SysrmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable M33 Lockup reset"]
    #[inline(always)]
    pub fn lockuprmode(&self) -> LockuprmodeR {
        LockuprmodeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable AVDD BOD reset"]
    #[inline(always)]
    pub fn avddbodrmode(&self) -> AvddbodrmodeR {
        AvddbodrmodeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable VDDIO0 BOD reset"]
    #[inline(always)]
    pub fn iovdd0bodrmode(&self) -> Iovdd0bodrmodeR {
        Iovdd0bodrmodeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable DECBOD reset"]
    #[inline(always)]
    pub fn decbodrmode(&self) -> DecbodrmodeR {
        DecbodrmodeR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable WDOG0 reset"]
    #[inline(always)]
    pub fn wdog0rmode(&mut self) -> Wdog0rmodeW<RstctrlSpec> {
        Wdog0rmodeW::new(self, 0)
    }
    #[doc = "Bit 2 - Enable M33 System reset"]
    #[inline(always)]
    pub fn sysrmode(&mut self) -> SysrmodeW<RstctrlSpec> {
        SysrmodeW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable M33 Lockup reset"]
    #[inline(always)]
    pub fn lockuprmode(&mut self) -> LockuprmodeW<RstctrlSpec> {
        LockuprmodeW::new(self, 3)
    }
    #[doc = "Bit 6 - Enable AVDD BOD reset"]
    #[inline(always)]
    pub fn avddbodrmode(&mut self) -> AvddbodrmodeW<RstctrlSpec> {
        AvddbodrmodeW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable VDDIO0 BOD reset"]
    #[inline(always)]
    pub fn iovdd0bodrmode(&mut self) -> Iovdd0bodrmodeW<RstctrlSpec> {
        Iovdd0bodrmodeW::new(self, 7)
    }
    #[doc = "Bit 10 - Enable DECBOD reset"]
    #[inline(always)]
    pub fn decbodrmode(&mut self) -> DecbodrmodeW<RstctrlSpec> {
        DecbodrmodeW::new(self, 10)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctrlSpec;
impl crate::RegisterSpec for RstctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctrl::R`](R) reader structure"]
impl crate::Readable for RstctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rstctrl::W`](W) writer structure"]
impl crate::Writable for RstctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSTCTRL to value 0x0006_0407"]
impl crate::Resettable for RstctrlSpec {
    const RESET_VALUE: u32 = 0x0006_0407;
}
