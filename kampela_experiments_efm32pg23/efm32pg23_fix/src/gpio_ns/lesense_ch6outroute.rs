#[doc = "Register `LESENSE_CH6OUTROUTE` reader"]
pub type R = crate::R<LesenseCh6outrouteSpec>;
#[doc = "Register `LESENSE_CH6OUTROUTE` writer"]
pub type W = crate::W<LesenseCh6outrouteSpec>;
#[doc = "Field `PORT` reader - CH6OUT port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - CH6OUT port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - CH6OUT pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - CH6OUT pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CH6OUT port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - CH6OUT pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CH6OUT port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<LesenseCh6outrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - CH6OUT pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<LesenseCh6outrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "CH6OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch6outroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch6outroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LesenseCh6outrouteSpec;
impl crate::RegisterSpec for LesenseCh6outrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lesense_ch6outroute::R`](R) reader structure"]
impl crate::Readable for LesenseCh6outrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`lesense_ch6outroute::W`](W) writer structure"]
impl crate::Writable for LesenseCh6outrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LESENSE_CH6OUTROUTE to value 0"]
impl crate::Resettable for LesenseCh6outrouteSpec {}
