#[doc = "Register `M33CTRL` reader"]
pub type R = crate::R<M33ctrlSpec>;
#[doc = "Register `M33CTRL` writer"]
pub type W = crate::W<M33ctrlSpec>;
#[doc = "Field `LOCKSVTAIRCR` reader - New BitField"]
pub type LocksvtaircrR = crate::BitReader;
#[doc = "Field `LOCKSVTAIRCR` writer - New BitField"]
pub type LocksvtaircrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKNSVTOR` reader - New BitField"]
pub type LocknsvtorR = crate::BitReader;
#[doc = "Field `LOCKNSVTOR` writer - New BitField"]
pub type LocknsvtorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKSMPU` reader - New BitField"]
pub type LocksmpuR = crate::BitReader;
#[doc = "Field `LOCKSMPU` writer - New BitField"]
pub type LocksmpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKNSMPU` reader - New BitField"]
pub type LocknsmpuR = crate::BitReader;
#[doc = "Field `LOCKNSMPU` writer - New BitField"]
pub type LocknsmpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKSAU` reader - New BitField"]
pub type LocksauR = crate::BitReader;
#[doc = "Field `LOCKSAU` writer - New BitField"]
pub type LocksauW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - New BitField"]
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LocksvtaircrR {
        LocksvtaircrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - New BitField"]
    #[inline(always)]
    pub fn locknsvtor(&self) -> LocknsvtorR {
        LocknsvtorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - New BitField"]
    #[inline(always)]
    pub fn locksmpu(&self) -> LocksmpuR {
        LocksmpuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - New BitField"]
    #[inline(always)]
    pub fn locknsmpu(&self) -> LocknsmpuR {
        LocknsmpuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - New BitField"]
    #[inline(always)]
    pub fn locksau(&self) -> LocksauR {
        LocksauR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - New BitField"]
    #[inline(always)]
    pub fn locksvtaircr(&mut self) -> LocksvtaircrW<M33ctrlSpec> {
        LocksvtaircrW::new(self, 0)
    }
    #[doc = "Bit 1 - New BitField"]
    #[inline(always)]
    pub fn locknsvtor(&mut self) -> LocknsvtorW<M33ctrlSpec> {
        LocknsvtorW::new(self, 1)
    }
    #[doc = "Bit 2 - New BitField"]
    #[inline(always)]
    pub fn locksmpu(&mut self) -> LocksmpuW<M33ctrlSpec> {
        LocksmpuW::new(self, 2)
    }
    #[doc = "Bit 3 - New BitField"]
    #[inline(always)]
    pub fn locknsmpu(&mut self) -> LocknsmpuW<M33ctrlSpec> {
        LocknsmpuW::new(self, 3)
    }
    #[doc = "Bit 4 - New BitField"]
    #[inline(always)]
    pub fn locksau(&mut self) -> LocksauW<M33ctrlSpec> {
        LocksauW::new(self, 4)
    }
}
#[doc = "Holds the M33 control settings\n\nYou can [`read`](crate::Reg::read) this register and get [`m33ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m33ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M33ctrlSpec;
impl crate::RegisterSpec for M33ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m33ctrl::R`](R) reader structure"]
impl crate::Readable for M33ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`m33ctrl::W`](W) writer structure"]
impl crate::Writable for M33ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M33CTRL to value 0"]
impl crate::Resettable for M33ctrlSpec {}
