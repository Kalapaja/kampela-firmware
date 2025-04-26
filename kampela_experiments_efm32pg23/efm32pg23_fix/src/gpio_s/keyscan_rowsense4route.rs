#[doc = "Register `KEYSCAN_ROWSENSE4ROUTE` reader"]
pub type R = crate::R<KeyscanRowsense4routeSpec>;
#[doc = "Register `KEYSCAN_ROWSENSE4ROUTE` writer"]
pub type W = crate::W<KeyscanRowsense4routeSpec>;
#[doc = "Field `PORT` reader - ROWSENSE4 port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - ROWSENSE4 port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - ROWSENSE4 pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - ROWSENSE4 pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - ROWSENSE4 port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - ROWSENSE4 pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ROWSENSE4 port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<KeyscanRowsense4routeSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - ROWSENSE4 pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<KeyscanRowsense4routeSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "ROWSENSE4 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_rowsense4route::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_rowsense4route::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyscanRowsense4routeSpec;
impl crate::RegisterSpec for KeyscanRowsense4routeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyscan_rowsense4route::R`](R) reader structure"]
impl crate::Readable for KeyscanRowsense4routeSpec {}
#[doc = "`write(|w| ..)` method takes [`keyscan_rowsense4route::W`](W) writer structure"]
impl crate::Writable for KeyscanRowsense4routeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYSCAN_ROWSENSE4ROUTE to value 0"]
impl crate::Resettable for KeyscanRowsense4routeSpec {}
