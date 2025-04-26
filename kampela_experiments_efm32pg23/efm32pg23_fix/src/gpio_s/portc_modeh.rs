#[doc = "Register `PORTC_MODEH` reader"]
pub type R = crate::R<PortcModehSpec>;
#[doc = "Register `PORTC_MODEH` writer"]
pub type W = crate::W<PortcModehSpec>;
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
}
impl W {
    #[doc = "Bits 0:3 - MODE n"]
    #[inline(always)]
    pub fn mode0(&mut self) -> Mode0W<PortcModehSpec> {
        Mode0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - MODE n"]
    #[inline(always)]
    pub fn mode1(&mut self) -> Mode1W<PortcModehSpec> {
        Mode1W::new(self, 4)
    }
}
#[doc = "mode high\n\nYou can [`read`](crate::Reg::read) this register and get [`portc_modeh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portc_modeh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortcModehSpec;
impl crate::RegisterSpec for PortcModehSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`portc_modeh::R`](R) reader structure"]
impl crate::Readable for PortcModehSpec {}
#[doc = "`write(|w| ..)` method takes [`portc_modeh::W`](W) writer structure"]
impl crate::Writable for PortcModehSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORTC_MODEH to value 0"]
impl crate::Resettable for PortcModehSpec {}
