#[doc = "Register `STMASK` reader"]
pub type R = crate::R<StmaskSpec>;
#[doc = "Field `STMASK` reader - Scan Table Mask"]
pub type StmaskR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Scan Table Mask"]
    #[inline(always)]
    pub fn stmask(&self) -> StmaskR {
        StmaskR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Scan Table Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`stmask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StmaskSpec;
impl crate::RegisterSpec for StmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmask::R`](R) reader structure"]
impl crate::Readable for StmaskSpec {}
#[doc = "`reset()` method sets STMASK to value 0"]
impl crate::Resettable for StmaskSpec {}
