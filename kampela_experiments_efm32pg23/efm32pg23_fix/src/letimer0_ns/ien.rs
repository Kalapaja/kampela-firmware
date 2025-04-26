#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `COMP0` reader - Compare Match 0 Interrupt Enable"]
pub type Comp0R = crate::BitReader;
#[doc = "Field `COMP0` writer - Compare Match 0 Interrupt Enable"]
pub type Comp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1` reader - Compare Match 1 Interrupt Enable"]
pub type Comp1R = crate::BitReader;
#[doc = "Field `COMP1` writer - Compare Match 1 Interrupt Enable"]
pub type Comp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` reader - Underflow Interrupt Enable"]
pub type UfR = crate::BitReader;
#[doc = "Field `UF` writer - Underflow Interrupt Enable"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REP0` reader - Repeat Counter 0 Interrupt Enable"]
pub type Rep0R = crate::BitReader;
#[doc = "Field `REP0` writer - Repeat Counter 0 Interrupt Enable"]
pub type Rep0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REP1` reader - Repeat Counter 1 Interrupt Enable"]
pub type Rep1R = crate::BitReader;
#[doc = "Field `REP1` writer - Repeat Counter 1 Interrupt Enable"]
pub type Rep1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Compare Match 0 Interrupt Enable"]
    #[inline(always)]
    pub fn comp0(&self) -> Comp0R {
        Comp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Match 1 Interrupt Enable"]
    #[inline(always)]
    pub fn comp1(&self) -> Comp1R {
        Comp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Repeat Counter 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rep0(&self) -> Rep0R {
        Rep0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Repeat Counter 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rep1(&self) -> Rep1R {
        Rep1R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Match 0 Interrupt Enable"]
    #[inline(always)]
    pub fn comp0(&mut self) -> Comp0W<IenSpec> {
        Comp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Compare Match 1 Interrupt Enable"]
    #[inline(always)]
    pub fn comp1(&mut self) -> Comp1W<IenSpec> {
        Comp1W::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<IenSpec> {
        UfW::new(self, 2)
    }
    #[doc = "Bit 3 - Repeat Counter 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rep0(&mut self) -> Rep0W<IenSpec> {
        Rep0W::new(self, 3)
    }
    #[doc = "Bit 4 - Repeat Counter 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rep1(&mut self) -> Rep1W<IenSpec> {
        Rep1W::new(self, 4)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
