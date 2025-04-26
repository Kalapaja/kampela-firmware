#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EM2DBGEN` reader - Enable debugging in EM2"]
pub type Em2dbgenR = crate::BitReader;
#[doc = "Field `EM2DBGEN` writer - Enable debugging in EM2"]
pub type Em2dbgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Averaged Temperature samples num\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tempavgnum {
    #[doc = "0: 16 measurements"]
    N16 = 0,
    #[doc = "1: 64 measurements"]
    N64 = 1,
}
impl From<Tempavgnum> for bool {
    #[inline(always)]
    fn from(variant: Tempavgnum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMPAVGNUM` reader - Averaged Temperature samples num"]
pub type TempavgnumR = crate::BitReader<Tempavgnum>;
impl TempavgnumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tempavgnum {
        match self.bits {
            false => Tempavgnum::N16,
            true => Tempavgnum::N64,
        }
    }
    #[doc = "16 measurements"]
    #[inline(always)]
    pub fn is_n16(&self) -> bool {
        *self == Tempavgnum::N16
    }
    #[doc = "64 measurements"]
    #[inline(always)]
    pub fn is_n64(&self) -> bool {
        *self == Tempavgnum::N64
    }
}
#[doc = "Field `TEMPAVGNUM` writer - Averaged Temperature samples num"]
pub type TempavgnumW<'a, REG> = crate::BitWriter<'a, REG, Tempavgnum>;
impl<'a, REG> TempavgnumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16 measurements"]
    #[inline(always)]
    pub fn n16(self) -> &'a mut crate::W<REG> {
        self.variant(Tempavgnum::N16)
    }
    #[doc = "64 measurements"]
    #[inline(always)]
    pub fn n64(self) -> &'a mut crate::W<REG> {
        self.variant(Tempavgnum::N64)
    }
}
#[doc = "EM2/EM3 Vscale\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em23vscale {
    #[doc = "0: VSCALE0. 0.9v"]
    Vscale0 = 0,
    #[doc = "1: VSCALE1. 1.0v"]
    Vscale1 = 1,
    #[doc = "2: VSCALE2. 1.1v"]
    Vscale2 = 2,
}
impl From<Em23vscale> for u8 {
    #[inline(always)]
    fn from(variant: Em23vscale) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em23vscale {
    type Ux = u8;
}
impl crate::IsEnum for Em23vscale {}
#[doc = "Field `EM23VSCALE` reader - EM2/EM3 Vscale"]
pub type Em23vscaleR = crate::FieldReader<Em23vscale>;
impl Em23vscaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Em23vscale> {
        match self.bits {
            0 => Some(Em23vscale::Vscale0),
            1 => Some(Em23vscale::Vscale1),
            2 => Some(Em23vscale::Vscale2),
            _ => None,
        }
    }
    #[doc = "VSCALE0. 0.9v"]
    #[inline(always)]
    pub fn is_vscale0(&self) -> bool {
        *self == Em23vscale::Vscale0
    }
    #[doc = "VSCALE1. 1.0v"]
    #[inline(always)]
    pub fn is_vscale1(&self) -> bool {
        *self == Em23vscale::Vscale1
    }
    #[doc = "VSCALE2. 1.1v"]
    #[inline(always)]
    pub fn is_vscale2(&self) -> bool {
        *self == Em23vscale::Vscale2
    }
}
#[doc = "Field `EM23VSCALE` writer - EM2/EM3 Vscale"]
pub type Em23vscaleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Em23vscale>;
impl<'a, REG> Em23vscaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VSCALE0. 0.9v"]
    #[inline(always)]
    pub fn vscale0(self) -> &'a mut crate::W<REG> {
        self.variant(Em23vscale::Vscale0)
    }
    #[doc = "VSCALE1. 1.0v"]
    #[inline(always)]
    pub fn vscale1(self) -> &'a mut crate::W<REG> {
        self.variant(Em23vscale::Vscale1)
    }
    #[doc = "VSCALE2. 1.1v"]
    #[inline(always)]
    pub fn vscale2(self) -> &'a mut crate::W<REG> {
        self.variant(Em23vscale::Vscale2)
    }
}
#[doc = "Field `FLASHPWRUPONDEMAND` reader - Enable flash on demand wakeup"]
pub type FlashpwrupondemandR = crate::BitReader;
#[doc = "Field `FLASHPWRUPONDEMAND` writer - Enable flash on demand wakeup"]
pub type FlashpwrupondemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFPDIRECTMODEEN` reader - EFP Direct Mode Enable"]
pub type EfpdirectmodeenR = crate::BitReader;
#[doc = "Field `EFPDIRECTMODEEN` writer - EFP Direct Mode Enable"]
pub type EfpdirectmodeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFPDRVDECOUPLE` reader - EFP drives DECOUPLE"]
pub type EfpdrvdecoupleR = crate::BitReader;
#[doc = "Field `EFPDRVDECOUPLE` writer - EFP drives DECOUPLE"]
pub type EfpdrvdecoupleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFPDRVDVDD` reader - EFP drives DVDD"]
pub type EfpdrvdvddR = crate::BitReader;
#[doc = "Field `EFPDRVDVDD` writer - EFP drives DVDD"]
pub type EfpdrvdvddW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable debugging in EM2"]
    #[inline(always)]
    pub fn em2dbgen(&self) -> Em2dbgenR {
        Em2dbgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Averaged Temperature samples num"]
    #[inline(always)]
    pub fn tempavgnum(&self) -> TempavgnumR {
        TempavgnumR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:9 - EM2/EM3 Vscale"]
    #[inline(always)]
    pub fn em23vscale(&self) -> Em23vscaleR {
        Em23vscaleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Enable flash on demand wakeup"]
    #[inline(always)]
    pub fn flashpwrupondemand(&self) -> FlashpwrupondemandR {
        FlashpwrupondemandR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 29 - EFP Direct Mode Enable"]
    #[inline(always)]
    pub fn efpdirectmodeen(&self) -> EfpdirectmodeenR {
        EfpdirectmodeenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - EFP drives DECOUPLE"]
    #[inline(always)]
    pub fn efpdrvdecouple(&self) -> EfpdrvdecoupleR {
        EfpdrvdecoupleR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - EFP drives DVDD"]
    #[inline(always)]
    pub fn efpdrvdvdd(&self) -> EfpdrvdvddR {
        EfpdrvdvddR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable debugging in EM2"]
    #[inline(always)]
    pub fn em2dbgen(&mut self) -> Em2dbgenW<CtrlSpec> {
        Em2dbgenW::new(self, 0)
    }
    #[doc = "Bit 3 - Averaged Temperature samples num"]
    #[inline(always)]
    pub fn tempavgnum(&mut self) -> TempavgnumW<CtrlSpec> {
        TempavgnumW::new(self, 3)
    }
    #[doc = "Bits 8:9 - EM2/EM3 Vscale"]
    #[inline(always)]
    pub fn em23vscale(&mut self) -> Em23vscaleW<CtrlSpec> {
        Em23vscaleW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable flash on demand wakeup"]
    #[inline(always)]
    pub fn flashpwrupondemand(&mut self) -> FlashpwrupondemandW<CtrlSpec> {
        FlashpwrupondemandW::new(self, 16)
    }
    #[doc = "Bit 29 - EFP Direct Mode Enable"]
    #[inline(always)]
    pub fn efpdirectmodeen(&mut self) -> EfpdirectmodeenW<CtrlSpec> {
        EfpdirectmodeenW::new(self, 29)
    }
    #[doc = "Bit 30 - EFP drives DECOUPLE"]
    #[inline(always)]
    pub fn efpdrvdecouple(&mut self) -> EfpdrvdecoupleW<CtrlSpec> {
        EfpdrvdecoupleW::new(self, 30)
    }
    #[doc = "Bit 31 - EFP drives DVDD"]
    #[inline(always)]
    pub fn efpdrvdvdd(&mut self) -> EfpdrvdvddW<CtrlSpec> {
        EfpdrvdvddW::new(self, 31)
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
#[doc = "`reset()` method sets CTRL to value 0x0200"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0200;
}
