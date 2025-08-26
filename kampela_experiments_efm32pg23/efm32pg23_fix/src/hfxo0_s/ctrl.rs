#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `BUFOUTFREEZE` reader - Freeze BUFOUT Controls"]
pub type BufoutfreezeR = crate::BitReader;
#[doc = "Field `BUFOUTFREEZE` writer - Freeze BUFOUT Controls"]
pub type BufoutfreezeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEEPWARM` reader - Keep Warm"]
pub type KeepwarmR = crate::BitReader;
#[doc = "Field `KEEPWARM` writer - Keep Warm"]
pub type KeepwarmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23ONDEMAND` reader - On-demand During EM23"]
pub type Em23ondemandR = crate::BitReader;
#[doc = "Field `EM23ONDEMAND` writer - On-demand During EM23"]
pub type Em23ondemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Force XI Pin to Ground\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forcexi2gndana {
    #[doc = "0: Disabled (not pulled)"]
    Disable = 0,
    #[doc = "1: Enabled (pulled)"]
    Enable = 1,
}
impl From<Forcexi2gndana> for bool {
    #[inline(always)]
    fn from(variant: Forcexi2gndana) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEXI2GNDANA` reader - Force XI Pin to Ground"]
pub type Forcexi2gndanaR = crate::BitReader<Forcexi2gndana>;
impl Forcexi2gndanaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forcexi2gndana {
        match self.bits {
            false => Forcexi2gndana::Disable,
            true => Forcexi2gndana::Enable,
        }
    }
    #[doc = "Disabled (not pulled)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Forcexi2gndana::Disable
    }
    #[doc = "Enabled (pulled)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Forcexi2gndana::Enable
    }
}
#[doc = "Field `FORCEXI2GNDANA` writer - Force XI Pin to Ground"]
pub type Forcexi2gndanaW<'a, REG> = crate::BitWriter<'a, REG, Forcexi2gndana>;
impl<'a, REG> Forcexi2gndanaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled (not pulled)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Forcexi2gndana::Disable)
    }
    #[doc = "Enabled (pulled)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Forcexi2gndana::Enable)
    }
}
#[doc = "Force XO Pin to Ground\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forcexo2gndana {
    #[doc = "0: Disabled (not pulled)"]
    Disable = 0,
    #[doc = "1: Enabled (pulled)"]
    Enable = 1,
}
impl From<Forcexo2gndana> for bool {
    #[inline(always)]
    fn from(variant: Forcexo2gndana) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEXO2GNDANA` reader - Force XO Pin to Ground"]
pub type Forcexo2gndanaR = crate::BitReader<Forcexo2gndana>;
impl Forcexo2gndanaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forcexo2gndana {
        match self.bits {
            false => Forcexo2gndana::Disable,
            true => Forcexo2gndana::Enable,
        }
    }
    #[doc = "Disabled (not pulled)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Forcexo2gndana::Disable
    }
    #[doc = "Enabled (pulled)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Forcexo2gndana::Enable
    }
}
#[doc = "Field `FORCEXO2GNDANA` writer - Force XO Pin to Ground"]
pub type Forcexo2gndanaW<'a, REG> = crate::BitWriter<'a, REG, Forcexo2gndana>;
impl<'a, REG> Forcexo2gndanaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled (not pulled)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Forcexo2gndana::Disable)
    }
    #[doc = "Enabled (pulled)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Forcexo2gndana::Enable)
    }
}
#[doc = "Field `FORCECTUNEMAX` reader - Force Tuning Cap to Max Value"]
pub type ForcectunemaxR = crate::BitReader;
#[doc = "Field `FORCECTUNEMAX` writer - Force Tuning Cap to Max Value"]
pub type ForcectunemaxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PRS Status 0 Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prsstatussel0 {
    #[doc = "0: PRS mux outputs 0"]
    Disabled = 0,
    #[doc = "1: PRS mux outputs enabled status"]
    Ens = 1,
    #[doc = "2: PRS mux outputs core bias optimization ready status"]
    Corebiasoptrdy = 2,
    #[doc = "3: PRS mux outputs ready status"]
    Rdy = 3,
    #[doc = "4: PRS mux outputs PRS ready status"]
    Prsrdy = 4,
    #[doc = "5: PRS mux outputs BUFOUT ready status"]
    Bufoutrdy = 5,
    #[doc = "8: PRS mux outputs oscillator requested by digital clock status"]
    Hwreq = 8,
    #[doc = "9: PRS mux outputs oscillator requested by PRS request status"]
    Prshwreq = 9,
    #[doc = "10: PRS mux outputs oscillator requested by BUFOUT request status"]
    Bufouthwreq = 10,
}
impl From<Prsstatussel0> for u8 {
    #[inline(always)]
    fn from(variant: Prsstatussel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prsstatussel0 {
    type Ux = u8;
}
impl crate::IsEnum for Prsstatussel0 {}
#[doc = "Field `PRSSTATUSSEL0` reader - PRS Status 0 Output Select"]
pub type Prsstatussel0R = crate::FieldReader<Prsstatussel0>;
impl Prsstatussel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prsstatussel0> {
        match self.bits {
            0 => Some(Prsstatussel0::Disabled),
            1 => Some(Prsstatussel0::Ens),
            2 => Some(Prsstatussel0::Corebiasoptrdy),
            3 => Some(Prsstatussel0::Rdy),
            4 => Some(Prsstatussel0::Prsrdy),
            5 => Some(Prsstatussel0::Bufoutrdy),
            8 => Some(Prsstatussel0::Hwreq),
            9 => Some(Prsstatussel0::Prshwreq),
            10 => Some(Prsstatussel0::Bufouthwreq),
            _ => None,
        }
    }
    #[doc = "PRS mux outputs 0"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Prsstatussel0::Disabled
    }
    #[doc = "PRS mux outputs enabled status"]
    #[inline(always)]
    pub fn is_ens(&self) -> bool {
        *self == Prsstatussel0::Ens
    }
    #[doc = "PRS mux outputs core bias optimization ready status"]
    #[inline(always)]
    pub fn is_corebiasoptrdy(&self) -> bool {
        *self == Prsstatussel0::Corebiasoptrdy
    }
    #[doc = "PRS mux outputs ready status"]
    #[inline(always)]
    pub fn is_rdy(&self) -> bool {
        *self == Prsstatussel0::Rdy
    }
    #[doc = "PRS mux outputs PRS ready status"]
    #[inline(always)]
    pub fn is_prsrdy(&self) -> bool {
        *self == Prsstatussel0::Prsrdy
    }
    #[doc = "PRS mux outputs BUFOUT ready status"]
    #[inline(always)]
    pub fn is_bufoutrdy(&self) -> bool {
        *self == Prsstatussel0::Bufoutrdy
    }
    #[doc = "PRS mux outputs oscillator requested by digital clock status"]
    #[inline(always)]
    pub fn is_hwreq(&self) -> bool {
        *self == Prsstatussel0::Hwreq
    }
    #[doc = "PRS mux outputs oscillator requested by PRS request status"]
    #[inline(always)]
    pub fn is_prshwreq(&self) -> bool {
        *self == Prsstatussel0::Prshwreq
    }
    #[doc = "PRS mux outputs oscillator requested by BUFOUT request status"]
    #[inline(always)]
    pub fn is_bufouthwreq(&self) -> bool {
        *self == Prsstatussel0::Bufouthwreq
    }
}
#[doc = "Field `PRSSTATUSSEL0` writer - PRS Status 0 Output Select"]
pub type Prsstatussel0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Prsstatussel0>;
impl<'a, REG> Prsstatussel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS mux outputs 0"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel0::Disabled)
    }
    #[doc = "PRS mux outputs enabled status"]
    #[inline(always)]
    pub fn ens(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel0::Ens)
    }
    #[doc = "PRS mux outputs core bias optimization ready status"]
    #[inline(always)]
    pub fn corebiasoptrdy(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel0::Corebiasoptrdy)
    }
    #[doc = "PRS mux outputs ready status"]
    #[inline(always)]
    pub fn rdy(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel0::Rdy)
    }
    #[doc = "PRS mux outputs PRS ready status"]
    #[inline(always)]
    pub fn prsrdy(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel0::Prsrdy)
    }
    #[doc = "PRS mux outputs BUFOUT ready status"]
    #[inline(always)]
    pub fn bufoutrdy(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel0::Bufoutrdy)
    }
    #[doc = "PRS mux outputs oscillator requested by digital clock status"]
    #[inline(always)]
    pub fn hwreq(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel0::Hwreq)
    }
    #[doc = "PRS mux outputs oscillator requested by PRS request status"]
    #[inline(always)]
    pub fn prshwreq(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel0::Prshwreq)
    }
    #[doc = "PRS mux outputs oscillator requested by BUFOUT request status"]
    #[inline(always)]
    pub fn bufouthwreq(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel0::Bufouthwreq)
    }
}
#[doc = "PRS Status 1 Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prsstatussel1 {
    #[doc = "0: PRS mux outputs 0"]
    Disabled = 0,
    #[doc = "1: PRS mux outputs enabled status"]
    Ens = 1,
    #[doc = "2: PRS mux outputs core bias optimization ready status"]
    Corebiasoptrdy = 2,
    #[doc = "3: PRS mux outputs ready status"]
    Rdy = 3,
    #[doc = "4: PRS mux outputs PRS ready status"]
    Prsrdy = 4,
    #[doc = "5: PRS mux outputs BUFOUT ready status"]
    Bufoutrdy = 5,
    #[doc = "8: PRS mux outputs oscillator requested by digital clock status"]
    Hwreq = 8,
    #[doc = "9: PRS mux outputs oscillator requested by PRS request status"]
    Prshwreq = 9,
    #[doc = "10: PRS mux outputs oscillator requested by BUFOUT request status"]
    Bufouthwreq = 10,
}
impl From<Prsstatussel1> for u8 {
    #[inline(always)]
    fn from(variant: Prsstatussel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prsstatussel1 {
    type Ux = u8;
}
impl crate::IsEnum for Prsstatussel1 {}
#[doc = "Field `PRSSTATUSSEL1` reader - PRS Status 1 Output Select"]
pub type Prsstatussel1R = crate::FieldReader<Prsstatussel1>;
impl Prsstatussel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prsstatussel1> {
        match self.bits {
            0 => Some(Prsstatussel1::Disabled),
            1 => Some(Prsstatussel1::Ens),
            2 => Some(Prsstatussel1::Corebiasoptrdy),
            3 => Some(Prsstatussel1::Rdy),
            4 => Some(Prsstatussel1::Prsrdy),
            5 => Some(Prsstatussel1::Bufoutrdy),
            8 => Some(Prsstatussel1::Hwreq),
            9 => Some(Prsstatussel1::Prshwreq),
            10 => Some(Prsstatussel1::Bufouthwreq),
            _ => None,
        }
    }
    #[doc = "PRS mux outputs 0"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Prsstatussel1::Disabled
    }
    #[doc = "PRS mux outputs enabled status"]
    #[inline(always)]
    pub fn is_ens(&self) -> bool {
        *self == Prsstatussel1::Ens
    }
    #[doc = "PRS mux outputs core bias optimization ready status"]
    #[inline(always)]
    pub fn is_corebiasoptrdy(&self) -> bool {
        *self == Prsstatussel1::Corebiasoptrdy
    }
    #[doc = "PRS mux outputs ready status"]
    #[inline(always)]
    pub fn is_rdy(&self) -> bool {
        *self == Prsstatussel1::Rdy
    }
    #[doc = "PRS mux outputs PRS ready status"]
    #[inline(always)]
    pub fn is_prsrdy(&self) -> bool {
        *self == Prsstatussel1::Prsrdy
    }
    #[doc = "PRS mux outputs BUFOUT ready status"]
    #[inline(always)]
    pub fn is_bufoutrdy(&self) -> bool {
        *self == Prsstatussel1::Bufoutrdy
    }
    #[doc = "PRS mux outputs oscillator requested by digital clock status"]
    #[inline(always)]
    pub fn is_hwreq(&self) -> bool {
        *self == Prsstatussel1::Hwreq
    }
    #[doc = "PRS mux outputs oscillator requested by PRS request status"]
    #[inline(always)]
    pub fn is_prshwreq(&self) -> bool {
        *self == Prsstatussel1::Prshwreq
    }
    #[doc = "PRS mux outputs oscillator requested by BUFOUT request status"]
    #[inline(always)]
    pub fn is_bufouthwreq(&self) -> bool {
        *self == Prsstatussel1::Bufouthwreq
    }
}
#[doc = "Field `PRSSTATUSSEL1` writer - PRS Status 1 Output Select"]
pub type Prsstatussel1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Prsstatussel1>;
impl<'a, REG> Prsstatussel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS mux outputs 0"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel1::Disabled)
    }
    #[doc = "PRS mux outputs enabled status"]
    #[inline(always)]
    pub fn ens(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel1::Ens)
    }
    #[doc = "PRS mux outputs core bias optimization ready status"]
    #[inline(always)]
    pub fn corebiasoptrdy(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel1::Corebiasoptrdy)
    }
    #[doc = "PRS mux outputs ready status"]
    #[inline(always)]
    pub fn rdy(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel1::Rdy)
    }
    #[doc = "PRS mux outputs PRS ready status"]
    #[inline(always)]
    pub fn prsrdy(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel1::Prsrdy)
    }
    #[doc = "PRS mux outputs BUFOUT ready status"]
    #[inline(always)]
    pub fn bufoutrdy(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel1::Bufoutrdy)
    }
    #[doc = "PRS mux outputs oscillator requested by digital clock status"]
    #[inline(always)]
    pub fn hwreq(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel1::Hwreq)
    }
    #[doc = "PRS mux outputs oscillator requested by PRS request status"]
    #[inline(always)]
    pub fn prshwreq(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel1::Prshwreq)
    }
    #[doc = "PRS mux outputs oscillator requested by BUFOUT request status"]
    #[inline(always)]
    pub fn bufouthwreq(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstatussel1::Bufouthwreq)
    }
}
#[doc = "Field `FORCEEN` reader - Force Digital Clock Request"]
pub type ForceenR = crate::BitReader;
#[doc = "Field `FORCEEN` writer - Force Digital Clock Request"]
pub type ForceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEENPRS` reader - Force PRS Oscillator Request"]
pub type ForceenprsR = crate::BitReader;
#[doc = "Field `FORCEENPRS` writer - Force PRS Oscillator Request"]
pub type ForceenprsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEENBUFOUT` reader - Force BUFOUT Request"]
pub type ForceenbufoutR = crate::BitReader;
#[doc = "Field `FORCEENBUFOUT` writer - Force BUFOUT Request"]
pub type ForceenbufoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISONDEMAND` reader - Disable On-demand For Digital Clock"]
pub type DisondemandR = crate::BitReader;
#[doc = "Field `DISONDEMAND` writer - Disable On-demand For Digital Clock"]
pub type DisondemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISONDEMANDPRS` reader - Disable On-demand For PRS"]
pub type DisondemandprsR = crate::BitReader;
#[doc = "Field `DISONDEMANDPRS` writer - Disable On-demand For PRS"]
pub type DisondemandprsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISONDEMANDBUFOUT` reader - Disable On-demand For BUFOUT"]
pub type DisondemandbufoutR = crate::BitReader;
#[doc = "Field `DISONDEMANDBUFOUT` writer - Disable On-demand For BUFOUT"]
pub type DisondemandbufoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Freeze BUFOUT Controls"]
    #[inline(always)]
    pub fn bufoutfreeze(&self) -> BufoutfreezeR {
        BufoutfreezeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Keep Warm"]
    #[inline(always)]
    pub fn keepwarm(&self) -> KeepwarmR {
        KeepwarmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - On-demand During EM23"]
    #[inline(always)]
    pub fn em23ondemand(&self) -> Em23ondemandR {
        Em23ondemandR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force XI Pin to Ground"]
    #[inline(always)]
    pub fn forcexi2gndana(&self) -> Forcexi2gndanaR {
        Forcexi2gndanaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force XO Pin to Ground"]
    #[inline(always)]
    pub fn forcexo2gndana(&self) -> Forcexo2gndanaR {
        Forcexo2gndanaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Tuning Cap to Max Value"]
    #[inline(always)]
    pub fn forcectunemax(&self) -> ForcectunemaxR {
        ForcectunemaxR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - PRS Status 0 Output Select"]
    #[inline(always)]
    pub fn prsstatussel0(&self) -> Prsstatussel0R {
        Prsstatussel0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PRS Status 1 Output Select"]
    #[inline(always)]
    pub fn prsstatussel1(&self) -> Prsstatussel1R {
        Prsstatussel1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Force Digital Clock Request"]
    #[inline(always)]
    pub fn forceen(&self) -> ForceenR {
        ForceenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Force PRS Oscillator Request"]
    #[inline(always)]
    pub fn forceenprs(&self) -> ForceenprsR {
        ForceenprsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Force BUFOUT Request"]
    #[inline(always)]
    pub fn forceenbufout(&self) -> ForceenbufoutR {
        ForceenbufoutR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable On-demand For Digital Clock"]
    #[inline(always)]
    pub fn disondemand(&self) -> DisondemandR {
        DisondemandR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Disable On-demand For PRS"]
    #[inline(always)]
    pub fn disondemandprs(&self) -> DisondemandprsR {
        DisondemandprsR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable On-demand For BUFOUT"]
    #[inline(always)]
    pub fn disondemandbufout(&self) -> DisondemandbufoutR {
        DisondemandbufoutR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Freeze BUFOUT Controls"]
    #[inline(always)]
    pub fn bufoutfreeze(&mut self) -> BufoutfreezeW<CtrlSpec> {
        BufoutfreezeW::new(self, 0)
    }
    #[doc = "Bit 2 - Keep Warm"]
    #[inline(always)]
    pub fn keepwarm(&mut self) -> KeepwarmW<CtrlSpec> {
        KeepwarmW::new(self, 2)
    }
    #[doc = "Bit 3 - On-demand During EM23"]
    #[inline(always)]
    pub fn em23ondemand(&mut self) -> Em23ondemandW<CtrlSpec> {
        Em23ondemandW::new(self, 3)
    }
    #[doc = "Bit 4 - Force XI Pin to Ground"]
    #[inline(always)]
    pub fn forcexi2gndana(&mut self) -> Forcexi2gndanaW<CtrlSpec> {
        Forcexi2gndanaW::new(self, 4)
    }
    #[doc = "Bit 5 - Force XO Pin to Ground"]
    #[inline(always)]
    pub fn forcexo2gndana(&mut self) -> Forcexo2gndanaW<CtrlSpec> {
        Forcexo2gndanaW::new(self, 5)
    }
    #[doc = "Bit 6 - Force Tuning Cap to Max Value"]
    #[inline(always)]
    pub fn forcectunemax(&mut self) -> ForcectunemaxW<CtrlSpec> {
        ForcectunemaxW::new(self, 6)
    }
    #[doc = "Bits 8:11 - PRS Status 0 Output Select"]
    #[inline(always)]
    pub fn prsstatussel0(&mut self) -> Prsstatussel0W<CtrlSpec> {
        Prsstatussel0W::new(self, 8)
    }
    #[doc = "Bits 12:15 - PRS Status 1 Output Select"]
    #[inline(always)]
    pub fn prsstatussel1(&mut self) -> Prsstatussel1W<CtrlSpec> {
        Prsstatussel1W::new(self, 12)
    }
    #[doc = "Bit 16 - Force Digital Clock Request"]
    #[inline(always)]
    pub fn forceen(&mut self) -> ForceenW<CtrlSpec> {
        ForceenW::new(self, 16)
    }
    #[doc = "Bit 17 - Force PRS Oscillator Request"]
    #[inline(always)]
    pub fn forceenprs(&mut self) -> ForceenprsW<CtrlSpec> {
        ForceenprsW::new(self, 17)
    }
    #[doc = "Bit 18 - Force BUFOUT Request"]
    #[inline(always)]
    pub fn forceenbufout(&mut self) -> ForceenbufoutW<CtrlSpec> {
        ForceenbufoutW::new(self, 18)
    }
    #[doc = "Bit 24 - Disable On-demand For Digital Clock"]
    #[inline(always)]
    pub fn disondemand(&mut self) -> DisondemandW<CtrlSpec> {
        DisondemandW::new(self, 24)
    }
    #[doc = "Bit 25 - Disable On-demand For PRS"]
    #[inline(always)]
    pub fn disondemandprs(&mut self) -> DisondemandprsW<CtrlSpec> {
        DisondemandprsW::new(self, 25)
    }
    #[doc = "Bit 26 - Disable On-demand For BUFOUT"]
    #[inline(always)]
    pub fn disondemandbufout(&mut self) -> DisondemandbufoutW<CtrlSpec> {
        DisondemandbufoutW::new(self, 26)
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
#[doc = "`reset()` method sets CTRL to value 0x0700_0040"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0700_0040;
}
