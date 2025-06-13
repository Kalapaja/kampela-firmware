#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `EXTIF0` reader - External Pin Flag"]
pub type Extif0R = crate::BitReader;
#[doc = "Field `EXTIF0` writer - External Pin Flag"]
pub type Extif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF1` reader - External Pin Flag"]
pub type Extif1R = crate::BitReader;
#[doc = "Field `EXTIF1` writer - External Pin Flag"]
pub type Extif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF2` reader - External Pin Flag"]
pub type Extif2R = crate::BitReader;
#[doc = "Field `EXTIF2` writer - External Pin Flag"]
pub type Extif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF3` reader - External Pin Flag"]
pub type Extif3R = crate::BitReader;
#[doc = "Field `EXTIF3` writer - External Pin Flag"]
pub type Extif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF4` reader - External Pin Flag"]
pub type Extif4R = crate::BitReader;
#[doc = "Field `EXTIF4` writer - External Pin Flag"]
pub type Extif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF5` reader - External Pin Flag"]
pub type Extif5R = crate::BitReader;
#[doc = "Field `EXTIF5` writer - External Pin Flag"]
pub type Extif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF6` reader - External Pin Flag"]
pub type Extif6R = crate::BitReader;
#[doc = "Field `EXTIF6` writer - External Pin Flag"]
pub type Extif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF7` reader - External Pin Flag"]
pub type Extif7R = crate::BitReader;
#[doc = "Field `EXTIF7` writer - External Pin Flag"]
pub type Extif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF8` reader - External Pin Flag"]
pub type Extif8R = crate::BitReader;
#[doc = "Field `EXTIF8` writer - External Pin Flag"]
pub type Extif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF9` reader - External Pin Flag"]
pub type Extif9R = crate::BitReader;
#[doc = "Field `EXTIF9` writer - External Pin Flag"]
pub type Extif9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF10` reader - External Pin Flag"]
pub type Extif10R = crate::BitReader;
#[doc = "Field `EXTIF10` writer - External Pin Flag"]
pub type Extif10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF11` reader - External Pin Flag"]
pub type Extif11R = crate::BitReader;
#[doc = "Field `EXTIF11` writer - External Pin Flag"]
pub type Extif11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU` reader - EM4 wake up"]
pub type Em4wuR = crate::FieldReader<u16>;
#[doc = "Field `EM4WU` writer - EM4 wake up"]
pub type Em4wuW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - External Pin Flag"]
    #[inline(always)]
    pub fn extif0(&self) -> Extif0R {
        Extif0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Pin Flag"]
    #[inline(always)]
    pub fn extif1(&self) -> Extif1R {
        Extif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Pin Flag"]
    #[inline(always)]
    pub fn extif2(&self) -> Extif2R {
        Extif2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Pin Flag"]
    #[inline(always)]
    pub fn extif3(&self) -> Extif3R {
        Extif3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Pin Flag"]
    #[inline(always)]
    pub fn extif4(&self) -> Extif4R {
        Extif4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Pin Flag"]
    #[inline(always)]
    pub fn extif5(&self) -> Extif5R {
        Extif5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Pin Flag"]
    #[inline(always)]
    pub fn extif6(&self) -> Extif6R {
        Extif6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Pin Flag"]
    #[inline(always)]
    pub fn extif7(&self) -> Extif7R {
        Extif7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External Pin Flag"]
    #[inline(always)]
    pub fn extif8(&self) -> Extif8R {
        Extif8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Pin Flag"]
    #[inline(always)]
    pub fn extif9(&self) -> Extif9R {
        Extif9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Pin Flag"]
    #[inline(always)]
    pub fn extif10(&self) -> Extif10R {
        Extif10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Pin Flag"]
    #[inline(always)]
    pub fn extif11(&self) -> Extif11R {
        Extif11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:27 - EM4 wake up"]
    #[inline(always)]
    pub fn em4wu(&self) -> Em4wuR {
        Em4wuR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - External Pin Flag"]
    #[inline(always)]
    pub fn extif0(&mut self) -> Extif0W<IfSpec> {
        Extif0W::new(self, 0)
    }
    #[doc = "Bit 1 - External Pin Flag"]
    #[inline(always)]
    pub fn extif1(&mut self) -> Extif1W<IfSpec> {
        Extif1W::new(self, 1)
    }
    #[doc = "Bit 2 - External Pin Flag"]
    #[inline(always)]
    pub fn extif2(&mut self) -> Extif2W<IfSpec> {
        Extif2W::new(self, 2)
    }
    #[doc = "Bit 3 - External Pin Flag"]
    #[inline(always)]
    pub fn extif3(&mut self) -> Extif3W<IfSpec> {
        Extif3W::new(self, 3)
    }
    #[doc = "Bit 4 - External Pin Flag"]
    #[inline(always)]
    pub fn extif4(&mut self) -> Extif4W<IfSpec> {
        Extif4W::new(self, 4)
    }
    #[doc = "Bit 5 - External Pin Flag"]
    #[inline(always)]
    pub fn extif5(&mut self) -> Extif5W<IfSpec> {
        Extif5W::new(self, 5)
    }
    #[doc = "Bit 6 - External Pin Flag"]
    #[inline(always)]
    pub fn extif6(&mut self) -> Extif6W<IfSpec> {
        Extif6W::new(self, 6)
    }
    #[doc = "Bit 7 - External Pin Flag"]
    #[inline(always)]
    pub fn extif7(&mut self) -> Extif7W<IfSpec> {
        Extif7W::new(self, 7)
    }
    #[doc = "Bit 8 - External Pin Flag"]
    #[inline(always)]
    pub fn extif8(&mut self) -> Extif8W<IfSpec> {
        Extif8W::new(self, 8)
    }
    #[doc = "Bit 9 - External Pin Flag"]
    #[inline(always)]
    pub fn extif9(&mut self) -> Extif9W<IfSpec> {
        Extif9W::new(self, 9)
    }
    #[doc = "Bit 10 - External Pin Flag"]
    #[inline(always)]
    pub fn extif10(&mut self) -> Extif10W<IfSpec> {
        Extif10W::new(self, 10)
    }
    #[doc = "Bit 11 - External Pin Flag"]
    #[inline(always)]
    pub fn extif11(&mut self) -> Extif11W<IfSpec> {
        Extif11W::new(self, 11)
    }
    #[doc = "Bits 16:27 - EM4 wake up"]
    #[inline(always)]
    pub fn em4wu(&mut self) -> Em4wuW<IfSpec> {
        Em4wuW::new(self, 16)
    }
}
#[doc = "Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`write(|w| ..)` method takes [`if_::W`](W) writer structure"]
impl crate::Writable for IfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
