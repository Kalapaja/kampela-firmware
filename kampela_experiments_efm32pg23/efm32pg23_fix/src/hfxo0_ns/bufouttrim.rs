#[doc = "Register `BUFOUTTRIM` reader"]
pub type R = crate::R<BufouttrimSpec>;
#[doc = "Register `BUFOUTTRIM` writer"]
pub type W = crate::W<BufouttrimSpec>;
#[doc = "Field `VTRTRIMANA` reader - BUFOUT Reference Trim"]
pub type VtrtrimanaR = crate::FieldReader;
#[doc = "Field `VTRTRIMANA` writer - BUFOUT Reference Trim"]
pub type VtrtrimanaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - BUFOUT Reference Trim"]
    #[inline(always)]
    pub fn vtrtrimana(&self) -> VtrtrimanaR {
        VtrtrimanaR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - BUFOUT Reference Trim"]
    #[inline(always)]
    pub fn vtrtrimana(&mut self) -> VtrtrimanaW<BufouttrimSpec> {
        VtrtrimanaW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bufouttrim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bufouttrim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufouttrimSpec;
impl crate::RegisterSpec for BufouttrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bufouttrim::R`](R) reader structure"]
impl crate::Readable for BufouttrimSpec {}
#[doc = "`write(|w| ..)` method takes [`bufouttrim::W`](W) writer structure"]
impl crate::Writable for BufouttrimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUFOUTTRIM to value 0x08"]
impl crate::Resettable for BufouttrimSpec {
    const RESET_VALUE: u32 = 0x08;
}
