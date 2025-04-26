#[doc = "Register `GRP0_CMP0VALUE` reader"]
pub type R = crate::R<Grp0Cmp0valueSpec>;
#[doc = "Register `GRP0_CMP0VALUE` writer"]
pub type W = crate::W<Grp0Cmp0valueSpec>;
#[doc = "Field `CMP0VALUE` reader - Compare 0 Value"]
pub type Cmp0valueR = crate::FieldReader<u32>;
#[doc = "Field `CMP0VALUE` writer - Compare 0 Value"]
pub type Cmp0valueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare 0 Value"]
    #[inline(always)]
    pub fn cmp0value(&self) -> Cmp0valueR {
        Cmp0valueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare 0 Value"]
    #[inline(always)]
    pub fn cmp0value(&mut self) -> Cmp0valueW<Grp0Cmp0valueSpec> {
        Cmp0valueW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grp0_cmp0value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp0_cmp0value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Grp0Cmp0valueSpec;
impl crate::RegisterSpec for Grp0Cmp0valueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp0_cmp0value::R`](R) reader structure"]
impl crate::Readable for Grp0Cmp0valueSpec {}
#[doc = "`write(|w| ..)` method takes [`grp0_cmp0value::W`](W) writer structure"]
impl crate::Writable for Grp0Cmp0valueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP0_CMP0VALUE to value 0"]
impl crate::Resettable for Grp0Cmp0valueSpec {}
