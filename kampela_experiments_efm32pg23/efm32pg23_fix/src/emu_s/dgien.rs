#[doc = "Register `DGIEN` reader"]
pub type R = crate::R<DgienSpec>;
#[doc = "Register `DGIEN` writer"]
pub type W = crate::W<DgienSpec>;
#[doc = "Field `EM23WAKEUPDGIEN` reader - EM23 Wake up Interrupt enable"]
pub type Em23wakeupdgienR = crate::BitReader;
#[doc = "Field `EM23WAKEUPDGIEN` writer - EM23 Wake up Interrupt enable"]
pub type Em23wakeupdgienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPDGIEN` reader - Temperature Interrupt enable"]
pub type TempdgienR = crate::BitReader;
#[doc = "Field `TEMPDGIEN` writer - Temperature Interrupt enable"]
pub type TempdgienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPLOWDGIEN` reader - Temperature low Interrupt enable"]
pub type TemplowdgienR = crate::BitReader;
#[doc = "Field `TEMPLOWDGIEN` writer - Temperature low Interrupt enable"]
pub type TemplowdgienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPHIGHDGIEN` reader - Temperature high Interrupt enable"]
pub type TemphighdgienR = crate::BitReader;
#[doc = "Field `TEMPHIGHDGIEN` writer - Temperature high Interrupt enable"]
pub type TemphighdgienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - EM23 Wake up Interrupt enable"]
    #[inline(always)]
    pub fn em23wakeupdgien(&self) -> Em23wakeupdgienR {
        Em23wakeupdgienR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - Temperature Interrupt enable"]
    #[inline(always)]
    pub fn tempdgien(&self) -> TempdgienR {
        TempdgienR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Temperature low Interrupt enable"]
    #[inline(always)]
    pub fn templowdgien(&self) -> TemplowdgienR {
        TemplowdgienR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Temperature high Interrupt enable"]
    #[inline(always)]
    pub fn temphighdgien(&self) -> TemphighdgienR {
        TemphighdgienR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - EM23 Wake up Interrupt enable"]
    #[inline(always)]
    pub fn em23wakeupdgien(&mut self) -> Em23wakeupdgienW<DgienSpec> {
        Em23wakeupdgienW::new(self, 24)
    }
    #[doc = "Bit 29 - Temperature Interrupt enable"]
    #[inline(always)]
    pub fn tempdgien(&mut self) -> TempdgienW<DgienSpec> {
        TempdgienW::new(self, 29)
    }
    #[doc = "Bit 30 - Temperature low Interrupt enable"]
    #[inline(always)]
    pub fn templowdgien(&mut self) -> TemplowdgienW<DgienSpec> {
        TemplowdgienW::new(self, 30)
    }
    #[doc = "Bit 31 - Temperature high Interrupt enable"]
    #[inline(always)]
    pub fn temphighdgien(&mut self) -> TemphighdgienW<DgienSpec> {
        TemphighdgienW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dgien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DgienSpec;
impl crate::RegisterSpec for DgienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dgien::R`](R) reader structure"]
impl crate::Readable for DgienSpec {}
#[doc = "`write(|w| ..)` method takes [`dgien::W`](W) writer structure"]
impl crate::Writable for DgienSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DGIEN to value 0"]
impl crate::Resettable for DgienSpec {}
