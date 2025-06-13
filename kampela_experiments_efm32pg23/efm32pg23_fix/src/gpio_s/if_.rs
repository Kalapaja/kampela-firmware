#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `EXTIF0` reader - External Pin Flag"]
pub type Extif0R = crate::BitReader;
#[doc = "Field `EXTIF1` reader - External Pin Flag"]
pub type Extif1R = crate::BitReader;
#[doc = "Field `EXTIF2` reader - External Pin Flag"]
pub type Extif2R = crate::BitReader;
#[doc = "Field `EXTIF3` reader - External Pin Flag"]
pub type Extif3R = crate::BitReader;
#[doc = "Field `EXTIF4` reader - External Pin Flag"]
pub type Extif4R = crate::BitReader;
#[doc = "Field `EXTIF5` reader - External Pin Flag"]
pub type Extif5R = crate::BitReader;
#[doc = "Field `EXTIF6` reader - External Pin Flag"]
pub type Extif6R = crate::BitReader;
#[doc = "Field `EXTIF7` reader - External Pin Flag"]
pub type Extif7R = crate::BitReader;
#[doc = "Field `EXTIF8` reader - External Pin Flag"]
pub type Extif8R = crate::BitReader;
#[doc = "Field `EXTIF9` reader - External Pin Flag"]
pub type Extif9R = crate::BitReader;
#[doc = "Field `EXTIF10` reader - External Pin Flag"]
pub type Extif10R = crate::BitReader;
#[doc = "Field `EXTIF11` reader - External Pin Flag"]
pub type Extif11R = crate::BitReader;
#[doc = "Field `EM4WU` reader - EM4 wake up"]
pub type Em4wuR = crate::FieldReader<u16>;
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
#[doc = "Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
