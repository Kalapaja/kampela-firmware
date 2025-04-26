#[doc = "Register `WRITECMD` writer"]
pub type W = crate::W<WritecmdSpec>;
#[doc = "Field `ERASEPAGE` writer - Erase Page"]
pub type ErasepageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITEEND` writer - End Write Mode"]
pub type WriteendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASERANGE` writer - Erase range of pages"]
pub type EraserangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASEABORT` writer - Abort erase sequence"]
pub type EraseabortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASEMAIN0` writer - Mass erase region 0"]
pub type Erasemain0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARWDATA` writer - Clear WDATA state"]
pub type ClearwdataW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - Erase Page"]
    #[inline(always)]
    pub fn erasepage(&mut self) -> ErasepageW<WritecmdSpec> {
        ErasepageW::new(self, 1)
    }
    #[doc = "Bit 2 - End Write Mode"]
    #[inline(always)]
    pub fn writeend(&mut self) -> WriteendW<WritecmdSpec> {
        WriteendW::new(self, 2)
    }
    #[doc = "Bit 4 - Erase range of pages"]
    #[inline(always)]
    pub fn eraserange(&mut self) -> EraserangeW<WritecmdSpec> {
        EraserangeW::new(self, 4)
    }
    #[doc = "Bit 5 - Abort erase sequence"]
    #[inline(always)]
    pub fn eraseabort(&mut self) -> EraseabortW<WritecmdSpec> {
        EraseabortW::new(self, 5)
    }
    #[doc = "Bit 8 - Mass erase region 0"]
    #[inline(always)]
    pub fn erasemain0(&mut self) -> Erasemain0W<WritecmdSpec> {
        Erasemain0W::new(self, 8)
    }
    #[doc = "Bit 12 - Clear WDATA state"]
    #[inline(always)]
    pub fn clearwdata(&mut self) -> ClearwdataW<WritecmdSpec> {
        ClearwdataW::new(self, 12)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writecmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WritecmdSpec;
impl crate::RegisterSpec for WritecmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`writecmd::W`](W) writer structure"]
impl crate::Writable for WritecmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRITECMD to value 0"]
impl crate::Resettable for WritecmdSpec {}
