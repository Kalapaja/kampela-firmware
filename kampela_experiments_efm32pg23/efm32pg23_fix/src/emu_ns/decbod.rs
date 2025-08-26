#[doc = "Register `DECBOD` reader"]
pub type R = crate::R<DecbodSpec>;
#[doc = "Register `DECBOD` writer"]
pub type W = crate::W<DecbodSpec>;
#[doc = "Field `DECBODEN` reader - DECBOD enable"]
pub type DecbodenR = crate::BitReader;
#[doc = "Field `DECBODEN` writer - DECBOD enable"]
pub type DecbodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECBODMASK` reader - DECBOD Mask"]
pub type DecbodmaskR = crate::BitReader;
#[doc = "Field `DECBODMASK` writer - DECBOD Mask"]
pub type DecbodmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECOVMBODEN` reader - Over Voltage Monitor enable"]
pub type DecovmbodenR = crate::BitReader;
#[doc = "Field `DECOVMBODEN` writer - Over Voltage Monitor enable"]
pub type DecovmbodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECOVMBODMASK` reader - Over Voltage Monitor Mask"]
pub type DecovmbodmaskR = crate::BitReader;
#[doc = "Field `DECOVMBODMASK` writer - Over Voltage Monitor Mask"]
pub type DecovmbodmaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DECBOD enable"]
    #[inline(always)]
    pub fn decboden(&self) -> DecbodenR {
        DecbodenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DECBOD Mask"]
    #[inline(always)]
    pub fn decbodmask(&self) -> DecbodmaskR {
        DecbodmaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Over Voltage Monitor enable"]
    #[inline(always)]
    pub fn decovmboden(&self) -> DecovmbodenR {
        DecovmbodenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Over Voltage Monitor Mask"]
    #[inline(always)]
    pub fn decovmbodmask(&self) -> DecovmbodmaskR {
        DecovmbodmaskR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DECBOD enable"]
    #[inline(always)]
    pub fn decboden(&mut self) -> DecbodenW<DecbodSpec> {
        DecbodenW::new(self, 0)
    }
    #[doc = "Bit 1 - DECBOD Mask"]
    #[inline(always)]
    pub fn decbodmask(&mut self) -> DecbodmaskW<DecbodSpec> {
        DecbodmaskW::new(self, 1)
    }
    #[doc = "Bit 4 - Over Voltage Monitor enable"]
    #[inline(always)]
    pub fn decovmboden(&mut self) -> DecovmbodenW<DecbodSpec> {
        DecovmbodenW::new(self, 4)
    }
    #[doc = "Bit 5 - Over Voltage Monitor Mask"]
    #[inline(always)]
    pub fn decovmbodmask(&mut self) -> DecovmbodmaskW<DecbodSpec> {
        DecovmbodmaskW::new(self, 5)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`decbod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decbod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DecbodSpec;
impl crate::RegisterSpec for DecbodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decbod::R`](R) reader structure"]
impl crate::Readable for DecbodSpec {}
#[doc = "`write(|w| ..)` method takes [`decbod::W`](W) writer structure"]
impl crate::Writable for DecbodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DECBOD to value 0x22"]
impl crate::Resettable for DecbodSpec {
    const RESET_VALUE: u32 = 0x22;
}
