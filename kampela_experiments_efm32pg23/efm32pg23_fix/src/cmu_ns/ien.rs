#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `CALRDY` reader - Calibration Ready Interrupt Enable"]
pub type CalrdyR = crate::BitReader;
#[doc = "Field `CALRDY` writer - Calibration Ready Interrupt Enable"]
pub type CalrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOF` reader - Calibration Overflow Interrupt Enable"]
pub type CalofR = crate::BitReader;
#[doc = "Field `CALOF` writer - Calibration Overflow Interrupt Enable"]
pub type CalofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Calibration Ready Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&self) -> CalrdyR {
        CalrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Calibration Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&self) -> CalofR {
        CalofR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration Ready Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&mut self) -> CalrdyW<IenSpec> {
        CalrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Calibration Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&mut self) -> CalofW<IenSpec> {
        CalofW::new(self, 1)
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
