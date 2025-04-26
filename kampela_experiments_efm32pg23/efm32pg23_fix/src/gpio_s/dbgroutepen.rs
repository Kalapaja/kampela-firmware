#[doc = "Register `DBGROUTEPEN` reader"]
pub type R = crate::R<DbgroutepenSpec>;
#[doc = "Register `DBGROUTEPEN` writer"]
pub type W = crate::W<DbgroutepenSpec>;
#[doc = "Field `SWCLKTCKPEN` reader - Route Pin Enable"]
pub type SwclktckpenR = crate::BitReader;
#[doc = "Field `SWCLKTCKPEN` writer - Route Pin Enable"]
pub type SwclktckpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDIOTMSPEN` reader - Route Location 0"]
pub type SwdiotmspenR = crate::BitReader;
#[doc = "Field `SWDIOTMSPEN` writer - Route Location 0"]
pub type SwdiotmspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDOPEN` reader - JTAG Test Debug Output Pin Enable"]
pub type TdopenR = crate::BitReader;
#[doc = "Field `TDOPEN` writer - JTAG Test Debug Output Pin Enable"]
pub type TdopenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDIPEN` reader - JTAG Test Debug Input Pin Enable"]
pub type TdipenR = crate::BitReader;
#[doc = "Field `TDIPEN` writer - JTAG Test Debug Input Pin Enable"]
pub type TdipenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Route Pin Enable"]
    #[inline(always)]
    pub fn swclktckpen(&self) -> SwclktckpenR {
        SwclktckpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Route Location 0"]
    #[inline(always)]
    pub fn swdiotmspen(&self) -> SwdiotmspenR {
        SwdiotmspenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    pub fn tdopen(&self) -> TdopenR {
        TdopenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    pub fn tdipen(&self) -> TdipenR {
        TdipenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Route Pin Enable"]
    #[inline(always)]
    pub fn swclktckpen(&mut self) -> SwclktckpenW<DbgroutepenSpec> {
        SwclktckpenW::new(self, 0)
    }
    #[doc = "Bit 1 - Route Location 0"]
    #[inline(always)]
    pub fn swdiotmspen(&mut self) -> SwdiotmspenW<DbgroutepenSpec> {
        SwdiotmspenW::new(self, 1)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    pub fn tdopen(&mut self) -> TdopenW<DbgroutepenSpec> {
        TdopenW::new(self, 2)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    pub fn tdipen(&mut self) -> TdipenW<DbgroutepenSpec> {
        TdipenW::new(self, 3)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgroutepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgroutepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgroutepenSpec;
impl crate::RegisterSpec for DbgroutepenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgroutepen::R`](R) reader structure"]
impl crate::Readable for DbgroutepenSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgroutepen::W`](W) writer structure"]
impl crate::Writable for DbgroutepenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGROUTEPEN to value 0x0f"]
impl crate::Resettable for DbgroutepenSpec {
    const RESET_VALUE: u32 = 0x0f;
}
