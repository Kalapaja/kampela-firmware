#[doc = "Register `EM23CTRL0` reader"]
pub type R = crate::R<Em23ctrl0Spec>;
#[doc = "Register `EM23CTRL0` writer"]
pub type W = crate::W<Em23ctrl0Spec>;
#[doc = "EM23 Peak Current Setting\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ipkval {
    #[doc = "3: Ipeak = 90mA, Iload = 5 mA"]
    Load5mA = 3,
    #[doc = "9: Ipeak = 150mA, Iload = 10 mA"]
    Load10mA = 9,
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
#[doc = "Field `IPKVAL` reader - EM23 Peak Current Setting"]
pub type IpkvalR = crate::FieldReader<Ipkval>;
impl IpkvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ipkval> {
        match self.bits {
            3 => Some(Ipkval::Load5mA),
            9 => Some(Ipkval::Load10mA),
            _ => None,
        }
    }
    #[doc = "Ipeak = 90mA, Iload = 5 mA"]
    #[inline(always)]
    pub fn is_load5m_a(&self) -> bool {
        *self == Ipkval::Load5mA
    }
    #[doc = "Ipeak = 150mA, Iload = 10 mA"]
    #[inline(always)]
    pub fn is_load10m_a(&self) -> bool {
        *self == Ipkval::Load10mA
    }
}
#[doc = "Field `IPKVAL` writer - EM23 Peak Current Setting"]
pub type IpkvalW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ipkval>;
impl<'a, REG> IpkvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ipeak = 90mA, Iload = 5 mA"]
    #[inline(always)]
    pub fn load5m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load5mA)
    }
    #[doc = "Ipeak = 150mA, Iload = 10 mA"]
    #[inline(always)]
    pub fn load10m_a(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load10mA)
    }
}
#[doc = "EM23 Drive Speed Setting\n\nValue on reset: 1"]
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
#[doc = "Field `DRVSPEED` reader - EM23 Drive Speed Setting"]
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
#[doc = "Field `DRVSPEED` writer - EM23 Drive Speed Setting"]
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
    #[doc = "Bits 0:3 - EM23 Peak Current Setting"]
    #[inline(always)]
    pub fn ipkval(&self) -> IpkvalR {
        IpkvalR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - EM23 Drive Speed Setting"]
    #[inline(always)]
    pub fn drvspeed(&self) -> DrvspeedR {
        DrvspeedR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EM23 Peak Current Setting"]
    #[inline(always)]
    pub fn ipkval(&mut self) -> IpkvalW<Em23ctrl0Spec> {
        IpkvalW::new(self, 0)
    }
    #[doc = "Bits 8:9 - EM23 Drive Speed Setting"]
    #[inline(always)]
    pub fn drvspeed(&mut self) -> DrvspeedW<Em23ctrl0Spec> {
        DrvspeedW::new(self, 8)
    }
}
#[doc = "EM23 Configurations\n\nYou can [`read`](crate::Reg::read) this register and get [`em23ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em23ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em23ctrl0Spec;
impl crate::RegisterSpec for Em23ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em23ctrl0::R`](R) reader structure"]
impl crate::Readable for Em23ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`em23ctrl0::W`](W) writer structure"]
impl crate::Writable for Em23ctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EM23CTRL0 to value 0x0103"]
impl crate::Resettable for Em23ctrl0Spec {
    const RESET_VALUE: u32 = 0x0103;
}
