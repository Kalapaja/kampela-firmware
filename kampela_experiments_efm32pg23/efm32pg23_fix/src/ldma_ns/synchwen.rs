#[doc = "Register `SYNCHWEN` reader"]
pub type R = crate::R<SynchwenSpec>;
#[doc = "Register `SYNCHWEN` writer"]
pub type W = crate::W<SynchwenSpec>;
#[doc = "Field `SYNCSETEN` reader - Hardware Sync Trigger Set Enable"]
pub type SyncsetenR = crate::FieldReader;
#[doc = "Field `SYNCSETEN` writer - Hardware Sync Trigger Set Enable"]
pub type SyncsetenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SYNCCLREN` reader - Hardware Sync Trigger Clear Enable"]
pub type SyncclrenR = crate::FieldReader;
#[doc = "Field `SYNCCLREN` writer - Hardware Sync Trigger Clear Enable"]
pub type SyncclrenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Hardware Sync Trigger Set Enable"]
    #[inline(always)]
    pub fn syncseten(&self) -> SyncsetenR {
        SyncsetenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Hardware Sync Trigger Clear Enable"]
    #[inline(always)]
    pub fn syncclren(&self) -> SyncclrenR {
        SyncclrenR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Hardware Sync Trigger Set Enable"]
    #[inline(always)]
    pub fn syncseten(&mut self) -> SyncsetenW<SynchwenSpec> {
        SyncsetenW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Hardware Sync Trigger Clear Enable"]
    #[inline(always)]
    pub fn syncclren(&mut self) -> SyncclrenW<SynchwenSpec> {
        SyncclrenW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`synchwen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synchwen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SynchwenSpec;
impl crate::RegisterSpec for SynchwenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synchwen::R`](R) reader structure"]
impl crate::Readable for SynchwenSpec {}
#[doc = "`write(|w| ..)` method takes [`synchwen::W`](W) writer structure"]
impl crate::Writable for SynchwenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNCHWEN to value 0"]
impl crate::Resettable for SynchwenSpec {}
