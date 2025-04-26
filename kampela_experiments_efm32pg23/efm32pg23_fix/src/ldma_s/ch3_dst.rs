#[doc = "Register `CH3_DST` reader"]
pub type R = crate::R<Ch3DstSpec>;
#[doc = "Register `CH3_DST` writer"]
pub type W = crate::W<Ch3DstSpec>;
#[doc = "Field `DSTADDR` reader - Destination Data Address"]
pub type DstaddrR = crate::FieldReader<u32>;
#[doc = "Field `DSTADDR` writer - Destination Data Address"]
pub type DstaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    pub fn dstaddr(&self) -> DstaddrR {
        DstaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    pub fn dstaddr(&mut self) -> DstaddrW<Ch3DstSpec> {
        DstaddrW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3DstSpec;
impl crate::RegisterSpec for Ch3DstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_dst::R`](R) reader structure"]
impl crate::Readable for Ch3DstSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3_dst::W`](W) writer structure"]
impl crate::Writable for Ch3DstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3_DST to value 0"]
impl crate::Resettable for Ch3DstSpec {}
