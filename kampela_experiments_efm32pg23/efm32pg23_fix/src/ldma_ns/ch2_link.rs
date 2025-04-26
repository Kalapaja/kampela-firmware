#[doc = "Register `CH2_LINK` reader"]
pub type R = crate::R<Ch2LinkSpec>;
#[doc = "Register `CH2_LINK` writer"]
pub type W = crate::W<Ch2LinkSpec>;
#[doc = "Link Structure Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linkmode {
    #[doc = "0: The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
    Absolute = 0,
    #[doc = "1: The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
    Relative = 1,
}
impl From<Linkmode> for bool {
    #[inline(always)]
    fn from(variant: Linkmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINKMODE` reader - Link Structure Addressing Mode"]
pub type LinkmodeR = crate::BitReader<Linkmode>;
impl LinkmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Linkmode {
        match self.bits {
            false => Linkmode::Absolute,
            true => Linkmode::Relative,
        }
    }
    #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
    #[inline(always)]
    pub fn is_absolute(&self) -> bool {
        *self == Linkmode::Absolute
    }
    #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
    #[inline(always)]
    pub fn is_relative(&self) -> bool {
        *self == Linkmode::Relative
    }
}
#[doc = "Field `LINK` reader - Link Next Structure"]
pub type LinkR = crate::BitReader;
#[doc = "Field `LINK` writer - Link Next Structure"]
pub type LinkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINKADDR` reader - Link Structure Address"]
pub type LinkaddrR = crate::FieldReader<u32>;
#[doc = "Field `LINKADDR` writer - Link Structure Address"]
pub type LinkaddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - Link Structure Addressing Mode"]
    #[inline(always)]
    pub fn linkmode(&self) -> LinkmodeR {
        LinkmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Link Next Structure"]
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - Link Structure Address"]
    #[inline(always)]
    pub fn linkaddr(&self) -> LinkaddrR {
        LinkaddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 1 - Link Next Structure"]
    #[inline(always)]
    pub fn link(&mut self) -> LinkW<Ch2LinkSpec> {
        LinkW::new(self, 1)
    }
    #[doc = "Bits 2:31 - Link Structure Address"]
    #[inline(always)]
    pub fn linkaddr(&mut self) -> LinkaddrW<Ch2LinkSpec> {
        LinkaddrW::new(self, 2)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_link::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_link::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2LinkSpec;
impl crate::RegisterSpec for Ch2LinkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_link::R`](R) reader structure"]
impl crate::Readable for Ch2LinkSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2_link::W`](W) writer structure"]
impl crate::Writable for Ch2LinkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH2_LINK to value 0"]
impl crate::Resettable for Ch2LinkSpec {}
