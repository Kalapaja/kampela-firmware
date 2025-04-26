#[doc = "Register `EM4CTRL` reader"]
pub type R = crate::R<Em4ctrlSpec>;
#[doc = "Register `EM4CTRL` writer"]
pub type W = crate::W<Em4ctrlSpec>;
#[doc = "Field `EM4ENTRY` reader - EM4 entry request"]
pub type Em4entryR = crate::FieldReader;
#[doc = "Field `EM4ENTRY` writer - EM4 entry request"]
pub type Em4entryW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "EM4 IO retention mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Em4ioretmode {
    #[doc = "0: No Retention: Pads enter reset state when entering EM4"]
    Disable = 0,
    #[doc = "1: Retention through EM4: Pads enter reset state when exiting EM4"]
    Em4exit = 1,
    #[doc = "2: Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention"]
    Swunlatch = 2,
}
impl From<Em4ioretmode> for u8 {
    #[inline(always)]
    fn from(variant: Em4ioretmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Em4ioretmode {
    type Ux = u8;
}
impl crate::IsEnum for Em4ioretmode {}
#[doc = "Field `EM4IORETMODE` reader - EM4 IO retention mode"]
pub type Em4ioretmodeR = crate::FieldReader<Em4ioretmode>;
impl Em4ioretmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Em4ioretmode> {
        match self.bits {
            0 => Some(Em4ioretmode::Disable),
            1 => Some(Em4ioretmode::Em4exit),
            2 => Some(Em4ioretmode::Swunlatch),
            _ => None,
        }
    }
    #[doc = "No Retention: Pads enter reset state when entering EM4"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Em4ioretmode::Disable
    }
    #[doc = "Retention through EM4: Pads enter reset state when exiting EM4"]
    #[inline(always)]
    pub fn is_em4exit(&self) -> bool {
        *self == Em4ioretmode::Em4exit
    }
    #[doc = "Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention"]
    #[inline(always)]
    pub fn is_swunlatch(&self) -> bool {
        *self == Em4ioretmode::Swunlatch
    }
}
#[doc = "Field `EM4IORETMODE` writer - EM4 IO retention mode"]
pub type Em4ioretmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Em4ioretmode>;
impl<'a, REG> Em4ioretmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Retention: Pads enter reset state when entering EM4"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Em4ioretmode::Disable)
    }
    #[doc = "Retention through EM4: Pads enter reset state when exiting EM4"]
    #[inline(always)]
    pub fn em4exit(self) -> &'a mut crate::W<REG> {
        self.variant(Em4ioretmode::Em4exit)
    }
    #[doc = "Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention"]
    #[inline(always)]
    pub fn swunlatch(self) -> &'a mut crate::W<REG> {
        self.variant(Em4ioretmode::Swunlatch)
    }
}
#[doc = "Field `BOD3SENSEEM4WU` reader - Set BOD3SENSE as EM4 wakeup"]
pub type Bod3senseem4wuR = crate::BitReader;
#[doc = "Field `BOD3SENSEEM4WU` writer - Set BOD3SENSE as EM4 wakeup"]
pub type Bod3senseem4wuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - EM4 entry request"]
    #[inline(always)]
    pub fn em4entry(&self) -> Em4entryR {
        Em4entryR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - EM4 IO retention mode"]
    #[inline(always)]
    pub fn em4ioretmode(&self) -> Em4ioretmodeR {
        Em4ioretmodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Set BOD3SENSE as EM4 wakeup"]
    #[inline(always)]
    pub fn bod3senseem4wu(&self) -> Bod3senseem4wuR {
        Bod3senseem4wuR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - EM4 entry request"]
    #[inline(always)]
    pub fn em4entry(&mut self) -> Em4entryW<Em4ctrlSpec> {
        Em4entryW::new(self, 0)
    }
    #[doc = "Bits 4:5 - EM4 IO retention mode"]
    #[inline(always)]
    pub fn em4ioretmode(&mut self) -> Em4ioretmodeW<Em4ctrlSpec> {
        Em4ioretmodeW::new(self, 4)
    }
    #[doc = "Bit 8 - Set BOD3SENSE as EM4 wakeup"]
    #[inline(always)]
    pub fn bod3senseem4wu(&mut self) -> Bod3senseem4wuW<Em4ctrlSpec> {
        Bod3senseem4wuW::new(self, 8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em4ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em4ctrlSpec;
impl crate::RegisterSpec for Em4ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em4ctrl::R`](R) reader structure"]
impl crate::Readable for Em4ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`em4ctrl::W`](W) writer structure"]
impl crate::Writable for Em4ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EM4CTRL to value 0"]
impl crate::Resettable for Em4ctrlSpec {}
