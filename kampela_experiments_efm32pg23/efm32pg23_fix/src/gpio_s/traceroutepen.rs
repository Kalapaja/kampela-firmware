#[doc = "Register `TRACEROUTEPEN` reader"]
pub type R = crate::R<TraceroutepenSpec>;
#[doc = "Register `TRACEROUTEPEN` writer"]
pub type W = crate::W<TraceroutepenSpec>;
#[doc = "Field `SWVPEN` reader - Serial Wire Viewer Output Pin Enable"]
pub type SwvpenR = crate::BitReader;
#[doc = "Field `SWVPEN` writer - Serial Wire Viewer Output Pin Enable"]
pub type SwvpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACECLKPEN` reader - Trace Clk Pin Enable"]
pub type TraceclkpenR = crate::BitReader;
#[doc = "Field `TRACECLKPEN` writer - Trace Clk Pin Enable"]
pub type TraceclkpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACEDATA0PEN` reader - Trace Data0 Pin Enable"]
pub type Tracedata0penR = crate::BitReader;
#[doc = "Field `TRACEDATA0PEN` writer - Trace Data0 Pin Enable"]
pub type Tracedata0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACEDATA1PEN` reader - Trace Data1 Pin Enable"]
pub type Tracedata1penR = crate::BitReader;
#[doc = "Field `TRACEDATA1PEN` writer - Trace Data1 Pin Enable"]
pub type Tracedata1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACEDATA2PEN` reader - Trace Data2 Pin Enable"]
pub type Tracedata2penR = crate::BitReader;
#[doc = "Field `TRACEDATA2PEN` writer - Trace Data2 Pin Enable"]
pub type Tracedata2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACEDATA3PEN` reader - Trace Data3 Pin Enable"]
pub type Tracedata3penR = crate::BitReader;
#[doc = "Field `TRACEDATA3PEN` writer - Trace Data3 Pin Enable"]
pub type Tracedata3penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swvpen(&self) -> SwvpenR {
        SwvpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trace Clk Pin Enable"]
    #[inline(always)]
    pub fn traceclkpen(&self) -> TraceclkpenR {
        TraceclkpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trace Data0 Pin Enable"]
    #[inline(always)]
    pub fn tracedata0pen(&self) -> Tracedata0penR {
        Tracedata0penR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trace Data1 Pin Enable"]
    #[inline(always)]
    pub fn tracedata1pen(&self) -> Tracedata1penR {
        Tracedata1penR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Trace Data2 Pin Enable"]
    #[inline(always)]
    pub fn tracedata2pen(&self) -> Tracedata2penR {
        Tracedata2penR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trace Data3 Pin Enable"]
    #[inline(always)]
    pub fn tracedata3pen(&self) -> Tracedata3penR {
        Tracedata3penR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swvpen(&mut self) -> SwvpenW<TraceroutepenSpec> {
        SwvpenW::new(self, 0)
    }
    #[doc = "Bit 1 - Trace Clk Pin Enable"]
    #[inline(always)]
    pub fn traceclkpen(&mut self) -> TraceclkpenW<TraceroutepenSpec> {
        TraceclkpenW::new(self, 1)
    }
    #[doc = "Bit 2 - Trace Data0 Pin Enable"]
    #[inline(always)]
    pub fn tracedata0pen(&mut self) -> Tracedata0penW<TraceroutepenSpec> {
        Tracedata0penW::new(self, 2)
    }
    #[doc = "Bit 3 - Trace Data1 Pin Enable"]
    #[inline(always)]
    pub fn tracedata1pen(&mut self) -> Tracedata1penW<TraceroutepenSpec> {
        Tracedata1penW::new(self, 3)
    }
    #[doc = "Bit 4 - Trace Data2 Pin Enable"]
    #[inline(always)]
    pub fn tracedata2pen(&mut self) -> Tracedata2penW<TraceroutepenSpec> {
        Tracedata2penW::new(self, 4)
    }
    #[doc = "Bit 5 - Trace Data3 Pin Enable"]
    #[inline(always)]
    pub fn tracedata3pen(&mut self) -> Tracedata3penW<TraceroutepenSpec> {
        Tracedata3penW::new(self, 5)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`traceroutepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceroutepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TraceroutepenSpec;
impl crate::RegisterSpec for TraceroutepenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`traceroutepen::R`](R) reader structure"]
impl crate::Readable for TraceroutepenSpec {}
#[doc = "`write(|w| ..)` method takes [`traceroutepen::W`](W) writer structure"]
impl crate::Writable for TraceroutepenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRACEROUTEPEN to value 0"]
impl crate::Resettable for TraceroutepenSpec {}
