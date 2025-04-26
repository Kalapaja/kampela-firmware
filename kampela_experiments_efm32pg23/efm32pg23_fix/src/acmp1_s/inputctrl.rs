#[doc = "Register `INPUTCTRL` reader"]
pub type R = crate::R<InputctrlSpec>;
#[doc = "Register `INPUTCTRL` writer"]
pub type W = crate::W<InputctrlSpec>;
#[doc = "Positive Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Possel {
    #[doc = "0: VSS"]
    Vss = 0,
    #[doc = "16: Divided AVDD"]
    Vrefdivavdd = 16,
    #[doc = "17: Low-Power Divided AVDD"]
    Vrefdivavddlp = 17,
    #[doc = "18: Divided 1V25 reference"]
    Vrefdiv1v25 = 18,
    #[doc = "19: Low-power Divided 1V25 reference"]
    Vrefdiv1v25lp = 19,
    #[doc = "20: Divided 2V5 reference"]
    Vrefdiv2v5 = 20,
    #[doc = "21: Low-power Divided 2V5 reference"]
    Vrefdiv2v5lp = 21,
    #[doc = "32: VSENSE0 divided by 4"]
    Vsense01div4 = 32,
    #[doc = "33: Low-power VSENSE0 divided by 4"]
    Vsense01div4lp = 33,
    #[doc = "34: VSENSE1 divided by 4"]
    Vsense11div4 = 34,
    #[doc = "35: Low-power VSENSE1 divided by 4"]
    Vsense11div4lp = 35,
    #[doc = "64: VDAC0 channel 0 output"]
    Vdacout0 = 64,
    #[doc = "65: VDAC0 channel 1 output"]
    Vdacout1 = 65,
    #[doc = "80: External interface, base is PA0."]
    Extpa = 80,
    #[doc = "81: External interface, base is PB0."]
    Extpb = 81,
    #[doc = "82: External interface, base is PC0."]
    Extpc = 82,
    #[doc = "83: External interface, base is PD0."]
    Extpd = 83,
    #[doc = "128: Port A, Pin0"]
    Pa0 = 128,
    #[doc = "129: Port A, Pin1"]
    Pa1 = 129,
    #[doc = "130: Port A, Pin2"]
    Pa2 = 130,
    #[doc = "131: Port A, Pin3"]
    Pa3 = 131,
    #[doc = "132: Port A, Pin4"]
    Pa4 = 132,
    #[doc = "133: Port A, Pin5"]
    Pa5 = 133,
    #[doc = "134: Port A, Pin6"]
    Pa6 = 134,
    #[doc = "135: Port A, Pin7"]
    Pa7 = 135,
    #[doc = "136: Port A, Pin8"]
    Pa8 = 136,
    #[doc = "137: Port A, Pin9"]
    Pa9 = 137,
    #[doc = "138: Port A, Pin10"]
    Pa10 = 138,
    #[doc = "139: Port A, Pin11"]
    Pa11 = 139,
    #[doc = "140: Port A, Pin12"]
    Pa12 = 140,
    #[doc = "141: Port A, Pin13"]
    Pa13 = 141,
    #[doc = "142: Port A, Pin14"]
    Pa14 = 142,
    #[doc = "143: Port A, Pin15"]
    Pa15 = 143,
    #[doc = "144: Port B, Pin0"]
    Pb0 = 144,
    #[doc = "145: Port B, Pin1"]
    Pb1 = 145,
    #[doc = "146: Port B, Pin2"]
    Pb2 = 146,
    #[doc = "147: Port B, Pin3"]
    Pb3 = 147,
    #[doc = "148: Port B, Pin4"]
    Pb4 = 148,
    #[doc = "149: Port B, Pin5"]
    Pb5 = 149,
    #[doc = "150: Port B, Pin6"]
    Pb6 = 150,
    #[doc = "151: Port B, Pin7"]
    Pb7 = 151,
    #[doc = "152: Port B, Pin8"]
    Pb8 = 152,
    #[doc = "153: Port B, Pin9"]
    Pb9 = 153,
    #[doc = "154: Port B, Pin10"]
    Pb10 = 154,
    #[doc = "155: Port B, Pin11"]
    Pb11 = 155,
    #[doc = "156: Port B, Pin12"]
    Pb12 = 156,
    #[doc = "157: Port B, Pin13"]
    Pb13 = 157,
    #[doc = "158: Port B, Pin14"]
    Pb14 = 158,
    #[doc = "159: Port B, Pin15"]
    Pb15 = 159,
    #[doc = "160: Port C, Pin0"]
    Pc0 = 160,
    #[doc = "161: Port C, Pin1"]
    Pc1 = 161,
    #[doc = "162: Port C, Pin2"]
    Pc2 = 162,
    #[doc = "163: Port C, Pin3"]
    Pc3 = 163,
    #[doc = "164: Port C, Pin4"]
    Pc4 = 164,
    #[doc = "165: Port C, Pin5"]
    Pc5 = 165,
    #[doc = "166: Port C, Pin6"]
    Pc6 = 166,
    #[doc = "167: Port C, Pin7"]
    Pc7 = 167,
    #[doc = "168: Port C, Pin8"]
    Pc8 = 168,
    #[doc = "169: Port C, Pin9"]
    Pc9 = 169,
    #[doc = "170: Port C, Pin10"]
    Pc10 = 170,
    #[doc = "171: Port C, Pin11"]
    Pc11 = 171,
    #[doc = "172: Port C, Pin12"]
    Pc12 = 172,
    #[doc = "173: Port C, Pin13"]
    Pc13 = 173,
    #[doc = "174: Port C, Pin14"]
    Pc14 = 174,
    #[doc = "175: Port C, Pin15"]
    Pc15 = 175,
    #[doc = "176: Port D, Pin0"]
    Pd0 = 176,
    #[doc = "177: Port D, Pin1"]
    Pd1 = 177,
    #[doc = "178: Port D, Pin2"]
    Pd2 = 178,
    #[doc = "179: Port D, Pin3"]
    Pd3 = 179,
    #[doc = "180: Port D, Pin4"]
    Pd4 = 180,
    #[doc = "181: Port D, Pin5"]
    Pd5 = 181,
    #[doc = "182: Port D, Pin6"]
    Pd6 = 182,
    #[doc = "183: Port D, Pin7"]
    Pd7 = 183,
    #[doc = "184: Port D, Pin8"]
    Pd8 = 184,
    #[doc = "185: Port D, Pin9"]
    Pd9 = 185,
    #[doc = "186: Port D, Pin10"]
    Pd10 = 186,
    #[doc = "187: Port D, Pin11"]
    Pd11 = 187,
    #[doc = "188: Port D, Pin12"]
    Pd12 = 188,
    #[doc = "189: Port D, Pin13"]
    Pd13 = 189,
    #[doc = "190: Port D, Pin14"]
    Pd14 = 190,
    #[doc = "191: Port D, Pin15"]
    Pd15 = 191,
}
impl From<Possel> for u8 {
    #[inline(always)]
    fn from(variant: Possel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Possel {
    type Ux = u8;
}
impl crate::IsEnum for Possel {}
#[doc = "Field `POSSEL` reader - Positive Input Select"]
pub type PosselR = crate::FieldReader<Possel>;
impl PosselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Possel> {
        match self.bits {
            0 => Some(Possel::Vss),
            16 => Some(Possel::Vrefdivavdd),
            17 => Some(Possel::Vrefdivavddlp),
            18 => Some(Possel::Vrefdiv1v25),
            19 => Some(Possel::Vrefdiv1v25lp),
            20 => Some(Possel::Vrefdiv2v5),
            21 => Some(Possel::Vrefdiv2v5lp),
            32 => Some(Possel::Vsense01div4),
            33 => Some(Possel::Vsense01div4lp),
            34 => Some(Possel::Vsense11div4),
            35 => Some(Possel::Vsense11div4lp),
            64 => Some(Possel::Vdacout0),
            65 => Some(Possel::Vdacout1),
            80 => Some(Possel::Extpa),
            81 => Some(Possel::Extpb),
            82 => Some(Possel::Extpc),
            83 => Some(Possel::Extpd),
            128 => Some(Possel::Pa0),
            129 => Some(Possel::Pa1),
            130 => Some(Possel::Pa2),
            131 => Some(Possel::Pa3),
            132 => Some(Possel::Pa4),
            133 => Some(Possel::Pa5),
            134 => Some(Possel::Pa6),
            135 => Some(Possel::Pa7),
            136 => Some(Possel::Pa8),
            137 => Some(Possel::Pa9),
            138 => Some(Possel::Pa10),
            139 => Some(Possel::Pa11),
            140 => Some(Possel::Pa12),
            141 => Some(Possel::Pa13),
            142 => Some(Possel::Pa14),
            143 => Some(Possel::Pa15),
            144 => Some(Possel::Pb0),
            145 => Some(Possel::Pb1),
            146 => Some(Possel::Pb2),
            147 => Some(Possel::Pb3),
            148 => Some(Possel::Pb4),
            149 => Some(Possel::Pb5),
            150 => Some(Possel::Pb6),
            151 => Some(Possel::Pb7),
            152 => Some(Possel::Pb8),
            153 => Some(Possel::Pb9),
            154 => Some(Possel::Pb10),
            155 => Some(Possel::Pb11),
            156 => Some(Possel::Pb12),
            157 => Some(Possel::Pb13),
            158 => Some(Possel::Pb14),
            159 => Some(Possel::Pb15),
            160 => Some(Possel::Pc0),
            161 => Some(Possel::Pc1),
            162 => Some(Possel::Pc2),
            163 => Some(Possel::Pc3),
            164 => Some(Possel::Pc4),
            165 => Some(Possel::Pc5),
            166 => Some(Possel::Pc6),
            167 => Some(Possel::Pc7),
            168 => Some(Possel::Pc8),
            169 => Some(Possel::Pc9),
            170 => Some(Possel::Pc10),
            171 => Some(Possel::Pc11),
            172 => Some(Possel::Pc12),
            173 => Some(Possel::Pc13),
            174 => Some(Possel::Pc14),
            175 => Some(Possel::Pc15),
            176 => Some(Possel::Pd0),
            177 => Some(Possel::Pd1),
            178 => Some(Possel::Pd2),
            179 => Some(Possel::Pd3),
            180 => Some(Possel::Pd4),
            181 => Some(Possel::Pd5),
            182 => Some(Possel::Pd6),
            183 => Some(Possel::Pd7),
            184 => Some(Possel::Pd8),
            185 => Some(Possel::Pd9),
            186 => Some(Possel::Pd10),
            187 => Some(Possel::Pd11),
            188 => Some(Possel::Pd12),
            189 => Some(Possel::Pd13),
            190 => Some(Possel::Pd14),
            191 => Some(Possel::Pd15),
            _ => None,
        }
    }
    #[doc = "VSS"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == Possel::Vss
    }
    #[doc = "Divided AVDD"]
    #[inline(always)]
    pub fn is_vrefdivavdd(&self) -> bool {
        *self == Possel::Vrefdivavdd
    }
    #[doc = "Low-Power Divided AVDD"]
    #[inline(always)]
    pub fn is_vrefdivavddlp(&self) -> bool {
        *self == Possel::Vrefdivavddlp
    }
    #[doc = "Divided 1V25 reference"]
    #[inline(always)]
    pub fn is_vrefdiv1v25(&self) -> bool {
        *self == Possel::Vrefdiv1v25
    }
    #[doc = "Low-power Divided 1V25 reference"]
    #[inline(always)]
    pub fn is_vrefdiv1v25lp(&self) -> bool {
        *self == Possel::Vrefdiv1v25lp
    }
    #[doc = "Divided 2V5 reference"]
    #[inline(always)]
    pub fn is_vrefdiv2v5(&self) -> bool {
        *self == Possel::Vrefdiv2v5
    }
    #[doc = "Low-power Divided 2V5 reference"]
    #[inline(always)]
    pub fn is_vrefdiv2v5lp(&self) -> bool {
        *self == Possel::Vrefdiv2v5lp
    }
    #[doc = "VSENSE0 divided by 4"]
    #[inline(always)]
    pub fn is_vsense01div4(&self) -> bool {
        *self == Possel::Vsense01div4
    }
    #[doc = "Low-power VSENSE0 divided by 4"]
    #[inline(always)]
    pub fn is_vsense01div4lp(&self) -> bool {
        *self == Possel::Vsense01div4lp
    }
    #[doc = "VSENSE1 divided by 4"]
    #[inline(always)]
    pub fn is_vsense11div4(&self) -> bool {
        *self == Possel::Vsense11div4
    }
    #[doc = "Low-power VSENSE1 divided by 4"]
    #[inline(always)]
    pub fn is_vsense11div4lp(&self) -> bool {
        *self == Possel::Vsense11div4lp
    }
    #[doc = "VDAC0 channel 0 output"]
    #[inline(always)]
    pub fn is_vdacout0(&self) -> bool {
        *self == Possel::Vdacout0
    }
    #[doc = "VDAC0 channel 1 output"]
    #[inline(always)]
    pub fn is_vdacout1(&self) -> bool {
        *self == Possel::Vdacout1
    }
    #[doc = "External interface, base is PA0."]
    #[inline(always)]
    pub fn is_extpa(&self) -> bool {
        *self == Possel::Extpa
    }
    #[doc = "External interface, base is PB0."]
    #[inline(always)]
    pub fn is_extpb(&self) -> bool {
        *self == Possel::Extpb
    }
    #[doc = "External interface, base is PC0."]
    #[inline(always)]
    pub fn is_extpc(&self) -> bool {
        *self == Possel::Extpc
    }
    #[doc = "External interface, base is PD0."]
    #[inline(always)]
    pub fn is_extpd(&self) -> bool {
        *self == Possel::Extpd
    }
    #[doc = "Port A, Pin0"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == Possel::Pa0
    }
    #[doc = "Port A, Pin1"]
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == Possel::Pa1
    }
    #[doc = "Port A, Pin2"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == Possel::Pa2
    }
    #[doc = "Port A, Pin3"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == Possel::Pa3
    }
    #[doc = "Port A, Pin4"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == Possel::Pa4
    }
    #[doc = "Port A, Pin5"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == Possel::Pa5
    }
    #[doc = "Port A, Pin6"]
    #[inline(always)]
    pub fn is_pa6(&self) -> bool {
        *self == Possel::Pa6
    }
    #[doc = "Port A, Pin7"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == Possel::Pa7
    }
    #[doc = "Port A, Pin8"]
    #[inline(always)]
    pub fn is_pa8(&self) -> bool {
        *self == Possel::Pa8
    }
    #[doc = "Port A, Pin9"]
    #[inline(always)]
    pub fn is_pa9(&self) -> bool {
        *self == Possel::Pa9
    }
    #[doc = "Port A, Pin10"]
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        *self == Possel::Pa10
    }
    #[doc = "Port A, Pin11"]
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        *self == Possel::Pa11
    }
    #[doc = "Port A, Pin12"]
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        *self == Possel::Pa12
    }
    #[doc = "Port A, Pin13"]
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        *self == Possel::Pa13
    }
    #[doc = "Port A, Pin14"]
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        *self == Possel::Pa14
    }
    #[doc = "Port A, Pin15"]
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == Possel::Pa15
    }
    #[doc = "Port B, Pin0"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == Possel::Pb0
    }
    #[doc = "Port B, Pin1"]
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == Possel::Pb1
    }
    #[doc = "Port B, Pin2"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == Possel::Pb2
    }
    #[doc = "Port B, Pin3"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == Possel::Pb3
    }
    #[doc = "Port B, Pin4"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == Possel::Pb4
    }
    #[doc = "Port B, Pin5"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        *self == Possel::Pb5
    }
    #[doc = "Port B, Pin6"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == Possel::Pb6
    }
    #[doc = "Port B, Pin7"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == Possel::Pb7
    }
    #[doc = "Port B, Pin8"]
    #[inline(always)]
    pub fn is_pb8(&self) -> bool {
        *self == Possel::Pb8
    }
    #[doc = "Port B, Pin9"]
    #[inline(always)]
    pub fn is_pb9(&self) -> bool {
        *self == Possel::Pb9
    }
    #[doc = "Port B, Pin10"]
    #[inline(always)]
    pub fn is_pb10(&self) -> bool {
        *self == Possel::Pb10
    }
    #[doc = "Port B, Pin11"]
    #[inline(always)]
    pub fn is_pb11(&self) -> bool {
        *self == Possel::Pb11
    }
    #[doc = "Port B, Pin12"]
    #[inline(always)]
    pub fn is_pb12(&self) -> bool {
        *self == Possel::Pb12
    }
    #[doc = "Port B, Pin13"]
    #[inline(always)]
    pub fn is_pb13(&self) -> bool {
        *self == Possel::Pb13
    }
    #[doc = "Port B, Pin14"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == Possel::Pb14
    }
    #[doc = "Port B, Pin15"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == Possel::Pb15
    }
    #[doc = "Port C, Pin0"]
    #[inline(always)]
    pub fn is_pc0(&self) -> bool {
        *self == Possel::Pc0
    }
    #[doc = "Port C, Pin1"]
    #[inline(always)]
    pub fn is_pc1(&self) -> bool {
        *self == Possel::Pc1
    }
    #[doc = "Port C, Pin2"]
    #[inline(always)]
    pub fn is_pc2(&self) -> bool {
        *self == Possel::Pc2
    }
    #[doc = "Port C, Pin3"]
    #[inline(always)]
    pub fn is_pc3(&self) -> bool {
        *self == Possel::Pc3
    }
    #[doc = "Port C, Pin4"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        *self == Possel::Pc4
    }
    #[doc = "Port C, Pin5"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == Possel::Pc5
    }
    #[doc = "Port C, Pin6"]
    #[inline(always)]
    pub fn is_pc6(&self) -> bool {
        *self == Possel::Pc6
    }
    #[doc = "Port C, Pin7"]
    #[inline(always)]
    pub fn is_pc7(&self) -> bool {
        *self == Possel::Pc7
    }
    #[doc = "Port C, Pin8"]
    #[inline(always)]
    pub fn is_pc8(&self) -> bool {
        *self == Possel::Pc8
    }
    #[doc = "Port C, Pin9"]
    #[inline(always)]
    pub fn is_pc9(&self) -> bool {
        *self == Possel::Pc9
    }
    #[doc = "Port C, Pin10"]
    #[inline(always)]
    pub fn is_pc10(&self) -> bool {
        *self == Possel::Pc10
    }
    #[doc = "Port C, Pin11"]
    #[inline(always)]
    pub fn is_pc11(&self) -> bool {
        *self == Possel::Pc11
    }
    #[doc = "Port C, Pin12"]
    #[inline(always)]
    pub fn is_pc12(&self) -> bool {
        *self == Possel::Pc12
    }
    #[doc = "Port C, Pin13"]
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        *self == Possel::Pc13
    }
    #[doc = "Port C, Pin14"]
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        *self == Possel::Pc14
    }
    #[doc = "Port C, Pin15"]
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        *self == Possel::Pc15
    }
    #[doc = "Port D, Pin0"]
    #[inline(always)]
    pub fn is_pd0(&self) -> bool {
        *self == Possel::Pd0
    }
    #[doc = "Port D, Pin1"]
    #[inline(always)]
    pub fn is_pd1(&self) -> bool {
        *self == Possel::Pd1
    }
    #[doc = "Port D, Pin2"]
    #[inline(always)]
    pub fn is_pd2(&self) -> bool {
        *self == Possel::Pd2
    }
    #[doc = "Port D, Pin3"]
    #[inline(always)]
    pub fn is_pd3(&self) -> bool {
        *self == Possel::Pd3
    }
    #[doc = "Port D, Pin4"]
    #[inline(always)]
    pub fn is_pd4(&self) -> bool {
        *self == Possel::Pd4
    }
    #[doc = "Port D, Pin5"]
    #[inline(always)]
    pub fn is_pd5(&self) -> bool {
        *self == Possel::Pd5
    }
    #[doc = "Port D, Pin6"]
    #[inline(always)]
    pub fn is_pd6(&self) -> bool {
        *self == Possel::Pd6
    }
    #[doc = "Port D, Pin7"]
    #[inline(always)]
    pub fn is_pd7(&self) -> bool {
        *self == Possel::Pd7
    }
    #[doc = "Port D, Pin8"]
    #[inline(always)]
    pub fn is_pd8(&self) -> bool {
        *self == Possel::Pd8
    }
    #[doc = "Port D, Pin9"]
    #[inline(always)]
    pub fn is_pd9(&self) -> bool {
        *self == Possel::Pd9
    }
    #[doc = "Port D, Pin10"]
    #[inline(always)]
    pub fn is_pd10(&self) -> bool {
        *self == Possel::Pd10
    }
    #[doc = "Port D, Pin11"]
    #[inline(always)]
    pub fn is_pd11(&self) -> bool {
        *self == Possel::Pd11
    }
    #[doc = "Port D, Pin12"]
    #[inline(always)]
    pub fn is_pd12(&self) -> bool {
        *self == Possel::Pd12
    }
    #[doc = "Port D, Pin13"]
    #[inline(always)]
    pub fn is_pd13(&self) -> bool {
        *self == Possel::Pd13
    }
    #[doc = "Port D, Pin14"]
    #[inline(always)]
    pub fn is_pd14(&self) -> bool {
        *self == Possel::Pd14
    }
    #[doc = "Port D, Pin15"]
    #[inline(always)]
    pub fn is_pd15(&self) -> bool {
        *self == Possel::Pd15
    }
}
#[doc = "Field `POSSEL` writer - Positive Input Select"]
pub type PosselW<'a, REG> = crate::FieldWriter<'a, REG, 8, Possel>;
impl<'a, REG> PosselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VSS"]
    #[inline(always)]
    pub fn vss(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Vss)
    }
    #[doc = "Divided AVDD"]
    #[inline(always)]
    pub fn vrefdivavdd(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Vrefdivavdd)
    }
    #[doc = "Low-Power Divided AVDD"]
    #[inline(always)]
    pub fn vrefdivavddlp(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Vrefdivavddlp)
    }
    #[doc = "Divided 1V25 reference"]
    #[inline(always)]
    pub fn vrefdiv1v25(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Vrefdiv1v25)
    }
    #[doc = "Low-power Divided 1V25 reference"]
    #[inline(always)]
    pub fn vrefdiv1v25lp(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Vrefdiv1v25lp)
    }
    #[doc = "Divided 2V5 reference"]
    #[inline(always)]
    pub fn vrefdiv2v5(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Vrefdiv2v5)
    }
    #[doc = "Low-power Divided 2V5 reference"]
    #[inline(always)]
    pub fn vrefdiv2v5lp(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Vrefdiv2v5lp)
    }
    #[doc = "VSENSE0 divided by 4"]
    #[inline(always)]
    pub fn vsense01div4(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Vsense01div4)
    }
    #[doc = "Low-power VSENSE0 divided by 4"]
    #[inline(always)]
    pub fn vsense01div4lp(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Vsense01div4lp)
    }
    #[doc = "VSENSE1 divided by 4"]
    #[inline(always)]
    pub fn vsense11div4(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Vsense11div4)
    }
    #[doc = "Low-power VSENSE1 divided by 4"]
    #[inline(always)]
    pub fn vsense11div4lp(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Vsense11div4lp)
    }
    #[doc = "VDAC0 channel 0 output"]
    #[inline(always)]
    pub fn vdacout0(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Vdacout0)
    }
    #[doc = "VDAC0 channel 1 output"]
    #[inline(always)]
    pub fn vdacout1(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Vdacout1)
    }
    #[doc = "External interface, base is PA0."]
    #[inline(always)]
    pub fn extpa(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Extpa)
    }
    #[doc = "External interface, base is PB0."]
    #[inline(always)]
    pub fn extpb(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Extpb)
    }
    #[doc = "External interface, base is PC0."]
    #[inline(always)]
    pub fn extpc(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Extpc)
    }
    #[doc = "External interface, base is PD0."]
    #[inline(always)]
    pub fn extpd(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Extpd)
    }
    #[doc = "Port A, Pin0"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa0)
    }
    #[doc = "Port A, Pin1"]
    #[inline(always)]
    pub fn pa1(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa1)
    }
    #[doc = "Port A, Pin2"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa2)
    }
    #[doc = "Port A, Pin3"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa3)
    }
    #[doc = "Port A, Pin4"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa4)
    }
    #[doc = "Port A, Pin5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa5)
    }
    #[doc = "Port A, Pin6"]
    #[inline(always)]
    pub fn pa6(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa6)
    }
    #[doc = "Port A, Pin7"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa7)
    }
    #[doc = "Port A, Pin8"]
    #[inline(always)]
    pub fn pa8(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa8)
    }
    #[doc = "Port A, Pin9"]
    #[inline(always)]
    pub fn pa9(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa9)
    }
    #[doc = "Port A, Pin10"]
    #[inline(always)]
    pub fn pa10(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa10)
    }
    #[doc = "Port A, Pin11"]
    #[inline(always)]
    pub fn pa11(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa11)
    }
    #[doc = "Port A, Pin12"]
    #[inline(always)]
    pub fn pa12(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa12)
    }
    #[doc = "Port A, Pin13"]
    #[inline(always)]
    pub fn pa13(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa13)
    }
    #[doc = "Port A, Pin14"]
    #[inline(always)]
    pub fn pa14(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa14)
    }
    #[doc = "Port A, Pin15"]
    #[inline(always)]
    pub fn pa15(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pa15)
    }
    #[doc = "Port B, Pin0"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb0)
    }
    #[doc = "Port B, Pin1"]
    #[inline(always)]
    pub fn pb1(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb1)
    }
    #[doc = "Port B, Pin2"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb2)
    }
    #[doc = "Port B, Pin3"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb3)
    }
    #[doc = "Port B, Pin4"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb4)
    }
    #[doc = "Port B, Pin5"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb5)
    }
    #[doc = "Port B, Pin6"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb6)
    }
    #[doc = "Port B, Pin7"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb7)
    }
    #[doc = "Port B, Pin8"]
    #[inline(always)]
    pub fn pb8(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb8)
    }
    #[doc = "Port B, Pin9"]
    #[inline(always)]
    pub fn pb9(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb9)
    }
    #[doc = "Port B, Pin10"]
    #[inline(always)]
    pub fn pb10(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb10)
    }
    #[doc = "Port B, Pin11"]
    #[inline(always)]
    pub fn pb11(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb11)
    }
    #[doc = "Port B, Pin12"]
    #[inline(always)]
    pub fn pb12(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb12)
    }
    #[doc = "Port B, Pin13"]
    #[inline(always)]
    pub fn pb13(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb13)
    }
    #[doc = "Port B, Pin14"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb14)
    }
    #[doc = "Port B, Pin15"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pb15)
    }
    #[doc = "Port C, Pin0"]
    #[inline(always)]
    pub fn pc0(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc0)
    }
    #[doc = "Port C, Pin1"]
    #[inline(always)]
    pub fn pc1(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc1)
    }
    #[doc = "Port C, Pin2"]
    #[inline(always)]
    pub fn pc2(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc2)
    }
    #[doc = "Port C, Pin3"]
    #[inline(always)]
    pub fn pc3(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc3)
    }
    #[doc = "Port C, Pin4"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc4)
    }
    #[doc = "Port C, Pin5"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc5)
    }
    #[doc = "Port C, Pin6"]
    #[inline(always)]
    pub fn pc6(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc6)
    }
    #[doc = "Port C, Pin7"]
    #[inline(always)]
    pub fn pc7(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc7)
    }
    #[doc = "Port C, Pin8"]
    #[inline(always)]
    pub fn pc8(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc8)
    }
    #[doc = "Port C, Pin9"]
    #[inline(always)]
    pub fn pc9(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc9)
    }
    #[doc = "Port C, Pin10"]
    #[inline(always)]
    pub fn pc10(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc10)
    }
    #[doc = "Port C, Pin11"]
    #[inline(always)]
    pub fn pc11(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc11)
    }
    #[doc = "Port C, Pin12"]
    #[inline(always)]
    pub fn pc12(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc12)
    }
    #[doc = "Port C, Pin13"]
    #[inline(always)]
    pub fn pc13(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc13)
    }
    #[doc = "Port C, Pin14"]
    #[inline(always)]
    pub fn pc14(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc14)
    }
    #[doc = "Port C, Pin15"]
    #[inline(always)]
    pub fn pc15(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pc15)
    }
    #[doc = "Port D, Pin0"]
    #[inline(always)]
    pub fn pd0(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd0)
    }
    #[doc = "Port D, Pin1"]
    #[inline(always)]
    pub fn pd1(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd1)
    }
    #[doc = "Port D, Pin2"]
    #[inline(always)]
    pub fn pd2(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd2)
    }
    #[doc = "Port D, Pin3"]
    #[inline(always)]
    pub fn pd3(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd3)
    }
    #[doc = "Port D, Pin4"]
    #[inline(always)]
    pub fn pd4(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd4)
    }
    #[doc = "Port D, Pin5"]
    #[inline(always)]
    pub fn pd5(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd5)
    }
    #[doc = "Port D, Pin6"]
    #[inline(always)]
    pub fn pd6(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd6)
    }
    #[doc = "Port D, Pin7"]
    #[inline(always)]
    pub fn pd7(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd7)
    }
    #[doc = "Port D, Pin8"]
    #[inline(always)]
    pub fn pd8(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd8)
    }
    #[doc = "Port D, Pin9"]
    #[inline(always)]
    pub fn pd9(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd9)
    }
    #[doc = "Port D, Pin10"]
    #[inline(always)]
    pub fn pd10(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd10)
    }
    #[doc = "Port D, Pin11"]
    #[inline(always)]
    pub fn pd11(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd11)
    }
    #[doc = "Port D, Pin12"]
    #[inline(always)]
    pub fn pd12(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd12)
    }
    #[doc = "Port D, Pin13"]
    #[inline(always)]
    pub fn pd13(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd13)
    }
    #[doc = "Port D, Pin14"]
    #[inline(always)]
    pub fn pd14(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd14)
    }
    #[doc = "Port D, Pin15"]
    #[inline(always)]
    pub fn pd15(self) -> &'a mut crate::W<REG> {
        self.variant(Possel::Pd15)
    }
}
#[doc = "Negative Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Negsel {
    #[doc = "0: VSS"]
    Vss = 0,
    #[doc = "16: Divided AVDD"]
    Vrefdivavdd = 16,
    #[doc = "17: Low-Power Divided AVDD"]
    Vrefdivavddlp = 17,
    #[doc = "18: Divided 1V25 reference"]
    Vrefdiv1v25 = 18,
    #[doc = "19: Low-power Divided 1V25 reference"]
    Vrefdiv1v25lp = 19,
    #[doc = "20: Divided 2V5 reference"]
    Vrefdiv2v5 = 20,
    #[doc = "21: Low-power Divided 2V5 reference"]
    Vrefdiv2v5lp = 21,
    #[doc = "32: VSENSE0 divided by 4"]
    Vsense01div4 = 32,
    #[doc = "33: Low-power VSENSE0 divided by 4"]
    Vsense01div4lp = 33,
    #[doc = "34: VSENSE1 divided by 4"]
    Vsense11div4 = 34,
    #[doc = "35: Low-power VSENSE1 divided by 4"]
    Vsense11div4lp = 35,
    #[doc = "48: Deprecated capacitive sensing feature, not recommended for new designs"]
    Capsense = 48,
    #[doc = "64: VDAC0 channel 0 output"]
    Vdacout0 = 64,
    #[doc = "65: VDAC0 channel 1 output"]
    Vdacout1 = 65,
    #[doc = "128: Port A, Pin0"]
    Pa0 = 128,
    #[doc = "129: Port A, Pin1"]
    Pa1 = 129,
    #[doc = "130: Port A, Pin2"]
    Pa2 = 130,
    #[doc = "131: Port A, Pin3"]
    Pa3 = 131,
    #[doc = "132: Port A, Pin4"]
    Pa4 = 132,
    #[doc = "133: Port A, Pin5"]
    Pa5 = 133,
    #[doc = "134: Port A, Pin6"]
    Pa6 = 134,
    #[doc = "135: Port A, Pin7"]
    Pa7 = 135,
    #[doc = "136: Port A, Pin8"]
    Pa8 = 136,
    #[doc = "137: Port A, Pin9"]
    Pa9 = 137,
    #[doc = "138: Port A, Pin10"]
    Pa10 = 138,
    #[doc = "139: Port A, Pin11"]
    Pa11 = 139,
    #[doc = "140: Port A, Pin12"]
    Pa12 = 140,
    #[doc = "141: Port A, Pin13"]
    Pa13 = 141,
    #[doc = "142: Port A, Pin14"]
    Pa14 = 142,
    #[doc = "143: Port A, Pin15"]
    Pa15 = 143,
    #[doc = "144: Port B, Pin0"]
    Pb0 = 144,
    #[doc = "145: Port B, Pin1"]
    Pb1 = 145,
    #[doc = "146: Port B, Pin2"]
    Pb2 = 146,
    #[doc = "147: Port B, Pin3"]
    Pb3 = 147,
    #[doc = "148: Port B, Pin4"]
    Pb4 = 148,
    #[doc = "149: Port B, Pin5"]
    Pb5 = 149,
    #[doc = "150: Port B, Pin6"]
    Pb6 = 150,
    #[doc = "151: Port B, Pin7"]
    Pb7 = 151,
    #[doc = "152: Port B, Pin8"]
    Pb8 = 152,
    #[doc = "153: Port B, Pin9"]
    Pb9 = 153,
    #[doc = "154: Port B, Pin10"]
    Pb10 = 154,
    #[doc = "155: Port B, Pin11"]
    Pb11 = 155,
    #[doc = "156: Port B, Pin12"]
    Pb12 = 156,
    #[doc = "157: Port B, Pin13"]
    Pb13 = 157,
    #[doc = "158: Port B, Pin14"]
    Pb14 = 158,
    #[doc = "159: Port B, Pin15"]
    Pb15 = 159,
    #[doc = "160: Port C, Pin0"]
    Pc0 = 160,
    #[doc = "161: Port C, Pin1"]
    Pc1 = 161,
    #[doc = "162: Port C, Pin2"]
    Pc2 = 162,
    #[doc = "163: Port C, Pin3"]
    Pc3 = 163,
    #[doc = "164: Port C, Pin4"]
    Pc4 = 164,
    #[doc = "165: Port C, Pin5"]
    Pc5 = 165,
    #[doc = "166: Port C, Pin6"]
    Pc6 = 166,
    #[doc = "167: Port C, Pin7"]
    Pc7 = 167,
    #[doc = "168: Port C, Pin8"]
    Pc8 = 168,
    #[doc = "169: Port C, Pin9"]
    Pc9 = 169,
    #[doc = "170: Port C, Pin10"]
    Pc10 = 170,
    #[doc = "171: Port C, Pin11"]
    Pc11 = 171,
    #[doc = "172: Port C, Pin12"]
    Pc12 = 172,
    #[doc = "173: Port C, Pin13"]
    Pc13 = 173,
    #[doc = "174: Port C, Pin14"]
    Pc14 = 174,
    #[doc = "175: Port C, Pin15"]
    Pc15 = 175,
    #[doc = "176: Port D, Pin0"]
    Pd0 = 176,
    #[doc = "177: Port D, Pin1"]
    Pd1 = 177,
    #[doc = "178: Port D, Pin2"]
    Pd2 = 178,
    #[doc = "179: Port D, Pin3"]
    Pd3 = 179,
    #[doc = "180: Port D, Pin4"]
    Pd4 = 180,
    #[doc = "181: Port D, Pin5"]
    Pd5 = 181,
    #[doc = "182: Port D, Pin6"]
    Pd6 = 182,
    #[doc = "183: Port D, Pin7"]
    Pd7 = 183,
    #[doc = "184: Port D, Pin8"]
    Pd8 = 184,
    #[doc = "185: Port D, Pin9"]
    Pd9 = 185,
    #[doc = "186: Port D, Pin10"]
    Pd10 = 186,
    #[doc = "187: Port D, Pin11"]
    Pd11 = 187,
    #[doc = "188: Port D, Pin12"]
    Pd12 = 188,
    #[doc = "189: Port D, Pin13"]
    Pd13 = 189,
    #[doc = "190: Port D, Pin14"]
    Pd14 = 190,
    #[doc = "191: Port D, Pin15"]
    Pd15 = 191,
}
impl From<Negsel> for u8 {
    #[inline(always)]
    fn from(variant: Negsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Negsel {
    type Ux = u8;
}
impl crate::IsEnum for Negsel {}
#[doc = "Field `NEGSEL` reader - Negative Input Select"]
pub type NegselR = crate::FieldReader<Negsel>;
impl NegselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Negsel> {
        match self.bits {
            0 => Some(Negsel::Vss),
            16 => Some(Negsel::Vrefdivavdd),
            17 => Some(Negsel::Vrefdivavddlp),
            18 => Some(Negsel::Vrefdiv1v25),
            19 => Some(Negsel::Vrefdiv1v25lp),
            20 => Some(Negsel::Vrefdiv2v5),
            21 => Some(Negsel::Vrefdiv2v5lp),
            32 => Some(Negsel::Vsense01div4),
            33 => Some(Negsel::Vsense01div4lp),
            34 => Some(Negsel::Vsense11div4),
            35 => Some(Negsel::Vsense11div4lp),
            48 => Some(Negsel::Capsense),
            64 => Some(Negsel::Vdacout0),
            65 => Some(Negsel::Vdacout1),
            128 => Some(Negsel::Pa0),
            129 => Some(Negsel::Pa1),
            130 => Some(Negsel::Pa2),
            131 => Some(Negsel::Pa3),
            132 => Some(Negsel::Pa4),
            133 => Some(Negsel::Pa5),
            134 => Some(Negsel::Pa6),
            135 => Some(Negsel::Pa7),
            136 => Some(Negsel::Pa8),
            137 => Some(Negsel::Pa9),
            138 => Some(Negsel::Pa10),
            139 => Some(Negsel::Pa11),
            140 => Some(Negsel::Pa12),
            141 => Some(Negsel::Pa13),
            142 => Some(Negsel::Pa14),
            143 => Some(Negsel::Pa15),
            144 => Some(Negsel::Pb0),
            145 => Some(Negsel::Pb1),
            146 => Some(Negsel::Pb2),
            147 => Some(Negsel::Pb3),
            148 => Some(Negsel::Pb4),
            149 => Some(Negsel::Pb5),
            150 => Some(Negsel::Pb6),
            151 => Some(Negsel::Pb7),
            152 => Some(Negsel::Pb8),
            153 => Some(Negsel::Pb9),
            154 => Some(Negsel::Pb10),
            155 => Some(Negsel::Pb11),
            156 => Some(Negsel::Pb12),
            157 => Some(Negsel::Pb13),
            158 => Some(Negsel::Pb14),
            159 => Some(Negsel::Pb15),
            160 => Some(Negsel::Pc0),
            161 => Some(Negsel::Pc1),
            162 => Some(Negsel::Pc2),
            163 => Some(Negsel::Pc3),
            164 => Some(Negsel::Pc4),
            165 => Some(Negsel::Pc5),
            166 => Some(Negsel::Pc6),
            167 => Some(Negsel::Pc7),
            168 => Some(Negsel::Pc8),
            169 => Some(Negsel::Pc9),
            170 => Some(Negsel::Pc10),
            171 => Some(Negsel::Pc11),
            172 => Some(Negsel::Pc12),
            173 => Some(Negsel::Pc13),
            174 => Some(Negsel::Pc14),
            175 => Some(Negsel::Pc15),
            176 => Some(Negsel::Pd0),
            177 => Some(Negsel::Pd1),
            178 => Some(Negsel::Pd2),
            179 => Some(Negsel::Pd3),
            180 => Some(Negsel::Pd4),
            181 => Some(Negsel::Pd5),
            182 => Some(Negsel::Pd6),
            183 => Some(Negsel::Pd7),
            184 => Some(Negsel::Pd8),
            185 => Some(Negsel::Pd9),
            186 => Some(Negsel::Pd10),
            187 => Some(Negsel::Pd11),
            188 => Some(Negsel::Pd12),
            189 => Some(Negsel::Pd13),
            190 => Some(Negsel::Pd14),
            191 => Some(Negsel::Pd15),
            _ => None,
        }
    }
    #[doc = "VSS"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == Negsel::Vss
    }
    #[doc = "Divided AVDD"]
    #[inline(always)]
    pub fn is_vrefdivavdd(&self) -> bool {
        *self == Negsel::Vrefdivavdd
    }
    #[doc = "Low-Power Divided AVDD"]
    #[inline(always)]
    pub fn is_vrefdivavddlp(&self) -> bool {
        *self == Negsel::Vrefdivavddlp
    }
    #[doc = "Divided 1V25 reference"]
    #[inline(always)]
    pub fn is_vrefdiv1v25(&self) -> bool {
        *self == Negsel::Vrefdiv1v25
    }
    #[doc = "Low-power Divided 1V25 reference"]
    #[inline(always)]
    pub fn is_vrefdiv1v25lp(&self) -> bool {
        *self == Negsel::Vrefdiv1v25lp
    }
    #[doc = "Divided 2V5 reference"]
    #[inline(always)]
    pub fn is_vrefdiv2v5(&self) -> bool {
        *self == Negsel::Vrefdiv2v5
    }
    #[doc = "Low-power Divided 2V5 reference"]
    #[inline(always)]
    pub fn is_vrefdiv2v5lp(&self) -> bool {
        *self == Negsel::Vrefdiv2v5lp
    }
    #[doc = "VSENSE0 divided by 4"]
    #[inline(always)]
    pub fn is_vsense01div4(&self) -> bool {
        *self == Negsel::Vsense01div4
    }
    #[doc = "Low-power VSENSE0 divided by 4"]
    #[inline(always)]
    pub fn is_vsense01div4lp(&self) -> bool {
        *self == Negsel::Vsense01div4lp
    }
    #[doc = "VSENSE1 divided by 4"]
    #[inline(always)]
    pub fn is_vsense11div4(&self) -> bool {
        *self == Negsel::Vsense11div4
    }
    #[doc = "Low-power VSENSE1 divided by 4"]
    #[inline(always)]
    pub fn is_vsense11div4lp(&self) -> bool {
        *self == Negsel::Vsense11div4lp
    }
    #[doc = "Deprecated capacitive sensing feature, not recommended for new designs"]
    #[inline(always)]
    pub fn is_capsense(&self) -> bool {
        *self == Negsel::Capsense
    }
    #[doc = "VDAC0 channel 0 output"]
    #[inline(always)]
    pub fn is_vdacout0(&self) -> bool {
        *self == Negsel::Vdacout0
    }
    #[doc = "VDAC0 channel 1 output"]
    #[inline(always)]
    pub fn is_vdacout1(&self) -> bool {
        *self == Negsel::Vdacout1
    }
    #[doc = "Port A, Pin0"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == Negsel::Pa0
    }
    #[doc = "Port A, Pin1"]
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == Negsel::Pa1
    }
    #[doc = "Port A, Pin2"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == Negsel::Pa2
    }
    #[doc = "Port A, Pin3"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == Negsel::Pa3
    }
    #[doc = "Port A, Pin4"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == Negsel::Pa4
    }
    #[doc = "Port A, Pin5"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == Negsel::Pa5
    }
    #[doc = "Port A, Pin6"]
    #[inline(always)]
    pub fn is_pa6(&self) -> bool {
        *self == Negsel::Pa6
    }
    #[doc = "Port A, Pin7"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == Negsel::Pa7
    }
    #[doc = "Port A, Pin8"]
    #[inline(always)]
    pub fn is_pa8(&self) -> bool {
        *self == Negsel::Pa8
    }
    #[doc = "Port A, Pin9"]
    #[inline(always)]
    pub fn is_pa9(&self) -> bool {
        *self == Negsel::Pa9
    }
    #[doc = "Port A, Pin10"]
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        *self == Negsel::Pa10
    }
    #[doc = "Port A, Pin11"]
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        *self == Negsel::Pa11
    }
    #[doc = "Port A, Pin12"]
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        *self == Negsel::Pa12
    }
    #[doc = "Port A, Pin13"]
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        *self == Negsel::Pa13
    }
    #[doc = "Port A, Pin14"]
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        *self == Negsel::Pa14
    }
    #[doc = "Port A, Pin15"]
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == Negsel::Pa15
    }
    #[doc = "Port B, Pin0"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == Negsel::Pb0
    }
    #[doc = "Port B, Pin1"]
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == Negsel::Pb1
    }
    #[doc = "Port B, Pin2"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == Negsel::Pb2
    }
    #[doc = "Port B, Pin3"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == Negsel::Pb3
    }
    #[doc = "Port B, Pin4"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == Negsel::Pb4
    }
    #[doc = "Port B, Pin5"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        *self == Negsel::Pb5
    }
    #[doc = "Port B, Pin6"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == Negsel::Pb6
    }
    #[doc = "Port B, Pin7"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == Negsel::Pb7
    }
    #[doc = "Port B, Pin8"]
    #[inline(always)]
    pub fn is_pb8(&self) -> bool {
        *self == Negsel::Pb8
    }
    #[doc = "Port B, Pin9"]
    #[inline(always)]
    pub fn is_pb9(&self) -> bool {
        *self == Negsel::Pb9
    }
    #[doc = "Port B, Pin10"]
    #[inline(always)]
    pub fn is_pb10(&self) -> bool {
        *self == Negsel::Pb10
    }
    #[doc = "Port B, Pin11"]
    #[inline(always)]
    pub fn is_pb11(&self) -> bool {
        *self == Negsel::Pb11
    }
    #[doc = "Port B, Pin12"]
    #[inline(always)]
    pub fn is_pb12(&self) -> bool {
        *self == Negsel::Pb12
    }
    #[doc = "Port B, Pin13"]
    #[inline(always)]
    pub fn is_pb13(&self) -> bool {
        *self == Negsel::Pb13
    }
    #[doc = "Port B, Pin14"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == Negsel::Pb14
    }
    #[doc = "Port B, Pin15"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == Negsel::Pb15
    }
    #[doc = "Port C, Pin0"]
    #[inline(always)]
    pub fn is_pc0(&self) -> bool {
        *self == Negsel::Pc0
    }
    #[doc = "Port C, Pin1"]
    #[inline(always)]
    pub fn is_pc1(&self) -> bool {
        *self == Negsel::Pc1
    }
    #[doc = "Port C, Pin2"]
    #[inline(always)]
    pub fn is_pc2(&self) -> bool {
        *self == Negsel::Pc2
    }
    #[doc = "Port C, Pin3"]
    #[inline(always)]
    pub fn is_pc3(&self) -> bool {
        *self == Negsel::Pc3
    }
    #[doc = "Port C, Pin4"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        *self == Negsel::Pc4
    }
    #[doc = "Port C, Pin5"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == Negsel::Pc5
    }
    #[doc = "Port C, Pin6"]
    #[inline(always)]
    pub fn is_pc6(&self) -> bool {
        *self == Negsel::Pc6
    }
    #[doc = "Port C, Pin7"]
    #[inline(always)]
    pub fn is_pc7(&self) -> bool {
        *self == Negsel::Pc7
    }
    #[doc = "Port C, Pin8"]
    #[inline(always)]
    pub fn is_pc8(&self) -> bool {
        *self == Negsel::Pc8
    }
    #[doc = "Port C, Pin9"]
    #[inline(always)]
    pub fn is_pc9(&self) -> bool {
        *self == Negsel::Pc9
    }
    #[doc = "Port C, Pin10"]
    #[inline(always)]
    pub fn is_pc10(&self) -> bool {
        *self == Negsel::Pc10
    }
    #[doc = "Port C, Pin11"]
    #[inline(always)]
    pub fn is_pc11(&self) -> bool {
        *self == Negsel::Pc11
    }
    #[doc = "Port C, Pin12"]
    #[inline(always)]
    pub fn is_pc12(&self) -> bool {
        *self == Negsel::Pc12
    }
    #[doc = "Port C, Pin13"]
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        *self == Negsel::Pc13
    }
    #[doc = "Port C, Pin14"]
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        *self == Negsel::Pc14
    }
    #[doc = "Port C, Pin15"]
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        *self == Negsel::Pc15
    }
    #[doc = "Port D, Pin0"]
    #[inline(always)]
    pub fn is_pd0(&self) -> bool {
        *self == Negsel::Pd0
    }
    #[doc = "Port D, Pin1"]
    #[inline(always)]
    pub fn is_pd1(&self) -> bool {
        *self == Negsel::Pd1
    }
    #[doc = "Port D, Pin2"]
    #[inline(always)]
    pub fn is_pd2(&self) -> bool {
        *self == Negsel::Pd2
    }
    #[doc = "Port D, Pin3"]
    #[inline(always)]
    pub fn is_pd3(&self) -> bool {
        *self == Negsel::Pd3
    }
    #[doc = "Port D, Pin4"]
    #[inline(always)]
    pub fn is_pd4(&self) -> bool {
        *self == Negsel::Pd4
    }
    #[doc = "Port D, Pin5"]
    #[inline(always)]
    pub fn is_pd5(&self) -> bool {
        *self == Negsel::Pd5
    }
    #[doc = "Port D, Pin6"]
    #[inline(always)]
    pub fn is_pd6(&self) -> bool {
        *self == Negsel::Pd6
    }
    #[doc = "Port D, Pin7"]
    #[inline(always)]
    pub fn is_pd7(&self) -> bool {
        *self == Negsel::Pd7
    }
    #[doc = "Port D, Pin8"]
    #[inline(always)]
    pub fn is_pd8(&self) -> bool {
        *self == Negsel::Pd8
    }
    #[doc = "Port D, Pin9"]
    #[inline(always)]
    pub fn is_pd9(&self) -> bool {
        *self == Negsel::Pd9
    }
    #[doc = "Port D, Pin10"]
    #[inline(always)]
    pub fn is_pd10(&self) -> bool {
        *self == Negsel::Pd10
    }
    #[doc = "Port D, Pin11"]
    #[inline(always)]
    pub fn is_pd11(&self) -> bool {
        *self == Negsel::Pd11
    }
    #[doc = "Port D, Pin12"]
    #[inline(always)]
    pub fn is_pd12(&self) -> bool {
        *self == Negsel::Pd12
    }
    #[doc = "Port D, Pin13"]
    #[inline(always)]
    pub fn is_pd13(&self) -> bool {
        *self == Negsel::Pd13
    }
    #[doc = "Port D, Pin14"]
    #[inline(always)]
    pub fn is_pd14(&self) -> bool {
        *self == Negsel::Pd14
    }
    #[doc = "Port D, Pin15"]
    #[inline(always)]
    pub fn is_pd15(&self) -> bool {
        *self == Negsel::Pd15
    }
}
#[doc = "Field `NEGSEL` writer - Negative Input Select"]
pub type NegselW<'a, REG> = crate::FieldWriter<'a, REG, 8, Negsel>;
impl<'a, REG> NegselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VSS"]
    #[inline(always)]
    pub fn vss(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Vss)
    }
    #[doc = "Divided AVDD"]
    #[inline(always)]
    pub fn vrefdivavdd(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Vrefdivavdd)
    }
    #[doc = "Low-Power Divided AVDD"]
    #[inline(always)]
    pub fn vrefdivavddlp(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Vrefdivavddlp)
    }
    #[doc = "Divided 1V25 reference"]
    #[inline(always)]
    pub fn vrefdiv1v25(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Vrefdiv1v25)
    }
    #[doc = "Low-power Divided 1V25 reference"]
    #[inline(always)]
    pub fn vrefdiv1v25lp(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Vrefdiv1v25lp)
    }
    #[doc = "Divided 2V5 reference"]
    #[inline(always)]
    pub fn vrefdiv2v5(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Vrefdiv2v5)
    }
    #[doc = "Low-power Divided 2V5 reference"]
    #[inline(always)]
    pub fn vrefdiv2v5lp(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Vrefdiv2v5lp)
    }
    #[doc = "VSENSE0 divided by 4"]
    #[inline(always)]
    pub fn vsense01div4(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Vsense01div4)
    }
    #[doc = "Low-power VSENSE0 divided by 4"]
    #[inline(always)]
    pub fn vsense01div4lp(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Vsense01div4lp)
    }
    #[doc = "VSENSE1 divided by 4"]
    #[inline(always)]
    pub fn vsense11div4(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Vsense11div4)
    }
    #[doc = "Low-power VSENSE1 divided by 4"]
    #[inline(always)]
    pub fn vsense11div4lp(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Vsense11div4lp)
    }
    #[doc = "Deprecated capacitive sensing feature, not recommended for new designs"]
    #[inline(always)]
    pub fn capsense(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Capsense)
    }
    #[doc = "VDAC0 channel 0 output"]
    #[inline(always)]
    pub fn vdacout0(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Vdacout0)
    }
    #[doc = "VDAC0 channel 1 output"]
    #[inline(always)]
    pub fn vdacout1(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Vdacout1)
    }
    #[doc = "Port A, Pin0"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa0)
    }
    #[doc = "Port A, Pin1"]
    #[inline(always)]
    pub fn pa1(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa1)
    }
    #[doc = "Port A, Pin2"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa2)
    }
    #[doc = "Port A, Pin3"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa3)
    }
    #[doc = "Port A, Pin4"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa4)
    }
    #[doc = "Port A, Pin5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa5)
    }
    #[doc = "Port A, Pin6"]
    #[inline(always)]
    pub fn pa6(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa6)
    }
    #[doc = "Port A, Pin7"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa7)
    }
    #[doc = "Port A, Pin8"]
    #[inline(always)]
    pub fn pa8(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa8)
    }
    #[doc = "Port A, Pin9"]
    #[inline(always)]
    pub fn pa9(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa9)
    }
    #[doc = "Port A, Pin10"]
    #[inline(always)]
    pub fn pa10(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa10)
    }
    #[doc = "Port A, Pin11"]
    #[inline(always)]
    pub fn pa11(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa11)
    }
    #[doc = "Port A, Pin12"]
    #[inline(always)]
    pub fn pa12(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa12)
    }
    #[doc = "Port A, Pin13"]
    #[inline(always)]
    pub fn pa13(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa13)
    }
    #[doc = "Port A, Pin14"]
    #[inline(always)]
    pub fn pa14(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa14)
    }
    #[doc = "Port A, Pin15"]
    #[inline(always)]
    pub fn pa15(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pa15)
    }
    #[doc = "Port B, Pin0"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb0)
    }
    #[doc = "Port B, Pin1"]
    #[inline(always)]
    pub fn pb1(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb1)
    }
    #[doc = "Port B, Pin2"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb2)
    }
    #[doc = "Port B, Pin3"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb3)
    }
    #[doc = "Port B, Pin4"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb4)
    }
    #[doc = "Port B, Pin5"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb5)
    }
    #[doc = "Port B, Pin6"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb6)
    }
    #[doc = "Port B, Pin7"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb7)
    }
    #[doc = "Port B, Pin8"]
    #[inline(always)]
    pub fn pb8(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb8)
    }
    #[doc = "Port B, Pin9"]
    #[inline(always)]
    pub fn pb9(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb9)
    }
    #[doc = "Port B, Pin10"]
    #[inline(always)]
    pub fn pb10(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb10)
    }
    #[doc = "Port B, Pin11"]
    #[inline(always)]
    pub fn pb11(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb11)
    }
    #[doc = "Port B, Pin12"]
    #[inline(always)]
    pub fn pb12(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb12)
    }
    #[doc = "Port B, Pin13"]
    #[inline(always)]
    pub fn pb13(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb13)
    }
    #[doc = "Port B, Pin14"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb14)
    }
    #[doc = "Port B, Pin15"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pb15)
    }
    #[doc = "Port C, Pin0"]
    #[inline(always)]
    pub fn pc0(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc0)
    }
    #[doc = "Port C, Pin1"]
    #[inline(always)]
    pub fn pc1(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc1)
    }
    #[doc = "Port C, Pin2"]
    #[inline(always)]
    pub fn pc2(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc2)
    }
    #[doc = "Port C, Pin3"]
    #[inline(always)]
    pub fn pc3(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc3)
    }
    #[doc = "Port C, Pin4"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc4)
    }
    #[doc = "Port C, Pin5"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc5)
    }
    #[doc = "Port C, Pin6"]
    #[inline(always)]
    pub fn pc6(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc6)
    }
    #[doc = "Port C, Pin7"]
    #[inline(always)]
    pub fn pc7(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc7)
    }
    #[doc = "Port C, Pin8"]
    #[inline(always)]
    pub fn pc8(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc8)
    }
    #[doc = "Port C, Pin9"]
    #[inline(always)]
    pub fn pc9(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc9)
    }
    #[doc = "Port C, Pin10"]
    #[inline(always)]
    pub fn pc10(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc10)
    }
    #[doc = "Port C, Pin11"]
    #[inline(always)]
    pub fn pc11(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc11)
    }
    #[doc = "Port C, Pin12"]
    #[inline(always)]
    pub fn pc12(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc12)
    }
    #[doc = "Port C, Pin13"]
    #[inline(always)]
    pub fn pc13(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc13)
    }
    #[doc = "Port C, Pin14"]
    #[inline(always)]
    pub fn pc14(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc14)
    }
    #[doc = "Port C, Pin15"]
    #[inline(always)]
    pub fn pc15(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pc15)
    }
    #[doc = "Port D, Pin0"]
    #[inline(always)]
    pub fn pd0(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd0)
    }
    #[doc = "Port D, Pin1"]
    #[inline(always)]
    pub fn pd1(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd1)
    }
    #[doc = "Port D, Pin2"]
    #[inline(always)]
    pub fn pd2(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd2)
    }
    #[doc = "Port D, Pin3"]
    #[inline(always)]
    pub fn pd3(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd3)
    }
    #[doc = "Port D, Pin4"]
    #[inline(always)]
    pub fn pd4(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd4)
    }
    #[doc = "Port D, Pin5"]
    #[inline(always)]
    pub fn pd5(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd5)
    }
    #[doc = "Port D, Pin6"]
    #[inline(always)]
    pub fn pd6(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd6)
    }
    #[doc = "Port D, Pin7"]
    #[inline(always)]
    pub fn pd7(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd7)
    }
    #[doc = "Port D, Pin8"]
    #[inline(always)]
    pub fn pd8(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd8)
    }
    #[doc = "Port D, Pin9"]
    #[inline(always)]
    pub fn pd9(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd9)
    }
    #[doc = "Port D, Pin10"]
    #[inline(always)]
    pub fn pd10(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd10)
    }
    #[doc = "Port D, Pin11"]
    #[inline(always)]
    pub fn pd11(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd11)
    }
    #[doc = "Port D, Pin12"]
    #[inline(always)]
    pub fn pd12(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd12)
    }
    #[doc = "Port D, Pin13"]
    #[inline(always)]
    pub fn pd13(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd13)
    }
    #[doc = "Port D, Pin14"]
    #[inline(always)]
    pub fn pd14(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd14)
    }
    #[doc = "Port D, Pin15"]
    #[inline(always)]
    pub fn pd15(self) -> &'a mut crate::W<REG> {
        self.variant(Negsel::Pd15)
    }
}
#[doc = "Field `VREFDIV` reader - VREF division"]
pub type VrefdivR = crate::FieldReader;
#[doc = "Field `VREFDIV` writer - VREF division"]
pub type VrefdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Capacitive Sense Mode Internal Resistor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csressel {
    #[doc = "0: Internal capacitive sense resistor value 0"]
    Res0 = 0,
    #[doc = "1: Internal capacitive sense resistor value 1"]
    Res1 = 1,
    #[doc = "2: Internal capacitive sense resistor value 2"]
    Res2 = 2,
    #[doc = "3: Internal capacitive sense resistor value 3"]
    Res3 = 3,
    #[doc = "4: Internal capacitive sense resistor value 4"]
    Res4 = 4,
    #[doc = "5: Internal capacitive sense resistor value 5"]
    Res5 = 5,
    #[doc = "6: Internal capacitive sense resistor value 6"]
    Res6 = 6,
}
impl From<Csressel> for u8 {
    #[inline(always)]
    fn from(variant: Csressel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csressel {
    type Ux = u8;
}
impl crate::IsEnum for Csressel {}
#[doc = "Field `CSRESSEL` reader - Capacitive Sense Mode Internal Resistor"]
pub type CsresselR = crate::FieldReader<Csressel>;
impl CsresselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csressel> {
        match self.bits {
            0 => Some(Csressel::Res0),
            1 => Some(Csressel::Res1),
            2 => Some(Csressel::Res2),
            3 => Some(Csressel::Res3),
            4 => Some(Csressel::Res4),
            5 => Some(Csressel::Res5),
            6 => Some(Csressel::Res6),
            _ => None,
        }
    }
    #[doc = "Internal capacitive sense resistor value 0"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == Csressel::Res0
    }
    #[doc = "Internal capacitive sense resistor value 1"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == Csressel::Res1
    }
    #[doc = "Internal capacitive sense resistor value 2"]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == Csressel::Res2
    }
    #[doc = "Internal capacitive sense resistor value 3"]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == Csressel::Res3
    }
    #[doc = "Internal capacitive sense resistor value 4"]
    #[inline(always)]
    pub fn is_res4(&self) -> bool {
        *self == Csressel::Res4
    }
    #[doc = "Internal capacitive sense resistor value 5"]
    #[inline(always)]
    pub fn is_res5(&self) -> bool {
        *self == Csressel::Res5
    }
    #[doc = "Internal capacitive sense resistor value 6"]
    #[inline(always)]
    pub fn is_res6(&self) -> bool {
        *self == Csressel::Res6
    }
}
#[doc = "Field `CSRESSEL` writer - Capacitive Sense Mode Internal Resistor"]
pub type CsresselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Csressel>;
impl<'a, REG> CsresselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal capacitive sense resistor value 0"]
    #[inline(always)]
    pub fn res0(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res0)
    }
    #[doc = "Internal capacitive sense resistor value 1"]
    #[inline(always)]
    pub fn res1(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res1)
    }
    #[doc = "Internal capacitive sense resistor value 2"]
    #[inline(always)]
    pub fn res2(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res2)
    }
    #[doc = "Internal capacitive sense resistor value 3"]
    #[inline(always)]
    pub fn res3(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res3)
    }
    #[doc = "Internal capacitive sense resistor value 4"]
    #[inline(always)]
    pub fn res4(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res4)
    }
    #[doc = "Internal capacitive sense resistor value 5"]
    #[inline(always)]
    pub fn res5(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res5)
    }
    #[doc = "Internal capacitive sense resistor value 6"]
    #[inline(always)]
    pub fn res6(self) -> &'a mut crate::W<REG> {
        self.variant(Csressel::Res6)
    }
}
impl R {
    #[doc = "Bits 0:7 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&self) -> PosselR {
        PosselR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&self) -> NegselR {
        NegselR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - VREF division"]
    #[inline(always)]
    pub fn vrefdiv(&self) -> VrefdivR {
        VrefdivR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 28:30 - Capacitive Sense Mode Internal Resistor"]
    #[inline(always)]
    pub fn csressel(&self) -> CsresselR {
        CsresselR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&mut self) -> PosselW<InputctrlSpec> {
        PosselW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&mut self) -> NegselW<InputctrlSpec> {
        NegselW::new(self, 8)
    }
    #[doc = "Bits 16:21 - VREF division"]
    #[inline(always)]
    pub fn vrefdiv(&mut self) -> VrefdivW<InputctrlSpec> {
        VrefdivW::new(self, 16)
    }
    #[doc = "Bits 28:30 - Capacitive Sense Mode Internal Resistor"]
    #[inline(always)]
    pub fn csressel(&mut self) -> CsresselW<InputctrlSpec> {
        CsresselW::new(self, 28)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`inputctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputctrlSpec;
impl crate::RegisterSpec for InputctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inputctrl::R`](R) reader structure"]
impl crate::Readable for InputctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`inputctrl::W`](W) writer structure"]
impl crate::Writable for InputctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUTCTRL to value 0"]
impl crate::Resettable for InputctrlSpec {}
