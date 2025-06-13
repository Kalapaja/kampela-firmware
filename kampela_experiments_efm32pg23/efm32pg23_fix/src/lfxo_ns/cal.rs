#[doc = "Register `CAL` reader"]
pub type R = crate::R<CalSpec>;
#[doc = "Register `CAL` writer"]
pub type W = crate::W<CalSpec>;
#[doc = "Field `CAPTUNE` reader - Internal Capacitance Tuning"]
pub type CaptuneR = crate::FieldReader;
#[doc = "Field `CAPTUNE` writer - Internal Capacitance Tuning"]
pub type CaptuneW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `GAIN` reader - LFXO Startup Gain"]
pub type GainR = crate::FieldReader;
#[doc = "Field `GAIN` writer - LFXO Startup Gain"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Internal Capacitance Tuning"]
    #[inline(always)]
    pub fn captune(&self) -> CaptuneR {
        CaptuneR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - LFXO Startup Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Internal Capacitance Tuning"]
    #[inline(always)]
    pub fn captune(&mut self) -> CaptuneW<CalSpec> {
        CaptuneW::new(self, 0)
    }
    #[doc = "Bits 8:9 - LFXO Startup Gain"]
    #[inline(always)]
    pub fn gain(&mut self) -> GainW<CalSpec> {
        GainW::new(self, 8)
    }
}
#[doc = "Do not write to this register unless CALBSY in SYNCBUSY register is low.\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalSpec;
impl crate::RegisterSpec for CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CalSpec {}
#[doc = "`write(|w| ..)` method takes [`cal::W`](W) writer structure"]
impl crate::Writable for CalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAL to value 0x0200"]
impl crate::Resettable for CalSpec {
    const RESET_VALUE: u32 = 0x0200;
}
