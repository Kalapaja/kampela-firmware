#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ECCEN` reader - Enable ECC functionality"]
pub type EccenR = crate::BitReader;
#[doc = "Field `ECCEN` writer - Enable ECC functionality"]
pub type EccenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCWEN` reader - Enable ECC syndrome writes"]
pub type EccwenR = crate::BitReader;
#[doc = "Field `ECCWEN` writer - Enable ECC syndrome writes"]
pub type EccwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCERRFAULTEN` reader - ECC Error bus fault enable"]
pub type EccerrfaultenR = crate::BitReader;
#[doc = "Field `ECCERRFAULTEN` writer - ECC Error bus fault enable"]
pub type EccerrfaultenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "AHB port arbitration priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ahbportpriority {
    #[doc = "0: No AHB port have raised priority."]
    None = 0,
    #[doc = "1: AHB port 0 has raised priority."]
    Port0 = 1,
    #[doc = "2: AHB port 1 has raised priority."]
    Port1 = 2,
}
impl From<Ahbportpriority> for u8 {
    #[inline(always)]
    fn from(variant: Ahbportpriority) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ahbportpriority {
    type Ux = u8;
}
impl crate::IsEnum for Ahbportpriority {}
#[doc = "Field `AHBPORTPRIORITY` reader - AHB port arbitration priority"]
pub type AhbportpriorityR = crate::FieldReader<Ahbportpriority>;
impl AhbportpriorityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ahbportpriority> {
        match self.bits {
            0 => Some(Ahbportpriority::None),
            1 => Some(Ahbportpriority::Port0),
            2 => Some(Ahbportpriority::Port1),
            _ => None,
        }
    }
    #[doc = "No AHB port have raised priority."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ahbportpriority::None
    }
    #[doc = "AHB port 0 has raised priority."]
    #[inline(always)]
    pub fn is_port0(&self) -> bool {
        *self == Ahbportpriority::Port0
    }
    #[doc = "AHB port 1 has raised priority."]
    #[inline(always)]
    pub fn is_port1(&self) -> bool {
        *self == Ahbportpriority::Port1
    }
}
#[doc = "Field `AHBPORTPRIORITY` writer - AHB port arbitration priority"]
pub type AhbportpriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ahbportpriority>;
impl<'a, REG> AhbportpriorityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No AHB port have raised priority."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbportpriority::None)
    }
    #[doc = "AHB port 0 has raised priority."]
    #[inline(always)]
    pub fn port0(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbportpriority::Port0)
    }
    #[doc = "AHB port 1 has raised priority."]
    #[inline(always)]
    pub fn port1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbportpriority::Port1)
    }
}
#[doc = "Field `ADDRFAULTEN` reader - Address fault bus fault enable"]
pub type AddrfaultenR = crate::BitReader;
#[doc = "Field `ADDRFAULTEN` writer - Address fault bus fault enable"]
pub type AddrfaultenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable ECC functionality"]
    #[inline(always)]
    pub fn eccen(&self) -> EccenR {
        EccenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable ECC syndrome writes"]
    #[inline(always)]
    pub fn eccwen(&self) -> EccwenR {
        EccwenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECC Error bus fault enable"]
    #[inline(always)]
    pub fn eccerrfaulten(&self) -> EccerrfaultenR {
        EccerrfaultenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - AHB port arbitration priority"]
    #[inline(always)]
    pub fn ahbportpriority(&self) -> AhbportpriorityR {
        AhbportpriorityR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Address fault bus fault enable"]
    #[inline(always)]
    pub fn addrfaulten(&self) -> AddrfaultenR {
        AddrfaultenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ECC functionality"]
    #[inline(always)]
    pub fn eccen(&mut self) -> EccenW<CtrlSpec> {
        EccenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable ECC syndrome writes"]
    #[inline(always)]
    pub fn eccwen(&mut self) -> EccwenW<CtrlSpec> {
        EccwenW::new(self, 1)
    }
    #[doc = "Bit 2 - ECC Error bus fault enable"]
    #[inline(always)]
    pub fn eccerrfaulten(&mut self) -> EccerrfaultenW<CtrlSpec> {
        EccerrfaultenW::new(self, 2)
    }
    #[doc = "Bits 3:5 - AHB port arbitration priority"]
    #[inline(always)]
    pub fn ahbportpriority(&mut self) -> AhbportpriorityW<CtrlSpec> {
        AhbportpriorityW::new(self, 3)
    }
    #[doc = "Bit 6 - Address fault bus fault enable"]
    #[inline(always)]
    pub fn addrfaulten(&mut self) -> AddrfaultenW<CtrlSpec> {
        AddrfaultenW::new(self, 6)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x40"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x40;
}
