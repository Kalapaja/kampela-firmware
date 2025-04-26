#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Debug Mode Run Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debugrun {
    #[doc = "0: BURTC is frozen in debug mode"]
    X0 = 0,
    #[doc = "1: BURTC is running in debug mode"]
    X1 = 1,
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
            false => Debugrun::X0,
            true => Debugrun::X1,
        }
    }
    #[doc = "BURTC is frozen in debug mode"]
    #[inline(always)]
    pub fn is_x0(&self) -> bool {
        *self == Debugrun::X0
    }
    #[doc = "BURTC is running in debug mode"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == Debugrun::X1
    }
}
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG, Debugrun>;
impl<'a, REG> DebugrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BURTC is frozen in debug mode"]
    #[inline(always)]
    pub fn x0(self) -> &'a mut crate::W<REG> {
        self.variant(Debugrun::X0)
    }
    #[doc = "BURTC is running in debug mode"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut crate::W<REG> {
        self.variant(Debugrun::X1)
    }
}
#[doc = "Compare Channel is Top Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comptop {
    #[doc = "0: The top value of the BURTC is 4294967295 (0xFFFFFFFF)"]
    Disable = 0,
    #[doc = "1: The top value of the BURTC is given by COMP"]
    Enable = 1,
}
impl From<Comptop> for bool {
    #[inline(always)]
    fn from(variant: Comptop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPTOP` reader - Compare Channel is Top Value"]
pub type ComptopR = crate::BitReader<Comptop>;
impl ComptopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comptop {
        match self.bits {
            false => Comptop::Disable,
            true => Comptop::Enable,
        }
    }
    #[doc = "The top value of the BURTC is 4294967295 (0xFFFFFFFF)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Comptop::Disable
    }
    #[doc = "The top value of the BURTC is given by COMP"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Comptop::Enable
    }
}
#[doc = "Field `COMPTOP` writer - Compare Channel is Top Value"]
pub type ComptopW<'a, REG> = crate::BitWriter<'a, REG, Comptop>;
impl<'a, REG> ComptopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The top value of the BURTC is 4294967295 (0xFFFFFFFF)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Comptop::Disable)
    }
    #[doc = "The top value of the BURTC is given by COMP"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Comptop::Enable)
    }
}
#[doc = "Counter prescaler value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cntpresc {
    #[doc = "0: CLK_CNT = (BURTC LF CLK)/1"]
    Div1 = 0,
    #[doc = "1: CLK_CNT = (BURTC LF CLK)/2"]
    Div2 = 1,
    #[doc = "2: CLK_CNT = (BURTC LF CLK)/4"]
    Div4 = 2,
    #[doc = "3: CLK_CNT = (BURTC LF CLK)/8"]
    Div8 = 3,
    #[doc = "4: CLK_CNT = (BURTC LF CLK)/16"]
    Div16 = 4,
    #[doc = "5: CLK_CNT = (BURTC LF CLK)/32"]
    Div32 = 5,
    #[doc = "6: CLK_CNT = (BURTC LF CLK)/64"]
    Div64 = 6,
    #[doc = "7: CLK_CNT = (BURTC LF CLK)/128"]
    Div128 = 7,
    #[doc = "8: CLK_CNT = (BURTC LF CLK)/256"]
    Div256 = 8,
    #[doc = "9: CLK_CNT = (BURTC LF CLK)/512"]
    Div512 = 9,
    #[doc = "10: CLK_CNT = (BURTC LF CLK)/1024"]
    Div1024 = 10,
    #[doc = "11: CLK_CNT = (BURTC LF CLK)/2048"]
    Div2048 = 11,
    #[doc = "12: CLK_CNT = (BURTC LF CLK)/4096"]
    Div4096 = 12,
    #[doc = "13: CLK_CNT = (BURTC LF CLK)/8192"]
    Div8192 = 13,
    #[doc = "14: CLK_CNT = (BURTC LF CLK)/16384"]
    Div16384 = 14,
    #[doc = "15: CLK_CNT = (BURTC LF CLK)/32768"]
    Div32768 = 15,
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
#[doc = "Field `CNTPRESC` reader - Counter prescaler value."]
pub type CntprescR = crate::FieldReader<Cntpresc>;
impl CntprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntpresc {
        match self.bits {
            0 => Cntpresc::Div1,
            1 => Cntpresc::Div2,
            2 => Cntpresc::Div4,
            3 => Cntpresc::Div8,
            4 => Cntpresc::Div16,
            5 => Cntpresc::Div32,
            6 => Cntpresc::Div64,
            7 => Cntpresc::Div128,
            8 => Cntpresc::Div256,
            9 => Cntpresc::Div512,
            10 => Cntpresc::Div1024,
            11 => Cntpresc::Div2048,
            12 => Cntpresc::Div4096,
            13 => Cntpresc::Div8192,
            14 => Cntpresc::Div16384,
            15 => Cntpresc::Div32768,
            _ => unreachable!(),
        }
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Cntpresc::Div1
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Cntpresc::Div2
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Cntpresc::Div4
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Cntpresc::Div8
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Cntpresc::Div16
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Cntpresc::Div32
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Cntpresc::Div64
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Cntpresc::Div128
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Cntpresc::Div256
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Cntpresc::Div512
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Cntpresc::Div1024
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/2048"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == Cntpresc::Div2048
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/4096"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == Cntpresc::Div4096
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/8192"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == Cntpresc::Div8192
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/16384"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == Cntpresc::Div16384
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/32768"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == Cntpresc::Div32768
    }
}
#[doc = "Field `CNTPRESC` writer - Counter prescaler value."]
pub type CntprescW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cntpresc, crate::Safe>;
impl<'a, REG> CntprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_CNT = (BURTC LF CLK)/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div1)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div2)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div4)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div8)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div16)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div32)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div64)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div128)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div256)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div512)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div1024)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div2048)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div4096)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div8192)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div16384)
    }
    #[doc = "CLK_CNT = (BURTC LF CLK)/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(Cntpresc::Div32768)
    }
}
impl R {
    #[doc = "Bit 0 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Channel is Top Value"]
    #[inline(always)]
    pub fn comptop(&self) -> ComptopR {
        ComptopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Counter prescaler value."]
    #[inline(always)]
    pub fn cntpresc(&self) -> CntprescR {
        CntprescR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DebugrunW<CfgSpec> {
        DebugrunW::new(self, 0)
    }
    #[doc = "Bit 1 - Compare Channel is Top Value"]
    #[inline(always)]
    pub fn comptop(&mut self) -> ComptopW<CfgSpec> {
        ComptopW::new(self, 1)
    }
    #[doc = "Bits 4:7 - Counter prescaler value."]
    #[inline(always)]
    pub fn cntpresc(&mut self) -> CntprescW<CfgSpec> {
        CntprescW::new(self, 4)
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
