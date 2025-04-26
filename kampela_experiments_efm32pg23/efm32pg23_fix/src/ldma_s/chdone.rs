#[doc = "Register `CHDONE` reader"]
pub type R = crate::R<ChdoneSpec>;
#[doc = "Register `CHDONE` writer"]
pub type W = crate::W<ChdoneSpec>;
#[doc = "Field `CHDONE0` reader - DMA Channel Link done intr flag"]
pub type Chdone0R = crate::BitReader;
#[doc = "Field `CHDONE0` writer - DMA Channel Link done intr flag"]
pub type Chdone0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE1` reader - DMA Channel Link done intr flag"]
pub type Chdone1R = crate::BitReader;
#[doc = "Field `CHDONE1` writer - DMA Channel Link done intr flag"]
pub type Chdone1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE2` reader - DMA Channel Link done intr flag"]
pub type Chdone2R = crate::BitReader;
#[doc = "Field `CHDONE2` writer - DMA Channel Link done intr flag"]
pub type Chdone2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE3` reader - DMA Channel Link done intr flag"]
pub type Chdone3R = crate::BitReader;
#[doc = "Field `CHDONE3` writer - DMA Channel Link done intr flag"]
pub type Chdone3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE4` reader - DMA Channel Link done intr flag"]
pub type Chdone4R = crate::BitReader;
#[doc = "Field `CHDONE4` writer - DMA Channel Link done intr flag"]
pub type Chdone4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE5` reader - DMA Channel Link done intr flag"]
pub type Chdone5R = crate::BitReader;
#[doc = "Field `CHDONE5` writer - DMA Channel Link done intr flag"]
pub type Chdone5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE6` reader - DMA Channel Link done intr flag"]
pub type Chdone6R = crate::BitReader;
#[doc = "Field `CHDONE6` writer - DMA Channel Link done intr flag"]
pub type Chdone6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDONE7` reader - DMA Channel Link done intr flag"]
pub type Chdone7R = crate::BitReader;
#[doc = "Field `CHDONE7` writer - DMA Channel Link done intr flag"]
pub type Chdone7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone0(&self) -> Chdone0R {
        Chdone0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone1(&self) -> Chdone1R {
        Chdone1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone2(&self) -> Chdone2R {
        Chdone2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone3(&self) -> Chdone3R {
        Chdone3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone4(&self) -> Chdone4R {
        Chdone4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone5(&self) -> Chdone5R {
        Chdone5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone6(&self) -> Chdone6R {
        Chdone6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone7(&self) -> Chdone7R {
        Chdone7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone0(&mut self) -> Chdone0W<ChdoneSpec> {
        Chdone0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone1(&mut self) -> Chdone1W<ChdoneSpec> {
        Chdone1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone2(&mut self) -> Chdone2W<ChdoneSpec> {
        Chdone2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone3(&mut self) -> Chdone3W<ChdoneSpec> {
        Chdone3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone4(&mut self) -> Chdone4W<ChdoneSpec> {
        Chdone4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone5(&mut self) -> Chdone5W<ChdoneSpec> {
        Chdone5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone6(&mut self) -> Chdone6W<ChdoneSpec> {
        Chdone6W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Channel Link done intr flag"]
    #[inline(always)]
    pub fn chdone7(&mut self) -> Chdone7W<ChdoneSpec> {
        Chdone7W::new(self, 7)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`chdone::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chdone::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChdoneSpec;
impl crate::RegisterSpec for ChdoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chdone::R`](R) reader structure"]
impl crate::Readable for ChdoneSpec {}
#[doc = "`write(|w| ..)` method takes [`chdone::W`](W) writer structure"]
impl crate::Writable for ChdoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHDONE to value 0"]
impl crate::Resettable for ChdoneSpec {}
