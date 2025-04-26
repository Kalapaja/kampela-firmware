#[doc = "Register `PRS0_ROUTEEN` reader"]
pub type R = crate::R<Prs0RouteenSpec>;
#[doc = "Register `PRS0_ROUTEEN` writer"]
pub type W = crate::W<Prs0RouteenSpec>;
#[doc = "Field `ASYNCH0PEN` reader - ASYNCH0 pin enable control bit"]
pub type Asynch0penR = crate::BitReader;
#[doc = "Field `ASYNCH0PEN` writer - ASYNCH0 pin enable control bit"]
pub type Asynch0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH1PEN` reader - ASYNCH1 pin enable control bit"]
pub type Asynch1penR = crate::BitReader;
#[doc = "Field `ASYNCH1PEN` writer - ASYNCH1 pin enable control bit"]
pub type Asynch1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH2PEN` reader - ASYNCH2 pin enable control bit"]
pub type Asynch2penR = crate::BitReader;
#[doc = "Field `ASYNCH2PEN` writer - ASYNCH2 pin enable control bit"]
pub type Asynch2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH3PEN` reader - ASYNCH3 pin enable control bit"]
pub type Asynch3penR = crate::BitReader;
#[doc = "Field `ASYNCH3PEN` writer - ASYNCH3 pin enable control bit"]
pub type Asynch3penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH4PEN` reader - ASYNCH4 pin enable control bit"]
pub type Asynch4penR = crate::BitReader;
#[doc = "Field `ASYNCH4PEN` writer - ASYNCH4 pin enable control bit"]
pub type Asynch4penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH5PEN` reader - ASYNCH5 pin enable control bit"]
pub type Asynch5penR = crate::BitReader;
#[doc = "Field `ASYNCH5PEN` writer - ASYNCH5 pin enable control bit"]
pub type Asynch5penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH6PEN` reader - ASYNCH6 pin enable control bit"]
pub type Asynch6penR = crate::BitReader;
#[doc = "Field `ASYNCH6PEN` writer - ASYNCH6 pin enable control bit"]
pub type Asynch6penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH7PEN` reader - ASYNCH7 pin enable control bit"]
pub type Asynch7penR = crate::BitReader;
#[doc = "Field `ASYNCH7PEN` writer - ASYNCH7 pin enable control bit"]
pub type Asynch7penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH8PEN` reader - ASYNCH8 pin enable control bit"]
pub type Asynch8penR = crate::BitReader;
#[doc = "Field `ASYNCH8PEN` writer - ASYNCH8 pin enable control bit"]
pub type Asynch8penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH9PEN` reader - ASYNCH9 pin enable control bit"]
pub type Asynch9penR = crate::BitReader;
#[doc = "Field `ASYNCH9PEN` writer - ASYNCH9 pin enable control bit"]
pub type Asynch9penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH10PEN` reader - ASYNCH10 pin enable control bit"]
pub type Asynch10penR = crate::BitReader;
#[doc = "Field `ASYNCH10PEN` writer - ASYNCH10 pin enable control bit"]
pub type Asynch10penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH11PEN` reader - ASYNCH11 pin enable control bit"]
pub type Asynch11penR = crate::BitReader;
#[doc = "Field `ASYNCH11PEN` writer - ASYNCH11 pin enable control bit"]
pub type Asynch11penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCH0PEN` reader - SYNCH0 pin enable control bit"]
pub type Synch0penR = crate::BitReader;
#[doc = "Field `SYNCH0PEN` writer - SYNCH0 pin enable control bit"]
pub type Synch0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCH1PEN` reader - SYNCH1 pin enable control bit"]
pub type Synch1penR = crate::BitReader;
#[doc = "Field `SYNCH1PEN` writer - SYNCH1 pin enable control bit"]
pub type Synch1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCH2PEN` reader - SYNCH2 pin enable control bit"]
pub type Synch2penR = crate::BitReader;
#[doc = "Field `SYNCH2PEN` writer - SYNCH2 pin enable control bit"]
pub type Synch2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCH3PEN` reader - SYNCH3 pin enable control bit"]
pub type Synch3penR = crate::BitReader;
#[doc = "Field `SYNCH3PEN` writer - SYNCH3 pin enable control bit"]
pub type Synch3penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ASYNCH0 pin enable control bit"]
    #[inline(always)]
    pub fn asynch0pen(&self) -> Asynch0penR {
        Asynch0penR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ASYNCH1 pin enable control bit"]
    #[inline(always)]
    pub fn asynch1pen(&self) -> Asynch1penR {
        Asynch1penR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ASYNCH2 pin enable control bit"]
    #[inline(always)]
    pub fn asynch2pen(&self) -> Asynch2penR {
        Asynch2penR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ASYNCH3 pin enable control bit"]
    #[inline(always)]
    pub fn asynch3pen(&self) -> Asynch3penR {
        Asynch3penR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ASYNCH4 pin enable control bit"]
    #[inline(always)]
    pub fn asynch4pen(&self) -> Asynch4penR {
        Asynch4penR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ASYNCH5 pin enable control bit"]
    #[inline(always)]
    pub fn asynch5pen(&self) -> Asynch5penR {
        Asynch5penR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ASYNCH6 pin enable control bit"]
    #[inline(always)]
    pub fn asynch6pen(&self) -> Asynch6penR {
        Asynch6penR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ASYNCH7 pin enable control bit"]
    #[inline(always)]
    pub fn asynch7pen(&self) -> Asynch7penR {
        Asynch7penR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ASYNCH8 pin enable control bit"]
    #[inline(always)]
    pub fn asynch8pen(&self) -> Asynch8penR {
        Asynch8penR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ASYNCH9 pin enable control bit"]
    #[inline(always)]
    pub fn asynch9pen(&self) -> Asynch9penR {
        Asynch9penR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ASYNCH10 pin enable control bit"]
    #[inline(always)]
    pub fn asynch10pen(&self) -> Asynch10penR {
        Asynch10penR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ASYNCH11 pin enable control bit"]
    #[inline(always)]
    pub fn asynch11pen(&self) -> Asynch11penR {
        Asynch11penR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SYNCH0 pin enable control bit"]
    #[inline(always)]
    pub fn synch0pen(&self) -> Synch0penR {
        Synch0penR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SYNCH1 pin enable control bit"]
    #[inline(always)]
    pub fn synch1pen(&self) -> Synch1penR {
        Synch1penR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SYNCH2 pin enable control bit"]
    #[inline(always)]
    pub fn synch2pen(&self) -> Synch2penR {
        Synch2penR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SYNCH3 pin enable control bit"]
    #[inline(always)]
    pub fn synch3pen(&self) -> Synch3penR {
        Synch3penR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ASYNCH0 pin enable control bit"]
    #[inline(always)]
    pub fn asynch0pen(&mut self) -> Asynch0penW<Prs0RouteenSpec> {
        Asynch0penW::new(self, 0)
    }
    #[doc = "Bit 1 - ASYNCH1 pin enable control bit"]
    #[inline(always)]
    pub fn asynch1pen(&mut self) -> Asynch1penW<Prs0RouteenSpec> {
        Asynch1penW::new(self, 1)
    }
    #[doc = "Bit 2 - ASYNCH2 pin enable control bit"]
    #[inline(always)]
    pub fn asynch2pen(&mut self) -> Asynch2penW<Prs0RouteenSpec> {
        Asynch2penW::new(self, 2)
    }
    #[doc = "Bit 3 - ASYNCH3 pin enable control bit"]
    #[inline(always)]
    pub fn asynch3pen(&mut self) -> Asynch3penW<Prs0RouteenSpec> {
        Asynch3penW::new(self, 3)
    }
    #[doc = "Bit 4 - ASYNCH4 pin enable control bit"]
    #[inline(always)]
    pub fn asynch4pen(&mut self) -> Asynch4penW<Prs0RouteenSpec> {
        Asynch4penW::new(self, 4)
    }
    #[doc = "Bit 5 - ASYNCH5 pin enable control bit"]
    #[inline(always)]
    pub fn asynch5pen(&mut self) -> Asynch5penW<Prs0RouteenSpec> {
        Asynch5penW::new(self, 5)
    }
    #[doc = "Bit 6 - ASYNCH6 pin enable control bit"]
    #[inline(always)]
    pub fn asynch6pen(&mut self) -> Asynch6penW<Prs0RouteenSpec> {
        Asynch6penW::new(self, 6)
    }
    #[doc = "Bit 7 - ASYNCH7 pin enable control bit"]
    #[inline(always)]
    pub fn asynch7pen(&mut self) -> Asynch7penW<Prs0RouteenSpec> {
        Asynch7penW::new(self, 7)
    }
    #[doc = "Bit 8 - ASYNCH8 pin enable control bit"]
    #[inline(always)]
    pub fn asynch8pen(&mut self) -> Asynch8penW<Prs0RouteenSpec> {
        Asynch8penW::new(self, 8)
    }
    #[doc = "Bit 9 - ASYNCH9 pin enable control bit"]
    #[inline(always)]
    pub fn asynch9pen(&mut self) -> Asynch9penW<Prs0RouteenSpec> {
        Asynch9penW::new(self, 9)
    }
    #[doc = "Bit 10 - ASYNCH10 pin enable control bit"]
    #[inline(always)]
    pub fn asynch10pen(&mut self) -> Asynch10penW<Prs0RouteenSpec> {
        Asynch10penW::new(self, 10)
    }
    #[doc = "Bit 11 - ASYNCH11 pin enable control bit"]
    #[inline(always)]
    pub fn asynch11pen(&mut self) -> Asynch11penW<Prs0RouteenSpec> {
        Asynch11penW::new(self, 11)
    }
    #[doc = "Bit 12 - SYNCH0 pin enable control bit"]
    #[inline(always)]
    pub fn synch0pen(&mut self) -> Synch0penW<Prs0RouteenSpec> {
        Synch0penW::new(self, 12)
    }
    #[doc = "Bit 13 - SYNCH1 pin enable control bit"]
    #[inline(always)]
    pub fn synch1pen(&mut self) -> Synch1penW<Prs0RouteenSpec> {
        Synch1penW::new(self, 13)
    }
    #[doc = "Bit 14 - SYNCH2 pin enable control bit"]
    #[inline(always)]
    pub fn synch2pen(&mut self) -> Synch2penW<Prs0RouteenSpec> {
        Synch2penW::new(self, 14)
    }
    #[doc = "Bit 15 - SYNCH3 pin enable control bit"]
    #[inline(always)]
    pub fn synch3pen(&mut self) -> Synch3penW<Prs0RouteenSpec> {
        Synch3penW::new(self, 15)
    }
}
#[doc = "PRS0 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_routeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_routeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prs0RouteenSpec;
impl crate::RegisterSpec for Prs0RouteenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prs0_routeen::R`](R) reader structure"]
impl crate::Readable for Prs0RouteenSpec {}
#[doc = "`write(|w| ..)` method takes [`prs0_routeen::W`](W) writer structure"]
impl crate::Writable for Prs0RouteenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRS0_ROUTEEN to value 0"]
impl crate::Resettable for Prs0RouteenSpec {}
