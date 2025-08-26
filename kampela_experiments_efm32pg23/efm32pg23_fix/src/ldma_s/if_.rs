#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `DONE0` reader - DMA Structure Operation Done"]
pub type Done0R = crate::BitReader;
#[doc = "Field `DONE0` writer - DMA Structure Operation Done"]
pub type Done0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE1` reader - DMA Structure Operation Done"]
pub type Done1R = crate::BitReader;
#[doc = "Field `DONE1` writer - DMA Structure Operation Done"]
pub type Done1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE2` reader - DMA Structure Operation Done"]
pub type Done2R = crate::BitReader;
#[doc = "Field `DONE2` writer - DMA Structure Operation Done"]
pub type Done2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE3` reader - DMA Structure Operation Done"]
pub type Done3R = crate::BitReader;
#[doc = "Field `DONE3` writer - DMA Structure Operation Done"]
pub type Done3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE4` reader - DMA Structure Operation Done"]
pub type Done4R = crate::BitReader;
#[doc = "Field `DONE4` writer - DMA Structure Operation Done"]
pub type Done4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE5` reader - DMA Structure Operation Done"]
pub type Done5R = crate::BitReader;
#[doc = "Field `DONE5` writer - DMA Structure Operation Done"]
pub type Done5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE6` reader - DMA Structure Operation Done"]
pub type Done6R = crate::BitReader;
#[doc = "Field `DONE6` writer - DMA Structure Operation Done"]
pub type Done6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE7` reader - DMA Structure Operation Done"]
pub type Done7R = crate::BitReader;
#[doc = "Field `DONE7` writer - DMA Structure Operation Done"]
pub type Done7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - Error Flag"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Error Flag"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done0(&self) -> Done0R {
        Done0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done1(&self) -> Done1R {
        Done1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done2(&self) -> Done2R {
        Done2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done3(&self) -> Done3R {
        Done3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done4(&self) -> Done4R {
        Done4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done5(&self) -> Done5R {
        Done5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done6(&self) -> Done6R {
        Done6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done7(&self) -> Done7R {
        Done7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - Error Flag"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done0(&mut self) -> Done0W<IfSpec> {
        Done0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done1(&mut self) -> Done1W<IfSpec> {
        Done1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done2(&mut self) -> Done2W<IfSpec> {
        Done2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done3(&mut self) -> Done3W<IfSpec> {
        Done3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done4(&mut self) -> Done4W<IfSpec> {
        Done4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done5(&mut self) -> Done5W<IfSpec> {
        Done5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done6(&mut self) -> Done6W<IfSpec> {
        Done6W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Structure Operation Done"]
    #[inline(always)]
    pub fn done7(&mut self) -> Done7W<IfSpec> {
        Done7W::new(self, 7)
    }
    #[doc = "Bit 31 - Error Flag"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<IfSpec> {
        ErrorW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
