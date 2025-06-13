#[doc = "Register `TEMPLIMITS` reader"]
pub type R = crate::R<TemplimitsSpec>;
#[doc = "Register `TEMPLIMITS` writer"]
pub type W = crate::W<TemplimitsSpec>;
#[doc = "Field `TEMPLOW` reader - Temp Low limit"]
pub type TemplowR = crate::FieldReader<u16>;
#[doc = "Field `TEMPLOW` writer - Temp Low limit"]
pub type TemplowW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TEMPHIGH` reader - Temp High limit"]
pub type TemphighR = crate::FieldReader<u16>;
#[doc = "Field `TEMPHIGH` writer - Temp High limit"]
pub type TemphighW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Temp Low limit"]
    #[inline(always)]
    pub fn templow(&self) -> TemplowR {
        TemplowR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Temp High limit"]
    #[inline(always)]
    pub fn temphigh(&self) -> TemphighR {
        TemphighR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Temp Low limit"]
    #[inline(always)]
    pub fn templow(&mut self) -> TemplowW<TemplimitsSpec> {
        TemplowW::new(self, 0)
    }
    #[doc = "Bits 16:24 - Temp High limit"]
    #[inline(always)]
    pub fn temphigh(&mut self) -> TemphighW<TemplimitsSpec> {
        TemphighW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`templimits::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`templimits::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TemplimitsSpec;
impl crate::RegisterSpec for TemplimitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`templimits::R`](R) reader structure"]
impl crate::Readable for TemplimitsSpec {}
#[doc = "`write(|w| ..)` method takes [`templimits::W`](W) writer structure"]
impl crate::Writable for TemplimitsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEMPLIMITS to value 0x01ff_0000"]
impl crate::Resettable for TemplimitsSpec {
    const RESET_VALUE: u32 = 0x01ff_0000;
}
