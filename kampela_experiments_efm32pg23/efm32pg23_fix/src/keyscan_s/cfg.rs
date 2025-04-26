#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `CLKDIV` reader - Clock Divider"]
pub type ClkdivR = crate::FieldReader<u32>;
#[doc = "Field `CLKDIV` writer - Clock Divider"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Single Press\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Singlepress {
    #[doc = "0: After KEYIF is set and then cleared, scanning will continue. This can give multiple interrupts for the same key press, but allow multiple key presses to be detected. To use this mode for multi-key detection, the ISR should update a section of memory of COLNUM bytes on each interrupt, until key release is detected. After key release, the section of memory where key presses are recorded can be processed."]
    Multipress = 0,
    #[doc = "1: After KEYIF has been set and cleared, it will not set again until no key press is detected. This allows faster response since the ISR can start processing data as soon as the KEYIF is set."]
    Singlepress = 1,
}
impl From<Singlepress> for bool {
    #[inline(always)]
    fn from(variant: Singlepress) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINGLEPRESS` reader - Single Press"]
pub type SinglepressR = crate::BitReader<Singlepress>;
impl SinglepressR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Singlepress {
        match self.bits {
            false => Singlepress::Multipress,
            true => Singlepress::Singlepress,
        }
    }
    #[doc = "After KEYIF is set and then cleared, scanning will continue. This can give multiple interrupts for the same key press, but allow multiple key presses to be detected. To use this mode for multi-key detection, the ISR should update a section of memory of COLNUM bytes on each interrupt, until key release is detected. After key release, the section of memory where key presses are recorded can be processed."]
    #[inline(always)]
    pub fn is_multipress(&self) -> bool {
        *self == Singlepress::Multipress
    }
    #[doc = "After KEYIF has been set and cleared, it will not set again until no key press is detected. This allows faster response since the ISR can start processing data as soon as the KEYIF is set."]
    #[inline(always)]
    pub fn is_singlepress(&self) -> bool {
        *self == Singlepress::Singlepress
    }
}
#[doc = "Field `SINGLEPRESS` writer - Single Press"]
pub type SinglepressW<'a, REG> = crate::BitWriter<'a, REG, Singlepress>;
impl<'a, REG> SinglepressW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "After KEYIF is set and then cleared, scanning will continue. This can give multiple interrupts for the same key press, but allow multiple key presses to be detected. To use this mode for multi-key detection, the ISR should update a section of memory of COLNUM bytes on each interrupt, until key release is detected. After key release, the section of memory where key presses are recorded can be processed."]
    #[inline(always)]
    pub fn multipress(self) -> &'a mut crate::W<REG> {
        self.variant(Singlepress::Multipress)
    }
    #[doc = "After KEYIF has been set and cleared, it will not set again until no key press is detected. This allows faster response since the ISR can start processing data as soon as the KEYIF is set."]
    #[inline(always)]
    pub fn singlepress(self) -> &'a mut crate::W<REG> {
        self.variant(Singlepress::Singlepress)
    }
}
#[doc = "Automatically Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autostart {
    #[doc = "0: Auto start is disabled"]
    Autostartdis = 0,
    #[doc = "1: Auto start is enabled"]
    Autostarten = 1,
}
impl From<Autostart> for bool {
    #[inline(always)]
    fn from(variant: Autostart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOSTART` reader - Automatically Start"]
pub type AutostartR = crate::BitReader<Autostart>;
impl AutostartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autostart {
        match self.bits {
            false => Autostart::Autostartdis,
            true => Autostart::Autostarten,
        }
    }
    #[doc = "Auto start is disabled"]
    #[inline(always)]
    pub fn is_autostartdis(&self) -> bool {
        *self == Autostart::Autostartdis
    }
    #[doc = "Auto start is enabled"]
    #[inline(always)]
    pub fn is_autostarten(&self) -> bool {
        *self == Autostart::Autostarten
    }
}
#[doc = "Field `AUTOSTART` writer - Automatically Start"]
pub type AutostartW<'a, REG> = crate::BitWriter<'a, REG, Autostart>;
impl<'a, REG> AutostartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto start is disabled"]
    #[inline(always)]
    pub fn autostartdis(self) -> &'a mut crate::W<REG> {
        self.variant(Autostart::Autostartdis)
    }
    #[doc = "Auto start is enabled"]
    #[inline(always)]
    pub fn autostarten(self) -> &'a mut crate::W<REG> {
        self.variant(Autostart::Autostarten)
    }
}
#[doc = "Number of Rows\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Numrows {
    #[doc = "0: 1 Row is not supported; defaults to 3 instead"]
    Rsv1 = 0,
    #[doc = "1: 2 Rows are not supported; defaults to 3 instead"]
    Rsv2 = 1,
    #[doc = "2: 3 Rows"]
    Row3 = 2,
    #[doc = "3: 4 Rows"]
    Row4 = 3,
    #[doc = "4: 5 Rows"]
    Row5 = 4,
    #[doc = "5: 6 Rows"]
    Row6 = 5,
}
impl From<Numrows> for u8 {
    #[inline(always)]
    fn from(variant: Numrows) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Numrows {
    type Ux = u8;
}
impl crate::IsEnum for Numrows {}
#[doc = "Field `NUMROWS` reader - Number of Rows"]
pub type NumrowsR = crate::FieldReader<Numrows>;
impl NumrowsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Numrows> {
        match self.bits {
            0 => Some(Numrows::Rsv1),
            1 => Some(Numrows::Rsv2),
            2 => Some(Numrows::Row3),
            3 => Some(Numrows::Row4),
            4 => Some(Numrows::Row5),
            5 => Some(Numrows::Row6),
            _ => None,
        }
    }
    #[doc = "1 Row is not supported; defaults to 3 instead"]
    #[inline(always)]
    pub fn is_rsv1(&self) -> bool {
        *self == Numrows::Rsv1
    }
    #[doc = "2 Rows are not supported; defaults to 3 instead"]
    #[inline(always)]
    pub fn is_rsv2(&self) -> bool {
        *self == Numrows::Rsv2
    }
    #[doc = "3 Rows"]
    #[inline(always)]
    pub fn is_row3(&self) -> bool {
        *self == Numrows::Row3
    }
    #[doc = "4 Rows"]
    #[inline(always)]
    pub fn is_row4(&self) -> bool {
        *self == Numrows::Row4
    }
    #[doc = "5 Rows"]
    #[inline(always)]
    pub fn is_row5(&self) -> bool {
        *self == Numrows::Row5
    }
    #[doc = "6 Rows"]
    #[inline(always)]
    pub fn is_row6(&self) -> bool {
        *self == Numrows::Row6
    }
}
#[doc = "Field `NUMROWS` writer - Number of Rows"]
pub type NumrowsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Numrows>;
impl<'a, REG> NumrowsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 Row is not supported; defaults to 3 instead"]
    #[inline(always)]
    pub fn rsv1(self) -> &'a mut crate::W<REG> {
        self.variant(Numrows::Rsv1)
    }
    #[doc = "2 Rows are not supported; defaults to 3 instead"]
    #[inline(always)]
    pub fn rsv2(self) -> &'a mut crate::W<REG> {
        self.variant(Numrows::Rsv2)
    }
    #[doc = "3 Rows"]
    #[inline(always)]
    pub fn row3(self) -> &'a mut crate::W<REG> {
        self.variant(Numrows::Row3)
    }
    #[doc = "4 Rows"]
    #[inline(always)]
    pub fn row4(self) -> &'a mut crate::W<REG> {
        self.variant(Numrows::Row4)
    }
    #[doc = "5 Rows"]
    #[inline(always)]
    pub fn row5(self) -> &'a mut crate::W<REG> {
        self.variant(Numrows::Row5)
    }
    #[doc = "6 Rows"]
    #[inline(always)]
    pub fn row6(self) -> &'a mut crate::W<REG> {
        self.variant(Numrows::Row6)
    }
}
#[doc = "Field `NUMCOLS` reader - Number of Columns"]
pub type NumcolsR = crate::FieldReader;
#[doc = "Field `NUMCOLS` writer - Number of Columns"]
pub type NumcolsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:17 - Clock Divider"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bit 20 - Single Press"]
    #[inline(always)]
    pub fn singlepress(&self) -> SinglepressR {
        SinglepressR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Automatically Start"]
    #[inline(always)]
    pub fn autostart(&self) -> AutostartR {
        AutostartR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Number of Rows"]
    #[inline(always)]
    pub fn numrows(&self) -> NumrowsR {
        NumrowsR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Number of Columns"]
    #[inline(always)]
    pub fn numcols(&self) -> NumcolsR {
        NumcolsR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - Clock Divider"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<CfgSpec> {
        ClkdivW::new(self, 0)
    }
    #[doc = "Bit 20 - Single Press"]
    #[inline(always)]
    pub fn singlepress(&mut self) -> SinglepressW<CfgSpec> {
        SinglepressW::new(self, 20)
    }
    #[doc = "Bit 22 - Automatically Start"]
    #[inline(always)]
    pub fn autostart(&mut self) -> AutostartW<CfgSpec> {
        AutostartW::new(self, 22)
    }
    #[doc = "Bits 24:26 - Number of Rows"]
    #[inline(always)]
    pub fn numrows(&mut self) -> NumrowsW<CfgSpec> {
        NumrowsW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Number of Columns"]
    #[inline(always)]
    pub fn numcols(&mut self) -> NumcolsW<CfgSpec> {
        NumcolsW::new(self, 28)
    }
}
#[doc = "Config\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CFG to value 0x2501_387f"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x2501_387f;
}
