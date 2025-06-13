#[doc = "Register `I2C1_ROUTEEN` reader"]
pub type R = crate::R<I2c1RouteenSpec>;
#[doc = "Register `I2C1_ROUTEEN` writer"]
pub type W = crate::W<I2c1RouteenSpec>;
#[doc = "Field `SCLPEN` reader - SCL pin enable control bit"]
pub type SclpenR = crate::BitReader;
#[doc = "Field `SCLPEN` writer - SCL pin enable control bit"]
pub type SclpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDAPEN` reader - SDA pin enable control bit"]
pub type SdapenR = crate::BitReader;
#[doc = "Field `SDAPEN` writer - SDA pin enable control bit"]
pub type SdapenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCL pin enable control bit"]
    #[inline(always)]
    pub fn sclpen(&self) -> SclpenR {
        SclpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDA pin enable control bit"]
    #[inline(always)]
    pub fn sdapen(&self) -> SdapenR {
        SdapenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCL pin enable control bit"]
    #[inline(always)]
    pub fn sclpen(&mut self) -> SclpenW<I2c1RouteenSpec> {
        SclpenW::new(self, 0)
    }
    #[doc = "Bit 1 - SDA pin enable control bit"]
    #[inline(always)]
    pub fn sdapen(&mut self) -> SdapenW<I2c1RouteenSpec> {
        SdapenW::new(self, 1)
    }
}
#[doc = "I2C1 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_routeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_routeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c1RouteenSpec;
impl crate::RegisterSpec for I2c1RouteenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c1_routeen::R`](R) reader structure"]
impl crate::Readable for I2c1RouteenSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c1_routeen::W`](W) writer structure"]
impl crate::Writable for I2c1RouteenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C1_ROUTEEN to value 0"]
impl crate::Resettable for I2c1RouteenSpec {}
