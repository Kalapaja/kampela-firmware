#[doc = "Register `PORTD_MODEL` reader"]
pub type R = crate::R<PortdModelSpec>;
#[doc = "Register `PORTD_MODEL` writer"]
pub type W = crate::W<PortdModelSpec>;
#[doc = "MODE n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode0 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set."]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction."]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output."]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control."]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output."]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down."]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output."]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter."]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup."]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control."]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 15,
}
impl From<Mode0> for u8 {
    #[inline(always)]
    fn from(variant: Mode0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode0 {
    type Ux = u8;
}
impl crate::IsEnum for Mode0 {}
#[doc = "Field `MODE0` reader - MODE n"]
pub type Mode0R = crate::FieldReader<Mode0>;
impl Mode0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode0 {
        match self.bits {
            0 => Mode0::Disabled,
            1 => Mode0::Input,
            2 => Mode0::Inputpull,
            3 => Mode0::Inputpullfilter,
            4 => Mode0::Pushpull,
            5 => Mode0::Pushpullalt,
            6 => Mode0::Wiredor,
            7 => Mode0::Wiredorpulldown,
            8 => Mode0::Wiredand,
            9 => Mode0::Wiredandfilter,
            10 => Mode0::Wiredandpullup,
            11 => Mode0::Wiredandpullupfilter,
            12 => Mode0::Wiredandalt,
            13 => Mode0::Wiredandaltfilter,
            14 => Mode0::Wiredandaltpullup,
            15 => Mode0::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode0::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mode0::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction."]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == Mode0::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == Mode0::Inputpullfilter
    }
    #[doc = "Push-pull output."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Mode0::Pushpull
    }
    #[doc = "Push-pull using alternate control."]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == Mode0::Pushpullalt
    }
    #[doc = "Wired-or output."]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == Mode0::Wiredor
    }
    #[doc = "Wired-or output with pull-down."]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == Mode0::Wiredorpulldown
    }
    #[doc = "Open-drain output."]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == Mode0::Wiredand
    }
    #[doc = "Open-drain output with filter."]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == Mode0::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup."]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == Mode0::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup."]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == Mode0::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control."]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == Mode0::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter."]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == Mode0::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup."]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == Mode0::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == Mode0::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE0` writer - MODE n"]
pub type Mode0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode0, crate::Safe>;
impl<'a, REG> Mode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Inputpullfilter)
    }
    #[doc = "Push-pull output."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Pushpull)
    }
    #[doc = "Push-pull using alternate control."]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Pushpullalt)
    }
    #[doc = "Wired-or output."]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Wiredor)
    }
    #[doc = "Wired-or output with pull-down."]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Wiredorpulldown)
    }
    #[doc = "Open-drain output."]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Wiredand)
    }
    #[doc = "Open-drain output with filter."]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup."]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup."]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control."]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter."]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup."]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode0::Wiredandaltpullupfilter)
    }
}
#[doc = "MODE n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode1 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set."]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction."]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output."]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control."]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output."]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down."]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output."]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter."]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup."]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control."]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 15,
}
impl From<Mode1> for u8 {
    #[inline(always)]
    fn from(variant: Mode1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode1 {
    type Ux = u8;
}
impl crate::IsEnum for Mode1 {}
#[doc = "Field `MODE1` reader - MODE n"]
pub type Mode1R = crate::FieldReader<Mode1>;
impl Mode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode1 {
        match self.bits {
            0 => Mode1::Disabled,
            1 => Mode1::Input,
            2 => Mode1::Inputpull,
            3 => Mode1::Inputpullfilter,
            4 => Mode1::Pushpull,
            5 => Mode1::Pushpullalt,
            6 => Mode1::Wiredor,
            7 => Mode1::Wiredorpulldown,
            8 => Mode1::Wiredand,
            9 => Mode1::Wiredandfilter,
            10 => Mode1::Wiredandpullup,
            11 => Mode1::Wiredandpullupfilter,
            12 => Mode1::Wiredandalt,
            13 => Mode1::Wiredandaltfilter,
            14 => Mode1::Wiredandaltpullup,
            15 => Mode1::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode1::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mode1::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction."]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == Mode1::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == Mode1::Inputpullfilter
    }
    #[doc = "Push-pull output."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Mode1::Pushpull
    }
    #[doc = "Push-pull using alternate control."]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == Mode1::Pushpullalt
    }
    #[doc = "Wired-or output."]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == Mode1::Wiredor
    }
    #[doc = "Wired-or output with pull-down."]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == Mode1::Wiredorpulldown
    }
    #[doc = "Open-drain output."]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == Mode1::Wiredand
    }
    #[doc = "Open-drain output with filter."]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == Mode1::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup."]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == Mode1::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup."]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == Mode1::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control."]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == Mode1::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter."]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == Mode1::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup."]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == Mode1::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == Mode1::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE1` writer - MODE n"]
pub type Mode1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode1, crate::Safe>;
impl<'a, REG> Mode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Inputpullfilter)
    }
    #[doc = "Push-pull output."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Pushpull)
    }
    #[doc = "Push-pull using alternate control."]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Pushpullalt)
    }
    #[doc = "Wired-or output."]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Wiredor)
    }
    #[doc = "Wired-or output with pull-down."]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Wiredorpulldown)
    }
    #[doc = "Open-drain output."]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Wiredand)
    }
    #[doc = "Open-drain output with filter."]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup."]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup."]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control."]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter."]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup."]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode1::Wiredandaltpullupfilter)
    }
}
#[doc = "MODE n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode2 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set."]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction."]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output."]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control."]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output."]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down."]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output."]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter."]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup."]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control."]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 15,
}
impl From<Mode2> for u8 {
    #[inline(always)]
    fn from(variant: Mode2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode2 {
    type Ux = u8;
}
impl crate::IsEnum for Mode2 {}
#[doc = "Field `MODE2` reader - MODE n"]
pub type Mode2R = crate::FieldReader<Mode2>;
impl Mode2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode2 {
        match self.bits {
            0 => Mode2::Disabled,
            1 => Mode2::Input,
            2 => Mode2::Inputpull,
            3 => Mode2::Inputpullfilter,
            4 => Mode2::Pushpull,
            5 => Mode2::Pushpullalt,
            6 => Mode2::Wiredor,
            7 => Mode2::Wiredorpulldown,
            8 => Mode2::Wiredand,
            9 => Mode2::Wiredandfilter,
            10 => Mode2::Wiredandpullup,
            11 => Mode2::Wiredandpullupfilter,
            12 => Mode2::Wiredandalt,
            13 => Mode2::Wiredandaltfilter,
            14 => Mode2::Wiredandaltpullup,
            15 => Mode2::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode2::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mode2::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction."]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == Mode2::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == Mode2::Inputpullfilter
    }
    #[doc = "Push-pull output."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Mode2::Pushpull
    }
    #[doc = "Push-pull using alternate control."]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == Mode2::Pushpullalt
    }
    #[doc = "Wired-or output."]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == Mode2::Wiredor
    }
    #[doc = "Wired-or output with pull-down."]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == Mode2::Wiredorpulldown
    }
    #[doc = "Open-drain output."]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == Mode2::Wiredand
    }
    #[doc = "Open-drain output with filter."]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == Mode2::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup."]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == Mode2::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup."]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == Mode2::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control."]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == Mode2::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter."]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == Mode2::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup."]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == Mode2::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == Mode2::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE2` writer - MODE n"]
pub type Mode2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode2, crate::Safe>;
impl<'a, REG> Mode2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Inputpullfilter)
    }
    #[doc = "Push-pull output."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Pushpull)
    }
    #[doc = "Push-pull using alternate control."]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Pushpullalt)
    }
    #[doc = "Wired-or output."]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Wiredor)
    }
    #[doc = "Wired-or output with pull-down."]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Wiredorpulldown)
    }
    #[doc = "Open-drain output."]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Wiredand)
    }
    #[doc = "Open-drain output with filter."]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup."]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup."]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control."]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter."]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup."]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode2::Wiredandaltpullupfilter)
    }
}
#[doc = "MODE n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode3 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set."]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction."]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output."]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control."]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output."]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down."]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output."]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter."]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup."]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control."]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 15,
}
impl From<Mode3> for u8 {
    #[inline(always)]
    fn from(variant: Mode3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode3 {
    type Ux = u8;
}
impl crate::IsEnum for Mode3 {}
#[doc = "Field `MODE3` reader - MODE n"]
pub type Mode3R = crate::FieldReader<Mode3>;
impl Mode3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode3 {
        match self.bits {
            0 => Mode3::Disabled,
            1 => Mode3::Input,
            2 => Mode3::Inputpull,
            3 => Mode3::Inputpullfilter,
            4 => Mode3::Pushpull,
            5 => Mode3::Pushpullalt,
            6 => Mode3::Wiredor,
            7 => Mode3::Wiredorpulldown,
            8 => Mode3::Wiredand,
            9 => Mode3::Wiredandfilter,
            10 => Mode3::Wiredandpullup,
            11 => Mode3::Wiredandpullupfilter,
            12 => Mode3::Wiredandalt,
            13 => Mode3::Wiredandaltfilter,
            14 => Mode3::Wiredandaltpullup,
            15 => Mode3::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode3::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mode3::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction."]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == Mode3::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == Mode3::Inputpullfilter
    }
    #[doc = "Push-pull output."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Mode3::Pushpull
    }
    #[doc = "Push-pull using alternate control."]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == Mode3::Pushpullalt
    }
    #[doc = "Wired-or output."]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == Mode3::Wiredor
    }
    #[doc = "Wired-or output with pull-down."]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == Mode3::Wiredorpulldown
    }
    #[doc = "Open-drain output."]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == Mode3::Wiredand
    }
    #[doc = "Open-drain output with filter."]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == Mode3::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup."]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == Mode3::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup."]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == Mode3::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control."]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == Mode3::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter."]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == Mode3::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup."]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == Mode3::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == Mode3::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE3` writer - MODE n"]
pub type Mode3W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode3, crate::Safe>;
impl<'a, REG> Mode3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Inputpullfilter)
    }
    #[doc = "Push-pull output."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Pushpull)
    }
    #[doc = "Push-pull using alternate control."]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Pushpullalt)
    }
    #[doc = "Wired-or output."]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Wiredor)
    }
    #[doc = "Wired-or output with pull-down."]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Wiredorpulldown)
    }
    #[doc = "Open-drain output."]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Wiredand)
    }
    #[doc = "Open-drain output with filter."]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup."]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup."]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control."]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter."]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup."]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Wiredandaltpullupfilter)
    }
}
#[doc = "MODE n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode4 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set."]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction."]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output."]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control."]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output."]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down."]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output."]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter."]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup."]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control."]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 15,
}
impl From<Mode4> for u8 {
    #[inline(always)]
    fn from(variant: Mode4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode4 {
    type Ux = u8;
}
impl crate::IsEnum for Mode4 {}
#[doc = "Field `MODE4` reader - MODE n"]
pub type Mode4R = crate::FieldReader<Mode4>;
impl Mode4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode4 {
        match self.bits {
            0 => Mode4::Disabled,
            1 => Mode4::Input,
            2 => Mode4::Inputpull,
            3 => Mode4::Inputpullfilter,
            4 => Mode4::Pushpull,
            5 => Mode4::Pushpullalt,
            6 => Mode4::Wiredor,
            7 => Mode4::Wiredorpulldown,
            8 => Mode4::Wiredand,
            9 => Mode4::Wiredandfilter,
            10 => Mode4::Wiredandpullup,
            11 => Mode4::Wiredandpullupfilter,
            12 => Mode4::Wiredandalt,
            13 => Mode4::Wiredandaltfilter,
            14 => Mode4::Wiredandaltpullup,
            15 => Mode4::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode4::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mode4::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction."]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == Mode4::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == Mode4::Inputpullfilter
    }
    #[doc = "Push-pull output."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Mode4::Pushpull
    }
    #[doc = "Push-pull using alternate control."]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == Mode4::Pushpullalt
    }
    #[doc = "Wired-or output."]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == Mode4::Wiredor
    }
    #[doc = "Wired-or output with pull-down."]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == Mode4::Wiredorpulldown
    }
    #[doc = "Open-drain output."]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == Mode4::Wiredand
    }
    #[doc = "Open-drain output with filter."]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == Mode4::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup."]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == Mode4::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup."]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == Mode4::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control."]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == Mode4::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter."]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == Mode4::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup."]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == Mode4::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == Mode4::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE4` writer - MODE n"]
pub type Mode4W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode4, crate::Safe>;
impl<'a, REG> Mode4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Inputpullfilter)
    }
    #[doc = "Push-pull output."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Pushpull)
    }
    #[doc = "Push-pull using alternate control."]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Pushpullalt)
    }
    #[doc = "Wired-or output."]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Wiredor)
    }
    #[doc = "Wired-or output with pull-down."]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Wiredorpulldown)
    }
    #[doc = "Open-drain output."]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Wiredand)
    }
    #[doc = "Open-drain output with filter."]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup."]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup."]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control."]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter."]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup."]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode4::Wiredandaltpullupfilter)
    }
}
#[doc = "MODE n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode5 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set."]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction."]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction."]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output."]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control."]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output."]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down."]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output."]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter."]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup."]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup."]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control."]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter."]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup."]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup."]
    Wiredandaltpullupfilter = 15,
}
impl From<Mode5> for u8 {
    #[inline(always)]
    fn from(variant: Mode5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode5 {
    type Ux = u8;
}
impl crate::IsEnum for Mode5 {}
#[doc = "Field `MODE5` reader - MODE n"]
pub type Mode5R = crate::FieldReader<Mode5>;
impl Mode5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode5 {
        match self.bits {
            0 => Mode5::Disabled,
            1 => Mode5::Input,
            2 => Mode5::Inputpull,
            3 => Mode5::Inputpullfilter,
            4 => Mode5::Pushpull,
            5 => Mode5::Pushpullalt,
            6 => Mode5::Wiredor,
            7 => Mode5::Wiredorpulldown,
            8 => Mode5::Wiredand,
            9 => Mode5::Wiredandfilter,
            10 => Mode5::Wiredandpullup,
            11 => Mode5::Wiredandpullupfilter,
            12 => Mode5::Wiredandalt,
            13 => Mode5::Wiredandaltfilter,
            14 => Mode5::Wiredandaltpullup,
            15 => Mode5::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mode5::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mode5::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction."]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == Mode5::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == Mode5::Inputpullfilter
    }
    #[doc = "Push-pull output."]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == Mode5::Pushpull
    }
    #[doc = "Push-pull using alternate control."]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == Mode5::Pushpullalt
    }
    #[doc = "Wired-or output."]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == Mode5::Wiredor
    }
    #[doc = "Wired-or output with pull-down."]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == Mode5::Wiredorpulldown
    }
    #[doc = "Open-drain output."]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == Mode5::Wiredand
    }
    #[doc = "Open-drain output with filter."]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == Mode5::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup."]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == Mode5::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup."]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == Mode5::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control."]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == Mode5::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter."]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == Mode5::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup."]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == Mode5::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == Mode5::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE5` writer - MODE n"]
pub type Mode5W<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode5, crate::Safe>;
impl<'a, REG> Mode5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction."]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Inputpullfilter)
    }
    #[doc = "Push-pull output."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Pushpull)
    }
    #[doc = "Push-pull using alternate control."]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Pushpullalt)
    }
    #[doc = "Wired-or output."]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Wiredor)
    }
    #[doc = "Wired-or output with pull-down."]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Wiredorpulldown)
    }
    #[doc = "Open-drain output."]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Wiredand)
    }
    #[doc = "Open-drain output with filter."]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup."]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup."]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control."]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter."]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup."]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup."]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(Mode5::Wiredandaltpullupfilter)
    }
}
impl R {
    #[doc = "Bits 0:3 - MODE n"]
    #[inline(always)]
    pub fn mode0(&self) -> Mode0R {
        Mode0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - MODE n"]
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - MODE n"]
    #[inline(always)]
    pub fn mode2(&self) -> Mode2R {
        Mode2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - MODE n"]
    #[inline(always)]
    pub fn mode3(&self) -> Mode3R {
        Mode3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - MODE n"]
    #[inline(always)]
    pub fn mode4(&self) -> Mode4R {
        Mode4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - MODE n"]
    #[inline(always)]
    pub fn mode5(&self) -> Mode5R {
        Mode5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - MODE n"]
    #[inline(always)]
    pub fn mode0(&mut self) -> Mode0W<PortdModelSpec> {
        Mode0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - MODE n"]
    #[inline(always)]
    pub fn mode1(&mut self) -> Mode1W<PortdModelSpec> {
        Mode1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - MODE n"]
    #[inline(always)]
    pub fn mode2(&mut self) -> Mode2W<PortdModelSpec> {
        Mode2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - MODE n"]
    #[inline(always)]
    pub fn mode3(&mut self) -> Mode3W<PortdModelSpec> {
        Mode3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - MODE n"]
    #[inline(always)]
    pub fn mode4(&mut self) -> Mode4W<PortdModelSpec> {
        Mode4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - MODE n"]
    #[inline(always)]
    pub fn mode5(&mut self) -> Mode5W<PortdModelSpec> {
        Mode5W::new(self, 20)
    }
}
#[doc = "mode low\n\nYou can [`read`](crate::Reg::read) this register and get [`portd_model::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portd_model::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortdModelSpec;
impl crate::RegisterSpec for PortdModelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`portd_model::R`](R) reader structure"]
impl crate::Readable for PortdModelSpec {}
#[doc = "`write(|w| ..)` method takes [`portd_model::W`](W) writer structure"]
impl crate::Writable for PortdModelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORTD_MODEL to value 0"]
impl crate::Resettable for PortdModelSpec {}
