#[doc = "Register `HFXOCAL` reader"]
pub type R = crate::R<HfxocalSpec>;
#[doc = "No Description\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Shuntbiasana {
    #[doc = "0: I20UA"]
    I20ua = 0,
    #[doc = "1: I30UA"]
    I30ua = 1,
    #[doc = "2: I40UA"]
    I40ua = 2,
    #[doc = "3: I50UA"]
    I50ua = 3,
    #[doc = "4: I60UA"]
    I60ua = 4,
    #[doc = "5: I70UA"]
    I70ua = 5,
    #[doc = "6: I80UA"]
    I80ua = 6,
    #[doc = "7: I90UA"]
    I90ua = 7,
    #[doc = "8: I100UA"]
    I100ua = 8,
    #[doc = "9: I110UA"]
    I110ua = 9,
    #[doc = "10: I120UA"]
    I120ua = 10,
    #[doc = "11: I130UA"]
    I130ua = 11,
    #[doc = "12: I140UA"]
    I140ua = 12,
    #[doc = "13: I150UA"]
    I150ua = 13,
    #[doc = "14: I160UA"]
    I160ua = 14,
    #[doc = "15: I170UA"]
    I170ua = 15,
}
impl From<Shuntbiasana> for u8 {
    #[inline(always)]
    fn from(variant: Shuntbiasana) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Shuntbiasana {
    type Ux = u8;
}
impl crate::IsEnum for Shuntbiasana {}
#[doc = "Field `SHUNTBIASANA` reader - No Description"]
pub type ShuntbiasanaR = crate::FieldReader<Shuntbiasana>;
impl ShuntbiasanaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shuntbiasana {
        match self.bits {
            0 => Shuntbiasana::I20ua,
            1 => Shuntbiasana::I30ua,
            2 => Shuntbiasana::I40ua,
            3 => Shuntbiasana::I50ua,
            4 => Shuntbiasana::I60ua,
            5 => Shuntbiasana::I70ua,
            6 => Shuntbiasana::I80ua,
            7 => Shuntbiasana::I90ua,
            8 => Shuntbiasana::I100ua,
            9 => Shuntbiasana::I110ua,
            10 => Shuntbiasana::I120ua,
            11 => Shuntbiasana::I130ua,
            12 => Shuntbiasana::I140ua,
            13 => Shuntbiasana::I150ua,
            14 => Shuntbiasana::I160ua,
            15 => Shuntbiasana::I170ua,
            _ => unreachable!(),
        }
    }
    #[doc = "I20UA"]
    #[inline(always)]
    pub fn is_i20ua(&self) -> bool {
        *self == Shuntbiasana::I20ua
    }
    #[doc = "I30UA"]
    #[inline(always)]
    pub fn is_i30ua(&self) -> bool {
        *self == Shuntbiasana::I30ua
    }
    #[doc = "I40UA"]
    #[inline(always)]
    pub fn is_i40ua(&self) -> bool {
        *self == Shuntbiasana::I40ua
    }
    #[doc = "I50UA"]
    #[inline(always)]
    pub fn is_i50ua(&self) -> bool {
        *self == Shuntbiasana::I50ua
    }
    #[doc = "I60UA"]
    #[inline(always)]
    pub fn is_i60ua(&self) -> bool {
        *self == Shuntbiasana::I60ua
    }
    #[doc = "I70UA"]
    #[inline(always)]
    pub fn is_i70ua(&self) -> bool {
        *self == Shuntbiasana::I70ua
    }
    #[doc = "I80UA"]
    #[inline(always)]
    pub fn is_i80ua(&self) -> bool {
        *self == Shuntbiasana::I80ua
    }
    #[doc = "I90UA"]
    #[inline(always)]
    pub fn is_i90ua(&self) -> bool {
        *self == Shuntbiasana::I90ua
    }
    #[doc = "I100UA"]
    #[inline(always)]
    pub fn is_i100ua(&self) -> bool {
        *self == Shuntbiasana::I100ua
    }
    #[doc = "I110UA"]
    #[inline(always)]
    pub fn is_i110ua(&self) -> bool {
        *self == Shuntbiasana::I110ua
    }
    #[doc = "I120UA"]
    #[inline(always)]
    pub fn is_i120ua(&self) -> bool {
        *self == Shuntbiasana::I120ua
    }
    #[doc = "I130UA"]
    #[inline(always)]
    pub fn is_i130ua(&self) -> bool {
        *self == Shuntbiasana::I130ua
    }
    #[doc = "I140UA"]
    #[inline(always)]
    pub fn is_i140ua(&self) -> bool {
        *self == Shuntbiasana::I140ua
    }
    #[doc = "I150UA"]
    #[inline(always)]
    pub fn is_i150ua(&self) -> bool {
        *self == Shuntbiasana::I150ua
    }
    #[doc = "I160UA"]
    #[inline(always)]
    pub fn is_i160ua(&self) -> bool {
        *self == Shuntbiasana::I160ua
    }
    #[doc = "I170UA"]
    #[inline(always)]
    pub fn is_i170ua(&self) -> bool {
        *self == Shuntbiasana::I170ua
    }
}
#[doc = "Field `VTRTRIMANA` reader - No Description"]
pub type VtrtrimanaR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - No Description"]
    #[inline(always)]
    pub fn shuntbiasana(&self) -> ShuntbiasanaR {
        ShuntbiasanaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - No Description"]
    #[inline(always)]
    pub fn vtrtrimana(&self) -> VtrtrimanaR {
        VtrtrimanaR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "High Frequency Crystal Oscillator Calibration data\n\nYou can [`read`](crate::Reg::read) this register and get [`hfxocal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfxocalSpec;
impl crate::RegisterSpec for HfxocalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxocal::R`](R) reader structure"]
impl crate::Readable for HfxocalSpec {}
#[doc = "`reset()` method sets HFXOCAL to value 0xffff_ff00"]
impl crate::Resettable for HfxocalSpec {
    const RESET_VALUE: u32 = 0xffff_ff00;
}
