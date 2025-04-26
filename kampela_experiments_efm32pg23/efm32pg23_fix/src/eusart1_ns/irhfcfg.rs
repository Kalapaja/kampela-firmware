#[doc = "Register `IRHFCFG` reader"]
pub type R = crate::R<IrhfcfgSpec>;
#[doc = "Register `IRHFCFG` writer"]
pub type W = crate::W<IrhfcfgSpec>;
#[doc = "Field `IRHFEN` reader - Enable IrDA Module"]
pub type IrhfenR = crate::BitReader;
#[doc = "Field `IRHFEN` writer - Enable IrDA Module"]
pub type IrhfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "IrDA TX Pulse Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irhfpw {
    #[doc = "0: IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    One = 0,
    #[doc = "1: IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    Two = 1,
    #[doc = "2: IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    Three = 2,
    #[doc = "3: IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    Four = 3,
}
impl From<Irhfpw> for u8 {
    #[inline(always)]
    fn from(variant: Irhfpw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irhfpw {
    type Ux = u8;
}
impl crate::IsEnum for Irhfpw {}
#[doc = "Field `IRHFPW` reader - IrDA TX Pulse Width"]
pub type IrhfpwR = crate::FieldReader<Irhfpw>;
impl IrhfpwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irhfpw {
        match self.bits {
            0 => Irhfpw::One,
            1 => Irhfpw::Two,
            2 => Irhfpw::Three,
            3 => Irhfpw::Four,
            _ => unreachable!(),
        }
    }
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Irhfpw::One
    }
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Irhfpw::Two
    }
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Irhfpw::Three
    }
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Irhfpw::Four
    }
}
#[doc = "Field `IRHFPW` writer - IrDA TX Pulse Width"]
pub type IrhfpwW<'a, REG> = crate::FieldWriter<'a, REG, 2, Irhfpw, crate::Safe>;
impl<'a, REG> IrhfpwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Irhfpw::One)
    }
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Irhfpw::Two)
    }
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Irhfpw::Three)
    }
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Irhfpw::Four)
    }
}
#[doc = "IrDA RX Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irhffilt {
    #[doc = "0: No filter enabled"]
    Disable = 0,
    #[doc = "1: Filter enabled. IrDA pulse must be high for at least 5 consecutive clock cycles to be detected"]
    Enable = 1,
}
impl From<Irhffilt> for bool {
    #[inline(always)]
    fn from(variant: Irhffilt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRHFFILT` reader - IrDA RX Filter"]
pub type IrhffiltR = crate::BitReader<Irhffilt>;
impl IrhffiltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irhffilt {
        match self.bits {
            false => Irhffilt::Disable,
            true => Irhffilt::Enable,
        }
    }
    #[doc = "No filter enabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Irhffilt::Disable
    }
    #[doc = "Filter enabled. IrDA pulse must be high for at least 5 consecutive clock cycles to be detected"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Irhffilt::Enable
    }
}
#[doc = "Field `IRHFFILT` writer - IrDA RX Filter"]
pub type IrhffiltW<'a, REG> = crate::BitWriter<'a, REG, Irhffilt>;
impl<'a, REG> IrhffiltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No filter enabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Irhffilt::Disable)
    }
    #[doc = "Filter enabled. IrDA pulse must be high for at least 5 consecutive clock cycles to be detected"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Irhffilt::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    pub fn irhfen(&self) -> IrhfenR {
        IrhfenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    pub fn irhfpw(&self) -> IrhfpwR {
        IrhfpwR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    pub fn irhffilt(&self) -> IrhffiltR {
        IrhffiltR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    pub fn irhfen(&mut self) -> IrhfenW<IrhfcfgSpec> {
        IrhfenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    pub fn irhfpw(&mut self) -> IrhfpwW<IrhfcfgSpec> {
        IrhfpwW::new(self, 1)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    pub fn irhffilt(&mut self) -> IrhffiltW<IrhfcfgSpec> {
        IrhffiltW::new(self, 3)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`irhfcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irhfcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrhfcfgSpec;
impl crate::RegisterSpec for IrhfcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irhfcfg::R`](R) reader structure"]
impl crate::Readable for IrhfcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`irhfcfg::W`](W) writer structure"]
impl crate::Writable for IrhfcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRHFCFG to value 0"]
impl crate::Resettable for IrhfcfgSpec {}
