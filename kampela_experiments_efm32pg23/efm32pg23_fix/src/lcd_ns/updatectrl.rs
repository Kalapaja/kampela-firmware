#[doc = "Register `UPDATECTRL` reader"]
pub type R = crate::R<UpdatectrlSpec>;
#[doc = "Register `UPDATECTRL` writer"]
pub type W = crate::W<UpdatectrlSpec>;
#[doc = "Auto Load\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autoload {
    #[doc = "0: CLK_BUS register to CLK_PER register loads must be done manually with a write to CMD.LOAD."]
    Manual = 0,
    #[doc = "1: CLK_BUS register to CLK_PER register loads will be started automatically after a write to the register in UPDATECTRL.LOADADDR is detected."]
    Auto = 1,
}
impl From<Autoload> for bool {
    #[inline(always)]
    fn from(variant: Autoload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOLOAD` reader - Auto Load"]
pub type AutoloadR = crate::BitReader<Autoload>;
impl AutoloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autoload {
        match self.bits {
            false => Autoload::Manual,
            true => Autoload::Auto,
        }
    }
    #[doc = "CLK_BUS register to CLK_PER register loads must be done manually with a write to CMD.LOAD."]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == Autoload::Manual
    }
    #[doc = "CLK_BUS register to CLK_PER register loads will be started automatically after a write to the register in UPDATECTRL.LOADADDR is detected."]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Autoload::Auto
    }
}
#[doc = "Field `AUTOLOAD` writer - Auto Load"]
pub type AutoloadW<'a, REG> = crate::BitWriter<'a, REG, Autoload>;
impl<'a, REG> AutoloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLK_BUS register to CLK_PER register loads must be done manually with a write to CMD.LOAD."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(Autoload::Manual)
    }
    #[doc = "CLK_BUS register to CLK_PER register loads will be started automatically after a write to the register in UPDATECTRL.LOADADDR is detected."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Autoload::Auto)
    }
}
#[doc = "Load Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Loadaddr {
    #[doc = "0: Starts synchronizing registers from CLK_BUS to CLK_PER after a write to BACTRL. Use with UPDATECTRL.AUTOLOAD"]
    Bactrlwr = 0,
    #[doc = "1: Starts synchronizing registers from CLK_BUS to CLK_PER after a write to AREGA. Use with UPDATECTRL.AUTOLOAD"]
    Aregawr = 1,
    #[doc = "2: Starts synchronizing registers from CLK_BUS to CLK_PER after a write to AREGB. Use with UPDATECTRL.AUTOLOAD"]
    Aregbwr = 2,
    #[doc = "3: Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD0. Use with UPDATECTRL.AUTOLOAD"]
    Segd0wr = 3,
    #[doc = "4: Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD1. Use with UPDATECTRL.AUTOLOAD"]
    Segd1wr = 4,
    #[doc = "5: Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD2. Use with UPDATECTRL.AUTOLOAD"]
    Segd2wr = 5,
    #[doc = "6: Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD3. Use with UPDATECTRL.AUTOLOAD"]
    Segd3wr = 6,
}
impl From<Loadaddr> for u8 {
    #[inline(always)]
    fn from(variant: Loadaddr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Loadaddr {
    type Ux = u8;
}
impl crate::IsEnum for Loadaddr {}
#[doc = "Field `LOADADDR` reader - Load Address"]
pub type LoadaddrR = crate::FieldReader<Loadaddr>;
impl LoadaddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Loadaddr> {
        match self.bits {
            0 => Some(Loadaddr::Bactrlwr),
            1 => Some(Loadaddr::Aregawr),
            2 => Some(Loadaddr::Aregbwr),
            3 => Some(Loadaddr::Segd0wr),
            4 => Some(Loadaddr::Segd1wr),
            5 => Some(Loadaddr::Segd2wr),
            6 => Some(Loadaddr::Segd3wr),
            _ => None,
        }
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to BACTRL. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn is_bactrlwr(&self) -> bool {
        *self == Loadaddr::Bactrlwr
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to AREGA. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn is_aregawr(&self) -> bool {
        *self == Loadaddr::Aregawr
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to AREGB. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn is_aregbwr(&self) -> bool {
        *self == Loadaddr::Aregbwr
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD0. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn is_segd0wr(&self) -> bool {
        *self == Loadaddr::Segd0wr
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD1. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn is_segd1wr(&self) -> bool {
        *self == Loadaddr::Segd1wr
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD2. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn is_segd2wr(&self) -> bool {
        *self == Loadaddr::Segd2wr
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD3. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn is_segd3wr(&self) -> bool {
        *self == Loadaddr::Segd3wr
    }
}
#[doc = "Field `LOADADDR` writer - Load Address"]
pub type LoadaddrW<'a, REG> = crate::FieldWriter<'a, REG, 4, Loadaddr>;
impl<'a, REG> LoadaddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to BACTRL. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn bactrlwr(self) -> &'a mut crate::W<REG> {
        self.variant(Loadaddr::Bactrlwr)
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to AREGA. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn aregawr(self) -> &'a mut crate::W<REG> {
        self.variant(Loadaddr::Aregawr)
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to AREGB. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn aregbwr(self) -> &'a mut crate::W<REG> {
        self.variant(Loadaddr::Aregbwr)
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD0. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn segd0wr(self) -> &'a mut crate::W<REG> {
        self.variant(Loadaddr::Segd0wr)
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD1. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn segd1wr(self) -> &'a mut crate::W<REG> {
        self.variant(Loadaddr::Segd1wr)
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD2. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn segd2wr(self) -> &'a mut crate::W<REG> {
        self.variant(Loadaddr::Segd2wr)
    }
    #[doc = "Starts synchronizing registers from CLK_BUS to CLK_PER after a write to SEGD3. Use with UPDATECTRL.AUTOLOAD"]
    #[inline(always)]
    pub fn segd3wr(self) -> &'a mut crate::W<REG> {
        self.variant(Loadaddr::Segd3wr)
    }
}
impl R {
    #[doc = "Bit 8 - Auto Load"]
    #[inline(always)]
    pub fn autoload(&self) -> AutoloadR {
        AutoloadR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Load Address"]
    #[inline(always)]
    pub fn loadaddr(&self) -> LoadaddrR {
        LoadaddrR::new(((self.bits >> 13) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Auto Load"]
    #[inline(always)]
    pub fn autoload(&mut self) -> AutoloadW<UpdatectrlSpec> {
        AutoloadW::new(self, 8)
    }
    #[doc = "Bits 13:16 - Load Address"]
    #[inline(always)]
    pub fn loadaddr(&mut self) -> LoadaddrW<UpdatectrlSpec> {
        LoadaddrW::new(self, 13)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`updatectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`updatectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UpdatectrlSpec;
impl crate::RegisterSpec for UpdatectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`updatectrl::R`](R) reader structure"]
impl crate::Readable for UpdatectrlSpec {}
#[doc = "`write(|w| ..)` method takes [`updatectrl::W`](W) writer structure"]
impl crate::Writable for UpdatectrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UPDATECTRL to value 0"]
impl crate::Resettable for UpdatectrlSpec {}
