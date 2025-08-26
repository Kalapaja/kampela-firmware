#[doc = "Register `DBGHALT` reader"]
pub type R = crate::R<DbghaltSpec>;
#[doc = "Register `DBGHALT` writer"]
pub type W = crate::W<DbghaltSpec>;
#[doc = "Field `DBGHALT` reader - DMA Debug Halt"]
pub type DbghaltR = crate::FieldReader;
#[doc = "Field `DBGHALT` writer - DMA Debug Halt"]
pub type DbghaltW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DbghaltR {
        DbghaltR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DbghaltW<DbghaltSpec> {
        DbghaltW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbghalt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbghalt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbghaltSpec;
impl crate::RegisterSpec for DbghaltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbghalt::R`](R) reader structure"]
impl crate::Readable for DbghaltSpec {}
#[doc = "`write(|w| ..)` method takes [`dbghalt::W`](W) writer structure"]
impl crate::Writable for DbghaltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGHALT to value 0"]
impl crate::Resettable for DbghaltSpec {}
