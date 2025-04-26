#[doc = "Register `CFGSYSTIC` reader"]
pub type R = crate::R<CfgsysticSpec>;
#[doc = "Register `CFGSYSTIC` writer"]
pub type W = crate::W<CfgsysticSpec>;
#[doc = "Field `SYSTICEXTCLKEN` reader - SysTick External Clock Enable"]
pub type SysticextclkenR = crate::BitReader;
#[doc = "Field `SYSTICEXTCLKEN` writer - SysTick External Clock Enable"]
pub type SysticextclkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SysTick External Clock Enable"]
    #[inline(always)]
    pub fn systicextclken(&self) -> SysticextclkenR {
        SysticextclkenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SysTick External Clock Enable"]
    #[inline(always)]
    pub fn systicextclken(&mut self) -> SysticextclkenW<CfgsysticSpec> {
        SysticextclkenW::new(self, 0)
    }
}
#[doc = "Configure the source of the system tick for the M33.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgsystic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgsystic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgsysticSpec;
impl crate::RegisterSpec for CfgsysticSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgsystic::R`](R) reader structure"]
impl crate::Readable for CfgsysticSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgsystic::W`](W) writer structure"]
impl crate::Writable for CfgsysticSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGSYSTIC to value 0"]
impl crate::Resettable for CfgsysticSpec {}
