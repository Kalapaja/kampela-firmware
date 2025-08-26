#[doc = "Register `KEYSCAN_COLOUT2ROUTE` reader"]
pub type R = crate::R<KeyscanColout2routeSpec>;
#[doc = "Register `KEYSCAN_COLOUT2ROUTE` writer"]
pub type W = crate::W<KeyscanColout2routeSpec>;
#[doc = "Field `PORT` reader - COLOUT2 port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - COLOUT2 port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - COLOUT2 pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - COLOUT2 pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - COLOUT2 port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - COLOUT2 pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - COLOUT2 port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<KeyscanColout2routeSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - COLOUT2 pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<KeyscanColout2routeSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "COLOUT2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_colout2route::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_colout2route::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyscanColout2routeSpec;
impl crate::RegisterSpec for KeyscanColout2routeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyscan_colout2route::R`](R) reader structure"]
impl crate::Readable for KeyscanColout2routeSpec {}
#[doc = "`write(|w| ..)` method takes [`keyscan_colout2route::W`](W) writer structure"]
impl crate::Writable for KeyscanColout2routeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYSCAN_COLOUT2ROUTE to value 0"]
impl crate::Resettable for KeyscanColout2routeSpec {}
