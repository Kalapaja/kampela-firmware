#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `OF` reader - Overflow Interrupt Flag"]
pub type OfR = crate::BitReader;
#[doc = "Field `OF` writer - Overflow Interrupt Flag"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` reader - Underflow Interrupt Flag"]
pub type UfR = crate::BitReader;
#[doc = "Field `UF` writer - Underflow Interrupt Flag"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCHG` reader - Direction Change Detect Interrupt Flag"]
pub type DirchgR = crate::BitReader;
#[doc = "Field `DIRCHG` writer - Direction Change Detect Interrupt Flag"]
pub type DirchgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0` reader - Capture Compare Channel 0 Interrupt Flag"]
pub type Cc0R = crate::BitReader;
#[doc = "Field `CC0` writer - Capture Compare Channel 0 Interrupt Flag"]
pub type Cc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1` reader - Capture Compare Channel 1 Interrupt Flag"]
pub type Cc1R = crate::BitReader;
#[doc = "Field `CC1` writer - Capture Compare Channel 1 Interrupt Flag"]
pub type Cc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2` reader - Capture Compare Channel 2 Interrupt Flag"]
pub type Cc2R = crate::BitReader;
#[doc = "Field `CC2` writer - Capture Compare Channel 2 Interrupt Flag"]
pub type Cc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFWLFULL0` reader - Input Capture Watermark Level Full"]
pub type Icfwlfull0R = crate::BitReader;
#[doc = "Field `ICFWLFULL0` writer - Input Capture Watermark Level Full"]
pub type Icfwlfull0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFWLFULL1` reader - Input Capture Watermark Level Full"]
pub type Icfwlfull1R = crate::BitReader;
#[doc = "Field `ICFWLFULL1` writer - Input Capture Watermark Level Full"]
pub type Icfwlfull1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFWLFULL2` reader - Input Capture Watermark Level Full"]
pub type Icfwlfull2R = crate::BitReader;
#[doc = "Field `ICFWLFULL2` writer - Input Capture Watermark Level Full"]
pub type Icfwlfull2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFOF0` reader - Input Capture FIFO overflow"]
pub type Icfof0R = crate::BitReader;
#[doc = "Field `ICFOF0` writer - Input Capture FIFO overflow"]
pub type Icfof0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFOF1` reader - Input Capture FIFO overflow"]
pub type Icfof1R = crate::BitReader;
#[doc = "Field `ICFOF1` writer - Input Capture FIFO overflow"]
pub type Icfof1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFOF2` reader - Input Capture FIFO overflow"]
pub type Icfof2R = crate::BitReader;
#[doc = "Field `ICFOF2` writer - Input Capture FIFO overflow"]
pub type Icfof2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFUF0` reader - Input capture FIFO underflow"]
pub type Icfuf0R = crate::BitReader;
#[doc = "Field `ICFUF0` writer - Input capture FIFO underflow"]
pub type Icfuf0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFUF1` reader - Input capture FIFO underflow"]
pub type Icfuf1R = crate::BitReader;
#[doc = "Field `ICFUF1` writer - Input capture FIFO underflow"]
pub type Icfuf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICFUF2` reader - Input capture FIFO underflow"]
pub type Icfuf2R = crate::BitReader;
#[doc = "Field `ICFUF2` writer - Input capture FIFO underflow"]
pub type Icfuf2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dirchg(&self) -> DirchgR {
        DirchgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture Compare Channel 0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&self) -> Cc0R {
        Cc0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture Compare Channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture Compare Channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - Input Capture Watermark Level Full"]
    #[inline(always)]
    pub fn icfwlfull0(&self) -> Icfwlfull0R {
        Icfwlfull0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input Capture Watermark Level Full"]
    #[inline(always)]
    pub fn icfwlfull1(&self) -> Icfwlfull1R {
        Icfwlfull1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input Capture Watermark Level Full"]
    #[inline(always)]
    pub fn icfwlfull2(&self) -> Icfwlfull2R {
        Icfwlfull2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Input Capture FIFO overflow"]
    #[inline(always)]
    pub fn icfof0(&self) -> Icfof0R {
        Icfof0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input Capture FIFO overflow"]
    #[inline(always)]
    pub fn icfof1(&self) -> Icfof1R {
        Icfof1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Input Capture FIFO overflow"]
    #[inline(always)]
    pub fn icfof2(&self) -> Icfof2R {
        Icfof2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Input capture FIFO underflow"]
    #[inline(always)]
    pub fn icfuf0(&self) -> Icfuf0R {
        Icfuf0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Input capture FIFO underflow"]
    #[inline(always)]
    pub fn icfuf1(&self) -> Icfuf1R {
        Icfuf1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Input capture FIFO underflow"]
    #[inline(always)]
    pub fn icfuf2(&self) -> Icfuf2R {
        Icfuf2R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<IfSpec> {
        OfW::new(self, 0)
    }
    #[doc = "Bit 1 - Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<IfSpec> {
        UfW::new(self, 1)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dirchg(&mut self) -> DirchgW<IfSpec> {
        DirchgW::new(self, 2)
    }
    #[doc = "Bit 4 - Capture Compare Channel 0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&mut self) -> Cc0W<IfSpec> {
        Cc0W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture Compare Channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&mut self) -> Cc1W<IfSpec> {
        Cc1W::new(self, 5)
    }
    #[doc = "Bit 6 - Capture Compare Channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&mut self) -> Cc2W<IfSpec> {
        Cc2W::new(self, 6)
    }
    #[doc = "Bit 16 - Input Capture Watermark Level Full"]
    #[inline(always)]
    pub fn icfwlfull0(&mut self) -> Icfwlfull0W<IfSpec> {
        Icfwlfull0W::new(self, 16)
    }
    #[doc = "Bit 17 - Input Capture Watermark Level Full"]
    #[inline(always)]
    pub fn icfwlfull1(&mut self) -> Icfwlfull1W<IfSpec> {
        Icfwlfull1W::new(self, 17)
    }
    #[doc = "Bit 18 - Input Capture Watermark Level Full"]
    #[inline(always)]
    pub fn icfwlfull2(&mut self) -> Icfwlfull2W<IfSpec> {
        Icfwlfull2W::new(self, 18)
    }
    #[doc = "Bit 20 - Input Capture FIFO overflow"]
    #[inline(always)]
    pub fn icfof0(&mut self) -> Icfof0W<IfSpec> {
        Icfof0W::new(self, 20)
    }
    #[doc = "Bit 21 - Input Capture FIFO overflow"]
    #[inline(always)]
    pub fn icfof1(&mut self) -> Icfof1W<IfSpec> {
        Icfof1W::new(self, 21)
    }
    #[doc = "Bit 22 - Input Capture FIFO overflow"]
    #[inline(always)]
    pub fn icfof2(&mut self) -> Icfof2W<IfSpec> {
        Icfof2W::new(self, 22)
    }
    #[doc = "Bit 24 - Input capture FIFO underflow"]
    #[inline(always)]
    pub fn icfuf0(&mut self) -> Icfuf0W<IfSpec> {
        Icfuf0W::new(self, 24)
    }
    #[doc = "Bit 25 - Input capture FIFO underflow"]
    #[inline(always)]
    pub fn icfuf1(&mut self) -> Icfuf1W<IfSpec> {
        Icfuf1W::new(self, 25)
    }
    #[doc = "Bit 26 - Input capture FIFO underflow"]
    #[inline(always)]
    pub fn icfuf2(&mut self) -> Icfuf2W<IfSpec> {
        Icfuf2W::new(self, 26)
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
