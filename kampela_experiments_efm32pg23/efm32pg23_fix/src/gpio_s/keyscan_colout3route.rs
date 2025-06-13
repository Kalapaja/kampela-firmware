#[doc = "Register `KEYSCAN_COLOUT3ROUTE` reader"]
pub type R = crate::R<KeyscanColout3routeSpec>;
#[doc = "Register `KEYSCAN_COLOUT3ROUTE` writer"]
pub type W = crate::W<KeyscanColout3routeSpec>;
#[doc = "Field `PORT` reader - COLOUT3 port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - COLOUT3 port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - COLOUT3 pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - COLOUT3 pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - COLOUT3 port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - COLOUT3 pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - COLOUT3 port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<KeyscanColout3routeSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - COLOUT3 pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<KeyscanColout3routeSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "COLOUT3 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_colout3route::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_colout3route::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyscanColout3routeSpec;
impl crate::RegisterSpec for KeyscanColout3routeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyscan_colout3route::R`](R) reader structure"]
impl crate::Readable for KeyscanColout3routeSpec {}
#[doc = "`write(|w| ..)` method takes [`keyscan_colout3route::W`](W) writer structure"]
impl crate::Writable for KeyscanColout3routeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYSCAN_COLOUT3ROUTE to value 0"]
impl crate::Resettable for KeyscanColout3routeSpec {}
