#[doc = "Register `I2C0_SCLROUTE` reader"]
pub type R = crate::R<I2c0SclrouteSpec>;
#[doc = "Register `I2C0_SCLROUTE` writer"]
pub type W = crate::W<I2c0SclrouteSpec>;
#[doc = "Field `PORT` reader - SCL port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - SCL port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - SCL pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - SCL pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - SCL port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - SCL pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SCL port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<I2c0SclrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - SCL pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<I2c0SclrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "SCL port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_sclroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_sclroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c0SclrouteSpec;
impl crate::RegisterSpec for I2c0SclrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_sclroute::R`](R) reader structure"]
impl crate::Readable for I2c0SclrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c0_sclroute::W`](W) writer structure"]
impl crate::Writable for I2c0SclrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C0_SCLROUTE to value 0"]
impl crate::Resettable for I2c0SclrouteSpec {}
