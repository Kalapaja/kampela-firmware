#[doc = "Register `ICACHERAMRETNCTRL` reader"]
pub type R = crate::R<IcacheramretnctrlSpec>;
#[doc = "Register `ICACHERAMRETNCTRL` writer"]
pub type W = crate::W<IcacheramretnctrlSpec>;
#[doc = "ICACHERAM Retention control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramretnctrl {
    #[doc = "0: None of the Host ICACHE RAM blocks powered down"]
    Allon = 0,
    #[doc = "1: Power down all Host ICACHE RAM blocks"]
    Alloff = 1,
}
impl From<Ramretnctrl> for bool {
    #[inline(always)]
    fn from(variant: Ramretnctrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRETNCTRL` reader - ICACHERAM Retention control"]
pub type RamretnctrlR = crate::BitReader<Ramretnctrl>;
impl RamretnctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramretnctrl {
        match self.bits {
            false => Ramretnctrl::Allon,
            true => Ramretnctrl::Alloff,
        }
    }
    #[doc = "None of the Host ICACHE RAM blocks powered down"]
    #[inline(always)]
    pub fn is_allon(&self) -> bool {
        *self == Ramretnctrl::Allon
    }
    #[doc = "Power down all Host ICACHE RAM blocks"]
    #[inline(always)]
    pub fn is_alloff(&self) -> bool {
        *self == Ramretnctrl::Alloff
    }
}
#[doc = "Field `RAMRETNCTRL` writer - ICACHERAM Retention control"]
pub type RamretnctrlW<'a, REG> = crate::BitWriter<'a, REG, Ramretnctrl>;
impl<'a, REG> RamretnctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "None of the Host ICACHE RAM blocks powered down"]
    #[inline(always)]
    pub fn allon(self) -> &'a mut crate::W<REG> {
        self.variant(Ramretnctrl::Allon)
    }
    #[doc = "Power down all Host ICACHE RAM blocks"]
    #[inline(always)]
    pub fn alloff(self) -> &'a mut crate::W<REG> {
        self.variant(Ramretnctrl::Alloff)
    }
}
impl R {
    #[doc = "Bit 0 - ICACHERAM Retention control"]
    #[inline(always)]
    pub fn ramretnctrl(&self) -> RamretnctrlR {
        RamretnctrlR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ICACHERAM Retention control"]
    #[inline(always)]
    pub fn ramretnctrl(&mut self) -> RamretnctrlW<IcacheramretnctrlSpec> {
        RamretnctrlW::new(self, 0)
    }
}
#[doc = "Configure Host ICACHERAM retention configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`icacheramretnctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icacheramretnctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcacheramretnctrlSpec;
impl crate::RegisterSpec for IcacheramretnctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icacheramretnctrl::R`](R) reader structure"]
impl crate::Readable for IcacheramretnctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`icacheramretnctrl::W`](W) writer structure"]
impl crate::Writable for IcacheramretnctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICACHERAMRETNCTRL to value 0"]
impl crate::Resettable for IcacheramretnctrlSpec {}
