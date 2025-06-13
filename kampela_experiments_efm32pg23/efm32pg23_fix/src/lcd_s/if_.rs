#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `FC` reader - Frame Counter"]
pub type FcR = crate::BitReader;
#[doc = "Field `FC` writer - Frame Counter"]
pub type FcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISPLAY` reader - Display Update Event"]
pub type DisplayR = crate::BitReader;
#[doc = "Field `DISPLAY` writer - Display Update Event"]
pub type DisplayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCBUSYDONE` reader - Synchronization is Done"]
pub type SyncbusydoneR = crate::BitReader;
#[doc = "Field `SYNCBUSYDONE` writer - Synchronization is Done"]
pub type SyncbusydoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Frame Counter"]
    #[inline(always)]
    pub fn fc(&self) -> FcR {
        FcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Display Update Event"]
    #[inline(always)]
    pub fn display(&self) -> DisplayR {
        DisplayR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization is Done"]
    #[inline(always)]
    pub fn syncbusydone(&self) -> SyncbusydoneR {
        SyncbusydoneR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Counter"]
    #[inline(always)]
    pub fn fc(&mut self) -> FcW<IfSpec> {
        FcW::new(self, 0)
    }
    #[doc = "Bit 1 - Display Update Event"]
    #[inline(always)]
    pub fn display(&mut self) -> DisplayW<IfSpec> {
        DisplayW::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization is Done"]
    #[inline(always)]
    pub fn syncbusydone(&mut self) -> SyncbusydoneW<IfSpec> {
        SyncbusydoneW::new(self, 2)
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
