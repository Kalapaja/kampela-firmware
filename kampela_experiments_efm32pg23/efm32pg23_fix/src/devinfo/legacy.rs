#[doc = "Register `LEGACY` reader"]
pub type R = crate::R<LegacySpec>;
#[doc = "Device Family\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Devicefamily {
    #[doc = "16: EFR32 Mighty Gecko Family Series 1 Device Config 1"]
    Efr32mg1p = 16,
    #[doc = "17: EFR32 Mighty Gecko Family Series 1 Device Config 1"]
    Efr32mg1b = 17,
    #[doc = "18: EFR32 Mighty Gecko Family Series 1 Device Config 1"]
    Efr32mg1v = 18,
    #[doc = "19: EFR32 Blue Gecko Family Series 1 Device Config 1"]
    Efr32bg1p = 19,
    #[doc = "20: EFR32 Blue Gecko Family Series 1 Device Config 1"]
    Efr32bg1b = 20,
    #[doc = "21: EFR32 Blue Gecko Family Series 1 Device Config 1"]
    Efr32bg1v = 21,
    #[doc = "25: EFR32 Flex Gecko Family Series 1 Device Config 1"]
    Efr32fg1p = 25,
    #[doc = "26: EFR32 Flex Gecko Family Series 1 Device Config 1"]
    Efr32fg1b = 26,
    #[doc = "27: EFR32 Flex Gecko Family Series 1 Device Config 1"]
    Efr32fg1v = 27,
    #[doc = "28: EFR32 Mighty Gecko Family Series 1 Device Config 2"]
    Efr32mg12p = 28,
    #[doc = "29: EFR32 Mighty Gecko Family Series 1 Device Config 2"]
    Efr32mg12b = 29,
    #[doc = "30: EFR32 Mighty Gecko Family Series 1 Device Config 2"]
    Efr32mg12v = 30,
    #[doc = "31: EFR32 Blue Gecko Family Series 1 Device Config 2"]
    Efr32bg12p = 31,
    #[doc = "32: EFR32 Blue Gecko Family Series 1 Device Config 2"]
    Efr32bg12b = 32,
    #[doc = "33: EFR32 Blue Gecko Family Series 1 Device Config 2"]
    Efr32bg12v = 33,
    #[doc = "37: EFR32 Flex Gecko Family Series 1 Device Config 2"]
    Efr32fg12p = 37,
    #[doc = "38: EFR32 Flex Gecko Family Series 1 Device Config 2"]
    Efr32fg12b = 38,
    #[doc = "39: EFR32 Flex Gecko Family Series 1 Device Config 2"]
    Efr32fg12v = 39,
    #[doc = "40: EFR32 Mighty Gecko Family Series 13 Device Config 3"]
    Efr32mg13p = 40,
    #[doc = "41: EFR32 Mighty Gecko Family Series 13 Device Config 3"]
    Efr32mg13b = 41,
    #[doc = "42: EFR32 Mighty Gecko Family Series 1 Device Config 3"]
    Efr32mg13v = 42,
    #[doc = "43: EFR32 Blue Gecko Family Series 1 Device Config 3"]
    Efr32bg13p = 43,
    #[doc = "44: EFR32 Blue Gecko Family Series 1 Device Config 3"]
    Efr32bg13b = 44,
    #[doc = "45: EFR32 Blue Gecko Family Series 1 Device Config 3"]
    Efr32bg13v = 45,
    #[doc = "49: EFR32 Flex Gecko Family Series 1 Device Config 3"]
    Efr32fg13p = 49,
    #[doc = "50: EFR32 Flex Gecko Family Series 1 Device Config 3"]
    Efr32fg13b = 50,
    #[doc = "51: EFR32 Flex Gecko Family Series 1 Device Config 3"]
    Efr32fg13v = 51,
    #[doc = "52: EFR32 Mighty Gecko Family Series 1 Device Config 4"]
    Efr32mg14p = 52,
    #[doc = "53: EFR32 Mighty Gecko Family Series 1 Device Config 4"]
    Efr32mg14b = 53,
    #[doc = "54: EFR32 Mighty Gecko Family Series 1 Device Config 4"]
    Efr32mg14v = 54,
    #[doc = "55: EFR32 Blue Gecko Family Series 1 Device Config 4"]
    Efr32bg14p = 55,
    #[doc = "56: EFR32 Blue Gecko Family Series 1 Device Config 4"]
    Efr32bg14b = 56,
    #[doc = "57: EFR32 Blue Gecko Family Series 1 Device Config 4"]
    Efr32bg14v = 57,
    #[doc = "61: EFR32 Flex Gecko Family Series 1 Device Config 4"]
    Efr32fg14p = 61,
    #[doc = "62: EFR32 Flex Gecko Family Series 1 Device Config 4"]
    Efr32fg14b = 62,
    #[doc = "63: EFR32 Flex Gecko Family Series 1 Device Config 4"]
    Efr32fg14v = 63,
    #[doc = "71: EFM32 Gecko Device Family"]
    Efm32g = 71,
    #[doc = "72: EFM32 Giant Gecko Device Family"]
    Efm32gg = 72,
    #[doc = "73: EFM32 Tiny Gecko Device Family"]
    Efm32tg = 73,
    #[doc = "74: EFM32 Leopard Gecko Device Family"]
    Efm32lg = 74,
    #[doc = "75: EFM32 Wonder Gecko Device Family"]
    Efm32wg = 75,
    #[doc = "76: EFM32 Zero Gecko Device Family"]
    Efm32zg = 76,
    #[doc = "77: EFM32 Happy Gecko Device Family"]
    Efm32hg = 77,
    #[doc = "81: EFM32 Pearl Gecko Device Family Series 1 Device Config 1"]
    Efm32pg1b = 81,
    #[doc = "83: EFM32 Jade Gecko Device Family Series 1 Device Config 1"]
    Efm32jg1b = 83,
    #[doc = "85: EFM32 Pearl Gecko Device Family Series 1 Device Config 2"]
    Efm32pg12b = 85,
    #[doc = "87: EFM32 Jade Gecko Device Family Series 1 Device Config 2"]
    Efm32jg12b = 87,
    #[doc = "89: EFM32 Pearl Gecko Device Family Series 1 Device Config 3"]
    Efm32pg13b = 89,
    #[doc = "91: EFM32 Jade Gecko Device Family Series 1 Device Config 3"]
    Efm32jg13b = 91,
    #[doc = "100: EFM32 Giant Gecko Device Family Series 1 Device Config 1"]
    Efm32gg11b = 100,
    #[doc = "103: EFM32 Giant Gecko Device Family Series 1 Device Config 1"]
    Efm32tg11b = 103,
    #[doc = "120: EZR32 Leopard Gecko Device Family"]
    Ezr32lg = 120,
    #[doc = "121: EZR32 Wonder Gecko Device Family"]
    Ezr32wg = 121,
    #[doc = "122: EZR32 Happy Gecko Device Family"]
    Ezr32hg = 122,
    #[doc = "128: DI page is encoded with the series 2 layout. Check alternate location."]
    Series2v0 = 128,
}
impl From<Devicefamily> for u8 {
    #[inline(always)]
    fn from(variant: Devicefamily) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Devicefamily {
    type Ux = u8;
}
impl crate::IsEnum for Devicefamily {}
#[doc = "Field `DEVICEFAMILY` reader - Device Family"]
pub type DevicefamilyR = crate::FieldReader<Devicefamily>;
impl DevicefamilyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Devicefamily> {
        match self.bits {
            16 => Some(Devicefamily::Efr32mg1p),
            17 => Some(Devicefamily::Efr32mg1b),
            18 => Some(Devicefamily::Efr32mg1v),
            19 => Some(Devicefamily::Efr32bg1p),
            20 => Some(Devicefamily::Efr32bg1b),
            21 => Some(Devicefamily::Efr32bg1v),
            25 => Some(Devicefamily::Efr32fg1p),
            26 => Some(Devicefamily::Efr32fg1b),
            27 => Some(Devicefamily::Efr32fg1v),
            28 => Some(Devicefamily::Efr32mg12p),
            29 => Some(Devicefamily::Efr32mg12b),
            30 => Some(Devicefamily::Efr32mg12v),
            31 => Some(Devicefamily::Efr32bg12p),
            32 => Some(Devicefamily::Efr32bg12b),
            33 => Some(Devicefamily::Efr32bg12v),
            37 => Some(Devicefamily::Efr32fg12p),
            38 => Some(Devicefamily::Efr32fg12b),
            39 => Some(Devicefamily::Efr32fg12v),
            40 => Some(Devicefamily::Efr32mg13p),
            41 => Some(Devicefamily::Efr32mg13b),
            42 => Some(Devicefamily::Efr32mg13v),
            43 => Some(Devicefamily::Efr32bg13p),
            44 => Some(Devicefamily::Efr32bg13b),
            45 => Some(Devicefamily::Efr32bg13v),
            49 => Some(Devicefamily::Efr32fg13p),
            50 => Some(Devicefamily::Efr32fg13b),
            51 => Some(Devicefamily::Efr32fg13v),
            52 => Some(Devicefamily::Efr32mg14p),
            53 => Some(Devicefamily::Efr32mg14b),
            54 => Some(Devicefamily::Efr32mg14v),
            55 => Some(Devicefamily::Efr32bg14p),
            56 => Some(Devicefamily::Efr32bg14b),
            57 => Some(Devicefamily::Efr32bg14v),
            61 => Some(Devicefamily::Efr32fg14p),
            62 => Some(Devicefamily::Efr32fg14b),
            63 => Some(Devicefamily::Efr32fg14v),
            71 => Some(Devicefamily::Efm32g),
            72 => Some(Devicefamily::Efm32gg),
            73 => Some(Devicefamily::Efm32tg),
            74 => Some(Devicefamily::Efm32lg),
            75 => Some(Devicefamily::Efm32wg),
            76 => Some(Devicefamily::Efm32zg),
            77 => Some(Devicefamily::Efm32hg),
            81 => Some(Devicefamily::Efm32pg1b),
            83 => Some(Devicefamily::Efm32jg1b),
            85 => Some(Devicefamily::Efm32pg12b),
            87 => Some(Devicefamily::Efm32jg12b),
            89 => Some(Devicefamily::Efm32pg13b),
            91 => Some(Devicefamily::Efm32jg13b),
            100 => Some(Devicefamily::Efm32gg11b),
            103 => Some(Devicefamily::Efm32tg11b),
            120 => Some(Devicefamily::Ezr32lg),
            121 => Some(Devicefamily::Ezr32wg),
            122 => Some(Devicefamily::Ezr32hg),
            128 => Some(Devicefamily::Series2v0),
            _ => None,
        }
    }
    #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 1"]
    #[inline(always)]
    pub fn is_efr32mg1p(&self) -> bool {
        *self == Devicefamily::Efr32mg1p
    }
    #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 1"]
    #[inline(always)]
    pub fn is_efr32mg1b(&self) -> bool {
        *self == Devicefamily::Efr32mg1b
    }
    #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 1"]
    #[inline(always)]
    pub fn is_efr32mg1v(&self) -> bool {
        *self == Devicefamily::Efr32mg1v
    }
    #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 1"]
    #[inline(always)]
    pub fn is_efr32bg1p(&self) -> bool {
        *self == Devicefamily::Efr32bg1p
    }
    #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 1"]
    #[inline(always)]
    pub fn is_efr32bg1b(&self) -> bool {
        *self == Devicefamily::Efr32bg1b
    }
    #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 1"]
    #[inline(always)]
    pub fn is_efr32bg1v(&self) -> bool {
        *self == Devicefamily::Efr32bg1v
    }
    #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 1"]
    #[inline(always)]
    pub fn is_efr32fg1p(&self) -> bool {
        *self == Devicefamily::Efr32fg1p
    }
    #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 1"]
    #[inline(always)]
    pub fn is_efr32fg1b(&self) -> bool {
        *self == Devicefamily::Efr32fg1b
    }
    #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 1"]
    #[inline(always)]
    pub fn is_efr32fg1v(&self) -> bool {
        *self == Devicefamily::Efr32fg1v
    }
    #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 2"]
    #[inline(always)]
    pub fn is_efr32mg12p(&self) -> bool {
        *self == Devicefamily::Efr32mg12p
    }
    #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 2"]
    #[inline(always)]
    pub fn is_efr32mg12b(&self) -> bool {
        *self == Devicefamily::Efr32mg12b
    }
    #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 2"]
    #[inline(always)]
    pub fn is_efr32mg12v(&self) -> bool {
        *self == Devicefamily::Efr32mg12v
    }
    #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 2"]
    #[inline(always)]
    pub fn is_efr32bg12p(&self) -> bool {
        *self == Devicefamily::Efr32bg12p
    }
    #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 2"]
    #[inline(always)]
    pub fn is_efr32bg12b(&self) -> bool {
        *self == Devicefamily::Efr32bg12b
    }
    #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 2"]
    #[inline(always)]
    pub fn is_efr32bg12v(&self) -> bool {
        *self == Devicefamily::Efr32bg12v
    }
    #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 2"]
    #[inline(always)]
    pub fn is_efr32fg12p(&self) -> bool {
        *self == Devicefamily::Efr32fg12p
    }
    #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 2"]
    #[inline(always)]
    pub fn is_efr32fg12b(&self) -> bool {
        *self == Devicefamily::Efr32fg12b
    }
    #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 2"]
    #[inline(always)]
    pub fn is_efr32fg12v(&self) -> bool {
        *self == Devicefamily::Efr32fg12v
    }
    #[doc = "EFR32 Mighty Gecko Family Series 13 Device Config 3"]
    #[inline(always)]
    pub fn is_efr32mg13p(&self) -> bool {
        *self == Devicefamily::Efr32mg13p
    }
    #[doc = "EFR32 Mighty Gecko Family Series 13 Device Config 3"]
    #[inline(always)]
    pub fn is_efr32mg13b(&self) -> bool {
        *self == Devicefamily::Efr32mg13b
    }
    #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 3"]
    #[inline(always)]
    pub fn is_efr32mg13v(&self) -> bool {
        *self == Devicefamily::Efr32mg13v
    }
    #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 3"]
    #[inline(always)]
    pub fn is_efr32bg13p(&self) -> bool {
        *self == Devicefamily::Efr32bg13p
    }
    #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 3"]
    #[inline(always)]
    pub fn is_efr32bg13b(&self) -> bool {
        *self == Devicefamily::Efr32bg13b
    }
    #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 3"]
    #[inline(always)]
    pub fn is_efr32bg13v(&self) -> bool {
        *self == Devicefamily::Efr32bg13v
    }
    #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 3"]
    #[inline(always)]
    pub fn is_efr32fg13p(&self) -> bool {
        *self == Devicefamily::Efr32fg13p
    }
    #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 3"]
    #[inline(always)]
    pub fn is_efr32fg13b(&self) -> bool {
        *self == Devicefamily::Efr32fg13b
    }
    #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 3"]
    #[inline(always)]
    pub fn is_efr32fg13v(&self) -> bool {
        *self == Devicefamily::Efr32fg13v
    }
    #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 4"]
    #[inline(always)]
    pub fn is_efr32mg14p(&self) -> bool {
        *self == Devicefamily::Efr32mg14p
    }
    #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 4"]
    #[inline(always)]
    pub fn is_efr32mg14b(&self) -> bool {
        *self == Devicefamily::Efr32mg14b
    }
    #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 4"]
    #[inline(always)]
    pub fn is_efr32mg14v(&self) -> bool {
        *self == Devicefamily::Efr32mg14v
    }
    #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 4"]
    #[inline(always)]
    pub fn is_efr32bg14p(&self) -> bool {
        *self == Devicefamily::Efr32bg14p
    }
    #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 4"]
    #[inline(always)]
    pub fn is_efr32bg14b(&self) -> bool {
        *self == Devicefamily::Efr32bg14b
    }
    #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 4"]
    #[inline(always)]
    pub fn is_efr32bg14v(&self) -> bool {
        *self == Devicefamily::Efr32bg14v
    }
    #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 4"]
    #[inline(always)]
    pub fn is_efr32fg14p(&self) -> bool {
        *self == Devicefamily::Efr32fg14p
    }
    #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 4"]
    #[inline(always)]
    pub fn is_efr32fg14b(&self) -> bool {
        *self == Devicefamily::Efr32fg14b
    }
    #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 4"]
    #[inline(always)]
    pub fn is_efr32fg14v(&self) -> bool {
        *self == Devicefamily::Efr32fg14v
    }
    #[doc = "EFM32 Gecko Device Family"]
    #[inline(always)]
    pub fn is_efm32g(&self) -> bool {
        *self == Devicefamily::Efm32g
    }
    #[doc = "EFM32 Giant Gecko Device Family"]
    #[inline(always)]
    pub fn is_efm32gg(&self) -> bool {
        *self == Devicefamily::Efm32gg
    }
    #[doc = "EFM32 Tiny Gecko Device Family"]
    #[inline(always)]
    pub fn is_efm32tg(&self) -> bool {
        *self == Devicefamily::Efm32tg
    }
    #[doc = "EFM32 Leopard Gecko Device Family"]
    #[inline(always)]
    pub fn is_efm32lg(&self) -> bool {
        *self == Devicefamily::Efm32lg
    }
    #[doc = "EFM32 Wonder Gecko Device Family"]
    #[inline(always)]
    pub fn is_efm32wg(&self) -> bool {
        *self == Devicefamily::Efm32wg
    }
    #[doc = "EFM32 Zero Gecko Device Family"]
    #[inline(always)]
    pub fn is_efm32zg(&self) -> bool {
        *self == Devicefamily::Efm32zg
    }
    #[doc = "EFM32 Happy Gecko Device Family"]
    #[inline(always)]
    pub fn is_efm32hg(&self) -> bool {
        *self == Devicefamily::Efm32hg
    }
    #[doc = "EFM32 Pearl Gecko Device Family Series 1 Device Config 1"]
    #[inline(always)]
    pub fn is_efm32pg1b(&self) -> bool {
        *self == Devicefamily::Efm32pg1b
    }
    #[doc = "EFM32 Jade Gecko Device Family Series 1 Device Config 1"]
    #[inline(always)]
    pub fn is_efm32jg1b(&self) -> bool {
        *self == Devicefamily::Efm32jg1b
    }
    #[doc = "EFM32 Pearl Gecko Device Family Series 1 Device Config 2"]
    #[inline(always)]
    pub fn is_efm32pg12b(&self) -> bool {
        *self == Devicefamily::Efm32pg12b
    }
    #[doc = "EFM32 Jade Gecko Device Family Series 1 Device Config 2"]
    #[inline(always)]
    pub fn is_efm32jg12b(&self) -> bool {
        *self == Devicefamily::Efm32jg12b
    }
    #[doc = "EFM32 Pearl Gecko Device Family Series 1 Device Config 3"]
    #[inline(always)]
    pub fn is_efm32pg13b(&self) -> bool {
        *self == Devicefamily::Efm32pg13b
    }
    #[doc = "EFM32 Jade Gecko Device Family Series 1 Device Config 3"]
    #[inline(always)]
    pub fn is_efm32jg13b(&self) -> bool {
        *self == Devicefamily::Efm32jg13b
    }
    #[doc = "EFM32 Giant Gecko Device Family Series 1 Device Config 1"]
    #[inline(always)]
    pub fn is_efm32gg11b(&self) -> bool {
        *self == Devicefamily::Efm32gg11b
    }
    #[doc = "EFM32 Giant Gecko Device Family Series 1 Device Config 1"]
    #[inline(always)]
    pub fn is_efm32tg11b(&self) -> bool {
        *self == Devicefamily::Efm32tg11b
    }
    #[doc = "EZR32 Leopard Gecko Device Family"]
    #[inline(always)]
    pub fn is_ezr32lg(&self) -> bool {
        *self == Devicefamily::Ezr32lg
    }
    #[doc = "EZR32 Wonder Gecko Device Family"]
    #[inline(always)]
    pub fn is_ezr32wg(&self) -> bool {
        *self == Devicefamily::Ezr32wg
    }
    #[doc = "EZR32 Happy Gecko Device Family"]
    #[inline(always)]
    pub fn is_ezr32hg(&self) -> bool {
        *self == Devicefamily::Ezr32hg
    }
    #[doc = "DI page is encoded with the series 2 layout. Check alternate location."]
    #[inline(always)]
    pub fn is_series2v0(&self) -> bool {
        *self == Devicefamily::Series2v0
    }
}
impl R {
    #[doc = "Bits 16:23 - Device Family"]
    #[inline(always)]
    pub fn devicefamily(&self) -> DevicefamilyR {
        DevicefamilyR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "This is the legacy device detection information for tools compatability\n\nYou can [`read`](crate::Reg::read) this register and get [`legacy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LegacySpec;
impl crate::RegisterSpec for LegacySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`legacy::R`](R) reader structure"]
impl crate::Readable for LegacySpec {}
#[doc = "`reset()` method sets LEGACY to value 0x0080_0000"]
impl crate::Resettable for LegacySpec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
