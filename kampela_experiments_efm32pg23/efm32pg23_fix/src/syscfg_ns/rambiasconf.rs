#[doc = "Register `RAMBIASCONF` reader"]
pub type R = crate::R<RambiasconfSpec>;
#[doc = "Register `RAMBIASCONF` writer"]
pub type W = crate::W<RambiasconfSpec>;
#[doc = "RAM Bias Control\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rambiasctrl {
    #[doc = "0: None"]
    No = 0,
    #[doc = "1: Voltage Source Bias 100mV"]
    Vsb100 = 1,
    #[doc = "2: Voltage Source Bias 200mV"]
    Vsb200 = 2,
    #[doc = "4: Voltage Source Bias 300mV"]
    Vsb300 = 4,
    #[doc = "8: Voltage Source Bias 400mV"]
    Vsb400 = 8,
}
impl From<Rambiasctrl> for u8 {
    #[inline(always)]
    fn from(variant: Rambiasctrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rambiasctrl {
    type Ux = u8;
}
impl crate::IsEnum for Rambiasctrl {}
#[doc = "Field `RAMBIASCTRL` reader - RAM Bias Control"]
pub type RambiasctrlR = crate::FieldReader<Rambiasctrl>;
impl RambiasctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rambiasctrl> {
        match self.bits {
            0 => Some(Rambiasctrl::No),
            1 => Some(Rambiasctrl::Vsb100),
            2 => Some(Rambiasctrl::Vsb200),
            4 => Some(Rambiasctrl::Vsb300),
            8 => Some(Rambiasctrl::Vsb400),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Rambiasctrl::No
    }
    #[doc = "Voltage Source Bias 100mV"]
    #[inline(always)]
    pub fn is_vsb100(&self) -> bool {
        *self == Rambiasctrl::Vsb100
    }
    #[doc = "Voltage Source Bias 200mV"]
    #[inline(always)]
    pub fn is_vsb200(&self) -> bool {
        *self == Rambiasctrl::Vsb200
    }
    #[doc = "Voltage Source Bias 300mV"]
    #[inline(always)]
    pub fn is_vsb300(&self) -> bool {
        *self == Rambiasctrl::Vsb300
    }
    #[doc = "Voltage Source Bias 400mV"]
    #[inline(always)]
    pub fn is_vsb400(&self) -> bool {
        *self == Rambiasctrl::Vsb400
    }
}
#[doc = "Field `RAMBIASCTRL` writer - RAM Bias Control"]
pub type RambiasctrlW<'a, REG> = crate::FieldWriter<'a, REG, 4, Rambiasctrl>;
impl<'a, REG> RambiasctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Rambiasctrl::No)
    }
    #[doc = "Voltage Source Bias 100mV"]
    #[inline(always)]
    pub fn vsb100(self) -> &'a mut crate::W<REG> {
        self.variant(Rambiasctrl::Vsb100)
    }
    #[doc = "Voltage Source Bias 200mV"]
    #[inline(always)]
    pub fn vsb200(self) -> &'a mut crate::W<REG> {
        self.variant(Rambiasctrl::Vsb200)
    }
    #[doc = "Voltage Source Bias 300mV"]
    #[inline(always)]
    pub fn vsb300(self) -> &'a mut crate::W<REG> {
        self.variant(Rambiasctrl::Vsb300)
    }
    #[doc = "Voltage Source Bias 400mV"]
    #[inline(always)]
    pub fn vsb400(self) -> &'a mut crate::W<REG> {
        self.variant(Rambiasctrl::Vsb400)
    }
}
impl R {
    #[doc = "Bits 0:3 - RAM Bias Control"]
    #[inline(always)]
    pub fn rambiasctrl(&self) -> RambiasctrlR {
        RambiasctrlR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RAM Bias Control"]
    #[inline(always)]
    pub fn rambiasctrl(&mut self) -> RambiasctrlW<RambiasconfSpec> {
        RambiasctrlW::new(self, 0)
    }
}
#[doc = "Configure RAM bias configure bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`rambiasconf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rambiasconf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RambiasconfSpec;
impl crate::RegisterSpec for RambiasconfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rambiasconf::R`](R) reader structure"]
impl crate::Readable for RambiasconfSpec {}
#[doc = "`write(|w| ..)` method takes [`rambiasconf::W`](W) writer structure"]
impl crate::Writable for RambiasconfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAMBIASCONF to value 0x02"]
impl crate::Resettable for RambiasconfSpec {
    const RESET_VALUE: u32 = 0x02;
}
