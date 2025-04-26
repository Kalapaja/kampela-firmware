#[doc = "Register `BACFG` reader"]
pub type R = crate::R<BacfgSpec>;
#[doc = "Register `BACFG` writer"]
pub type W = crate::W<BacfgSpec>;
#[doc = "Field `ASTATETOP` reader - ASTATE top cnt"]
pub type AstatetopR = crate::FieldReader;
#[doc = "Field `ASTATETOP` writer - ASTATE top cnt"]
pub type AstatetopW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Frame Counter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fcpresc {
    #[doc = "0: every frame clock"]
    Div1 = 0,
    #[doc = "1: every 2nd frame clock"]
    Div2 = 1,
    #[doc = "2: every 4th frame clock"]
    Div4 = 2,
    #[doc = "3: every 8th frame clock"]
    Div8 = 3,
}
impl From<Fcpresc> for u8 {
    #[inline(always)]
    fn from(variant: Fcpresc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fcpresc {
    type Ux = u8;
}
impl crate::IsEnum for Fcpresc {}
#[doc = "Field `FCPRESC` reader - Frame Counter Prescaler"]
pub type FcprescR = crate::FieldReader<Fcpresc>;
impl FcprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcpresc {
        match self.bits {
            0 => Fcpresc::Div1,
            1 => Fcpresc::Div2,
            2 => Fcpresc::Div4,
            3 => Fcpresc::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "every frame clock"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Fcpresc::Div1
    }
    #[doc = "every 2nd frame clock"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Fcpresc::Div2
    }
    #[doc = "every 4th frame clock"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Fcpresc::Div4
    }
    #[doc = "every 8th frame clock"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Fcpresc::Div8
    }
}
#[doc = "Field `FCPRESC` writer - Frame Counter Prescaler"]
pub type FcprescW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fcpresc, crate::Safe>;
impl<'a, REG> FcprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "every frame clock"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcpresc::Div1)
    }
    #[doc = "every 2nd frame clock"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Fcpresc::Div2)
    }
    #[doc = "every 4th frame clock"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Fcpresc::Div4)
    }
    #[doc = "every 8th frame clock"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Fcpresc::Div8)
    }
}
#[doc = "Field `FCTOP` reader - Frame Counter Top"]
pub type FctopR = crate::FieldReader;
#[doc = "Field `FCTOP` writer - Frame Counter Top"]
pub type FctopW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:2 - ASTATE top cnt"]
    #[inline(always)]
    pub fn astatetop(&self) -> AstatetopR {
        AstatetopR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:17 - Frame Counter Prescaler"]
    #[inline(always)]
    pub fn fcpresc(&self) -> FcprescR {
        FcprescR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - Frame Counter Top"]
    #[inline(always)]
    pub fn fctop(&self) -> FctopR {
        FctopR::new(((self.bits >> 18) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ASTATE top cnt"]
    #[inline(always)]
    pub fn astatetop(&mut self) -> AstatetopW<BacfgSpec> {
        AstatetopW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Frame Counter Prescaler"]
    #[inline(always)]
    pub fn fcpresc(&mut self) -> FcprescW<BacfgSpec> {
        FcprescW::new(self, 16)
    }
    #[doc = "Bits 18:23 - Frame Counter Top"]
    #[inline(always)]
    pub fn fctop(&mut self) -> FctopW<BacfgSpec> {
        FctopW::new(self, 18)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bacfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bacfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BacfgSpec;
impl crate::RegisterSpec for BacfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bacfg::R`](R) reader structure"]
impl crate::Readable for BacfgSpec {}
#[doc = "`write(|w| ..)` method takes [`bacfg::W`](W) writer structure"]
impl crate::Writable for BacfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BACFG to value 0x07"]
impl crate::Resettable for BacfgSpec {
    const RESET_VALUE: u32 = 0x07;
}
