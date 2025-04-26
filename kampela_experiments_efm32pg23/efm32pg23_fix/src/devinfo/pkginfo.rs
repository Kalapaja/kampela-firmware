#[doc = "Register `PKGINFO` reader"]
pub type R = crate::R<PkginfoSpec>;
#[doc = "Temperature Grade\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tempgrade {
    #[doc = "0: -40 to 85 degC"]
    N40to85 = 0,
    #[doc = "1: -40 to 125 degC"]
    N40to125 = 1,
    #[doc = "2: -40 to 105 degC"]
    N40to105 = 2,
    #[doc = "3: 0 to 70 degC"]
    N0to70 = 3,
}
impl From<Tempgrade> for u8 {
    #[inline(always)]
    fn from(variant: Tempgrade) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tempgrade {
    type Ux = u8;
}
impl crate::IsEnum for Tempgrade {}
#[doc = "Field `TEMPGRADE` reader - Temperature Grade"]
pub type TempgradeR = crate::FieldReader<Tempgrade>;
impl TempgradeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tempgrade> {
        match self.bits {
            0 => Some(Tempgrade::N40to85),
            1 => Some(Tempgrade::N40to125),
            2 => Some(Tempgrade::N40to105),
            3 => Some(Tempgrade::N0to70),
            _ => None,
        }
    }
    #[doc = "-40 to 85 degC"]
    #[inline(always)]
    pub fn is_n40to85(&self) -> bool {
        *self == Tempgrade::N40to85
    }
    #[doc = "-40 to 125 degC"]
    #[inline(always)]
    pub fn is_n40to125(&self) -> bool {
        *self == Tempgrade::N40to125
    }
    #[doc = "-40 to 105 degC"]
    #[inline(always)]
    pub fn is_n40to105(&self) -> bool {
        *self == Tempgrade::N40to105
    }
    #[doc = "0 to 70 degC"]
    #[inline(always)]
    pub fn is_n0to70(&self) -> bool {
        *self == Tempgrade::N0to70
    }
}
#[doc = "Package Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pkgtype {
    #[doc = "74: WLCSP package"]
    Wlcsp = 74,
    #[doc = "76: BGA package"]
    Bga = 76,
    #[doc = "77: QFN package"]
    Qfn = 77,
    #[doc = "81: QFP package"]
    Qfp = 81,
}
impl From<Pkgtype> for u8 {
    #[inline(always)]
    fn from(variant: Pkgtype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pkgtype {
    type Ux = u8;
}
impl crate::IsEnum for Pkgtype {}
#[doc = "Field `PKGTYPE` reader - Package Type"]
pub type PkgtypeR = crate::FieldReader<Pkgtype>;
impl PkgtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pkgtype> {
        match self.bits {
            74 => Some(Pkgtype::Wlcsp),
            76 => Some(Pkgtype::Bga),
            77 => Some(Pkgtype::Qfn),
            81 => Some(Pkgtype::Qfp),
            _ => None,
        }
    }
    #[doc = "WLCSP package"]
    #[inline(always)]
    pub fn is_wlcsp(&self) -> bool {
        *self == Pkgtype::Wlcsp
    }
    #[doc = "BGA package"]
    #[inline(always)]
    pub fn is_bga(&self) -> bool {
        *self == Pkgtype::Bga
    }
    #[doc = "QFN package"]
    #[inline(always)]
    pub fn is_qfn(&self) -> bool {
        *self == Pkgtype::Qfn
    }
    #[doc = "QFP package"]
    #[inline(always)]
    pub fn is_qfp(&self) -> bool {
        *self == Pkgtype::Qfp
    }
}
#[doc = "Field `PINCOUNT` reader - Pin Count"]
pub type PincountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Temperature Grade"]
    #[inline(always)]
    pub fn tempgrade(&self) -> TempgradeR {
        TempgradeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Package Type"]
    #[inline(always)]
    pub fn pkgtype(&self) -> PkgtypeR {
        PkgtypeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Pin Count"]
    #[inline(always)]
    pub fn pincount(&self) -> PincountR {
        PincountR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Miscellaneous device information\n\nYou can [`read`](crate::Reg::read) this register and get [`pkginfo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkginfoSpec;
impl crate::RegisterSpec for PkginfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkginfo::R`](R) reader structure"]
impl crate::Readable for PkginfoSpec {}
#[doc = "`reset()` method sets PKGINFO to value 0"]
impl crate::Resettable for PkginfoSpec {}
