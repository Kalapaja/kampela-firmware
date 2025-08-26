#[doc = "Register `ADDRB` reader"]
pub type R = crate::R<AddrbSpec>;
#[doc = "Register `ADDRB` writer"]
pub type W = crate::W<AddrbSpec>;
#[doc = "Field `ADDRB` reader - Page Erase or Write Address Buffer"]
pub type AddrbR = crate::FieldReader<u32>;
#[doc = "Field `ADDRB` writer - Page Erase or Write Address Buffer"]
pub type AddrbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Page Erase or Write Address Buffer"]
    #[inline(always)]
    pub fn addrb(&self) -> AddrbR {
        AddrbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Page Erase or Write Address Buffer"]
    #[inline(always)]
    pub fn addrb(&mut self) -> AddrbW<AddrbSpec> {
        AddrbW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`addrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrbSpec;
impl crate::RegisterSpec for AddrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addrb::R`](R) reader structure"]
impl crate::Readable for AddrbSpec {}
#[doc = "`write(|w| ..)` method takes [`addrb::W`](W) writer structure"]
impl crate::Writable for AddrbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDRB to value 0"]
impl crate::Resettable for AddrbSpec {}
