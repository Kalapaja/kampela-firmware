#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `LOCK` reader - Lock Interrupt Flag"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock Interrupt Flag"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKFAILLOW` reader - Lock Failure Low Interrupt Flag"]
pub type LockfaillowR = crate::BitReader;
#[doc = "Field `LOCKFAILLOW` writer - Lock Failure Low Interrupt Flag"]
pub type LockfaillowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKFAILHIGH` reader - Lock Failure High Interrupt Flag"]
pub type LockfailhighR = crate::BitReader;
#[doc = "Field `LOCKFAILHIGH` writer - Lock Failure High Interrupt Flag"]
pub type LockfailhighW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock Interrupt Flag"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Failure Low Interrupt Flag"]
    #[inline(always)]
    pub fn lockfaillow(&self) -> LockfaillowR {
        LockfaillowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock Failure High Interrupt Flag"]
    #[inline(always)]
    pub fn lockfailhigh(&self) -> LockfailhighR {
        LockfailhighR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock Interrupt Flag"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<IfSpec> {
        LockW::new(self, 0)
    }
    #[doc = "Bit 1 - Lock Failure Low Interrupt Flag"]
    #[inline(always)]
    pub fn lockfaillow(&mut self) -> LockfaillowW<IfSpec> {
        LockfaillowW::new(self, 1)
    }
    #[doc = "Bit 2 - Lock Failure High Interrupt Flag"]
    #[inline(always)]
    pub fn lockfailhigh(&mut self) -> LockfailhighW<IfSpec> {
        LockfailhighW::new(self, 2)
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
