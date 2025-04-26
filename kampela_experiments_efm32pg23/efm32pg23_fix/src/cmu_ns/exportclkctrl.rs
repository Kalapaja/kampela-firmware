#[doc = "Register `EXPORTCLKCTRL` reader"]
pub type R = crate::R<ExportclkctrlSpec>;
#[doc = "Register `EXPORTCLKCTRL` writer"]
pub type W = crate::W<ExportclkctrlSpec>;
#[doc = "Clock Output Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkoutsel0 {
    #[doc = "0: CLKOUT0 is not clocked"]
    Disabled = 0,
    #[doc = "1: HCLK is clocking CLKOUT0"]
    Hclk = 1,
    #[doc = "2: EXPORTCLK is clocking CLKOUT0"]
    Hfexpclk = 2,
    #[doc = "3: ULFRCO is clocking CLKOUT0"]
    Ulfrco = 3,
    #[doc = "4: LFRCO is clocking CLKOUT0"]
    Lfrco = 4,
    #[doc = "5: LFXO is clocking CLKOUT0"]
    Lfxo = 5,
    #[doc = "6: HFRCODPLL is clocking CLKOUT0"]
    Hfrcodpll = 6,
    #[doc = "7: HFXO is clocking CLKOUT0"]
    Hfxo = 7,
    #[doc = "8: FSRCO is clocking CLKOUT0"]
    Fsrco = 8,
    #[doc = "9: HFRCOEM23 is clocking CLKOUT0"]
    Hfrcoem23 = 9,
}
impl From<Clkoutsel0> for u8 {
    #[inline(always)]
    fn from(variant: Clkoutsel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkoutsel0 {
    type Ux = u8;
}
impl crate::IsEnum for Clkoutsel0 {}
#[doc = "Field `CLKOUTSEL0` reader - Clock Output Select 0"]
pub type Clkoutsel0R = crate::FieldReader<Clkoutsel0>;
impl Clkoutsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkoutsel0> {
        match self.bits {
            0 => Some(Clkoutsel0::Disabled),
            1 => Some(Clkoutsel0::Hclk),
            2 => Some(Clkoutsel0::Hfexpclk),
            3 => Some(Clkoutsel0::Ulfrco),
            4 => Some(Clkoutsel0::Lfrco),
            5 => Some(Clkoutsel0::Lfxo),
            6 => Some(Clkoutsel0::Hfrcodpll),
            7 => Some(Clkoutsel0::Hfxo),
            8 => Some(Clkoutsel0::Fsrco),
            9 => Some(Clkoutsel0::Hfrcoem23),
            _ => None,
        }
    }
    #[doc = "CLKOUT0 is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clkoutsel0::Disabled
    }
    #[doc = "HCLK is clocking CLKOUT0"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == Clkoutsel0::Hclk
    }
    #[doc = "EXPORTCLK is clocking CLKOUT0"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == Clkoutsel0::Hfexpclk
    }
    #[doc = "ULFRCO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Clkoutsel0::Ulfrco
    }
    #[doc = "LFRCO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Clkoutsel0::Lfrco
    }
    #[doc = "LFXO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Clkoutsel0::Lfxo
    }
    #[doc = "HFRCODPLL is clocking CLKOUT0"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == Clkoutsel0::Hfrcodpll
    }
    #[doc = "HFXO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Clkoutsel0::Hfxo
    }
    #[doc = "FSRCO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == Clkoutsel0::Fsrco
    }
    #[doc = "HFRCOEM23 is clocking CLKOUT0"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == Clkoutsel0::Hfrcoem23
    }
}
#[doc = "Field `CLKOUTSEL0` writer - Clock Output Select 0"]
pub type Clkoutsel0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Clkoutsel0>;
impl<'a, REG> Clkoutsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLKOUT0 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Disabled)
    }
    #[doc = "HCLK is clocking CLKOUT0"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hclk)
    }
    #[doc = "EXPORTCLK is clocking CLKOUT0"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfexpclk)
    }
    #[doc = "ULFRCO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Ulfrco)
    }
    #[doc = "LFRCO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Lfrco)
    }
    #[doc = "LFXO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Lfxo)
    }
    #[doc = "HFRCODPLL is clocking CLKOUT0"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfrcodpll)
    }
    #[doc = "HFXO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfxo)
    }
    #[doc = "FSRCO is clocking CLKOUT0"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Fsrco)
    }
    #[doc = "HFRCOEM23 is clocking CLKOUT0"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel0::Hfrcoem23)
    }
}
#[doc = "Clock Output Select 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkoutsel1 {
    #[doc = "0: CLKOUT1 is not clocked"]
    Disabled = 0,
    #[doc = "1: HCLK is clocking CLKOUT1"]
    Hclk = 1,
    #[doc = "2: EXPORTCLK is clocking CLKOUT1"]
    Hfexpclk = 2,
    #[doc = "3: ULFRCO is clocking CLKOUT1"]
    Ulfrco = 3,
    #[doc = "4: LFRCO is clocking CLKOUT1"]
    Lfrco = 4,
    #[doc = "5: LFXO is clocking CLKOUT1"]
    Lfxo = 5,
    #[doc = "6: HFRCODPLL is clocking CLKOUT1"]
    Hfrcodpll = 6,
    #[doc = "7: HFXO is clocking CLKOUT1"]
    Hfxo = 7,
    #[doc = "8: FSRCO is clocking CLKOUT1"]
    Fsrco = 8,
    #[doc = "9: HFRCOEM23 is clocking CLKOUT1"]
    Hfrcoem23 = 9,
}
impl From<Clkoutsel1> for u8 {
    #[inline(always)]
    fn from(variant: Clkoutsel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkoutsel1 {
    type Ux = u8;
}
impl crate::IsEnum for Clkoutsel1 {}
#[doc = "Field `CLKOUTSEL1` reader - Clock Output Select 1"]
pub type Clkoutsel1R = crate::FieldReader<Clkoutsel1>;
impl Clkoutsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkoutsel1> {
        match self.bits {
            0 => Some(Clkoutsel1::Disabled),
            1 => Some(Clkoutsel1::Hclk),
            2 => Some(Clkoutsel1::Hfexpclk),
            3 => Some(Clkoutsel1::Ulfrco),
            4 => Some(Clkoutsel1::Lfrco),
            5 => Some(Clkoutsel1::Lfxo),
            6 => Some(Clkoutsel1::Hfrcodpll),
            7 => Some(Clkoutsel1::Hfxo),
            8 => Some(Clkoutsel1::Fsrco),
            9 => Some(Clkoutsel1::Hfrcoem23),
            _ => None,
        }
    }
    #[doc = "CLKOUT1 is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clkoutsel1::Disabled
    }
    #[doc = "HCLK is clocking CLKOUT1"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == Clkoutsel1::Hclk
    }
    #[doc = "EXPORTCLK is clocking CLKOUT1"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == Clkoutsel1::Hfexpclk
    }
    #[doc = "ULFRCO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Clkoutsel1::Ulfrco
    }
    #[doc = "LFRCO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Clkoutsel1::Lfrco
    }
    #[doc = "LFXO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Clkoutsel1::Lfxo
    }
    #[doc = "HFRCODPLL is clocking CLKOUT1"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == Clkoutsel1::Hfrcodpll
    }
    #[doc = "HFXO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Clkoutsel1::Hfxo
    }
    #[doc = "FSRCO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == Clkoutsel1::Fsrco
    }
    #[doc = "HFRCOEM23 is clocking CLKOUT1"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == Clkoutsel1::Hfrcoem23
    }
}
#[doc = "Field `CLKOUTSEL1` writer - Clock Output Select 1"]
pub type Clkoutsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Clkoutsel1>;
impl<'a, REG> Clkoutsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLKOUT1 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Disabled)
    }
    #[doc = "HCLK is clocking CLKOUT1"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Hclk)
    }
    #[doc = "EXPORTCLK is clocking CLKOUT1"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Hfexpclk)
    }
    #[doc = "ULFRCO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Ulfrco)
    }
    #[doc = "LFRCO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Lfrco)
    }
    #[doc = "LFXO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Lfxo)
    }
    #[doc = "HFRCODPLL is clocking CLKOUT1"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Hfrcodpll)
    }
    #[doc = "HFXO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Hfxo)
    }
    #[doc = "FSRCO is clocking CLKOUT1"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Fsrco)
    }
    #[doc = "HFRCOEM23 is clocking CLKOUT1"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel1::Hfrcoem23)
    }
}
#[doc = "Clock Output Select 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkoutsel2 {
    #[doc = "0: CLKOUT2 is not clocked"]
    Disabled = 0,
    #[doc = "1: HCLK is clocking CLKOUT2"]
    Hclk = 1,
    #[doc = "2: EXPORTCLK is clocking CLKOUT2"]
    Hfexpclk = 2,
    #[doc = "3: ULFRCO is clocking CLKOUT2"]
    Ulfrco = 3,
    #[doc = "4: LFRCO is clocking CLKOUT2"]
    Lfrco = 4,
    #[doc = "5: LFXO is clocking CLKOUT2"]
    Lfxo = 5,
    #[doc = "6: HFRCODPLL is clocking CLKOUT2"]
    Hfrcodpll = 6,
    #[doc = "7: HFXO is clocking CLKOUT2"]
    Hfxo = 7,
    #[doc = "8: FSRCO is clocking CLKOUT2"]
    Fsrco = 8,
    #[doc = "9: HFRCOEM23 is clocking CLKOUT2"]
    Hfrcoem23 = 9,
}
impl From<Clkoutsel2> for u8 {
    #[inline(always)]
    fn from(variant: Clkoutsel2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkoutsel2 {
    type Ux = u8;
}
impl crate::IsEnum for Clkoutsel2 {}
#[doc = "Field `CLKOUTSEL2` reader - Clock Output Select 2"]
pub type Clkoutsel2R = crate::FieldReader<Clkoutsel2>;
impl Clkoutsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkoutsel2> {
        match self.bits {
            0 => Some(Clkoutsel2::Disabled),
            1 => Some(Clkoutsel2::Hclk),
            2 => Some(Clkoutsel2::Hfexpclk),
            3 => Some(Clkoutsel2::Ulfrco),
            4 => Some(Clkoutsel2::Lfrco),
            5 => Some(Clkoutsel2::Lfxo),
            6 => Some(Clkoutsel2::Hfrcodpll),
            7 => Some(Clkoutsel2::Hfxo),
            8 => Some(Clkoutsel2::Fsrco),
            9 => Some(Clkoutsel2::Hfrcoem23),
            _ => None,
        }
    }
    #[doc = "CLKOUT2 is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clkoutsel2::Disabled
    }
    #[doc = "HCLK is clocking CLKOUT2"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == Clkoutsel2::Hclk
    }
    #[doc = "EXPORTCLK is clocking CLKOUT2"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == Clkoutsel2::Hfexpclk
    }
    #[doc = "ULFRCO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Clkoutsel2::Ulfrco
    }
    #[doc = "LFRCO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Clkoutsel2::Lfrco
    }
    #[doc = "LFXO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Clkoutsel2::Lfxo
    }
    #[doc = "HFRCODPLL is clocking CLKOUT2"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == Clkoutsel2::Hfrcodpll
    }
    #[doc = "HFXO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Clkoutsel2::Hfxo
    }
    #[doc = "FSRCO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == Clkoutsel2::Fsrco
    }
    #[doc = "HFRCOEM23 is clocking CLKOUT2"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == Clkoutsel2::Hfrcoem23
    }
}
#[doc = "Field `CLKOUTSEL2` writer - Clock Output Select 2"]
pub type Clkoutsel2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Clkoutsel2>;
impl<'a, REG> Clkoutsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLKOUT2 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Disabled)
    }
    #[doc = "HCLK is clocking CLKOUT2"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Hclk)
    }
    #[doc = "EXPORTCLK is clocking CLKOUT2"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Hfexpclk)
    }
    #[doc = "ULFRCO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Ulfrco)
    }
    #[doc = "LFRCO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Lfrco)
    }
    #[doc = "LFXO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Lfxo)
    }
    #[doc = "HFRCODPLL is clocking CLKOUT2"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Hfrcodpll)
    }
    #[doc = "HFXO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Hfxo)
    }
    #[doc = "FSRCO is clocking CLKOUT2"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Fsrco)
    }
    #[doc = "HFRCOEM23 is clocking CLKOUT2"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel2::Hfrcoem23)
    }
}
#[doc = "Field `PRESC` reader - EXPORTCLK Prescaler"]
pub type PrescR = crate::FieldReader;
#[doc = "Field `PRESC` writer - EXPORTCLK Prescaler"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&self) -> Clkoutsel0R {
        Clkoutsel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&self) -> Clkoutsel1R {
        Clkoutsel1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Clock Output Select 2"]
    #[inline(always)]
    pub fn clkoutsel2(&self) -> Clkoutsel2R {
        Clkoutsel2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - EXPORTCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&mut self) -> Clkoutsel0W<ExportclkctrlSpec> {
        Clkoutsel0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&mut self) -> Clkoutsel1W<ExportclkctrlSpec> {
        Clkoutsel1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Clock Output Select 2"]
    #[inline(always)]
    pub fn clkoutsel2(&mut self) -> Clkoutsel2W<ExportclkctrlSpec> {
        Clkoutsel2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - EXPORTCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<ExportclkctrlSpec> {
        PrescW::new(self, 24)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`exportclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exportclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExportclkctrlSpec;
impl crate::RegisterSpec for ExportclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exportclkctrl::R`](R) reader structure"]
impl crate::Readable for ExportclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`exportclkctrl::W`](W) writer structure"]
impl crate::Writable for ExportclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXPORTCLKCTRL to value 0"]
impl crate::Resettable for ExportclkctrlSpec {}
