#[doc = "Register `IRCTRL` reader"]
pub type R = crate::R<IrctrlSpec>;
#[doc = "Register `IRCTRL` writer"]
pub type W = crate::W<IrctrlSpec>;
#[doc = "Field `IREN` reader - Enable IrDA Module"]
pub type IrenR = crate::BitReader;
#[doc = "Field `IREN` writer - Enable IrDA Module"]
pub type IrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "IrDA TX Pulse Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irpw {
    #[doc = "0: IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    One = 0,
    #[doc = "1: IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    Two = 1,
    #[doc = "2: IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    Three = 2,
    #[doc = "3: IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    Four = 3,
}
impl From<Irpw> for u8 {
    #[inline(always)]
    fn from(variant: Irpw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irpw {
    type Ux = u8;
}
impl crate::IsEnum for Irpw {}
#[doc = "Field `IRPW` reader - IrDA TX Pulse Width"]
pub type IrpwR = crate::FieldReader<Irpw>;
impl IrpwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpw {
        match self.bits {
            0 => Irpw::One,
            1 => Irpw::Two,
            2 => Irpw::Three,
            3 => Irpw::Four,
            _ => unreachable!(),
        }
    }
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Irpw::One
    }
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Irpw::Two
    }
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Irpw::Three
    }
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Irpw::Four
    }
}
#[doc = "Field `IRPW` writer - IrDA TX Pulse Width"]
pub type IrpwW<'a, REG> = crate::FieldWriter<'a, REG, 2, Irpw, crate::Safe>;
impl<'a, REG> IrpwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Irpw::One)
    }
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Irpw::Two)
    }
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Irpw::Three)
    }
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Irpw::Four)
    }
}
#[doc = "IrDA RX Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irfilt {
    #[doc = "0: No filter enabled"]
    Disable = 0,
    #[doc = "1: Filter enabled. IrDA pulse must be high for at least 5 consecutive clock cycles to be detected"]
    Enable = 1,
}
impl From<Irfilt> for bool {
    #[inline(always)]
    fn from(variant: Irfilt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRFILT` reader - IrDA RX Filter"]
pub type IrfiltR = crate::BitReader<Irfilt>;
impl IrfiltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irfilt {
        match self.bits {
            false => Irfilt::Disable,
            true => Irfilt::Enable,
        }
    }
    #[doc = "No filter enabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Irfilt::Disable
    }
    #[doc = "Filter enabled. IrDA pulse must be high for at least 5 consecutive clock cycles to be detected"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Irfilt::Enable
    }
}
#[doc = "Field `IRFILT` writer - IrDA RX Filter"]
pub type IrfiltW<'a, REG> = crate::BitWriter<'a, REG, Irfilt>;
impl<'a, REG> IrfiltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No filter enabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Irfilt::Disable)
    }
    #[doc = "Filter enabled. IrDA pulse must be high for at least 5 consecutive clock cycles to be detected"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Irfilt::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    pub fn iren(&self) -> IrenR {
        IrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    pub fn irpw(&self) -> IrpwR {
        IrpwR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    pub fn irfilt(&self) -> IrfiltR {
        IrfiltR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    pub fn iren(&mut self) -> IrenW<IrctrlSpec> {
        IrenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    pub fn irpw(&mut self) -> IrpwW<IrctrlSpec> {
        IrpwW::new(self, 1)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    pub fn irfilt(&mut self) -> IrfiltW<IrctrlSpec> {
        IrfiltW::new(self, 3)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`irctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrctrlSpec;
impl crate::RegisterSpec for IrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irctrl::R`](R) reader structure"]
impl crate::Readable for IrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`irctrl::W`](W) writer structure"]
impl crate::Writable for IrctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRCTRL to value 0"]
impl crate::Resettable for IrctrlSpec {}
