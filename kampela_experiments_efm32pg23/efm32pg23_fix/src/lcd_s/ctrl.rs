#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Update Data Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Udctrl {
    #[doc = "0: The data transfer is controlled by SW. Transfer is performed as soon as possible on the next CTRL.PRESCALE clock. This is primarily available for debug only since only some of the new SEGMENT data may be ready by the time of the UPDATE. This should not be used with interrupts since partially updating SEGMENT data may have indeterminant results."]
    Regular = 0,
    #[doc = "1: Data is loaded continuously at every frame start"]
    Framestart = 1,
    #[doc = "2: The data transfer is done at the next Frame Counter event"]
    Fcevent = 2,
    #[doc = "3: The data transfer is done at the next Display Counter event"]
    Displayevent = 3,
}
impl From<Udctrl> for u8 {
    #[inline(always)]
    fn from(variant: Udctrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Udctrl {
    type Ux = u8;
}
impl crate::IsEnum for Udctrl {}
#[doc = "Field `UDCTRL` reader - Update Data Control"]
pub type UdctrlR = crate::FieldReader<Udctrl>;
impl UdctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Udctrl {
        match self.bits {
            0 => Udctrl::Regular,
            1 => Udctrl::Framestart,
            2 => Udctrl::Fcevent,
            3 => Udctrl::Displayevent,
            _ => unreachable!(),
        }
    }
    #[doc = "The data transfer is controlled by SW. Transfer is performed as soon as possible on the next CTRL.PRESCALE clock. This is primarily available for debug only since only some of the new SEGMENT data may be ready by the time of the UPDATE. This should not be used with interrupts since partially updating SEGMENT data may have indeterminant results."]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == Udctrl::Regular
    }
    #[doc = "Data is loaded continuously at every frame start"]
    #[inline(always)]
    pub fn is_framestart(&self) -> bool {
        *self == Udctrl::Framestart
    }
    #[doc = "The data transfer is done at the next Frame Counter event"]
    #[inline(always)]
    pub fn is_fcevent(&self) -> bool {
        *self == Udctrl::Fcevent
    }
    #[doc = "The data transfer is done at the next Display Counter event"]
    #[inline(always)]
    pub fn is_displayevent(&self) -> bool {
        *self == Udctrl::Displayevent
    }
}
#[doc = "Field `UDCTRL` writer - Update Data Control"]
pub type UdctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Udctrl, crate::Safe>;
impl<'a, REG> UdctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The data transfer is controlled by SW. Transfer is performed as soon as possible on the next CTRL.PRESCALE clock. This is primarily available for debug only since only some of the new SEGMENT data may be ready by the time of the UPDATE. This should not be used with interrupts since partially updating SEGMENT data may have indeterminant results."]
    #[inline(always)]
    pub fn regular(self) -> &'a mut crate::W<REG> {
        self.variant(Udctrl::Regular)
    }
    #[doc = "Data is loaded continuously at every frame start"]
    #[inline(always)]
    pub fn framestart(self) -> &'a mut crate::W<REG> {
        self.variant(Udctrl::Framestart)
    }
    #[doc = "The data transfer is done at the next Frame Counter event"]
    #[inline(always)]
    pub fn fcevent(self) -> &'a mut crate::W<REG> {
        self.variant(Udctrl::Fcevent)
    }
    #[doc = "The data transfer is done at the next Display Counter event"]
    #[inline(always)]
    pub fn displayevent(self) -> &'a mut crate::W<REG> {
        self.variant(Udctrl::Displayevent)
    }
}
#[doc = "Direct Segment Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsc {
    #[doc = "0: DSC disable"]
    Disable = 0,
    #[doc = "1: DSC enable"]
    Enable = 1,
}
impl From<Dsc> for bool {
    #[inline(always)]
    fn from(variant: Dsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSC` reader - Direct Segment Control"]
pub type DscR = crate::BitReader<Dsc>;
impl DscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsc {
        match self.bits {
            false => Dsc::Disable,
            true => Dsc::Enable,
        }
    }
    #[doc = "DSC disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dsc::Disable
    }
    #[doc = "DSC enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dsc::Enable
    }
}
#[doc = "Field `DSC` writer - Direct Segment Control"]
pub type DscW<'a, REG> = crate::BitWriter<'a, REG, Dsc>;
impl<'a, REG> DscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DSC disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dsc::Disable)
    }
    #[doc = "DSC enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dsc::Enable)
    }
}
#[doc = "Warmup Delay\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Warmupdly {
    #[doc = "0: 1mswarm up"]
    Warmup1 = 0,
    #[doc = "1: 31ms warm up"]
    Warmup31 = 1,
    #[doc = "2: 62ms warm up"]
    Warmup63 = 2,
    #[doc = "3: 125ms warm up"]
    Warmup125 = 3,
    #[doc = "4: 250ms warm up"]
    Warmup250 = 4,
    #[doc = "5: 500ms warm up"]
    Warmup500 = 5,
    #[doc = "6: 1000ms warm up"]
    Warmup1000 = 6,
    #[doc = "7: 2000ms warm up"]
    Warmup2000 = 7,
}
impl From<Warmupdly> for u8 {
    #[inline(always)]
    fn from(variant: Warmupdly) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Warmupdly {
    type Ux = u8;
}
impl crate::IsEnum for Warmupdly {}
#[doc = "Field `WARMUPDLY` reader - Warmup Delay"]
pub type WarmupdlyR = crate::FieldReader<Warmupdly>;
impl WarmupdlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Warmupdly {
        match self.bits {
            0 => Warmupdly::Warmup1,
            1 => Warmupdly::Warmup31,
            2 => Warmupdly::Warmup63,
            3 => Warmupdly::Warmup125,
            4 => Warmupdly::Warmup250,
            5 => Warmupdly::Warmup500,
            6 => Warmupdly::Warmup1000,
            7 => Warmupdly::Warmup2000,
            _ => unreachable!(),
        }
    }
    #[doc = "1mswarm up"]
    #[inline(always)]
    pub fn is_warmup1(&self) -> bool {
        *self == Warmupdly::Warmup1
    }
    #[doc = "31ms warm up"]
    #[inline(always)]
    pub fn is_warmup31(&self) -> bool {
        *self == Warmupdly::Warmup31
    }
    #[doc = "62ms warm up"]
    #[inline(always)]
    pub fn is_warmup63(&self) -> bool {
        *self == Warmupdly::Warmup63
    }
    #[doc = "125ms warm up"]
    #[inline(always)]
    pub fn is_warmup125(&self) -> bool {
        *self == Warmupdly::Warmup125
    }
    #[doc = "250ms warm up"]
    #[inline(always)]
    pub fn is_warmup250(&self) -> bool {
        *self == Warmupdly::Warmup250
    }
    #[doc = "500ms warm up"]
    #[inline(always)]
    pub fn is_warmup500(&self) -> bool {
        *self == Warmupdly::Warmup500
    }
    #[doc = "1000ms warm up"]
    #[inline(always)]
    pub fn is_warmup1000(&self) -> bool {
        *self == Warmupdly::Warmup1000
    }
    #[doc = "2000ms warm up"]
    #[inline(always)]
    pub fn is_warmup2000(&self) -> bool {
        *self == Warmupdly::Warmup2000
    }
}
#[doc = "Field `WARMUPDLY` writer - Warmup Delay"]
pub type WarmupdlyW<'a, REG> = crate::FieldWriter<'a, REG, 3, Warmupdly, crate::Safe>;
impl<'a, REG> WarmupdlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1mswarm up"]
    #[inline(always)]
    pub fn warmup1(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupdly::Warmup1)
    }
    #[doc = "31ms warm up"]
    #[inline(always)]
    pub fn warmup31(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupdly::Warmup31)
    }
    #[doc = "62ms warm up"]
    #[inline(always)]
    pub fn warmup63(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupdly::Warmup63)
    }
    #[doc = "125ms warm up"]
    #[inline(always)]
    pub fn warmup125(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupdly::Warmup125)
    }
    #[doc = "250ms warm up"]
    #[inline(always)]
    pub fn warmup250(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupdly::Warmup250)
    }
    #[doc = "500ms warm up"]
    #[inline(always)]
    pub fn warmup500(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupdly::Warmup500)
    }
    #[doc = "1000ms warm up"]
    #[inline(always)]
    pub fn warmup1000(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupdly::Warmup1000)
    }
    #[doc = "2000ms warm up"]
    #[inline(always)]
    pub fn warmup2000(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupdly::Warmup2000)
    }
}
#[doc = "Field `PRESCALE` reader - Presclae"]
pub type PrescaleR = crate::FieldReader;
#[doc = "Field `PRESCALE` writer - Presclae"]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:2 - Update Data Control"]
    #[inline(always)]
    pub fn udctrl(&self) -> UdctrlR {
        UdctrlR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 16 - Direct Segment Control"]
    #[inline(always)]
    pub fn dsc(&self) -> DscR {
        DscR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Warmup Delay"]
    #[inline(always)]
    pub fn warmupdly(&self) -> WarmupdlyR {
        WarmupdlyR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 24:30 - Presclae"]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - Update Data Control"]
    #[inline(always)]
    pub fn udctrl(&mut self) -> UdctrlW<CtrlSpec> {
        UdctrlW::new(self, 1)
    }
    #[doc = "Bit 16 - Direct Segment Control"]
    #[inline(always)]
    pub fn dsc(&mut self) -> DscW<CtrlSpec> {
        DscW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Warmup Delay"]
    #[inline(always)]
    pub fn warmupdly(&mut self) -> WarmupdlyW<CtrlSpec> {
        WarmupdlyW::new(self, 18)
    }
    #[doc = "Bits 24:30 - Presclae"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PrescaleW<CtrlSpec> {
        PrescaleW::new(self, 24)
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
#[doc = "`reset()` method sets CTRL to value 0x0010_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
