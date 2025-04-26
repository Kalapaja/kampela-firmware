#[doc = "Register `EM01CTRL0` reader"]
pub type R = crate::R<Em01ctrl0Spec>;
#[doc = "Register `EM01CTRL0` writer"]
pub type W = crate::W<Em01ctrl0Spec>;
#[doc = "EM01 Peak Current Setting\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ipkval {
    #[doc = "3: Ipeak = 90mA, Iload = 36mA"]
    Load36mA = 3,
    #[doc = "4: Ipeak = 100mA, Iload = 40mA"]
    Load40mA = 4,
    #[doc = "5: Ipeak = 110mA, Iload = 44mA"]
    Load44mA = 5,
    #[doc = "6: Ipeak = 120mA, Iload = 48mA"]
    Load48mA = 6,
    #[doc = "7: Ipeak = 130mA, Iload = 52mA"]
    Load52mA = 7,
    #[doc = "8: Ipeak = 140mA, Iload = 56mA"]
    Load56mA = 8,
    #[doc = "9: Ipeak = 150mA, Iload = 60mA"]
    Load60mA = 9,
}
impl From<Ipkval> for u8 {
    #[inline(always)]
    fn from(variant: Ipkval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ipkval {
    type Ux = u8;
}
impl crate::IsEnum for Ipkval {}
#[doc = "Field `IPKVAL` reader - EM01 Peak Current Setting"]
pub type IpkvalR = crate::FieldReader<Ipkval>;
impl IpkvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ipkval> {
        match self.bits {
            3 => Some(Ipkval::Load36mA),
            4 => Some(Ipkval::Load40mA),
            5 => Some(Ipkval::Load44mA),
            6 => Some(Ipkval::Load48mA),
            7 => Some(Ipkval::Load52mA),
            8 => Some(Ipkval::Load56mA),
            9 => Some(Ipkval::Load60mA),
            _ => None,
        }
    }
    #[doc = "Ipeak = 90mA, Iload = 36mA"]
    #[inline(always)]
    pub fn is_load36m_a(&self) -> bool {
        *self == Ipkval::Load36mA
    }
    #[doc = "Ipeak = 100mA, Iload = 40mA"]
    #[inline(always)]
    pub fn is_load40m_a(&self) -> bool {
        *self == Ipkval::Load40mA
    }
    #[doc = "Ipeak = 110mA, Iload = 44mA"]
    #[inline(always)]
    pub fn is_load44m_a(&self) -> bool {
        *self == Ipkval::Load44mA
    }
    #[doc = "Ipeak = 120mA, Iload = 48mA"]
    #[inline(always)]
    pub fn is_load48m_a(&self) -> bool {
        *self == Ipkval::Load48mA
    }
    #[doc = "Ipeak = 130mA, Iload = 52mA"]
    #[inline(always)]
    pub fn is_load52m_a(&self) -> bool {
        *self == Ipkval::Load52mA
    }
    #[doc = "Ipeak = 140mA, Iload = 56mA"]
    #[inline(always)]
    pub fn is_load56m_a(&self) -> bool {
        *self == Ipkval::Load56mA
    }
    #[doc = "Ipeak = 150mA, Iload = 60mA"]
    #[inline(always)]
    pub fn is_load60m_a(&self) -> bool {
        *self == Ipkval::Load60mA
    }
}
#[doc = "Field `IPKVAL` writer - EM01 Peak Current Setting"]
pub type IpkvalW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ipkval>;
impl<'a, REG> IpkvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ipeak = 90mA, Iload = 36mA"]
    #[inline(always)]
    pub fn load36m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load36mA)
    }
    #[doc = "Ipeak = 100mA, Iload = 40mA"]
    #[inline(always)]
    pub fn load40m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load40mA)
    }
    #[doc = "Ipeak = 110mA, Iload = 44mA"]
    #[inline(always)]
    pub fn load44m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load44mA)
    }
    #[doc = "Ipeak = 120mA, Iload = 48mA"]
    #[inline(always)]
    pub fn load48m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load48mA)
    }
    #[doc = "Ipeak = 130mA, Iload = 52mA"]
    #[inline(always)]
    pub fn load52m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load52mA)
    }
    #[doc = "Ipeak = 140mA, Iload = 56mA"]
    #[inline(always)]
    pub fn load56m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load56mA)
    }
    #[doc = "Ipeak = 150mA, Iload = 60mA"]
    #[inline(always)]
    pub fn load60m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load60mA)
    }
}
#[doc = "EM01 Drive Speed Setting\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drvspeed {
    #[doc = "0: Not recommended for use (no benefit to this setting)"]
    BestEmi = 0,
    #[doc = "1: Recommended for use for best efficiency and low EMI"]
    DefaultSetting = 1,
    #[doc = "2: Not recommended for use (no benefit to this setting)"]
    Intermediate = 2,
    #[doc = "3: Not recommended for use (no benefit to this setting)"]
    BestEfficiency = 3,
}
impl From<Drvspeed> for u8 {
    #[inline(always)]
    fn from(variant: Drvspeed) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Drvspeed {
    type Ux = u8;
}
impl crate::IsEnum for Drvspeed {}
#[doc = "Field `DRVSPEED` reader - EM01 Drive Speed Setting"]
pub type DrvspeedR = crate::FieldReader<Drvspeed>;
impl DrvspeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drvspeed {
        match self.bits {
            0 => Drvspeed::BestEmi,
            1 => Drvspeed::DefaultSetting,
            2 => Drvspeed::Intermediate,
            3 => Drvspeed::BestEfficiency,
            _ => unreachable!(),
        }
    }
    #[doc = "Not recommended for use (no benefit to this setting)"]
    #[inline(always)]
    pub fn is_best_emi(&self) -> bool {
        *self == Drvspeed::BestEmi
    }
    #[doc = "Recommended for use for best efficiency and low EMI"]
    #[inline(always)]
    pub fn is_default_setting(&self) -> bool {
        *self == Drvspeed::DefaultSetting
    }
    #[doc = "Not recommended for use (no benefit to this setting)"]
    #[inline(always)]
    pub fn is_intermediate(&self) -> bool {
        *self == Drvspeed::Intermediate
    }
    #[doc = "Not recommended for use (no benefit to this setting)"]
    #[inline(always)]
    pub fn is_best_efficiency(&self) -> bool {
        *self == Drvspeed::BestEfficiency
    }
}
#[doc = "Field `DRVSPEED` writer - EM01 Drive Speed Setting"]
pub type DrvspeedW<'a, REG> = crate::FieldWriter<'a, REG, 2, Drvspeed, crate::Safe>;
impl<'a, REG> DrvspeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not recommended for use (no benefit to this setting)"]
    #[inline(always)]
    pub fn best_emi(self) -> &'a mut crate::W<REG> {
        self.variant(Drvspeed::BestEmi)
    }
    #[doc = "Recommended for use for best efficiency and low EMI"]
    #[inline(always)]
    pub fn default_setting(self) -> &'a mut crate::W<REG> {
        self.variant(Drvspeed::DefaultSetting)
    }
    #[doc = "Not recommended for use (no benefit to this setting)"]
    #[inline(always)]
    pub fn intermediate(self) -> &'a mut crate::W<REG> {
        self.variant(Drvspeed::Intermediate)
    }
    #[doc = "Not recommended for use (no benefit to this setting)"]
    #[inline(always)]
    pub fn best_efficiency(self) -> &'a mut crate::W<REG> {
        self.variant(Drvspeed::BestEfficiency)
    }
}
impl R {
    #[doc = "Bits 0:3 - EM01 Peak Current Setting"]
    #[inline(always)]
    pub fn ipkval(&self) -> IpkvalR {
        IpkvalR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - EM01 Drive Speed Setting"]
    #[inline(always)]
    pub fn drvspeed(&self) -> DrvspeedR {
        DrvspeedR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EM01 Peak Current Setting"]
    #[inline(always)]
    pub fn ipkval(&mut self) -> IpkvalW<Em01ctrl0Spec> {
        IpkvalW::new(self, 0)
    }
    #[doc = "Bits 8:9 - EM01 Drive Speed Setting"]
    #[inline(always)]
    pub fn drvspeed(&mut self) -> DrvspeedW<Em01ctrl0Spec> {
        DrvspeedW::new(self, 8)
    }
}
#[doc = "EM01 Configurations\n\nYou can [`read`](crate::Reg::read) this register and get [`em01ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em01ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em01ctrl0Spec;
impl crate::RegisterSpec for Em01ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em01ctrl0::R`](R) reader structure"]
impl crate::Readable for Em01ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`em01ctrl0::W`](W) writer structure"]
impl crate::Writable for Em01ctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EM01CTRL0 to value 0x0109"]
impl crate::Resettable for Em01ctrl0Spec {
    const RESET_VALUE: u32 = 0x0109;
}
