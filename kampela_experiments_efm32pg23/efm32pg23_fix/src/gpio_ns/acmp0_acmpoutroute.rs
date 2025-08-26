#[doc = "Register `ACMP0_ACMPOUTROUTE` reader"]
pub type R = crate::R<Acmp0AcmpoutrouteSpec>;
#[doc = "Register `ACMP0_ACMPOUTROUTE` writer"]
pub type W = crate::W<Acmp0AcmpoutrouteSpec>;
#[doc = "Field `PORT` reader - ACMPOUT port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - ACMPOUT port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - ACMPOUT pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - ACMPOUT pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - ACMPOUT port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - ACMPOUT pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ACMPOUT port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<Acmp0AcmpoutrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - ACMPOUT pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<Acmp0AcmpoutrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "ACMPOUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`acmp0_acmpoutroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmp0_acmpoutroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Acmp0AcmpoutrouteSpec;
impl crate::RegisterSpec for Acmp0AcmpoutrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acmp0_acmpoutroute::R`](R) reader structure"]
impl crate::Readable for Acmp0AcmpoutrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`acmp0_acmpoutroute::W`](W) writer structure"]
impl crate::Writable for Acmp0AcmpoutrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACMP0_ACMPOUTROUTE to value 0"]
impl crate::Resettable for Acmp0AcmpoutrouteSpec {}
