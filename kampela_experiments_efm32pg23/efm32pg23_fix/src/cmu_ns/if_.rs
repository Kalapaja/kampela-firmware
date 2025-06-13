#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `CALRDY` reader - Calibration Ready Interrupt Flag"]
pub type CalrdyR = crate::BitReader;
#[doc = "Field `CALRDY` writer - Calibration Ready Interrupt Flag"]
pub type CalrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOF` reader - Calibration Overflow Interrupt Flag"]
pub type CalofR = crate::BitReader;
#[doc = "Field `CALOF` writer - Calibration Overflow Interrupt Flag"]
pub type CalofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Calibration Ready Interrupt Flag"]
    #[inline(always)]
    pub fn calrdy(&self) -> CalrdyR {
        CalrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Calibration Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn calof(&self) -> CalofR {
        CalofR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration Ready Interrupt Flag"]
    #[inline(always)]
    pub fn calrdy(&mut self) -> CalrdyW<IfSpec> {
        CalrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Calibration Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn calof(&mut self) -> CalofW<IfSpec> {
        CalofW::new(self, 1)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`write(|w| ..)` method takes [`if_::W`](W) writer structure"]
impl crate::Writable for IfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
