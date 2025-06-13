#[doc = "Register `LESENSE_CH4OUTROUTE` reader"]
pub type R = crate::R<LesenseCh4outrouteSpec>;
#[doc = "Register `LESENSE_CH4OUTROUTE` writer"]
pub type W = crate::W<LesenseCh4outrouteSpec>;
#[doc = "Field `PORT` reader - CH4OUT port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - CH4OUT port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - CH4OUT pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - CH4OUT pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CH4OUT port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - CH4OUT pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CH4OUT port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<LesenseCh4outrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - CH4OUT pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<LesenseCh4outrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "CH4OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch4outroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch4outroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LesenseCh4outrouteSpec;
impl crate::RegisterSpec for LesenseCh4outrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lesense_ch4outroute::R`](R) reader structure"]
impl crate::Readable for LesenseCh4outrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`lesense_ch4outroute::W`](W) writer structure"]
impl crate::Writable for LesenseCh4outrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LESENSE_CH4OUTROUTE to value 0"]
impl crate::Resettable for LesenseCh4outrouteSpec {}
