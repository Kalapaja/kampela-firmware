#[doc = "Register `DGIF` reader"]
pub type R = crate::R<DgifSpec>;
#[doc = "Register `DGIF` writer"]
pub type W = crate::W<DgifSpec>;
#[doc = "Field `EM23WAKEUPDGIF` reader - EM23 Wake up Interrupt flag"]
pub type Em23wakeupdgifR = crate::BitReader;
#[doc = "Field `EM23WAKEUPDGIF` writer - EM23 Wake up Interrupt flag"]
pub type Em23wakeupdgifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPDGIF` reader - Temperature Interrupt flag"]
pub type TempdgifR = crate::BitReader;
#[doc = "Field `TEMPDGIF` writer - Temperature Interrupt flag"]
pub type TempdgifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPLOWDGIF` reader - Temperature low Interrupt flag"]
pub type TemplowdgifR = crate::BitReader;
#[doc = "Field `TEMPLOWDGIF` writer - Temperature low Interrupt flag"]
pub type TemplowdgifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPHIGHDGIF` reader - Temperature high Interrupt flag"]
pub type TemphighdgifR = crate::BitReader;
#[doc = "Field `TEMPHIGHDGIF` writer - Temperature high Interrupt flag"]
pub type TemphighdgifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - EM23 Wake up Interrupt flag"]
    #[inline(always)]
    pub fn em23wakeupdgif(&self) -> Em23wakeupdgifR {
        Em23wakeupdgifR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - Temperature Interrupt flag"]
    #[inline(always)]
    pub fn tempdgif(&self) -> TempdgifR {
        TempdgifR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Temperature low Interrupt flag"]
    #[inline(always)]
    pub fn templowdgif(&self) -> TemplowdgifR {
        TemplowdgifR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Temperature high Interrupt flag"]
    #[inline(always)]
    pub fn temphighdgif(&self) -> TemphighdgifR {
        TemphighdgifR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - EM23 Wake up Interrupt flag"]
    #[inline(always)]
    pub fn em23wakeupdgif(&mut self) -> Em23wakeupdgifW<DgifSpec> {
        Em23wakeupdgifW::new(self, 24)
    }
    #[doc = "Bit 29 - Temperature Interrupt flag"]
    #[inline(always)]
    pub fn tempdgif(&mut self) -> TempdgifW<DgifSpec> {
        TempdgifW::new(self, 29)
    }
    #[doc = "Bit 30 - Temperature low Interrupt flag"]
    #[inline(always)]
    pub fn templowdgif(&mut self) -> TemplowdgifW<DgifSpec> {
        TemplowdgifW::new(self, 30)
    }
    #[doc = "Bit 31 - Temperature high Interrupt flag"]
    #[inline(always)]
    pub fn temphighdgif(&mut self) -> TemphighdgifW<DgifSpec> {
        TemphighdgifW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dgif::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgif::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DgifSpec;
impl crate::RegisterSpec for DgifSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dgif::R`](R) reader structure"]
impl crate::Readable for DgifSpec {}
#[doc = "`write(|w| ..)` method takes [`dgif::W`](W) writer structure"]
impl crate::Writable for DgifSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DGIF to value 0"]
impl crate::Resettable for DgifSpec {}
