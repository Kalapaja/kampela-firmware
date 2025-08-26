#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "DCDC/Bypass Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: DCDC is OFF, bypass switch is enabled"]
    Bypass = 0,
    #[doc = "1: Request DCDC regulation, bypass switch disabled"]
    Dcdcregulation = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - DCDC/Bypass Mode Control"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Bypass,
            true => Mode::Dcdcregulation,
        }
    }
    #[doc = "DCDC is OFF, bypass switch is enabled"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Mode::Bypass
    }
    #[doc = "Request DCDC regulation, bypass switch disabled"]
    #[inline(always)]
    pub fn is_dcdcregulation(&self) -> bool {
        *self == Mode::Dcdcregulation
    }
}
#[doc = "Field `MODE` writer - DCDC/Bypass Mode Control"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DCDC is OFF, bypass switch is enabled"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Bypass)
    }
    #[doc = "Request DCDC regulation, bypass switch disabled"]
    #[inline(always)]
    pub fn dcdcregulation(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Dcdcregulation)
    }
}
#[doc = "Field `IPKTMAXCTRL` reader - Ton_max timeout control"]
pub type IpktmaxctrlR = crate::FieldReader;
#[doc = "Field `IPKTMAXCTRL` writer - Ton_max timeout control"]
pub type IpktmaxctrlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - DCDC/Bypass Mode Control"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:8 - Ton_max timeout control"]
    #[inline(always)]
    pub fn ipktmaxctrl(&self) -> IpktmaxctrlR {
        IpktmaxctrlR::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DCDC/Bypass Mode Control"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<CtrlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 4:8 - Ton_max timeout control"]
    #[inline(always)]
    pub fn ipktmaxctrl(&mut self) -> IpktmaxctrlW<CtrlSpec> {
        IpktmaxctrlW::new(self, 4)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x0100"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0100;
}
