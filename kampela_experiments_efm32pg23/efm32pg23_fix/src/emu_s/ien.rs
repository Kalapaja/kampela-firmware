#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `AVDDBOD` reader - AVDD BOD Interrupt enable"]
pub type AvddbodR = crate::BitReader;
#[doc = "Field `AVDDBOD` writer - AVDD BOD Interrupt enable"]
pub type AvddbodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOVDD0BOD` reader - VDDIO0 BOD Interrupt enable"]
pub type Iovdd0bodR = crate::BitReader;
#[doc = "Field `IOVDD0BOD` writer - VDDIO0 BOD Interrupt enable"]
pub type Iovdd0bodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23WAKEUP` reader - EM23 Wake up Interrupt enable"]
pub type Em23wakeupR = crate::BitReader;
#[doc = "Field `EM23WAKEUP` writer - EM23 Wake up Interrupt enable"]
pub type Em23wakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSCALEDONE` reader - Vscale done Interrupt enable"]
pub type VscaledoneR = crate::BitReader;
#[doc = "Field `VSCALEDONE` writer - Vscale done Interrupt enable"]
pub type VscaledoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPAVG` reader - Temperature Interrupt enable"]
pub type TempavgR = crate::BitReader;
#[doc = "Field `TEMPAVG` writer - Temperature Interrupt enable"]
pub type TempavgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMP` reader - Temperature Interrupt enable"]
pub type TempR = crate::BitReader;
#[doc = "Field `TEMP` writer - Temperature Interrupt enable"]
pub type TempW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPLOW` reader - Temperature low Interrupt enable"]
pub type TemplowR = crate::BitReader;
#[doc = "Field `TEMPLOW` writer - Temperature low Interrupt enable"]
pub type TemplowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPHIGH` reader - Temperature high Interrupt enable"]
pub type TemphighR = crate::BitReader;
#[doc = "Field `TEMPHIGH` writer - Temperature high Interrupt enable"]
pub type TemphighW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - AVDD BOD Interrupt enable"]
    #[inline(always)]
    pub fn avddbod(&self) -> AvddbodR {
        AvddbodR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VDDIO0 BOD Interrupt enable"]
    #[inline(always)]
    pub fn iovdd0bod(&self) -> Iovdd0bodR {
        Iovdd0bodR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - EM23 Wake up Interrupt enable"]
    #[inline(always)]
    pub fn em23wakeup(&self) -> Em23wakeupR {
        Em23wakeupR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Vscale done Interrupt enable"]
    #[inline(always)]
    pub fn vscaledone(&self) -> VscaledoneR {
        VscaledoneR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Temperature Interrupt enable"]
    #[inline(always)]
    pub fn tempavg(&self) -> TempavgR {
        TempavgR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Temperature Interrupt enable"]
    #[inline(always)]
    pub fn temp(&self) -> TempR {
        TempR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Temperature low Interrupt enable"]
    #[inline(always)]
    pub fn templow(&self) -> TemplowR {
        TemplowR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Temperature high Interrupt enable"]
    #[inline(always)]
    pub fn temphigh(&self) -> TemphighR {
        TemphighR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - AVDD BOD Interrupt enable"]
    #[inline(always)]
    pub fn avddbod(&mut self) -> AvddbodW<IenSpec> {
        AvddbodW::new(self, 16)
    }
    #[doc = "Bit 17 - VDDIO0 BOD Interrupt enable"]
    #[inline(always)]
    pub fn iovdd0bod(&mut self) -> Iovdd0bodW<IenSpec> {
        Iovdd0bodW::new(self, 17)
    }
    #[doc = "Bit 24 - EM23 Wake up Interrupt enable"]
    #[inline(always)]
    pub fn em23wakeup(&mut self) -> Em23wakeupW<IenSpec> {
        Em23wakeupW::new(self, 24)
    }
    #[doc = "Bit 25 - Vscale done Interrupt enable"]
    #[inline(always)]
    pub fn vscaledone(&mut self) -> VscaledoneW<IenSpec> {
        VscaledoneW::new(self, 25)
    }
    #[doc = "Bit 27 - Temperature Interrupt enable"]
    #[inline(always)]
    pub fn tempavg(&mut self) -> TempavgW<IenSpec> {
        TempavgW::new(self, 27)
    }
    #[doc = "Bit 29 - Temperature Interrupt enable"]
    #[inline(always)]
    pub fn temp(&mut self) -> TempW<IenSpec> {
        TempW::new(self, 29)
    }
    #[doc = "Bit 30 - Temperature low Interrupt enable"]
    #[inline(always)]
    pub fn templow(&mut self) -> TemplowW<IenSpec> {
        TemplowW::new(self, 30)
    }
    #[doc = "Bit 31 - Temperature high Interrupt enable"]
    #[inline(always)]
    pub fn temphigh(&mut self) -> TemphighW<IenSpec> {
        TemphighW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
