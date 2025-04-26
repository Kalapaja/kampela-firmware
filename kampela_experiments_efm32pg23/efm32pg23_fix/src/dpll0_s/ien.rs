#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `LOCK` reader - LOCK interrupt Enable"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - LOCK interrupt Enable"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKFAILLOW` reader - LOCKFAILLOW Interrupe Enable"]
pub type LockfaillowR = crate::BitReader;
#[doc = "Field `LOCKFAILLOW` writer - LOCKFAILLOW Interrupe Enable"]
pub type LockfaillowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKFAILHIGH` reader - LOCKFAILHIGH Interrupt Enable"]
pub type LockfailhighR = crate::BitReader;
#[doc = "Field `LOCKFAILHIGH` writer - LOCKFAILHIGH Interrupt Enable"]
pub type LockfailhighW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LOCK interrupt Enable"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LOCKFAILLOW Interrupe Enable"]
    #[inline(always)]
    pub fn lockfaillow(&self) -> LockfaillowR {
        LockfaillowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LOCKFAILHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn lockfailhigh(&self) -> LockfailhighR {
        LockfailhighR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCK interrupt Enable"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<IenSpec> {
        LockW::new(self, 0)
    }
    #[doc = "Bit 1 - LOCKFAILLOW Interrupe Enable"]
    #[inline(always)]
    pub fn lockfaillow(&mut self) -> LockfaillowW<IenSpec> {
        LockfaillowW::new(self, 1)
    }
    #[doc = "Bit 2 - LOCKFAILHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn lockfailhigh(&mut self) -> LockfailhighW<IenSpec> {
        LockfailhighW::new(self, 2)
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
