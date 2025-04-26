#[doc = "Register `IF_SET` writer"]
pub type W = crate::W<IfSetSpec>;
#[doc = "Field `EXTIF0` writer - External Pin Flag"]
pub type Extif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF1` writer - External Pin Flag"]
pub type Extif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF2` writer - External Pin Flag"]
pub type Extif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF3` writer - External Pin Flag"]
pub type Extif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF4` writer - External Pin Flag"]
pub type Extif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF5` writer - External Pin Flag"]
pub type Extif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF6` writer - External Pin Flag"]
pub type Extif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF7` writer - External Pin Flag"]
pub type Extif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF8` writer - External Pin Flag"]
pub type Extif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF9` writer - External Pin Flag"]
pub type Extif9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF10` writer - External Pin Flag"]
pub type Extif10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIF11` writer - External Pin Flag"]
pub type Extif11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4WU` writer - EM4 wake up"]
pub type Em4wuW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl W {
    #[doc = "Bit 0 - External Pin Flag"]
    #[inline(always)]
    pub fn extif0(&mut self) -> Extif0W<IfSetSpec> {
        Extif0W::new(self, 0)
    }
    #[doc = "Bit 1 - External Pin Flag"]
    #[inline(always)]
    pub fn extif1(&mut self) -> Extif1W<IfSetSpec> {
        Extif1W::new(self, 1)
    }
    #[doc = "Bit 2 - External Pin Flag"]
    #[inline(always)]
    pub fn extif2(&mut self) -> Extif2W<IfSetSpec> {
        Extif2W::new(self, 2)
    }
    #[doc = "Bit 3 - External Pin Flag"]
    #[inline(always)]
    pub fn extif3(&mut self) -> Extif3W<IfSetSpec> {
        Extif3W::new(self, 3)
    }
    #[doc = "Bit 4 - External Pin Flag"]
    #[inline(always)]
    pub fn extif4(&mut self) -> Extif4W<IfSetSpec> {
        Extif4W::new(self, 4)
    }
    #[doc = "Bit 5 - External Pin Flag"]
    #[inline(always)]
    pub fn extif5(&mut self) -> Extif5W<IfSetSpec> {
        Extif5W::new(self, 5)
    }
    #[doc = "Bit 6 - External Pin Flag"]
    #[inline(always)]
    pub fn extif6(&mut self) -> Extif6W<IfSetSpec> {
        Extif6W::new(self, 6)
    }
    #[doc = "Bit 7 - External Pin Flag"]
    #[inline(always)]
    pub fn extif7(&mut self) -> Extif7W<IfSetSpec> {
        Extif7W::new(self, 7)
    }
    #[doc = "Bit 8 - External Pin Flag"]
    #[inline(always)]
    pub fn extif8(&mut self) -> Extif8W<IfSetSpec> {
        Extif8W::new(self, 8)
    }
    #[doc = "Bit 9 - External Pin Flag"]
    #[inline(always)]
    pub fn extif9(&mut self) -> Extif9W<IfSetSpec> {
        Extif9W::new(self, 9)
    }
    #[doc = "Bit 10 - External Pin Flag"]
    #[inline(always)]
    pub fn extif10(&mut self) -> Extif10W<IfSetSpec> {
        Extif10W::new(self, 10)
    }
    #[doc = "Bit 11 - External Pin Flag"]
    #[inline(always)]
    pub fn extif11(&mut self) -> Extif11W<IfSetSpec> {
        Extif11W::new(self, 11)
    }
    #[doc = "Bits 16:27 - EM4 wake up"]
    #[inline(always)]
    pub fn em4wu(&mut self) -> Em4wuW<IfSetSpec> {
        Em4wuW::new(self, 16)
    }
}
#[doc = "Interrupt Flag Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSetSpec;
impl crate::RegisterSpec for IfSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`if_set::W`](W) writer structure"]
impl crate::Writable for IfSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF_SET to value 0"]
impl crate::Resettable for IfSetSpec {}
