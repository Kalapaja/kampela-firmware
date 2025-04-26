#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `AGC` reader - LFXO AGC Enable"]
pub type AgcR = crate::BitReader;
#[doc = "Field `AGC` writer - LFXO AGC Enable"]
pub type AgcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGHAMPL` reader - LFXO High Amplitude Enable"]
pub type HighamplR = crate::BitReader;
#[doc = "Field `HIGHAMPL` writer - LFXO High Amplitude Enable"]
pub type HighamplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "LFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: A 32768Hz crystal should be connected to the LF crystal pads. Voltage must not exceed VDDIO."]
    Xtal = 0,
    #[doc = "1: An external sine source with minimum amplitude 100mv (zero-to-peak) and maximum amplitude 500mV (zero-to-peak) should be connected in series with LFXTAL_I pin. Minimum voltage should be larger than ground and maximum voltage smaller than VDDIO. The sine source does not need to be ac coupled externally as it is ac couples inside LFXO. LFXTAL_O is free to be used as a general purpose GPIO."]
    Bufextclk = 1,
    #[doc = "2: An external 32KHz CMOS clock should be provided on LFXTAL_I. LFXTAL_O is free to be used as a general purpose GPIO."]
    Digextclk = 2,
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
#[doc = "Field `MODE` reader - LFXO Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Xtal),
            1 => Some(Mode::Bufextclk),
            2 => Some(Mode::Digextclk),
            _ => None,
        }
    }
    #[doc = "A 32768Hz crystal should be connected to the LF crystal pads. Voltage must not exceed VDDIO."]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == Mode::Xtal
    }
    #[doc = "An external sine source with minimum amplitude 100mv (zero-to-peak) and maximum amplitude 500mV (zero-to-peak) should be connected in series with LFXTAL_I pin. Minimum voltage should be larger than ground and maximum voltage smaller than VDDIO. The sine source does not need to be ac coupled externally as it is ac couples inside LFXO. LFXTAL_O is free to be used as a general purpose GPIO."]
    #[inline(always)]
    pub fn is_bufextclk(&self) -> bool {
        *self == Mode::Bufextclk
    }
    #[doc = "An external 32KHz CMOS clock should be provided on LFXTAL_I. LFXTAL_O is free to be used as a general purpose GPIO."]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == Mode::Digextclk
    }
}
#[doc = "Field `MODE` writer - LFXO Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A 32768Hz crystal should be connected to the LF crystal pads. Voltage must not exceed VDDIO."]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Xtal)
    }
    #[doc = "An external sine source with minimum amplitude 100mv (zero-to-peak) and maximum amplitude 500mV (zero-to-peak) should be connected in series with LFXTAL_I pin. Minimum voltage should be larger than ground and maximum voltage smaller than VDDIO. The sine source does not need to be ac coupled externally as it is ac couples inside LFXO. LFXTAL_O is free to be used as a general purpose GPIO."]
    #[inline(always)]
    pub fn bufextclk(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Bufextclk)
    }
    #[doc = "An external 32KHz CMOS clock should be provided on LFXTAL_I. LFXTAL_O is free to be used as a general purpose GPIO."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Digextclk)
    }
}
#[doc = "LFXO Start-up Delay\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timeout {
    #[doc = "0: Timeout period of 2 cycles"]
    Cycles2 = 0,
    #[doc = "1: Timeout period of 256 cycles"]
    Cycles256 = 1,
    #[doc = "2: Timeout period of 1024 cycles"]
    Cycles1k = 2,
    #[doc = "3: Timeout period of 2048 cycles"]
    Cycles2k = 3,
    #[doc = "4: Timeout period of 4096 cycles"]
    Cycles4k = 4,
    #[doc = "5: Timeout period of 8192 cycles"]
    Cycles8k = 5,
    #[doc = "6: Timeout period of 16384 cycles"]
    Cycles16k = 6,
    #[doc = "7: Timeout period of 32768 cycles"]
    Cycles32k = 7,
}
impl From<Timeout> for u8 {
    #[inline(always)]
    fn from(variant: Timeout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timeout {
    type Ux = u8;
}
impl crate::IsEnum for Timeout {}
#[doc = "Field `TIMEOUT` reader - LFXO Start-up Delay"]
pub type TimeoutR = crate::FieldReader<Timeout>;
impl TimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timeout {
        match self.bits {
            0 => Timeout::Cycles2,
            1 => Timeout::Cycles256,
            2 => Timeout::Cycles1k,
            3 => Timeout::Cycles2k,
            4 => Timeout::Cycles4k,
            5 => Timeout::Cycles8k,
            6 => Timeout::Cycles16k,
            7 => Timeout::Cycles32k,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        *self == Timeout::Cycles2
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn is_cycles256(&self) -> bool {
        *self == Timeout::Cycles256
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn is_cycles1k(&self) -> bool {
        *self == Timeout::Cycles1k
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn is_cycles2k(&self) -> bool {
        *self == Timeout::Cycles2k
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn is_cycles4k(&self) -> bool {
        *self == Timeout::Cycles4k
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn is_cycles8k(&self) -> bool {
        *self == Timeout::Cycles8k
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn is_cycles16k(&self) -> bool {
        *self == Timeout::Cycles16k
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn is_cycles32k(&self) -> bool {
        *self == Timeout::Cycles32k
    }
}
#[doc = "Field `TIMEOUT` writer - LFXO Start-up Delay"]
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 3, Timeout, crate::Safe>;
impl<'a, REG> TimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period of 2 cycles"]
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::Cycles2)
    }
    #[doc = "Timeout period of 256 cycles"]
    #[inline(always)]
    pub fn cycles256(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::Cycles256)
    }
    #[doc = "Timeout period of 1024 cycles"]
    #[inline(always)]
    pub fn cycles1k(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::Cycles1k)
    }
    #[doc = "Timeout period of 2048 cycles"]
    #[inline(always)]
    pub fn cycles2k(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::Cycles2k)
    }
    #[doc = "Timeout period of 4096 cycles"]
    #[inline(always)]
    pub fn cycles4k(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::Cycles4k)
    }
    #[doc = "Timeout period of 8192 cycles"]
    #[inline(always)]
    pub fn cycles8k(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::Cycles8k)
    }
    #[doc = "Timeout period of 16384 cycles"]
    #[inline(always)]
    pub fn cycles16k(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::Cycles16k)
    }
    #[doc = "Timeout period of 32768 cycles"]
    #[inline(always)]
    pub fn cycles32k(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::Cycles32k)
    }
}
impl R {
    #[doc = "Bit 0 - LFXO AGC Enable"]
    #[inline(always)]
    pub fn agc(&self) -> AgcR {
        AgcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LFXO High Amplitude Enable"]
    #[inline(always)]
    pub fn highampl(&self) -> HighamplR {
        HighamplR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - LFXO Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - LFXO Start-up Delay"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LFXO AGC Enable"]
    #[inline(always)]
    pub fn agc(&mut self) -> AgcW<CfgSpec> {
        AgcW::new(self, 0)
    }
    #[doc = "Bit 1 - LFXO High Amplitude Enable"]
    #[inline(always)]
    pub fn highampl(&mut self) -> HighamplW<CfgSpec> {
        HighamplW::new(self, 1)
    }
    #[doc = "Bits 4:5 - LFXO Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<CfgSpec> {
        ModeW::new(self, 4)
    }
    #[doc = "Bits 8:10 - LFXO Start-up Delay"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<CfgSpec> {
        TimeoutW::new(self, 8)
    }
}
#[doc = "Do not write to this register unless the oscillator is forced off. The oscillator is forced off if DISONDEMAND is set and FORCEEN is cleared.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CFG to value 0x0701"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x0701;
}
