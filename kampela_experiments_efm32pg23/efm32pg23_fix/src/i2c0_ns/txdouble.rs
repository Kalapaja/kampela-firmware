#[doc = "Register `TXDOUBLE` writer"]
pub type W = crate::W<TxdoubleSpec>;
#[doc = "Field `TXDATA0` writer - TX Data"]
pub type Txdata0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXDATA1` writer - TX Data"]
pub type Txdata1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    pub fn txdata0(&mut self) -> Txdata0W<TxdoubleSpec> {
        Txdata0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - TX Data"]
    #[inline(always)]
    pub fn txdata1(&mut self) -> Txdata1W<TxdoubleSpec> {
        Txdata1W::new(self, 8)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdouble::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdoubleSpec;
impl crate::RegisterSpec for TxdoubleSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txdouble::W`](W) writer structure"]
impl crate::Writable for TxdoubleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDOUBLE to value 0"]
impl crate::Resettable for TxdoubleSpec {}
