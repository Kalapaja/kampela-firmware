#[doc = "Register `CFG2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "Main mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Master {
    #[doc = "0: Secondary mode"]
    Slave = 0,
    #[doc = "1: Main mode"]
    Master = 1,
}
impl From<Master> for bool {
    #[inline(always)]
    fn from(variant: Master) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTER` reader - Main mode"]
pub type MasterR = crate::BitReader<Master>;
impl MasterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Master {
        match self.bits {
            false => Master::Slave,
            true => Master::Master,
        }
    }
    #[doc = "Secondary mode"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Master::Slave
    }
    #[doc = "Main mode"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Master::Master
    }
}
#[doc = "Field `MASTER` writer - Main mode"]
pub type MasterW<'a, REG> = crate::BitWriter<'a, REG, Master>;
impl<'a, REG> MasterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Secondary mode"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Master::Slave)
    }
    #[doc = "Main mode"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Master::Master)
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
#[doc = "Clock Edge for Setup/Sample\n\nValue on reset: 0"]
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
#[doc = "Field `CLKPHA` reader - Clock Edge for Setup/Sample"]
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
#[doc = "Field `CLKPHA` writer - Clock Edge for Setup/Sample"]
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
#[doc = "Chip Select Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csinv {
    #[doc = "0: Chip select is active low"]
    Al = 0,
    #[doc = "1: Chip select is active high"]
    Ah = 1,
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
            false => Csinv::Al,
            true => Csinv::Ah,
        }
    }
    #[doc = "Chip select is active low"]
    #[inline(always)]
    pub fn is_al(&self) -> bool {
        *self == Csinv::Al
    }
    #[doc = "Chip select is active high"]
    #[inline(always)]
    pub fn is_ah(&self) -> bool {
        *self == Csinv::Ah
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
    pub fn al(self) -> &'a mut crate::W<REG> {
        self.variant(Csinv::Al)
    }
    #[doc = "Chip select is active high"]
    #[inline(always)]
    pub fn ah(self) -> &'a mut crate::W<REG> {
        self.variant(Csinv::Ah)
    }
}
#[doc = "Field `AUTOTX` reader - Always Transmit When RXFIFO Not Full"]
pub type AutotxR = crate::BitReader;
#[doc = "Field `AUTOTX` writer - Always Transmit When RXFIFO Not Full"]
pub type AutotxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCS` reader - Automatic Chip Select"]
pub type AutocsR = crate::BitReader;
#[doc = "Field `AUTOCS` writer - Automatic Chip Select"]
pub type AutocsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPRSEN` reader - PRS CLK Enable"]
pub type ClkprsenR = crate::BitReader;
#[doc = "Field `CLKPRSEN` writer - PRS CLK Enable"]
pub type ClkprsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCELOAD` reader - Force Load to Shift Register"]
pub type ForceloadR = crate::BitReader;
#[doc = "Field `FORCELOAD` writer - Force Load to Shift Register"]
pub type ForceloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIV` reader - Sync Clock Div"]
pub type SdivR = crate::FieldReader;
#[doc = "Field `SDIV` writer - Sync Clock Div"]
pub type SdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Main mode"]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> ClkpolR {
        ClkpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Edge for Setup/Sample"]
    #[inline(always)]
    pub fn clkpha(&self) -> ClkphaR {
        ClkphaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Chip Select Invert"]
    #[inline(always)]
    pub fn csinv(&self) -> CsinvR {
        CsinvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Always Transmit When RXFIFO Not Full"]
    #[inline(always)]
    pub fn autotx(&self) -> AutotxR {
        AutotxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Chip Select"]
    #[inline(always)]
    pub fn autocs(&self) -> AutocsR {
        AutocsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PRS CLK Enable"]
    #[inline(always)]
    pub fn clkprsen(&self) -> ClkprsenR {
        ClkprsenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Load to Shift Register"]
    #[inline(always)]
    pub fn forceload(&self) -> ForceloadR {
        ForceloadR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Sync Clock Div"]
    #[inline(always)]
    pub fn sdiv(&self) -> SdivR {
        SdivR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Main mode"]
    #[inline(always)]
    pub fn master(&mut self) -> MasterW<Cfg2Spec> {
        MasterW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Polarity"]
    #[inline(always)]
    pub fn clkpol(&mut self) -> ClkpolW<Cfg2Spec> {
        ClkpolW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock Edge for Setup/Sample"]
    #[inline(always)]
    pub fn clkpha(&mut self) -> ClkphaW<Cfg2Spec> {
        ClkphaW::new(self, 2)
    }
    #[doc = "Bit 3 - Chip Select Invert"]
    #[inline(always)]
    pub fn csinv(&mut self) -> CsinvW<Cfg2Spec> {
        CsinvW::new(self, 3)
    }
    #[doc = "Bit 4 - Always Transmit When RXFIFO Not Full"]
    #[inline(always)]
    pub fn autotx(&mut self) -> AutotxW<Cfg2Spec> {
        AutotxW::new(self, 4)
    }
    #[doc = "Bit 5 - Automatic Chip Select"]
    #[inline(always)]
    pub fn autocs(&mut self) -> AutocsW<Cfg2Spec> {
        AutocsW::new(self, 5)
    }
    #[doc = "Bit 6 - PRS CLK Enable"]
    #[inline(always)]
    pub fn clkprsen(&mut self) -> ClkprsenW<Cfg2Spec> {
        ClkprsenW::new(self, 6)
    }
    #[doc = "Bit 7 - Force Load to Shift Register"]
    #[inline(always)]
    pub fn forceload(&mut self) -> ForceloadW<Cfg2Spec> {
        ForceloadW::new(self, 7)
    }
    #[doc = "Bits 24:31 - Sync Clock Div"]
    #[inline(always)]
    pub fn sdiv(&mut self) -> SdivW<Cfg2Spec> {
        SdivW::new(self, 24)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg2Spec;
impl crate::RegisterSpec for Cfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for Cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for Cfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG2 to value 0x20"]
impl crate::Resettable for Cfg2Spec {
    const RESET_VALUE: u32 = 0x20;
}
