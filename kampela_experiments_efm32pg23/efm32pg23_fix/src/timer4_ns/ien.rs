#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `OF` reader - Overflow Interrupt Enable"]
pub type OfR = crate::BitReader;
#[doc = "Field `OF` writer - Overflow Interrupt Enable"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` reader - Underflow Interrupt Enable"]
pub type UfR = crate::BitReader;
#[doc = "Field `UF` writer - Underflow Interrupt Enable"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCHG` reader - Direction Change Detect Interrupt Enable"]
pub type DirchgR = crate::BitReader;
#[doc = "Field `DIRCHG` writer - Direction Change Detect Interrupt Enable"]
pub type DirchgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0` reader - CC0 Interrupt Enable"]
pub type Cc0R = crate::BitReader;
#[doc = "Field `CC0` writer - CC0 Interrupt Enable"]
pub type Cc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1` reader - CC1 Interrupt Enable"]
pub type Cc1R = crate::BitReader;
#[doc = "Field `CC1` writer - CC1 Interrupt Enable"]
pub type Cc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2` reader - CC2 Interrupt Enable"]
pub type Cc2R = crate::BitReader;
#[doc = "Field `CC2` writer - CC2 Interrupt Enable"]
pub type Cc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFWLFULL0` reader - ICFWLFULL0 Interrupt Enable"]
pub type Icfwlfull0R = crate::BitReader;
#[doc = "Field `ICFWLFULL0` writer - ICFWLFULL0 Interrupt Enable"]
pub type Icfwlfull0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFWLFULL1` reader - ICFWLFULL1 Interrupt Enable"]
pub type Icfwlfull1R = crate::BitReader;
#[doc = "Field `ICFWLFULL1` writer - ICFWLFULL1 Interrupt Enable"]
pub type Icfwlfull1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFWLFULL2` reader - ICFWLFULL2 Interrupt Enable"]
pub type Icfwlfull2R = crate::BitReader;
#[doc = "Field `ICFWLFULL2` writer - ICFWLFULL2 Interrupt Enable"]
pub type Icfwlfull2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFOF0` reader - ICFOF0 Interrupt Enable"]
pub type Icfof0R = crate::BitReader;
#[doc = "Field `ICFOF0` writer - ICFOF0 Interrupt Enable"]
pub type Icfof0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFOF1` reader - ICFOF1 Interrupt Enable"]
pub type Icfof1R = crate::BitReader;
#[doc = "Field `ICFOF1` writer - ICFOF1 Interrupt Enable"]
pub type Icfof1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFOF2` reader - ICFOF2 Interrupt Enable"]
pub type Icfof2R = crate::BitReader;
#[doc = "Field `ICFOF2` writer - ICFOF2 Interrupt Enable"]
pub type Icfof2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFUF0` reader - ICFUF0 Interrupt Enable"]
pub type Icfuf0R = crate::BitReader;
#[doc = "Field `ICFUF0` writer - ICFUF0 Interrupt Enable"]
pub type Icfuf0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFUF1` reader - ICFUF1 Interrupt Enable"]
pub type Icfuf1R = crate::BitReader;
#[doc = "Field `ICFUF1` writer - ICFUF1 Interrupt Enable"]
pub type Icfuf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFUF2` reader - ICFUF2 Interrupt Enable"]
pub type Icfuf2R = crate::BitReader;
#[doc = "Field `ICFUF2` writer - ICFUF2 Interrupt Enable"]
pub type Icfuf2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dirchg(&self) -> DirchgR {
        DirchgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&self) -> Cc0R {
        Cc0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - ICFWLFULL0 Interrupt Enable"]
    #[inline(always)]
    pub fn icfwlfull0(&self) -> Icfwlfull0R {
        Icfwlfull0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ICFWLFULL1 Interrupt Enable"]
    #[inline(always)]
    pub fn icfwlfull1(&self) -> Icfwlfull1R {
        Icfwlfull1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ICFWLFULL2 Interrupt Enable"]
    #[inline(always)]
    pub fn icfwlfull2(&self) -> Icfwlfull2R {
        Icfwlfull2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ICFOF0 Interrupt Enable"]
    #[inline(always)]
    pub fn icfof0(&self) -> Icfof0R {
        Icfof0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ICFOF1 Interrupt Enable"]
    #[inline(always)]
    pub fn icfof1(&self) -> Icfof1R {
        Icfof1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ICFOF2 Interrupt Enable"]
    #[inline(always)]
    pub fn icfof2(&self) -> Icfof2R {
        Icfof2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - ICFUF0 Interrupt Enable"]
    #[inline(always)]
    pub fn icfuf0(&self) -> Icfuf0R {
        Icfuf0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ICFUF1 Interrupt Enable"]
    #[inline(always)]
    pub fn icfuf1(&self) -> Icfuf1R {
        Icfuf1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ICFUF2 Interrupt Enable"]
    #[inline(always)]
    pub fn icfuf2(&self) -> Icfuf2R {
        Icfuf2R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<IenSpec> {
        OfW::new(self, 0)
    }
    #[doc = "Bit 1 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<IenSpec> {
        UfW::new(self, 1)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dirchg(&mut self) -> DirchgW<IenSpec> {
        DirchgW::new(self, 2)
    }
    #[doc = "Bit 4 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&mut self) -> Cc0W<IenSpec> {
        Cc0W::new(self, 4)
    }
    #[doc = "Bit 5 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&mut self) -> Cc1W<IenSpec> {
        Cc1W::new(self, 5)
    }
    #[doc = "Bit 6 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&mut self) -> Cc2W<IenSpec> {
        Cc2W::new(self, 6)
    }
    #[doc = "Bit 16 - ICFWLFULL0 Interrupt Enable"]
    #[inline(always)]
    pub fn icfwlfull0(&mut self) -> Icfwlfull0W<IenSpec> {
        Icfwlfull0W::new(self, 16)
    }
    #[doc = "Bit 17 - ICFWLFULL1 Interrupt Enable"]
    #[inline(always)]
    pub fn icfwlfull1(&mut self) -> Icfwlfull1W<IenSpec> {
        Icfwlfull1W::new(self, 17)
    }
    #[doc = "Bit 18 - ICFWLFULL2 Interrupt Enable"]
    #[inline(always)]
    pub fn icfwlfull2(&mut self) -> Icfwlfull2W<IenSpec> {
        Icfwlfull2W::new(self, 18)
    }
    #[doc = "Bit 20 - ICFOF0 Interrupt Enable"]
    #[inline(always)]
    pub fn icfof0(&mut self) -> Icfof0W<IenSpec> {
        Icfof0W::new(self, 20)
    }
    #[doc = "Bit 21 - ICFOF1 Interrupt Enable"]
    #[inline(always)]
    pub fn icfof1(&mut self) -> Icfof1W<IenSpec> {
        Icfof1W::new(self, 21)
    }
    #[doc = "Bit 22 - ICFOF2 Interrupt Enable"]
    #[inline(always)]
    pub fn icfof2(&mut self) -> Icfof2W<IenSpec> {
        Icfof2W::new(self, 22)
    }
    #[doc = "Bit 24 - ICFUF0 Interrupt Enable"]
    #[inline(always)]
    pub fn icfuf0(&mut self) -> Icfuf0W<IenSpec> {
        Icfuf0W::new(self, 24)
    }
    #[doc = "Bit 25 - ICFUF1 Interrupt Enable"]
    #[inline(always)]
    pub fn icfuf1(&mut self) -> Icfuf1W<IenSpec> {
        Icfuf1W::new(self, 25)
    }
    #[doc = "Bit 26 - ICFUF2 Interrupt Enable"]
    #[inline(always)]
    pub fn icfuf2(&mut self) -> Icfuf2W<IenSpec> {
        Icfuf2W::new(self, 26)
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
