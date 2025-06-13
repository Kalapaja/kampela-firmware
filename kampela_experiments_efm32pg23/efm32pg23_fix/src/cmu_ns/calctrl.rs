#[doc = "Register `CALCTRL` reader"]
pub type R = crate::R<CalctrlSpec>;
#[doc = "Register `CALCTRL` writer"]
pub type W = crate::W<CalctrlSpec>;
#[doc = "Field `CALTOP` reader - Calibration Counter Top Value"]
pub type CaltopR = crate::FieldReader<u32>;
#[doc = "Field `CALTOP` writer - Calibration Counter Top Value"]
pub type CaltopW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `CONT` reader - Continuous Calibration"]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - Continuous Calibration"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Calibration Up-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Upsel {
    #[doc = "0: Up-counter is not clocked"]
    Disabled = 0,
    #[doc = "1: PRS CMU_CALUP consumer is clocking up-counter"]
    Prs = 1,
    #[doc = "2: HFXO is clocking up-counter"]
    Hfxo = 2,
    #[doc = "3: LFXO is clocking up-counter"]
    Lfxo = 3,
    #[doc = "4: HFRCODPLL is clocking up-counter"]
    Hfrcodpll = 4,
    #[doc = "5: HFRCOEM23 is clocking up-counter"]
    Hfrcoem23 = 5,
    #[doc = "8: FSRCO is clocking up-counter"]
    Fsrco = 8,
    #[doc = "9: LFRCO is clocking up-counter"]
    Lfrco = 9,
    #[doc = "10: ULFRCO is clocking up-counter"]
    Ulfrco = 10,
}
impl From<Upsel> for u8 {
    #[inline(always)]
    fn from(variant: Upsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Upsel {
    type Ux = u8;
}
impl crate::IsEnum for Upsel {}
#[doc = "Field `UPSEL` reader - Calibration Up-counter Select"]
pub type UpselR = crate::FieldReader<Upsel>;
impl UpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Upsel> {
        match self.bits {
            0 => Some(Upsel::Disabled),
            1 => Some(Upsel::Prs),
            2 => Some(Upsel::Hfxo),
            3 => Some(Upsel::Lfxo),
            4 => Some(Upsel::Hfrcodpll),
            5 => Some(Upsel::Hfrcoem23),
            8 => Some(Upsel::Fsrco),
            9 => Some(Upsel::Lfrco),
            10 => Some(Upsel::Ulfrco),
            _ => None,
        }
    }
    #[doc = "Up-counter is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Upsel::Disabled
    }
    #[doc = "PRS CMU_CALUP consumer is clocking up-counter"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == Upsel::Prs
    }
    #[doc = "HFXO is clocking up-counter"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Upsel::Hfxo
    }
    #[doc = "LFXO is clocking up-counter"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Upsel::Lfxo
    }
    #[doc = "HFRCODPLL is clocking up-counter"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == Upsel::Hfrcodpll
    }
    #[doc = "HFRCOEM23 is clocking up-counter"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == Upsel::Hfrcoem23
    }
    #[doc = "FSRCO is clocking up-counter"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == Upsel::Fsrco
    }
    #[doc = "LFRCO is clocking up-counter"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Upsel::Lfrco
    }
    #[doc = "ULFRCO is clocking up-counter"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Upsel::Ulfrco
    }
}
#[doc = "Field `UPSEL` writer - Calibration Up-counter Select"]
pub type UpselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Upsel>;
impl<'a, REG> UpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Up-counter is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Disabled)
    }
    #[doc = "PRS CMU_CALUP consumer is clocking up-counter"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Prs)
    }
    #[doc = "HFXO is clocking up-counter"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Hfxo)
    }
    #[doc = "LFXO is clocking up-counter"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Lfxo)
    }
    #[doc = "HFRCODPLL is clocking up-counter"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Hfrcodpll)
    }
    #[doc = "HFRCOEM23 is clocking up-counter"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Hfrcoem23)
    }
    #[doc = "FSRCO is clocking up-counter"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Fsrco)
    }
    #[doc = "LFRCO is clocking up-counter"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Lfrco)
    }
    #[doc = "ULFRCO is clocking up-counter"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Upsel::Ulfrco)
    }
}
#[doc = "Calibration Down-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Downsel {
    #[doc = "0: Down-counter is not clocked"]
    Disabled = 0,
    #[doc = "1: HCLK is clocking down-counter"]
    Hclk = 1,
    #[doc = "2: PRS CMU_CALDN consumer is clocking down-counter"]
    Prs = 2,
    #[doc = "3: HFXO is clocking down-counter"]
    Hfxo = 3,
    #[doc = "4: LFXO is clocking down-counter"]
    Lfxo = 4,
    #[doc = "5: HFRCODPLL is clocking down-counter"]
    Hfrcodpll = 5,
    #[doc = "6: HFRCOEM23 is clocking down-counter"]
    Hfrcoem23 = 6,
    #[doc = "9: FSRCO is clocking down-counter"]
    Fsrco = 9,
    #[doc = "10: LFRCO is clocking down-counter"]
    Lfrco = 10,
    #[doc = "11: ULFRCO is clocking down-counter"]
    Ulfrco = 11,
}
impl From<Downsel> for u8 {
    #[inline(always)]
    fn from(variant: Downsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Downsel {
    type Ux = u8;
}
impl crate::IsEnum for Downsel {}
#[doc = "Field `DOWNSEL` reader - Calibration Down-counter Select"]
pub type DownselR = crate::FieldReader<Downsel>;
impl DownselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Downsel> {
        match self.bits {
            0 => Some(Downsel::Disabled),
            1 => Some(Downsel::Hclk),
            2 => Some(Downsel::Prs),
            3 => Some(Downsel::Hfxo),
            4 => Some(Downsel::Lfxo),
            5 => Some(Downsel::Hfrcodpll),
            6 => Some(Downsel::Hfrcoem23),
            9 => Some(Downsel::Fsrco),
            10 => Some(Downsel::Lfrco),
            11 => Some(Downsel::Ulfrco),
            _ => None,
        }
    }
    #[doc = "Down-counter is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Downsel::Disabled
    }
    #[doc = "HCLK is clocking down-counter"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == Downsel::Hclk
    }
    #[doc = "PRS CMU_CALDN consumer is clocking down-counter"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == Downsel::Prs
    }
    #[doc = "HFXO is clocking down-counter"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Downsel::Hfxo
    }
    #[doc = "LFXO is clocking down-counter"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Downsel::Lfxo
    }
    #[doc = "HFRCODPLL is clocking down-counter"]
    #[inline(always)]
    pub fn is_hfrcodpll(&self) -> bool {
        *self == Downsel::Hfrcodpll
    }
    #[doc = "HFRCOEM23 is clocking down-counter"]
    #[inline(always)]
    pub fn is_hfrcoem23(&self) -> bool {
        *self == Downsel::Hfrcoem23
    }
    #[doc = "FSRCO is clocking down-counter"]
    #[inline(always)]
    pub fn is_fsrco(&self) -> bool {
        *self == Downsel::Fsrco
    }
    #[doc = "LFRCO is clocking down-counter"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == Downsel::Lfrco
    }
    #[doc = "ULFRCO is clocking down-counter"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == Downsel::Ulfrco
    }
}
#[doc = "Field `DOWNSEL` writer - Calibration Down-counter Select"]
pub type DownselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Downsel>;
impl<'a, REG> DownselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Down-counter is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Disabled)
    }
    #[doc = "HCLK is clocking down-counter"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Hclk)
    }
    #[doc = "PRS CMU_CALDN consumer is clocking down-counter"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Prs)
    }
    #[doc = "HFXO is clocking down-counter"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Hfxo)
    }
    #[doc = "LFXO is clocking down-counter"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Lfxo)
    }
    #[doc = "HFRCODPLL is clocking down-counter"]
    #[inline(always)]
    pub fn hfrcodpll(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Hfrcodpll)
    }
    #[doc = "HFRCOEM23 is clocking down-counter"]
    #[inline(always)]
    pub fn hfrcoem23(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Hfrcoem23)
    }
    #[doc = "FSRCO is clocking down-counter"]
    #[inline(always)]
    pub fn fsrco(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Fsrco)
    }
    #[doc = "LFRCO is clocking down-counter"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Lfrco)
    }
    #[doc = "ULFRCO is clocking down-counter"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(Downsel::Ulfrco)
    }
}
impl R {
    #[doc = "Bits 0:19 - Calibration Counter Top Value"]
    #[inline(always)]
    pub fn caltop(&self) -> CaltopR {
        CaltopR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 23 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&self) -> UpselR {
        UpselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&self) -> DownselR {
        DownselR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - Calibration Counter Top Value"]
    #[inline(always)]
    pub fn caltop(&mut self) -> CaltopW<CalctrlSpec> {
        CaltopW::new(self, 0)
    }
    #[doc = "Bit 23 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&mut self) -> ContW<CalctrlSpec> {
        ContW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&mut self) -> UpselW<CalctrlSpec> {
        UpselW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&mut self) -> DownselW<CalctrlSpec> {
        DownselW::new(self, 28)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`calctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalctrlSpec;
impl crate::RegisterSpec for CalctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calctrl::R`](R) reader structure"]
impl crate::Readable for CalctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`calctrl::W`](W) writer structure"]
impl crate::Writable for CalctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALCTRL to value 0"]
impl crate::Resettable for CalctrlSpec {}
