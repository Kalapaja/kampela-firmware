#[doc = "Register `LESENSE_CH7OUTROUTE` reader"]
pub type R = crate::R<LesenseCh7outrouteSpec>;
#[doc = "Register `LESENSE_CH7OUTROUTE` writer"]
pub type W = crate::W<LesenseCh7outrouteSpec>;
#[doc = "Field `PORT` reader - CH7OUT port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - CH7OUT port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - CH7OUT pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - CH7OUT pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CH7OUT port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - CH7OUT pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CH7OUT port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<LesenseCh7outrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - CH7OUT pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<LesenseCh7outrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "CH7OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch7outroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch7outroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LesenseCh7outrouteSpec;
impl crate::RegisterSpec for LesenseCh7outrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lesense_ch7outroute::R`](R) reader structure"]
impl crate::Readable for LesenseCh7outrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`lesense_ch7outroute::W`](W) writer structure"]
impl crate::Writable for LesenseCh7outrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LESENSE_CH7OUTROUTE to value 0"]
impl crate::Resettable for LesenseCh7outrouteSpec {}
