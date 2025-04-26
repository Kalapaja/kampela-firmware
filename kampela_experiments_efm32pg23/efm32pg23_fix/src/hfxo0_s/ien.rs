#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `RDY` reader - Digital Clock Ready Interrupt"]
pub type RdyR = crate::BitReader;
#[doc = "Field `RDY` writer - Digital Clock Ready Interrupt"]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREBIASOPTRDY` reader - Core Bias Optimization Ready Interrupt"]
pub type CorebiasoptrdyR = crate::BitReader;
#[doc = "Field `COREBIASOPTRDY` writer - Core Bias Optimization Ready Interrupt"]
pub type CorebiasoptrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSRDY` reader - PRS Ready Interrupt"]
pub type PrsrdyR = crate::BitReader;
#[doc = "Field `PRSRDY` writer - PRS Ready Interrupt"]
pub type PrsrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFOUTRDY` reader - BUFOUT Ready Interrupt"]
pub type BufoutrdyR = crate::BitReader;
#[doc = "Field `BUFOUTRDY` writer - BUFOUT Ready Interrupt"]
pub type BufoutrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFOUTFROZEN` reader - BUFOUT FROZEN Interrupt"]
pub type BufoutfrozenR = crate::BitReader;
#[doc = "Field `BUFOUTFROZEN` writer - BUFOUT FROZEN Interrupt"]
pub type BufoutfrozenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSERR` reader - PRS Requset Error Interrupt"]
pub type PrserrR = crate::BitReader;
#[doc = "Field `PRSERR` writer - PRS Requset Error Interrupt"]
pub type PrserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFOUTERR` reader - BUFOUT Request Error Interrupt"]
pub type BufouterrR = crate::BitReader;
#[doc = "Field `BUFOUTERR` writer - BUFOUT Request Error Interrupt"]
pub type BufouterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFOUTFREEZEERR` reader - BUFOUT Freeze Error Interrupt"]
pub type BufoutfreezeerrR = crate::BitReader;
#[doc = "Field `BUFOUTFREEZEERR` writer - BUFOUT Freeze Error Interrupt"]
pub type BufoutfreezeerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFOUTDNSERR` reader - BUFOUT Did Not Start Error Interrupt"]
pub type BufoutdnserrR = crate::BitReader;
#[doc = "Field `BUFOUTDNSERR` writer - BUFOUT Did Not Start Error Interrupt"]
pub type BufoutdnserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNSERR` reader - Did Not Start Error Interrupt"]
pub type DnserrR = crate::BitReader;
#[doc = "Field `DNSERR` writer - Did Not Start Error Interrupt"]
pub type DnserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFTIMEOUTERR` reader - Low Frequency Timeout Error Interrupt"]
pub type LftimeouterrR = crate::BitReader;
#[doc = "Field `LFTIMEOUTERR` writer - Low Frequency Timeout Error Interrupt"]
pub type LftimeouterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREBIASOPTERR` reader - Core Bias Optimization Error Interrupt"]
pub type CorebiasopterrR = crate::BitReader;
#[doc = "Field `COREBIASOPTERR` writer - Core Bias Optimization Error Interrupt"]
pub type CorebiasopterrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Digital Clock Ready Interrupt"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core Bias Optimization Ready Interrupt"]
    #[inline(always)]
    pub fn corebiasoptrdy(&self) -> CorebiasoptrdyR {
        CorebiasoptrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PRS Ready Interrupt"]
    #[inline(always)]
    pub fn prsrdy(&self) -> PrsrdyR {
        PrsrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUFOUT Ready Interrupt"]
    #[inline(always)]
    pub fn bufoutrdy(&self) -> BufoutrdyR {
        BufoutrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - BUFOUT FROZEN Interrupt"]
    #[inline(always)]
    pub fn bufoutfrozen(&self) -> BufoutfrozenR {
        BufoutfrozenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - PRS Requset Error Interrupt"]
    #[inline(always)]
    pub fn prserr(&self) -> PrserrR {
        PrserrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BUFOUT Request Error Interrupt"]
    #[inline(always)]
    pub fn bufouterr(&self) -> BufouterrR {
        BufouterrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - BUFOUT Freeze Error Interrupt"]
    #[inline(always)]
    pub fn bufoutfreezeerr(&self) -> BufoutfreezeerrR {
        BufoutfreezeerrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - BUFOUT Did Not Start Error Interrupt"]
    #[inline(always)]
    pub fn bufoutdnserr(&self) -> BufoutdnserrR {
        BufoutdnserrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Did Not Start Error Interrupt"]
    #[inline(always)]
    pub fn dnserr(&self) -> DnserrR {
        DnserrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Low Frequency Timeout Error Interrupt"]
    #[inline(always)]
    pub fn lftimeouterr(&self) -> LftimeouterrR {
        LftimeouterrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Core Bias Optimization Error Interrupt"]
    #[inline(always)]
    pub fn corebiasopterr(&self) -> CorebiasopterrR {
        CorebiasopterrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Digital Clock Ready Interrupt"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<IenSpec> {
        RdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Core Bias Optimization Ready Interrupt"]
    #[inline(always)]
    pub fn corebiasoptrdy(&mut self) -> CorebiasoptrdyW<IenSpec> {
        CorebiasoptrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - PRS Ready Interrupt"]
    #[inline(always)]
    pub fn prsrdy(&mut self) -> PrsrdyW<IenSpec> {
        PrsrdyW::new(self, 2)
    }
    #[doc = "Bit 3 - BUFOUT Ready Interrupt"]
    #[inline(always)]
    pub fn bufoutrdy(&mut self) -> BufoutrdyW<IenSpec> {
        BufoutrdyW::new(self, 3)
    }
    #[doc = "Bit 15 - BUFOUT FROZEN Interrupt"]
    #[inline(always)]
    pub fn bufoutfrozen(&mut self) -> BufoutfrozenW<IenSpec> {
        BufoutfrozenW::new(self, 15)
    }
    #[doc = "Bit 20 - PRS Requset Error Interrupt"]
    #[inline(always)]
    pub fn prserr(&mut self) -> PrserrW<IenSpec> {
        PrserrW::new(self, 20)
    }
    #[doc = "Bit 21 - BUFOUT Request Error Interrupt"]
    #[inline(always)]
    pub fn bufouterr(&mut self) -> BufouterrW<IenSpec> {
        BufouterrW::new(self, 21)
    }
    #[doc = "Bit 27 - BUFOUT Freeze Error Interrupt"]
    #[inline(always)]
    pub fn bufoutfreezeerr(&mut self) -> BufoutfreezeerrW<IenSpec> {
        BufoutfreezeerrW::new(self, 27)
    }
    #[doc = "Bit 28 - BUFOUT Did Not Start Error Interrupt"]
    #[inline(always)]
    pub fn bufoutdnserr(&mut self) -> BufoutdnserrW<IenSpec> {
        BufoutdnserrW::new(self, 28)
    }
    #[doc = "Bit 29 - Did Not Start Error Interrupt"]
    #[inline(always)]
    pub fn dnserr(&mut self) -> DnserrW<IenSpec> {
        DnserrW::new(self, 29)
    }
    #[doc = "Bit 30 - Low Frequency Timeout Error Interrupt"]
    #[inline(always)]
    pub fn lftimeouterr(&mut self) -> LftimeouterrW<IenSpec> {
        LftimeouterrW::new(self, 30)
    }
    #[doc = "Bit 31 - Core Bias Optimization Error Interrupt"]
    #[inline(always)]
    pub fn corebiasopterr(&mut self) -> CorebiasopterrW<IenSpec> {
        CorebiasopterrW::new(self, 31)
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
