#[doc = "Register `SYSCLKCTRL` reader"]
pub type R = crate::R<SysclkctrlSpec>;
#[doc = "Register `SYSCLKCTRL` writer"]
pub type W = crate::W<SysclkctrlSpec>;
#[doc = "Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "1: FSRCO is clocking SYSCLK"]
    Fsrco = 1,
    #[doc = "2: HFRCODPLL is clocking SYSCLK"]
    Hfrcodpll = 2,
    #[doc = "3: HFXO is clocking SYSCLK"]
    Hfxo = 3,
    #[doc = "4: CLKIN0 is clocking SYSCLK"]
    Clkin0 = 4,
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
#[doc = "Field `CLKSEL` reader - Clock Select"]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clksel> {
        match self.bits {
            1 => Some(Clksel::Fsrco),
            2 => Some(Clksel::Hfrcodpll),
            3 => Some(Clksel::Hfxo),
            4 => Some(Clksel::Clkin0),
            _ => None,
        }
    }
    #[doc = "FSRCO is clocking SYSCLK"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == Clksel::Fsrco
    }
    #[doc = "HFRCODPLL is clocking SYSCLK"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == Clksel::Hfrcodpll
    }
    #[doc = "HFXO is clocking SYSCLK"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Clksel::Hfxo
    }
    #[doc = "CLKIN0 is clocking SYSCLK"]
    #[inline(always)]
    pub fn is_clkin0(&self) -> bool {
        *self == Clksel::Clkin0
    }
}
#[doc = "Field `CLKSEL` writer - Clock Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FSRCO is clocking SYSCLK"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Fsrco)
    }
    #[doc = "HFRCODPLL is clocking SYSCLK"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfrcodpll)
    }
    #[doc = "HFXO is clocking SYSCLK"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hfxo)
    }
    #[doc = "CLKIN0 is clocking SYSCLK"]
    #[inline(always)]
    pub fn clkin0(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Clkin0)
    }
}
#[doc = "PCLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pclkpresc {
    #[doc = "0: PCLK is HCLK divided by 1"]
    Div1 = 0,
    #[doc = "1: PCLK is HCLK divided by 2"]
    Div2 = 1,
}
impl From<Pclkpresc> for bool {
    #[inline(always)]
    fn from(variant: Pclkpresc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCLKPRESC` reader - PCLK Prescaler"]
pub type PclkprescR = crate::BitReader<Pclkpresc>;
impl PclkprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pclkpresc {
        match self.bits {
            false => Pclkpresc::Div1,
            true => Pclkpresc::Div2,
        }
    }
    #[doc = "PCLK is HCLK divided by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Pclkpresc::Div1
    }
    #[doc = "PCLK is HCLK divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Pclkpresc::Div2
    }
}
#[doc = "Field `PCLKPRESC` writer - PCLK Prescaler"]
pub type PclkprescW<'a, REG> = crate::BitWriter<'a, REG, Pclkpresc>;
impl<'a, REG> PclkprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PCLK is HCLK divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Pclkpresc::Div1)
    }
    #[doc = "PCLK is HCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Pclkpresc::Div2)
    }
}
#[doc = "HCLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hclkpresc {
    #[doc = "0: HCLK is SYSCLK divided by 1"]
    Div1 = 0,
    #[doc = "1: HCLK is SYSCLK divided by 2"]
    Div2 = 1,
    #[doc = "3: HCLK is SYSCLK divided by 4"]
    Div4 = 3,
    #[doc = "7: HCLK is SYSCLK divided by 8"]
    Div8 = 7,
    #[doc = "15: HCLK is SYSCLK divided by 16"]
    Div16 = 15,
}
impl From<Hclkpresc> for u8 {
    #[inline(always)]
    fn from(variant: Hclkpresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hclkpresc {
    type Ux = u8;
}
impl crate::IsEnum for Hclkpresc {}
#[doc = "Field `HCLKPRESC` reader - HCLK Prescaler"]
pub type HclkprescR = crate::FieldReader<Hclkpresc>;
impl HclkprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hclkpresc> {
        match self.bits {
            0 => Some(Hclkpresc::Div1),
            1 => Some(Hclkpresc::Div2),
            3 => Some(Hclkpresc::Div4),
            7 => Some(Hclkpresc::Div8),
            15 => Some(Hclkpresc::Div16),
            _ => None,
        }
    }
    #[doc = "HCLK is SYSCLK divided by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Hclkpresc::Div1
    }
    #[doc = "HCLK is SYSCLK divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Hclkpresc::Div2
    }
    #[doc = "HCLK is SYSCLK divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Hclkpresc::Div4
    }
    #[doc = "HCLK is SYSCLK divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Hclkpresc::Div8
    }
    #[doc = "HCLK is SYSCLK divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Hclkpresc::Div16
    }
}
#[doc = "Field `HCLKPRESC` writer - HCLK Prescaler"]
pub type HclkprescW<'a, REG> = crate::FieldWriter<'a, REG, 4, Hclkpresc>;
impl<'a, REG> HclkprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HCLK is SYSCLK divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Hclkpresc::Div1)
    }
    #[doc = "HCLK is SYSCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Hclkpresc::Div2)
    }
    #[doc = "HCLK is SYSCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Hclkpresc::Div4)
    }
    #[doc = "HCLK is SYSCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Hclkpresc::Div8)
    }
    #[doc = "HCLK is SYSCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Hclkpresc::Div16)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 10 - PCLK Prescaler"]
    #[inline(always)]
    pub fn pclkpresc(&self) -> PclkprescR {
        PclkprescR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:15 - HCLK Prescaler"]
    #[inline(always)]
    pub fn hclkpresc(&self) -> HclkprescR {
        HclkprescR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<SysclkctrlSpec> {
        ClkselW::new(self, 0)
    }
    #[doc = "Bit 10 - PCLK Prescaler"]
    #[inline(always)]
    pub fn pclkpresc(&mut self) -> PclkprescW<SysclkctrlSpec> {
        PclkprescW::new(self, 10)
    }
    #[doc = "Bits 12:15 - HCLK Prescaler"]
    #[inline(always)]
    pub fn hclkpresc(&mut self) -> HclkprescW<SysclkctrlSpec> {
        HclkprescW::new(self, 12)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysclkctrlSpec;
impl crate::RegisterSpec for SysclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysclkctrl::R`](R) reader structure"]
impl crate::Readable for SysclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sysclkctrl::W`](W) writer structure"]
impl crate::Writable for SysclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCLKCTRL to value 0x01"]
impl crate::Resettable for SysclkctrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
