#[doc = "Register `DISPCTRLX` reader"]
pub type R = crate::R<DispctrlxSpec>;
#[doc = "Register `DISPCTRLX` writer"]
pub type W = crate::W<DispctrlxSpec>;
#[doc = "Field `DISPLAYDIV` reader - Display Divider"]
pub type DisplaydivR = crate::FieldReader<u16>;
#[doc = "Field `DISPLAYDIV` writer - Display Divider"]
pub type DisplaydivW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Display Divider"]
    #[inline(always)]
    pub fn displaydiv(&self) -> DisplaydivR {
        DisplaydivR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Display Divider"]
    #[inline(always)]
    pub fn displaydiv(&mut self) -> DisplaydivW<DispctrlxSpec> {
        DisplaydivW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dispctrlx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dispctrlx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DispctrlxSpec;
impl crate::RegisterSpec for DispctrlxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dispctrlx::R`](R) reader structure"]
impl crate::Readable for DispctrlxSpec {}
#[doc = "`write(|w| ..)` method takes [`dispctrlx::W`](W) writer structure"]
impl crate::Writable for DispctrlxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DISPCTRLX to value 0"]
impl crate::Resettable for DispctrlxSpec {}
