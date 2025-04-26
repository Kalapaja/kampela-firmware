#[doc = "Register `PFMXCTRL` reader"]
pub type R = crate::R<PfmxctrlSpec>;
#[doc = "Register `PFMXCTRL` writer"]
pub type W = crate::W<PfmxctrlSpec>;
#[doc = "PFMX mode Peak Current Setting\n\nValue on reset: 12"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ipkval {
    #[doc = "3: Ipeak = 90 mA, Iload = 50 mA"]
    Load50ma = 3,
    #[doc = "4: Ipeak = 100 mA, Iload = 65 mA"]
    Load65ma = 4,
    #[doc = "5: Ipeak = 110 mA, Iload = 73 mA"]
    Load73ma = 5,
    #[doc = "6: Ipeak = 120 mA, Iload = 80 mA"]
    Load80ma = 6,
    #[doc = "7: Ipeak = 130 mA, Iload = 86 mA"]
    Load86ma = 7,
    #[doc = "8: Ipeak = 140 mA, Iload = 93 mA"]
    Load93ma = 8,
    #[doc = "9: Ipeak = 150 mA, Iload = 100 mA"]
    Load100ma = 9,
    #[doc = "10: Ipeak = 160 mA, Iload = 106 mA"]
    Load106ma = 10,
    #[doc = "11: Ipeak = 170 mA, Iload = 113 mA"]
    Load113ma = 11,
    #[doc = "12: Ipeak = 180 mA, Iload = 120 mA"]
    Load120ma = 12,
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
#[doc = "Field `IPKVAL` reader - PFMX mode Peak Current Setting"]
pub type IpkvalR = crate::FieldReader<Ipkval>;
impl IpkvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ipkval> {
        match self.bits {
            3 => Some(Ipkval::Load50ma),
            4 => Some(Ipkval::Load65ma),
            5 => Some(Ipkval::Load73ma),
            6 => Some(Ipkval::Load80ma),
            7 => Some(Ipkval::Load86ma),
            8 => Some(Ipkval::Load93ma),
            9 => Some(Ipkval::Load100ma),
            10 => Some(Ipkval::Load106ma),
            11 => Some(Ipkval::Load113ma),
            12 => Some(Ipkval::Load120ma),
            _ => None,
        }
    }
    #[doc = "Ipeak = 90 mA, Iload = 50 mA"]
    #[inline(always)]
    pub fn is_load50ma(&self) -> bool {
        *self == Ipkval::Load50ma
    }
    #[doc = "Ipeak = 100 mA, Iload = 65 mA"]
    #[inline(always)]
    pub fn is_load65ma(&self) -> bool {
        *self == Ipkval::Load65ma
    }
    #[doc = "Ipeak = 110 mA, Iload = 73 mA"]
    #[inline(always)]
    pub fn is_load73ma(&self) -> bool {
        *self == Ipkval::Load73ma
    }
    #[doc = "Ipeak = 120 mA, Iload = 80 mA"]
    #[inline(always)]
    pub fn is_load80ma(&self) -> bool {
        *self == Ipkval::Load80ma
    }
    #[doc = "Ipeak = 130 mA, Iload = 86 mA"]
    #[inline(always)]
    pub fn is_load86ma(&self) -> bool {
        *self == Ipkval::Load86ma
    }
    #[doc = "Ipeak = 140 mA, Iload = 93 mA"]
    #[inline(always)]
    pub fn is_load93ma(&self) -> bool {
        *self == Ipkval::Load93ma
    }
    #[doc = "Ipeak = 150 mA, Iload = 100 mA"]
    #[inline(always)]
    pub fn is_load100ma(&self) -> bool {
        *self == Ipkval::Load100ma
    }
    #[doc = "Ipeak = 160 mA, Iload = 106 mA"]
    #[inline(always)]
    pub fn is_load106ma(&self) -> bool {
        *self == Ipkval::Load106ma
    }
    #[doc = "Ipeak = 170 mA, Iload = 113 mA"]
    #[inline(always)]
    pub fn is_load113ma(&self) -> bool {
        *self == Ipkval::Load113ma
    }
    #[doc = "Ipeak = 180 mA, Iload = 120 mA"]
    #[inline(always)]
    pub fn is_load120ma(&self) -> bool {
        *self == Ipkval::Load120ma
    }
}
#[doc = "Field `IPKVAL` writer - PFMX mode Peak Current Setting"]
pub type IpkvalW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ipkval>;
impl<'a, REG> IpkvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ipeak = 90 mA, Iload = 50 mA"]
    #[inline(always)]
    pub fn load50ma(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load50ma)
    }
    #[doc = "Ipeak = 100 mA, Iload = 65 mA"]
    #[inline(always)]
    pub fn load65ma(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load65ma)
    }
    #[doc = "Ipeak = 110 mA, Iload = 73 mA"]
    #[inline(always)]
    pub fn load73ma(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load73ma)
    }
    #[doc = "Ipeak = 120 mA, Iload = 80 mA"]
    #[inline(always)]
    pub fn load80ma(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load80ma)
    }
    #[doc = "Ipeak = 130 mA, Iload = 86 mA"]
    #[inline(always)]
    pub fn load86ma(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load86ma)
    }
    #[doc = "Ipeak = 140 mA, Iload = 93 mA"]
    #[inline(always)]
    pub fn load93ma(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load93ma)
    }
    #[doc = "Ipeak = 150 mA, Iload = 100 mA"]
    #[inline(always)]
    pub fn load100ma(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load100ma)
    }
    #[doc = "Ipeak = 160 mA, Iload = 106 mA"]
    #[inline(always)]
    pub fn load106ma(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load106ma)
    }
    #[doc = "Ipeak = 170 mA, Iload = 113 mA"]
    #[inline(always)]
    pub fn load113ma(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load113ma)
    }
    #[doc = "Ipeak = 180 mA, Iload = 120 mA"]
    #[inline(always)]
    pub fn load120ma(self) -> &'a mut crate::W<REG> {
        self.variant(Ipkval::Load120ma)
    }
}
#[doc = "Field `IPKTMAXCTRL` reader - Ton_max timeout control"]
pub type IpktmaxctrlR = crate::FieldReader;
#[doc = "Field `IPKTMAXCTRL` writer - Ton_max timeout control"]
pub type IpktmaxctrlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - PFMX mode Peak Current Setting"]
    #[inline(always)]
    pub fn ipkval(&self) -> IpkvalR {
        IpkvalR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Ton_max timeout control"]
    #[inline(always)]
    pub fn ipktmaxctrl(&self) -> IpktmaxctrlR {
        IpktmaxctrlR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PFMX mode Peak Current Setting"]
    #[inline(always)]
    pub fn ipkval(&mut self) -> IpkvalW<PfmxctrlSpec> {
        IpkvalW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Ton_max timeout control"]
    #[inline(always)]
    pub fn ipktmaxctrl(&mut self) -> IpktmaxctrlW<PfmxctrlSpec> {
        IpktmaxctrlW::new(self, 8)
    }
}
#[doc = "PFMX Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pfmxctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfmxctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfmxctrlSpec;
impl crate::RegisterSpec for PfmxctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfmxctrl::R`](R) reader structure"]
impl crate::Readable for PfmxctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pfmxctrl::W`](W) writer structure"]
impl crate::Writable for PfmxctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PFMXCTRL to value 0x0c0c"]
impl crate::Resettable for PfmxctrlSpec {
    const RESET_VALUE: u32 = 0x0c0c;
}
