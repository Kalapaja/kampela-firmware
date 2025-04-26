#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    _reserved1: [u8; 0x2c],
    porta_ctrl: PortaCtrl,
    porta_model: PortaModel,
    _reserved3: [u8; 0x04],
    porta_modeh: PortaModeh,
    porta_dout: PortaDout,
    porta_din: PortaDin,
    _reserved6: [u8; 0x18],
    portb_ctrl: PortbCtrl,
    portb_model: PortbModel,
    _reserved8: [u8; 0x08],
    portb_dout: PortbDout,
    portb_din: PortbDin,
    _reserved10: [u8; 0x18],
    portc_ctrl: PortcCtrl,
    portc_model: PortcModel,
    _reserved12: [u8; 0x04],
    portc_modeh: PortcModeh,
    portc_dout: PortcDout,
    portc_din: PortcDin,
    _reserved15: [u8; 0x18],
    portd_ctrl: PortdCtrl,
    portd_model: PortdModel,
    _reserved17: [u8; 0x08],
    portd_dout: PortdDout,
    portd_din: PortdDin,
    _reserved19: [u8; 0x0228],
    lock: Lock,
    _reserved20: [u8; 0x0c],
    gpiolockstatus: Gpiolockstatus,
    _reserved21: [u8; 0x0c],
    abusalloc: Abusalloc,
    bbusalloc: Bbusalloc,
    cdbusalloc: Cdbusalloc,
    _reserved24: [u8; 0xd4],
    extipsell: Extipsell,
    extipselh: Extipselh,
    extipinsell: Extipinsell,
    extipinselh: Extipinselh,
    extirise: Extirise,
    extifall: Extifall,
    _reserved30: [u8; 0x08],
    if_: If,
    ien: Ien,
    _reserved32: [u8; 0x04],
    em4wuen: Em4wuen,
    em4wupol: Em4wupol,
    _reserved34: [u8; 0x0c],
    dbgroutepen: Dbgroutepen,
    traceroutepen: Traceroutepen,
    _reserved36: [u8; 0x18],
    lcdseg: Lcdseg,
    _reserved37: [u8; 0x0c],
    lcdcom: Lcdcom,
    _reserved38: [u8; 0x0c],
    acmp0_routeen: Acmp0Routeen,
    acmp0_acmpoutroute: Acmp0Acmpoutroute,
    _reserved40: [u8; 0x04],
    acmp1_routeen: Acmp1Routeen,
    acmp1_acmpoutroute: Acmp1Acmpoutroute,
    _reserved42: [u8; 0x04],
    cmu_routeen: CmuRouteen,
    cmu_clkin0route: CmuClkin0route,
    cmu_clkout0route: CmuClkout0route,
    cmu_clkout1route: CmuClkout1route,
    cmu_clkout2route: CmuClkout2route,
    _reserved47: [u8; 0x18],
    eusart0_routeen: Eusart0Routeen,
    eusart0_csroute: Eusart0Csroute,
    eusart0_ctsroute: Eusart0Ctsroute,
    eusart0_rtsroute: Eusart0Rtsroute,
    eusart0_rxroute: Eusart0Rxroute,
    eusart0_sclkroute: Eusart0Sclkroute,
    eusart0_txroute: Eusart0Txroute,
    _reserved54: [u8; 0x04],
    eusart1_routeen: Eusart1Routeen,
    eusart1_csroute: Eusart1Csroute,
    eusart1_ctsroute: Eusart1Ctsroute,
    eusart1_rtsroute: Eusart1Rtsroute,
    eusart1_rxroute: Eusart1Rxroute,
    eusart1_sclkroute: Eusart1Sclkroute,
    eusart1_txroute: Eusart1Txroute,
    _reserved61: [u8; 0x04],
    eusart2_routeen: Eusart2Routeen,
    eusart2_csroute: Eusart2Csroute,
    eusart2_ctsroute: Eusart2Ctsroute,
    eusart2_rtsroute: Eusart2Rtsroute,
    eusart2_rxroute: Eusart2Rxroute,
    eusart2_sclkroute: Eusart2Sclkroute,
    eusart2_txroute: Eusart2Txroute,
    _reserved68: [u8; 0x18],
    i2c0_routeen: I2c0Routeen,
    i2c0_sclroute: I2c0Sclroute,
    i2c0_sdaroute: I2c0Sdaroute,
    _reserved71: [u8; 0x04],
    i2c1_routeen: I2c1Routeen,
    i2c1_sclroute: I2c1Sclroute,
    i2c1_sdaroute: I2c1Sdaroute,
    _reserved74: [u8; 0x04],
    keyscan_routeen: KeyscanRouteen,
    keyscan_colout0route: KeyscanColout0route,
    keyscan_colout1route: KeyscanColout1route,
    keyscan_colout2route: KeyscanColout2route,
    keyscan_colout3route: KeyscanColout3route,
    keyscan_colout4route: KeyscanColout4route,
    keyscan_colout5route: KeyscanColout5route,
    keyscan_colout6route: KeyscanColout6route,
    keyscan_colout7route: KeyscanColout7route,
    keyscan_rowsense0route: KeyscanRowsense0route,
    keyscan_rowsense1route: KeyscanRowsense1route,
    keyscan_rowsense2route: KeyscanRowsense2route,
    keyscan_rowsense3route: KeyscanRowsense3route,
    keyscan_rowsense4route: KeyscanRowsense4route,
    keyscan_rowsense5route: KeyscanRowsense5route,
    _reserved89: [u8; 0x04],
    lesense_routeen: LesenseRouteen,
    lesense_ch0outroute: LesenseCh0outroute,
    lesense_ch1outroute: LesenseCh1outroute,
    lesense_ch2outroute: LesenseCh2outroute,
    lesense_ch3outroute: LesenseCh3outroute,
    lesense_ch4outroute: LesenseCh4outroute,
    lesense_ch5outroute: LesenseCh5outroute,
    lesense_ch6outroute: LesenseCh6outroute,
    lesense_ch7outroute: LesenseCh7outroute,
    lesense_ch8outroute: LesenseCh8outroute,
    lesense_ch9outroute: LesenseCh9outroute,
    lesense_ch10outroute: LesenseCh10outroute,
    lesense_ch11outroute: LesenseCh11outroute,
    lesense_ch12outroute: LesenseCh12outroute,
    lesense_ch13outroute: LesenseCh13outroute,
    lesense_ch14outroute: LesenseCh14outroute,
    lesense_ch15outroute: LesenseCh15outroute,
    _reserved106: [u8; 0x04],
    letimer_routeen: LetimerRouteen,
    letimer_out0route: LetimerOut0route,
    letimer_out1route: LetimerOut1route,
    _reserved109: [u8; 0x50],
    pcnt0_s0inroute: Pcnt0S0inroute,
    pcnt0_s1inroute: Pcnt0S1inroute,
    _reserved111: [u8; 0x04],
    prs0_routeen: Prs0Routeen,
    prs0_asynch0route: Prs0Asynch0route,
    prs0_asynch1route: Prs0Asynch1route,
    prs0_asynch2route: Prs0Asynch2route,
    prs0_asynch3route: Prs0Asynch3route,
    prs0_asynch4route: Prs0Asynch4route,
    prs0_asynch5route: Prs0Asynch5route,
    prs0_asynch6route: Prs0Asynch6route,
    prs0_asynch7route: Prs0Asynch7route,
    prs0_asynch8route: Prs0Asynch8route,
    prs0_asynch9route: Prs0Asynch9route,
    prs0_asynch10route: Prs0Asynch10route,
    prs0_asynch11route: Prs0Asynch11route,
    prs0_synch0route: Prs0Synch0route,
    prs0_synch1route: Prs0Synch1route,
    prs0_synch2route: Prs0Synch2route,
    prs0_synch3route: Prs0Synch3route,
    _reserved128: [u8; 0x64],
    syxo0_bufoutreqinasyncroute: Syxo0Bufoutreqinasyncroute,
    _reserved129: [u8; 0x04],
    timer0_routeen: Timer0Routeen,
    timer0_cc0route: Timer0Cc0route,
    timer0_cc1route: Timer0Cc1route,
    timer0_cc2route: Timer0Cc2route,
    timer0_cdti0route: Timer0Cdti0route,
    timer0_cdti1route: Timer0Cdti1route,
    timer0_cdti2route: Timer0Cdti2route,
    _reserved136: [u8; 0x04],
    timer1_routeen: Timer1Routeen,
    timer1_cc0route: Timer1Cc0route,
    timer1_cc1route: Timer1Cc1route,
    timer1_cc2route: Timer1Cc2route,
    timer1_cdti0route: Timer1Cdti0route,
    timer1_cdti1route: Timer1Cdti1route,
    timer1_cdti2route: Timer1Cdti2route,
    _reserved143: [u8; 0x04],
    timer2_routeen: Timer2Routeen,
    timer2_cc0route: Timer2Cc0route,
    timer2_cc1route: Timer2Cc1route,
    timer2_cc2route: Timer2Cc2route,
    timer2_cdti0route: Timer2Cdti0route,
    timer2_cdti1route: Timer2Cdti1route,
    timer2_cdti2route: Timer2Cdti2route,
    _reserved150: [u8; 0x04],
    timer3_routeen: Timer3Routeen,
    timer3_cc0route: Timer3Cc0route,
    timer3_cc1route: Timer3Cc1route,
    timer3_cc2route: Timer3Cc2route,
    timer3_cdti0route: Timer3Cdti0route,
    timer3_cdti1route: Timer3Cdti1route,
    timer3_cdti2route: Timer3Cdti2route,
    _reserved157: [u8; 0x04],
    timer4_routeen: Timer4Routeen,
    timer4_cc0route: Timer4Cc0route,
    timer4_cc1route: Timer4Cc1route,
    timer4_cc2route: Timer4Cc2route,
    timer4_cdti0route: Timer4Cdti0route,
    timer4_cdti1route: Timer4Cdti1route,
    timer4_cdti2route: Timer4Cdti2route,
    _reserved164: [u8; 0x04],
    usart0_routeen: Usart0Routeen,
    usart0_csroute: Usart0Csroute,
    usart0_ctsroute: Usart0Ctsroute,
    usart0_rtsroute: Usart0Rtsroute,
    usart0_rxroute: Usart0Rxroute,
    usart0_clkroute: Usart0Clkroute,
    usart0_txroute: Usart0Txroute,
    _reserved171: [u8; 0x0c6c],
    if_set: IfSet,
    _reserved172: [u8; 0x0ffc],
    if_clr: IfClr,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x30 - Port control"]
    #[inline(always)]
    pub const fn porta_ctrl(&self) -> &PortaCtrl {
        &self.porta_ctrl
    }
    #[doc = "0x34 - mode low"]
    #[inline(always)]
    pub const fn porta_model(&self) -> &PortaModel {
        &self.porta_model
    }
    #[doc = "0x3c - mode high"]
    #[inline(always)]
    pub const fn porta_modeh(&self) -> &PortaModeh {
        &self.porta_modeh
    }
    #[doc = "0x40 - data out"]
    #[inline(always)]
    pub const fn porta_dout(&self) -> &PortaDout {
        &self.porta_dout
    }
    #[doc = "0x44 - data in"]
    #[inline(always)]
    pub const fn porta_din(&self) -> &PortaDin {
        &self.porta_din
    }
    #[doc = "0x60 - Port control"]
    #[inline(always)]
    pub const fn portb_ctrl(&self) -> &PortbCtrl {
        &self.portb_ctrl
    }
    #[doc = "0x64 - mode low"]
    #[inline(always)]
    pub const fn portb_model(&self) -> &PortbModel {
        &self.portb_model
    }
    #[doc = "0x70 - data out"]
    #[inline(always)]
    pub const fn portb_dout(&self) -> &PortbDout {
        &self.portb_dout
    }
    #[doc = "0x74 - data in"]
    #[inline(always)]
    pub const fn portb_din(&self) -> &PortbDin {
        &self.portb_din
    }
    #[doc = "0x90 - Port control"]
    #[inline(always)]
    pub const fn portc_ctrl(&self) -> &PortcCtrl {
        &self.portc_ctrl
    }
    #[doc = "0x94 - mode low"]
    #[inline(always)]
    pub const fn portc_model(&self) -> &PortcModel {
        &self.portc_model
    }
    #[doc = "0x9c - mode high"]
    #[inline(always)]
    pub const fn portc_modeh(&self) -> &PortcModeh {
        &self.portc_modeh
    }
    #[doc = "0xa0 - data out"]
    #[inline(always)]
    pub const fn portc_dout(&self) -> &PortcDout {
        &self.portc_dout
    }
    #[doc = "0xa4 - data in"]
    #[inline(always)]
    pub const fn portc_din(&self) -> &PortcDin {
        &self.portc_din
    }
    #[doc = "0xc0 - Port control"]
    #[inline(always)]
    pub const fn portd_ctrl(&self) -> &PortdCtrl {
        &self.portd_ctrl
    }
    #[doc = "0xc4 - mode low"]
    #[inline(always)]
    pub const fn portd_model(&self) -> &PortdModel {
        &self.portd_model
    }
    #[doc = "0xd0 - data out"]
    #[inline(always)]
    pub const fn portd_dout(&self) -> &PortdDout {
        &self.portd_dout
    }
    #[doc = "0xd4 - data in"]
    #[inline(always)]
    pub const fn portd_din(&self) -> &PortdDin {
        &self.portd_din
    }
    #[doc = "0x300 - No Description"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x310 - No Description"]
    #[inline(always)]
    pub const fn gpiolockstatus(&self) -> &Gpiolockstatus {
        &self.gpiolockstatus
    }
    #[doc = "0x320 - A Bus allocation"]
    #[inline(always)]
    pub const fn abusalloc(&self) -> &Abusalloc {
        &self.abusalloc
    }
    #[doc = "0x324 - B Bus allocation"]
    #[inline(always)]
    pub const fn bbusalloc(&self) -> &Bbusalloc {
        &self.bbusalloc
    }
    #[doc = "0x328 - CD Bus allocation"]
    #[inline(always)]
    pub const fn cdbusalloc(&self) -> &Cdbusalloc {
        &self.cdbusalloc
    }
    #[doc = "0x400 - External Interrupt Port Select Low"]
    #[inline(always)]
    pub const fn extipsell(&self) -> &Extipsell {
        &self.extipsell
    }
    #[doc = "0x404 - External interrupt Port Select High"]
    #[inline(always)]
    pub const fn extipselh(&self) -> &Extipselh {
        &self.extipselh
    }
    #[doc = "0x408 - External Interrupt Pin Select Low"]
    #[inline(always)]
    pub const fn extipinsell(&self) -> &Extipinsell {
        &self.extipinsell
    }
    #[doc = "0x40c - External Interrupt Pin Select High"]
    #[inline(always)]
    pub const fn extipinselh(&self) -> &Extipinselh {
        &self.extipinselh
    }
    #[doc = "0x410 - External Interrupt Rising Edge Trigger"]
    #[inline(always)]
    pub const fn extirise(&self) -> &Extirise {
        &self.extirise
    }
    #[doc = "0x414 - External Interrupt Falling Edge Trigger"]
    #[inline(always)]
    pub const fn extifall(&self) -> &Extifall {
        &self.extifall
    }
    #[doc = "0x420 - Interrupt Flag"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x424 - Interrupt Enable"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x42c - No Description"]
    #[inline(always)]
    pub const fn em4wuen(&self) -> &Em4wuen {
        &self.em4wuen
    }
    #[doc = "0x430 - No Description"]
    #[inline(always)]
    pub const fn em4wupol(&self) -> &Em4wupol {
        &self.em4wupol
    }
    #[doc = "0x440 - No Description"]
    #[inline(always)]
    pub const fn dbgroutepen(&self) -> &Dbgroutepen {
        &self.dbgroutepen
    }
    #[doc = "0x444 - No Description"]
    #[inline(always)]
    pub const fn traceroutepen(&self) -> &Traceroutepen {
        &self.traceroutepen
    }
    #[doc = "0x460 - LCD Segment Enable"]
    #[inline(always)]
    pub const fn lcdseg(&self) -> &Lcdseg {
        &self.lcdseg
    }
    #[doc = "0x470 - LCD Common Enable"]
    #[inline(always)]
    pub const fn lcdcom(&self) -> &Lcdcom {
        &self.lcdcom
    }
    #[doc = "0x480 - ACMP0 pin enable"]
    #[inline(always)]
    pub const fn acmp0_routeen(&self) -> &Acmp0Routeen {
        &self.acmp0_routeen
    }
    #[doc = "0x484 - ACMPOUT port/pin select"]
    #[inline(always)]
    pub const fn acmp0_acmpoutroute(&self) -> &Acmp0Acmpoutroute {
        &self.acmp0_acmpoutroute
    }
    #[doc = "0x48c - ACMP1 pin enable"]
    #[inline(always)]
    pub const fn acmp1_routeen(&self) -> &Acmp1Routeen {
        &self.acmp1_routeen
    }
    #[doc = "0x490 - ACMPOUT port/pin select"]
    #[inline(always)]
    pub const fn acmp1_acmpoutroute(&self) -> &Acmp1Acmpoutroute {
        &self.acmp1_acmpoutroute
    }
    #[doc = "0x498 - CMU pin enable"]
    #[inline(always)]
    pub const fn cmu_routeen(&self) -> &CmuRouteen {
        &self.cmu_routeen
    }
    #[doc = "0x49c - CLKIN0 port/pin select"]
    #[inline(always)]
    pub const fn cmu_clkin0route(&self) -> &CmuClkin0route {
        &self.cmu_clkin0route
    }
    #[doc = "0x4a0 - CLKOUT0 port/pin select"]
    #[inline(always)]
    pub const fn cmu_clkout0route(&self) -> &CmuClkout0route {
        &self.cmu_clkout0route
    }
    #[doc = "0x4a4 - CLKOUT1 port/pin select"]
    #[inline(always)]
    pub const fn cmu_clkout1route(&self) -> &CmuClkout1route {
        &self.cmu_clkout1route
    }
    #[doc = "0x4a8 - CLKOUT2 port/pin select"]
    #[inline(always)]
    pub const fn cmu_clkout2route(&self) -> &CmuClkout2route {
        &self.cmu_clkout2route
    }
    #[doc = "0x4c4 - EUSART0 pin enable"]
    #[inline(always)]
    pub const fn eusart0_routeen(&self) -> &Eusart0Routeen {
        &self.eusart0_routeen
    }
    #[doc = "0x4c8 - CS port/pin select"]
    #[inline(always)]
    pub const fn eusart0_csroute(&self) -> &Eusart0Csroute {
        &self.eusart0_csroute
    }
    #[doc = "0x4cc - CTS port/pin select"]
    #[inline(always)]
    pub const fn eusart0_ctsroute(&self) -> &Eusart0Ctsroute {
        &self.eusart0_ctsroute
    }
    #[doc = "0x4d0 - RTS port/pin select"]
    #[inline(always)]
    pub const fn eusart0_rtsroute(&self) -> &Eusart0Rtsroute {
        &self.eusart0_rtsroute
    }
    #[doc = "0x4d4 - RX port/pin select"]
    #[inline(always)]
    pub const fn eusart0_rxroute(&self) -> &Eusart0Rxroute {
        &self.eusart0_rxroute
    }
    #[doc = "0x4d8 - SCLK port/pin select"]
    #[inline(always)]
    pub const fn eusart0_sclkroute(&self) -> &Eusart0Sclkroute {
        &self.eusart0_sclkroute
    }
    #[doc = "0x4dc - TX port/pin select"]
    #[inline(always)]
    pub const fn eusart0_txroute(&self) -> &Eusart0Txroute {
        &self.eusart0_txroute
    }
    #[doc = "0x4e4 - EUSART1 pin enable"]
    #[inline(always)]
    pub const fn eusart1_routeen(&self) -> &Eusart1Routeen {
        &self.eusart1_routeen
    }
    #[doc = "0x4e8 - CS port/pin select"]
    #[inline(always)]
    pub const fn eusart1_csroute(&self) -> &Eusart1Csroute {
        &self.eusart1_csroute
    }
    #[doc = "0x4ec - CTS port/pin select"]
    #[inline(always)]
    pub const fn eusart1_ctsroute(&self) -> &Eusart1Ctsroute {
        &self.eusart1_ctsroute
    }
    #[doc = "0x4f0 - RTS port/pin select"]
    #[inline(always)]
    pub const fn eusart1_rtsroute(&self) -> &Eusart1Rtsroute {
        &self.eusart1_rtsroute
    }
    #[doc = "0x4f4 - RX port/pin select"]
    #[inline(always)]
    pub const fn eusart1_rxroute(&self) -> &Eusart1Rxroute {
        &self.eusart1_rxroute
    }
    #[doc = "0x4f8 - SCLK port/pin select"]
    #[inline(always)]
    pub const fn eusart1_sclkroute(&self) -> &Eusart1Sclkroute {
        &self.eusart1_sclkroute
    }
    #[doc = "0x4fc - TX port/pin select"]
    #[inline(always)]
    pub const fn eusart1_txroute(&self) -> &Eusart1Txroute {
        &self.eusart1_txroute
    }
    #[doc = "0x504 - EUSART2 pin enable"]
    #[inline(always)]
    pub const fn eusart2_routeen(&self) -> &Eusart2Routeen {
        &self.eusart2_routeen
    }
    #[doc = "0x508 - CS port/pin select"]
    #[inline(always)]
    pub const fn eusart2_csroute(&self) -> &Eusart2Csroute {
        &self.eusart2_csroute
    }
    #[doc = "0x50c - CTS port/pin select"]
    #[inline(always)]
    pub const fn eusart2_ctsroute(&self) -> &Eusart2Ctsroute {
        &self.eusart2_ctsroute
    }
    #[doc = "0x510 - RTS port/pin select"]
    #[inline(always)]
    pub const fn eusart2_rtsroute(&self) -> &Eusart2Rtsroute {
        &self.eusart2_rtsroute
    }
    #[doc = "0x514 - RX port/pin select"]
    #[inline(always)]
    pub const fn eusart2_rxroute(&self) -> &Eusart2Rxroute {
        &self.eusart2_rxroute
    }
    #[doc = "0x518 - SCLK port/pin select"]
    #[inline(always)]
    pub const fn eusart2_sclkroute(&self) -> &Eusart2Sclkroute {
        &self.eusart2_sclkroute
    }
    #[doc = "0x51c - TX port/pin select"]
    #[inline(always)]
    pub const fn eusart2_txroute(&self) -> &Eusart2Txroute {
        &self.eusart2_txroute
    }
    #[doc = "0x538 - I2C0 pin enable"]
    #[inline(always)]
    pub const fn i2c0_routeen(&self) -> &I2c0Routeen {
        &self.i2c0_routeen
    }
    #[doc = "0x53c - SCL port/pin select"]
    #[inline(always)]
    pub const fn i2c0_sclroute(&self) -> &I2c0Sclroute {
        &self.i2c0_sclroute
    }
    #[doc = "0x540 - SDA port/pin select"]
    #[inline(always)]
    pub const fn i2c0_sdaroute(&self) -> &I2c0Sdaroute {
        &self.i2c0_sdaroute
    }
    #[doc = "0x548 - I2C1 pin enable"]
    #[inline(always)]
    pub const fn i2c1_routeen(&self) -> &I2c1Routeen {
        &self.i2c1_routeen
    }
    #[doc = "0x54c - SCL port/pin select"]
    #[inline(always)]
    pub const fn i2c1_sclroute(&self) -> &I2c1Sclroute {
        &self.i2c1_sclroute
    }
    #[doc = "0x550 - SDA port/pin select"]
    #[inline(always)]
    pub const fn i2c1_sdaroute(&self) -> &I2c1Sdaroute {
        &self.i2c1_sdaroute
    }
    #[doc = "0x558 - KEYSCAN pin enable"]
    #[inline(always)]
    pub const fn keyscan_routeen(&self) -> &KeyscanRouteen {
        &self.keyscan_routeen
    }
    #[doc = "0x55c - COLOUT0 port/pin select"]
    #[inline(always)]
    pub const fn keyscan_colout0route(&self) -> &KeyscanColout0route {
        &self.keyscan_colout0route
    }
    #[doc = "0x560 - COLOUT1 port/pin select"]
    #[inline(always)]
    pub const fn keyscan_colout1route(&self) -> &KeyscanColout1route {
        &self.keyscan_colout1route
    }
    #[doc = "0x564 - COLOUT2 port/pin select"]
    #[inline(always)]
    pub const fn keyscan_colout2route(&self) -> &KeyscanColout2route {
        &self.keyscan_colout2route
    }
    #[doc = "0x568 - COLOUT3 port/pin select"]
    #[inline(always)]
    pub const fn keyscan_colout3route(&self) -> &KeyscanColout3route {
        &self.keyscan_colout3route
    }
    #[doc = "0x56c - COLOUT4 port/pin select"]
    #[inline(always)]
    pub const fn keyscan_colout4route(&self) -> &KeyscanColout4route {
        &self.keyscan_colout4route
    }
    #[doc = "0x570 - COLOUT5 port/pin select"]
    #[inline(always)]
    pub const fn keyscan_colout5route(&self) -> &KeyscanColout5route {
        &self.keyscan_colout5route
    }
    #[doc = "0x574 - COLOUT6 port/pin select"]
    #[inline(always)]
    pub const fn keyscan_colout6route(&self) -> &KeyscanColout6route {
        &self.keyscan_colout6route
    }
    #[doc = "0x578 - COLOUT7 port/pin select"]
    #[inline(always)]
    pub const fn keyscan_colout7route(&self) -> &KeyscanColout7route {
        &self.keyscan_colout7route
    }
    #[doc = "0x57c - ROWSENSE0 port/pin select"]
    #[inline(always)]
    pub const fn keyscan_rowsense0route(&self) -> &KeyscanRowsense0route {
        &self.keyscan_rowsense0route
    }
    #[doc = "0x580 - ROWSENSE1 port/pin select"]
    #[inline(always)]
    pub const fn keyscan_rowsense1route(&self) -> &KeyscanRowsense1route {
        &self.keyscan_rowsense1route
    }
    #[doc = "0x584 - ROWSENSE2 port/pin select"]
    #[inline(always)]
    pub const fn keyscan_rowsense2route(&self) -> &KeyscanRowsense2route {
        &self.keyscan_rowsense2route
    }
    #[doc = "0x588 - ROWSENSE3 port/pin select"]
    #[inline(always)]
    pub const fn keyscan_rowsense3route(&self) -> &KeyscanRowsense3route {
        &self.keyscan_rowsense3route
    }
    #[doc = "0x58c - ROWSENSE4 port/pin select"]
    #[inline(always)]
    pub const fn keyscan_rowsense4route(&self) -> &KeyscanRowsense4route {
        &self.keyscan_rowsense4route
    }
    #[doc = "0x590 - ROWSENSE5 port/pin select"]
    #[inline(always)]
    pub const fn keyscan_rowsense5route(&self) -> &KeyscanRowsense5route {
        &self.keyscan_rowsense5route
    }
    #[doc = "0x598 - LESENSE pin enable"]
    #[inline(always)]
    pub const fn lesense_routeen(&self) -> &LesenseRouteen {
        &self.lesense_routeen
    }
    #[doc = "0x59c - CH0OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch0outroute(&self) -> &LesenseCh0outroute {
        &self.lesense_ch0outroute
    }
    #[doc = "0x5a0 - CH1OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch1outroute(&self) -> &LesenseCh1outroute {
        &self.lesense_ch1outroute
    }
    #[doc = "0x5a4 - CH2OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch2outroute(&self) -> &LesenseCh2outroute {
        &self.lesense_ch2outroute
    }
    #[doc = "0x5a8 - CH3OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch3outroute(&self) -> &LesenseCh3outroute {
        &self.lesense_ch3outroute
    }
    #[doc = "0x5ac - CH4OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch4outroute(&self) -> &LesenseCh4outroute {
        &self.lesense_ch4outroute
    }
    #[doc = "0x5b0 - CH5OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch5outroute(&self) -> &LesenseCh5outroute {
        &self.lesense_ch5outroute
    }
    #[doc = "0x5b4 - CH6OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch6outroute(&self) -> &LesenseCh6outroute {
        &self.lesense_ch6outroute
    }
    #[doc = "0x5b8 - CH7OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch7outroute(&self) -> &LesenseCh7outroute {
        &self.lesense_ch7outroute
    }
    #[doc = "0x5bc - CH8OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch8outroute(&self) -> &LesenseCh8outroute {
        &self.lesense_ch8outroute
    }
    #[doc = "0x5c0 - CH9OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch9outroute(&self) -> &LesenseCh9outroute {
        &self.lesense_ch9outroute
    }
    #[doc = "0x5c4 - CH10OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch10outroute(&self) -> &LesenseCh10outroute {
        &self.lesense_ch10outroute
    }
    #[doc = "0x5c8 - CH11OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch11outroute(&self) -> &LesenseCh11outroute {
        &self.lesense_ch11outroute
    }
    #[doc = "0x5cc - CH12OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch12outroute(&self) -> &LesenseCh12outroute {
        &self.lesense_ch12outroute
    }
    #[doc = "0x5d0 - CH13OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch13outroute(&self) -> &LesenseCh13outroute {
        &self.lesense_ch13outroute
    }
    #[doc = "0x5d4 - CH14OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch14outroute(&self) -> &LesenseCh14outroute {
        &self.lesense_ch14outroute
    }
    #[doc = "0x5d8 - CH15OUT port/pin select"]
    #[inline(always)]
    pub const fn lesense_ch15outroute(&self) -> &LesenseCh15outroute {
        &self.lesense_ch15outroute
    }
    #[doc = "0x5e0 - LETIMER pin enable"]
    #[inline(always)]
    pub const fn letimer_routeen(&self) -> &LetimerRouteen {
        &self.letimer_routeen
    }
    #[doc = "0x5e4 - OUT0 port/pin select"]
    #[inline(always)]
    pub const fn letimer_out0route(&self) -> &LetimerOut0route {
        &self.letimer_out0route
    }
    #[doc = "0x5e8 - OUT1 port/pin select"]
    #[inline(always)]
    pub const fn letimer_out1route(&self) -> &LetimerOut1route {
        &self.letimer_out1route
    }
    #[doc = "0x63c - S0IN port/pin select"]
    #[inline(always)]
    pub const fn pcnt0_s0inroute(&self) -> &Pcnt0S0inroute {
        &self.pcnt0_s0inroute
    }
    #[doc = "0x640 - S1IN port/pin select"]
    #[inline(always)]
    pub const fn pcnt0_s1inroute(&self) -> &Pcnt0S1inroute {
        &self.pcnt0_s1inroute
    }
    #[doc = "0x648 - PRS0 pin enable"]
    #[inline(always)]
    pub const fn prs0_routeen(&self) -> &Prs0Routeen {
        &self.prs0_routeen
    }
    #[doc = "0x64c - ASYNCH0 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch0route(&self) -> &Prs0Asynch0route {
        &self.prs0_asynch0route
    }
    #[doc = "0x650 - ASYNCH1 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch1route(&self) -> &Prs0Asynch1route {
        &self.prs0_asynch1route
    }
    #[doc = "0x654 - ASYNCH2 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch2route(&self) -> &Prs0Asynch2route {
        &self.prs0_asynch2route
    }
    #[doc = "0x658 - ASYNCH3 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch3route(&self) -> &Prs0Asynch3route {
        &self.prs0_asynch3route
    }
    #[doc = "0x65c - ASYNCH4 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch4route(&self) -> &Prs0Asynch4route {
        &self.prs0_asynch4route
    }
    #[doc = "0x660 - ASYNCH5 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch5route(&self) -> &Prs0Asynch5route {
        &self.prs0_asynch5route
    }
    #[doc = "0x664 - ASYNCH6 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch6route(&self) -> &Prs0Asynch6route {
        &self.prs0_asynch6route
    }
    #[doc = "0x668 - ASYNCH7 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch7route(&self) -> &Prs0Asynch7route {
        &self.prs0_asynch7route
    }
    #[doc = "0x66c - ASYNCH8 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch8route(&self) -> &Prs0Asynch8route {
        &self.prs0_asynch8route
    }
    #[doc = "0x670 - ASYNCH9 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch9route(&self) -> &Prs0Asynch9route {
        &self.prs0_asynch9route
    }
    #[doc = "0x674 - ASYNCH10 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch10route(&self) -> &Prs0Asynch10route {
        &self.prs0_asynch10route
    }
    #[doc = "0x678 - ASYNCH11 port/pin select"]
    #[inline(always)]
    pub const fn prs0_asynch11route(&self) -> &Prs0Asynch11route {
        &self.prs0_asynch11route
    }
    #[doc = "0x67c - SYNCH0 port/pin select"]
    #[inline(always)]
    pub const fn prs0_synch0route(&self) -> &Prs0Synch0route {
        &self.prs0_synch0route
    }
    #[doc = "0x680 - SYNCH1 port/pin select"]
    #[inline(always)]
    pub const fn prs0_synch1route(&self) -> &Prs0Synch1route {
        &self.prs0_synch1route
    }
    #[doc = "0x684 - SYNCH2 port/pin select"]
    #[inline(always)]
    pub const fn prs0_synch2route(&self) -> &Prs0Synch2route {
        &self.prs0_synch2route
    }
    #[doc = "0x688 - SYNCH3 port/pin select"]
    #[inline(always)]
    pub const fn prs0_synch3route(&self) -> &Prs0Synch3route {
        &self.prs0_synch3route
    }
    #[doc = "0x6f0 - BUFOUTREQINASYNC port/pin select"]
    #[inline(always)]
    pub const fn syxo0_bufoutreqinasyncroute(&self) -> &Syxo0Bufoutreqinasyncroute {
        &self.syxo0_bufoutreqinasyncroute
    }
    #[doc = "0x6f8 - TIMER0 pin enable"]
    #[inline(always)]
    pub const fn timer0_routeen(&self) -> &Timer0Routeen {
        &self.timer0_routeen
    }
    #[doc = "0x6fc - CC0 port/pin select"]
    #[inline(always)]
    pub const fn timer0_cc0route(&self) -> &Timer0Cc0route {
        &self.timer0_cc0route
    }
    #[doc = "0x700 - CC1 port/pin select"]
    #[inline(always)]
    pub const fn timer0_cc1route(&self) -> &Timer0Cc1route {
        &self.timer0_cc1route
    }
    #[doc = "0x704 - CC2 port/pin select"]
    #[inline(always)]
    pub const fn timer0_cc2route(&self) -> &Timer0Cc2route {
        &self.timer0_cc2route
    }
    #[doc = "0x708 - CDTI0 port/pin select"]
    #[inline(always)]
    pub const fn timer0_cdti0route(&self) -> &Timer0Cdti0route {
        &self.timer0_cdti0route
    }
    #[doc = "0x70c - CDTI1 port/pin select"]
    #[inline(always)]
    pub const fn timer0_cdti1route(&self) -> &Timer0Cdti1route {
        &self.timer0_cdti1route
    }
    #[doc = "0x710 - CDTI2 port/pin select"]
    #[inline(always)]
    pub const fn timer0_cdti2route(&self) -> &Timer0Cdti2route {
        &self.timer0_cdti2route
    }
    #[doc = "0x718 - TIMER1 pin enable"]
    #[inline(always)]
    pub const fn timer1_routeen(&self) -> &Timer1Routeen {
        &self.timer1_routeen
    }
    #[doc = "0x71c - CC0 port/pin select"]
    #[inline(always)]
    pub const fn timer1_cc0route(&self) -> &Timer1Cc0route {
        &self.timer1_cc0route
    }
    #[doc = "0x720 - CC1 port/pin select"]
    #[inline(always)]
    pub const fn timer1_cc1route(&self) -> &Timer1Cc1route {
        &self.timer1_cc1route
    }
    #[doc = "0x724 - CC2 port/pin select"]
    #[inline(always)]
    pub const fn timer1_cc2route(&self) -> &Timer1Cc2route {
        &self.timer1_cc2route
    }
    #[doc = "0x728 - CDTI0 port/pin select"]
    #[inline(always)]
    pub const fn timer1_cdti0route(&self) -> &Timer1Cdti0route {
        &self.timer1_cdti0route
    }
    #[doc = "0x72c - CDTI1 port/pin select"]
    #[inline(always)]
    pub const fn timer1_cdti1route(&self) -> &Timer1Cdti1route {
        &self.timer1_cdti1route
    }
    #[doc = "0x730 - CDTI2 port/pin select"]
    #[inline(always)]
    pub const fn timer1_cdti2route(&self) -> &Timer1Cdti2route {
        &self.timer1_cdti2route
    }
    #[doc = "0x738 - TIMER2 pin enable"]
    #[inline(always)]
    pub const fn timer2_routeen(&self) -> &Timer2Routeen {
        &self.timer2_routeen
    }
    #[doc = "0x73c - CC0 port/pin select"]
    #[inline(always)]
    pub const fn timer2_cc0route(&self) -> &Timer2Cc0route {
        &self.timer2_cc0route
    }
    #[doc = "0x740 - CC1 port/pin select"]
    #[inline(always)]
    pub const fn timer2_cc1route(&self) -> &Timer2Cc1route {
        &self.timer2_cc1route
    }
    #[doc = "0x744 - CC2 port/pin select"]
    #[inline(always)]
    pub const fn timer2_cc2route(&self) -> &Timer2Cc2route {
        &self.timer2_cc2route
    }
    #[doc = "0x748 - CDTI0 port/pin select"]
    #[inline(always)]
    pub const fn timer2_cdti0route(&self) -> &Timer2Cdti0route {
        &self.timer2_cdti0route
    }
    #[doc = "0x74c - CDTI1 port/pin select"]
    #[inline(always)]
    pub const fn timer2_cdti1route(&self) -> &Timer2Cdti1route {
        &self.timer2_cdti1route
    }
    #[doc = "0x750 - CDTI2 port/pin select"]
    #[inline(always)]
    pub const fn timer2_cdti2route(&self) -> &Timer2Cdti2route {
        &self.timer2_cdti2route
    }
    #[doc = "0x758 - TIMER3 pin enable"]
    #[inline(always)]
    pub const fn timer3_routeen(&self) -> &Timer3Routeen {
        &self.timer3_routeen
    }
    #[doc = "0x75c - CC0 port/pin select"]
    #[inline(always)]
    pub const fn timer3_cc0route(&self) -> &Timer3Cc0route {
        &self.timer3_cc0route
    }
    #[doc = "0x760 - CC1 port/pin select"]
    #[inline(always)]
    pub const fn timer3_cc1route(&self) -> &Timer3Cc1route {
        &self.timer3_cc1route
    }
    #[doc = "0x764 - CC2 port/pin select"]
    #[inline(always)]
    pub const fn timer3_cc2route(&self) -> &Timer3Cc2route {
        &self.timer3_cc2route
    }
    #[doc = "0x768 - CDTI0 port/pin select"]
    #[inline(always)]
    pub const fn timer3_cdti0route(&self) -> &Timer3Cdti0route {
        &self.timer3_cdti0route
    }
    #[doc = "0x76c - CDTI1 port/pin select"]
    #[inline(always)]
    pub const fn timer3_cdti1route(&self) -> &Timer3Cdti1route {
        &self.timer3_cdti1route
    }
    #[doc = "0x770 - CDTI2 port/pin select"]
    #[inline(always)]
    pub const fn timer3_cdti2route(&self) -> &Timer3Cdti2route {
        &self.timer3_cdti2route
    }
    #[doc = "0x778 - TIMER4 pin enable"]
    #[inline(always)]
    pub const fn timer4_routeen(&self) -> &Timer4Routeen {
        &self.timer4_routeen
    }
    #[doc = "0x77c - CC0 port/pin select"]
    #[inline(always)]
    pub const fn timer4_cc0route(&self) -> &Timer4Cc0route {
        &self.timer4_cc0route
    }
    #[doc = "0x780 - CC1 port/pin select"]
    #[inline(always)]
    pub const fn timer4_cc1route(&self) -> &Timer4Cc1route {
        &self.timer4_cc1route
    }
    #[doc = "0x784 - CC2 port/pin select"]
    #[inline(always)]
    pub const fn timer4_cc2route(&self) -> &Timer4Cc2route {
        &self.timer4_cc2route
    }
    #[doc = "0x788 - CDTI0 port/pin select"]
    #[inline(always)]
    pub const fn timer4_cdti0route(&self) -> &Timer4Cdti0route {
        &self.timer4_cdti0route
    }
    #[doc = "0x78c - CDTI1 port/pin select"]
    #[inline(always)]
    pub const fn timer4_cdti1route(&self) -> &Timer4Cdti1route {
        &self.timer4_cdti1route
    }
    #[doc = "0x790 - CDTI2 port/pin select"]
    #[inline(always)]
    pub const fn timer4_cdti2route(&self) -> &Timer4Cdti2route {
        &self.timer4_cdti2route
    }
    #[doc = "0x798 - USART0 pin enable"]
    #[inline(always)]
    pub const fn usart0_routeen(&self) -> &Usart0Routeen {
        &self.usart0_routeen
    }
    #[doc = "0x79c - CS port/pin select"]
    #[inline(always)]
    pub const fn usart0_csroute(&self) -> &Usart0Csroute {
        &self.usart0_csroute
    }
    #[doc = "0x7a0 - CTS port/pin select"]
    #[inline(always)]
    pub const fn usart0_ctsroute(&self) -> &Usart0Ctsroute {
        &self.usart0_ctsroute
    }
    #[doc = "0x7a4 - RTS port/pin select"]
    #[inline(always)]
    pub const fn usart0_rtsroute(&self) -> &Usart0Rtsroute {
        &self.usart0_rtsroute
    }
    #[doc = "0x7a8 - RX port/pin select"]
    #[inline(always)]
    pub const fn usart0_rxroute(&self) -> &Usart0Rxroute {
        &self.usart0_rxroute
    }
    #[doc = "0x7ac - SCLK port/pin select"]
    #[inline(always)]
    pub const fn usart0_clkroute(&self) -> &Usart0Clkroute {
        &self.usart0_clkroute
    }
    #[doc = "0x7b0 - TX port/pin select"]
    #[inline(always)]
    pub const fn usart0_txroute(&self) -> &Usart0Txroute {
        &self.usart0_txroute
    }
    #[doc = "0x1420 - Interrupt Flag Set"]
    #[inline(always)]
    pub const fn if_set(&self) -> &IfSet {
        &self.if_set
    }
    #[doc = "0x2420 - Interrupt Flag Clear"]
    #[inline(always)]
    pub const fn if_clr(&self) -> &IfClr {
        &self.if_clr
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "PORTA_CTRL (rw) register accessor: Port control\n\nYou can [`read`](crate::Reg::read) this register and get [`porta_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porta_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porta_ctrl`] module"]
#[doc(alias = "PORTA_CTRL")]
pub type PortaCtrl = crate::Reg<porta_ctrl::PortaCtrlSpec>;
#[doc = "Port control"]
pub mod porta_ctrl;
#[doc = "PORTA_MODEL (rw) register accessor: mode low\n\nYou can [`read`](crate::Reg::read) this register and get [`porta_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porta_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porta_model`] module"]
#[doc(alias = "PORTA_MODEL")]
pub type PortaModel = crate::Reg<porta_model::PortaModelSpec>;
#[doc = "mode low"]
pub mod porta_model;
#[doc = "PORTA_MODEH (rw) register accessor: mode high\n\nYou can [`read`](crate::Reg::read) this register and get [`porta_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porta_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porta_modeh`] module"]
#[doc(alias = "PORTA_MODEH")]
pub type PortaModeh = crate::Reg<porta_modeh::PortaModehSpec>;
#[doc = "mode high"]
pub mod porta_modeh;
#[doc = "PORTA_DOUT (rw) register accessor: data out\n\nYou can [`read`](crate::Reg::read) this register and get [`porta_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porta_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porta_dout`] module"]
#[doc(alias = "PORTA_DOUT")]
pub type PortaDout = crate::Reg<porta_dout::PortaDoutSpec>;
#[doc = "data out"]
pub mod porta_dout;
#[doc = "PORTA_DIN (r) register accessor: data in\n\nYou can [`read`](crate::Reg::read) this register and get [`porta_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porta_din`] module"]
#[doc(alias = "PORTA_DIN")]
pub type PortaDin = crate::Reg<porta_din::PortaDinSpec>;
#[doc = "data in"]
pub mod porta_din;
#[doc = "PORTB_CTRL (rw) register accessor: Port control\n\nYou can [`read`](crate::Reg::read) this register and get [`portb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portb_ctrl`] module"]
#[doc(alias = "PORTB_CTRL")]
pub type PortbCtrl = crate::Reg<portb_ctrl::PortbCtrlSpec>;
#[doc = "Port control"]
pub mod portb_ctrl;
#[doc = "PORTB_MODEL (rw) register accessor: mode low\n\nYou can [`read`](crate::Reg::read) this register and get [`portb_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portb_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portb_model`] module"]
#[doc(alias = "PORTB_MODEL")]
pub type PortbModel = crate::Reg<portb_model::PortbModelSpec>;
#[doc = "mode low"]
pub mod portb_model;
#[doc = "PORTB_DOUT (rw) register accessor: data out\n\nYou can [`read`](crate::Reg::read) this register and get [`portb_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portb_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portb_dout`] module"]
#[doc(alias = "PORTB_DOUT")]
pub type PortbDout = crate::Reg<portb_dout::PortbDoutSpec>;
#[doc = "data out"]
pub mod portb_dout;
#[doc = "PORTB_DIN (r) register accessor: data in\n\nYou can [`read`](crate::Reg::read) this register and get [`portb_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portb_din`] module"]
#[doc(alias = "PORTB_DIN")]
pub type PortbDin = crate::Reg<portb_din::PortbDinSpec>;
#[doc = "data in"]
pub mod portb_din;
#[doc = "PORTC_CTRL (rw) register accessor: Port control\n\nYou can [`read`](crate::Reg::read) this register and get [`portc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portc_ctrl`] module"]
#[doc(alias = "PORTC_CTRL")]
pub type PortcCtrl = crate::Reg<portc_ctrl::PortcCtrlSpec>;
#[doc = "Port control"]
pub mod portc_ctrl;
#[doc = "PORTC_MODEL (rw) register accessor: mode low\n\nYou can [`read`](crate::Reg::read) this register and get [`portc_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portc_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portc_model`] module"]
#[doc(alias = "PORTC_MODEL")]
pub type PortcModel = crate::Reg<portc_model::PortcModelSpec>;
#[doc = "mode low"]
pub mod portc_model;
#[doc = "PORTC_MODEH (rw) register accessor: mode high\n\nYou can [`read`](crate::Reg::read) this register and get [`portc_modeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portc_modeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portc_modeh`] module"]
#[doc(alias = "PORTC_MODEH")]
pub type PortcModeh = crate::Reg<portc_modeh::PortcModehSpec>;
#[doc = "mode high"]
pub mod portc_modeh;
#[doc = "PORTC_DOUT (rw) register accessor: data out\n\nYou can [`read`](crate::Reg::read) this register and get [`portc_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portc_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portc_dout`] module"]
#[doc(alias = "PORTC_DOUT")]
pub type PortcDout = crate::Reg<portc_dout::PortcDoutSpec>;
#[doc = "data out"]
pub mod portc_dout;
#[doc = "PORTC_DIN (r) register accessor: data in\n\nYou can [`read`](crate::Reg::read) this register and get [`portc_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portc_din`] module"]
#[doc(alias = "PORTC_DIN")]
pub type PortcDin = crate::Reg<portc_din::PortcDinSpec>;
#[doc = "data in"]
pub mod portc_din;
#[doc = "PORTD_CTRL (rw) register accessor: Port control\n\nYou can [`read`](crate::Reg::read) this register and get [`portd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portd_ctrl`] module"]
#[doc(alias = "PORTD_CTRL")]
pub type PortdCtrl = crate::Reg<portd_ctrl::PortdCtrlSpec>;
#[doc = "Port control"]
pub mod portd_ctrl;
#[doc = "PORTD_MODEL (rw) register accessor: mode low\n\nYou can [`read`](crate::Reg::read) this register and get [`portd_model::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portd_model::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portd_model`] module"]
#[doc(alias = "PORTD_MODEL")]
pub type PortdModel = crate::Reg<portd_model::PortdModelSpec>;
#[doc = "mode low"]
pub mod portd_model;
#[doc = "PORTD_DOUT (rw) register accessor: data out\n\nYou can [`read`](crate::Reg::read) this register and get [`portd_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portd_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portd_dout`] module"]
#[doc(alias = "PORTD_DOUT")]
pub type PortdDout = crate::Reg<portd_dout::PortdDoutSpec>;
#[doc = "data out"]
pub mod portd_dout;
#[doc = "PORTD_DIN (r) register accessor: data in\n\nYou can [`read`](crate::Reg::read) this register and get [`portd_din::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portd_din`] module"]
#[doc(alias = "PORTD_DIN")]
pub type PortdDin = crate::Reg<portd_din::PortdDinSpec>;
#[doc = "data in"]
pub mod portd_din;
#[doc = "LOCK (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "No Description"]
pub mod lock;
#[doc = "GPIOLOCKSTATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiolockstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiolockstatus`] module"]
#[doc(alias = "GPIOLOCKSTATUS")]
pub type Gpiolockstatus = crate::Reg<gpiolockstatus::GpiolockstatusSpec>;
#[doc = "No Description"]
pub mod gpiolockstatus;
#[doc = "ABUSALLOC (rw) register accessor: A Bus allocation\n\nYou can [`read`](crate::Reg::read) this register and get [`abusalloc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abusalloc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abusalloc`] module"]
#[doc(alias = "ABUSALLOC")]
pub type Abusalloc = crate::Reg<abusalloc::AbusallocSpec>;
#[doc = "A Bus allocation"]
pub mod abusalloc;
#[doc = "BBUSALLOC (rw) register accessor: B Bus allocation\n\nYou can [`read`](crate::Reg::read) this register and get [`bbusalloc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bbusalloc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bbusalloc`] module"]
#[doc(alias = "BBUSALLOC")]
pub type Bbusalloc = crate::Reg<bbusalloc::BbusallocSpec>;
#[doc = "B Bus allocation"]
pub mod bbusalloc;
#[doc = "CDBUSALLOC (rw) register accessor: CD Bus allocation\n\nYou can [`read`](crate::Reg::read) this register and get [`cdbusalloc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdbusalloc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdbusalloc`] module"]
#[doc(alias = "CDBUSALLOC")]
pub type Cdbusalloc = crate::Reg<cdbusalloc::CdbusallocSpec>;
#[doc = "CD Bus allocation"]
pub mod cdbusalloc;
#[doc = "EXTIPSELL (rw) register accessor: External Interrupt Port Select Low\n\nYou can [`read`](crate::Reg::read) this register and get [`extipsell::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipsell::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipsell`] module"]
#[doc(alias = "EXTIPSELL")]
pub type Extipsell = crate::Reg<extipsell::ExtipsellSpec>;
#[doc = "External Interrupt Port Select Low"]
pub mod extipsell;
#[doc = "EXTIPSELH (rw) register accessor: External interrupt Port Select High\n\nYou can [`read`](crate::Reg::read) this register and get [`extipselh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipselh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipselh`] module"]
#[doc(alias = "EXTIPSELH")]
pub type Extipselh = crate::Reg<extipselh::ExtipselhSpec>;
#[doc = "External interrupt Port Select High"]
pub mod extipselh;
#[doc = "EXTIPINSELL (rw) register accessor: External Interrupt Pin Select Low\n\nYou can [`read`](crate::Reg::read) this register and get [`extipinsell::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipinsell::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipinsell`] module"]
#[doc(alias = "EXTIPINSELL")]
pub type Extipinsell = crate::Reg<extipinsell::ExtipinsellSpec>;
#[doc = "External Interrupt Pin Select Low"]
pub mod extipinsell;
#[doc = "EXTIPINSELH (rw) register accessor: External Interrupt Pin Select High\n\nYou can [`read`](crate::Reg::read) this register and get [`extipinselh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipinselh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extipinselh`] module"]
#[doc(alias = "EXTIPINSELH")]
pub type Extipinselh = crate::Reg<extipinselh::ExtipinselhSpec>;
#[doc = "External Interrupt Pin Select High"]
pub mod extipinselh;
#[doc = "EXTIRISE (rw) register accessor: External Interrupt Rising Edge Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`extirise::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extirise::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extirise`] module"]
#[doc(alias = "EXTIRISE")]
pub type Extirise = crate::Reg<extirise::ExtiriseSpec>;
#[doc = "External Interrupt Rising Edge Trigger"]
pub mod extirise;
#[doc = "EXTIFALL (rw) register accessor: External Interrupt Falling Edge Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`extifall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extifall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extifall`] module"]
#[doc(alias = "EXTIFALL")]
pub type Extifall = crate::Reg<extifall::ExtifallSpec>;
#[doc = "External Interrupt Falling Edge Trigger"]
pub mod extifall;
#[doc = "IF (r) register accessor: Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "Interrupt Flag"]
pub mod if_;
#[doc = "IF_SET (w) register accessor: Interrupt Flag Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_set`] module"]
#[doc(alias = "IF_SET")]
pub type IfSet = crate::Reg<if_set::IfSetSpec>;
#[doc = "Interrupt Flag Set"]
pub mod if_set;
#[doc = "IF_CLR (w) register accessor: Interrupt Flag Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_clr`] module"]
#[doc(alias = "IF_CLR")]
pub type IfClr = crate::Reg<if_clr::IfClrSpec>;
#[doc = "Interrupt Flag Clear"]
pub mod if_clr;
#[doc = "IEN (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Interrupt Enable"]
pub mod ien;
#[doc = "EM4WUEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wuen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wuen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wuen`] module"]
#[doc(alias = "EM4WUEN")]
pub type Em4wuen = crate::Reg<em4wuen::Em4wuenSpec>;
#[doc = "No Description"]
pub mod em4wuen;
#[doc = "EM4WUPOL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wupol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wupol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4wupol`] module"]
#[doc(alias = "EM4WUPOL")]
pub type Em4wupol = crate::Reg<em4wupol::Em4wupolSpec>;
#[doc = "No Description"]
pub mod em4wupol;
#[doc = "DBGROUTEPEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgroutepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgroutepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgroutepen`] module"]
#[doc(alias = "DBGROUTEPEN")]
pub type Dbgroutepen = crate::Reg<dbgroutepen::DbgroutepenSpec>;
#[doc = "No Description"]
pub mod dbgroutepen;
#[doc = "TRACEROUTEPEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`traceroutepen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceroutepen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceroutepen`] module"]
#[doc(alias = "TRACEROUTEPEN")]
pub type Traceroutepen = crate::Reg<traceroutepen::TraceroutepenSpec>;
#[doc = "No Description"]
pub mod traceroutepen;
#[doc = "LCDSEG (rw) register accessor: LCD Segment Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdseg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdseg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdseg`] module"]
#[doc(alias = "LCDSEG")]
pub type Lcdseg = crate::Reg<lcdseg::LcdsegSpec>;
#[doc = "LCD Segment Enable"]
pub mod lcdseg;
#[doc = "LCDCOM (rw) register accessor: LCD Common Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdcom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdcom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdcom`] module"]
#[doc(alias = "LCDCOM")]
pub type Lcdcom = crate::Reg<lcdcom::LcdcomSpec>;
#[doc = "LCD Common Enable"]
pub mod lcdcom;
#[doc = "ACMP0_ROUTEEN (rw) register accessor: ACMP0 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`acmp0_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmp0_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmp0_routeen`] module"]
#[doc(alias = "ACMP0_ROUTEEN")]
pub type Acmp0Routeen = crate::Reg<acmp0_routeen::Acmp0RouteenSpec>;
#[doc = "ACMP0 pin enable"]
pub mod acmp0_routeen;
#[doc = "ACMP0_ACMPOUTROUTE (rw) register accessor: ACMPOUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`acmp0_acmpoutroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmp0_acmpoutroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmp0_acmpoutroute`] module"]
#[doc(alias = "ACMP0_ACMPOUTROUTE")]
pub type Acmp0Acmpoutroute = crate::Reg<acmp0_acmpoutroute::Acmp0AcmpoutrouteSpec>;
#[doc = "ACMPOUT port/pin select"]
pub mod acmp0_acmpoutroute;
#[doc = "ACMP1_ROUTEEN (rw) register accessor: ACMP1 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`acmp1_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmp1_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmp1_routeen`] module"]
#[doc(alias = "ACMP1_ROUTEEN")]
pub type Acmp1Routeen = crate::Reg<acmp1_routeen::Acmp1RouteenSpec>;
#[doc = "ACMP1 pin enable"]
pub mod acmp1_routeen;
#[doc = "ACMP1_ACMPOUTROUTE (rw) register accessor: ACMPOUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`acmp1_acmpoutroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmp1_acmpoutroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmp1_acmpoutroute`] module"]
#[doc(alias = "ACMP1_ACMPOUTROUTE")]
pub type Acmp1Acmpoutroute = crate::Reg<acmp1_acmpoutroute::Acmp1AcmpoutrouteSpec>;
#[doc = "ACMPOUT port/pin select"]
pub mod acmp1_acmpoutroute;
#[doc = "CMU_ROUTEEN (rw) register accessor: CMU pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`cmu_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmu_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmu_routeen`] module"]
#[doc(alias = "CMU_ROUTEEN")]
pub type CmuRouteen = crate::Reg<cmu_routeen::CmuRouteenSpec>;
#[doc = "CMU pin enable"]
pub mod cmu_routeen;
#[doc = "CMU_CLKIN0ROUTE (rw) register accessor: CLKIN0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`cmu_clkin0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmu_clkin0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmu_clkin0route`] module"]
#[doc(alias = "CMU_CLKIN0ROUTE")]
pub type CmuClkin0route = crate::Reg<cmu_clkin0route::CmuClkin0routeSpec>;
#[doc = "CLKIN0 port/pin select"]
pub mod cmu_clkin0route;
#[doc = "CMU_CLKOUT0ROUTE (rw) register accessor: CLKOUT0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`cmu_clkout0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmu_clkout0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmu_clkout0route`] module"]
#[doc(alias = "CMU_CLKOUT0ROUTE")]
pub type CmuClkout0route = crate::Reg<cmu_clkout0route::CmuClkout0routeSpec>;
#[doc = "CLKOUT0 port/pin select"]
pub mod cmu_clkout0route;
#[doc = "CMU_CLKOUT1ROUTE (rw) register accessor: CLKOUT1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`cmu_clkout1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmu_clkout1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmu_clkout1route`] module"]
#[doc(alias = "CMU_CLKOUT1ROUTE")]
pub type CmuClkout1route = crate::Reg<cmu_clkout1route::CmuClkout1routeSpec>;
#[doc = "CLKOUT1 port/pin select"]
pub mod cmu_clkout1route;
#[doc = "CMU_CLKOUT2ROUTE (rw) register accessor: CLKOUT2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`cmu_clkout2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmu_clkout2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmu_clkout2route`] module"]
#[doc(alias = "CMU_CLKOUT2ROUTE")]
pub type CmuClkout2route = crate::Reg<cmu_clkout2route::CmuClkout2routeSpec>;
#[doc = "CLKOUT2 port/pin select"]
pub mod cmu_clkout2route;
#[doc = "EUSART0_ROUTEEN (rw) register accessor: EUSART0 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0_routeen`] module"]
#[doc(alias = "EUSART0_ROUTEEN")]
pub type Eusart0Routeen = crate::Reg<eusart0_routeen::Eusart0RouteenSpec>;
#[doc = "EUSART0 pin enable"]
pub mod eusart0_routeen;
#[doc = "EUSART0_CSROUTE (rw) register accessor: CS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_csroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_csroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0_csroute`] module"]
#[doc(alias = "EUSART0_CSROUTE")]
pub type Eusart0Csroute = crate::Reg<eusart0_csroute::Eusart0CsrouteSpec>;
#[doc = "CS port/pin select"]
pub mod eusart0_csroute;
#[doc = "EUSART0_CTSROUTE (rw) register accessor: CTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_ctsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_ctsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0_ctsroute`] module"]
#[doc(alias = "EUSART0_CTSROUTE")]
pub type Eusart0Ctsroute = crate::Reg<eusart0_ctsroute::Eusart0CtsrouteSpec>;
#[doc = "CTS port/pin select"]
pub mod eusart0_ctsroute;
#[doc = "EUSART0_RTSROUTE (rw) register accessor: RTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_rtsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_rtsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0_rtsroute`] module"]
#[doc(alias = "EUSART0_RTSROUTE")]
pub type Eusart0Rtsroute = crate::Reg<eusart0_rtsroute::Eusart0RtsrouteSpec>;
#[doc = "RTS port/pin select"]
pub mod eusart0_rtsroute;
#[doc = "EUSART0_RXROUTE (rw) register accessor: RX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_rxroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_rxroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0_rxroute`] module"]
#[doc(alias = "EUSART0_RXROUTE")]
pub type Eusart0Rxroute = crate::Reg<eusart0_rxroute::Eusart0RxrouteSpec>;
#[doc = "RX port/pin select"]
pub mod eusart0_rxroute;
#[doc = "EUSART0_SCLKROUTE (rw) register accessor: SCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_sclkroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_sclkroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0_sclkroute`] module"]
#[doc(alias = "EUSART0_SCLKROUTE")]
pub type Eusart0Sclkroute = crate::Reg<eusart0_sclkroute::Eusart0SclkrouteSpec>;
#[doc = "SCLK port/pin select"]
pub mod eusart0_sclkroute;
#[doc = "EUSART0_TXROUTE (rw) register accessor: TX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_txroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_txroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0_txroute`] module"]
#[doc(alias = "EUSART0_TXROUTE")]
pub type Eusart0Txroute = crate::Reg<eusart0_txroute::Eusart0TxrouteSpec>;
#[doc = "TX port/pin select"]
pub mod eusart0_txroute;
#[doc = "EUSART1_ROUTEEN (rw) register accessor: EUSART1 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart1_routeen`] module"]
#[doc(alias = "EUSART1_ROUTEEN")]
pub type Eusart1Routeen = crate::Reg<eusart1_routeen::Eusart1RouteenSpec>;
#[doc = "EUSART1 pin enable"]
pub mod eusart1_routeen;
#[doc = "EUSART1_CSROUTE (rw) register accessor: CS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_csroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_csroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart1_csroute`] module"]
#[doc(alias = "EUSART1_CSROUTE")]
pub type Eusart1Csroute = crate::Reg<eusart1_csroute::Eusart1CsrouteSpec>;
#[doc = "CS port/pin select"]
pub mod eusart1_csroute;
#[doc = "EUSART1_CTSROUTE (rw) register accessor: CTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_ctsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_ctsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart1_ctsroute`] module"]
#[doc(alias = "EUSART1_CTSROUTE")]
pub type Eusart1Ctsroute = crate::Reg<eusart1_ctsroute::Eusart1CtsrouteSpec>;
#[doc = "CTS port/pin select"]
pub mod eusart1_ctsroute;
#[doc = "EUSART1_RTSROUTE (rw) register accessor: RTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_rtsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_rtsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart1_rtsroute`] module"]
#[doc(alias = "EUSART1_RTSROUTE")]
pub type Eusart1Rtsroute = crate::Reg<eusart1_rtsroute::Eusart1RtsrouteSpec>;
#[doc = "RTS port/pin select"]
pub mod eusart1_rtsroute;
#[doc = "EUSART1_RXROUTE (rw) register accessor: RX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_rxroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_rxroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart1_rxroute`] module"]
#[doc(alias = "EUSART1_RXROUTE")]
pub type Eusart1Rxroute = crate::Reg<eusart1_rxroute::Eusart1RxrouteSpec>;
#[doc = "RX port/pin select"]
pub mod eusart1_rxroute;
#[doc = "EUSART1_SCLKROUTE (rw) register accessor: SCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_sclkroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_sclkroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart1_sclkroute`] module"]
#[doc(alias = "EUSART1_SCLKROUTE")]
pub type Eusart1Sclkroute = crate::Reg<eusart1_sclkroute::Eusart1SclkrouteSpec>;
#[doc = "SCLK port/pin select"]
pub mod eusart1_sclkroute;
#[doc = "EUSART1_TXROUTE (rw) register accessor: TX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_txroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_txroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart1_txroute`] module"]
#[doc(alias = "EUSART1_TXROUTE")]
pub type Eusart1Txroute = crate::Reg<eusart1_txroute::Eusart1TxrouteSpec>;
#[doc = "TX port/pin select"]
pub mod eusart1_txroute;
#[doc = "EUSART2_ROUTEEN (rw) register accessor: EUSART2 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart2_routeen`] module"]
#[doc(alias = "EUSART2_ROUTEEN")]
pub type Eusart2Routeen = crate::Reg<eusart2_routeen::Eusart2RouteenSpec>;
#[doc = "EUSART2 pin enable"]
pub mod eusart2_routeen;
#[doc = "EUSART2_CSROUTE (rw) register accessor: CS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_csroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_csroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart2_csroute`] module"]
#[doc(alias = "EUSART2_CSROUTE")]
pub type Eusart2Csroute = crate::Reg<eusart2_csroute::Eusart2CsrouteSpec>;
#[doc = "CS port/pin select"]
pub mod eusart2_csroute;
#[doc = "EUSART2_CTSROUTE (rw) register accessor: CTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_ctsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_ctsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart2_ctsroute`] module"]
#[doc(alias = "EUSART2_CTSROUTE")]
pub type Eusart2Ctsroute = crate::Reg<eusart2_ctsroute::Eusart2CtsrouteSpec>;
#[doc = "CTS port/pin select"]
pub mod eusart2_ctsroute;
#[doc = "EUSART2_RTSROUTE (rw) register accessor: RTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_rtsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_rtsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart2_rtsroute`] module"]
#[doc(alias = "EUSART2_RTSROUTE")]
pub type Eusart2Rtsroute = crate::Reg<eusart2_rtsroute::Eusart2RtsrouteSpec>;
#[doc = "RTS port/pin select"]
pub mod eusart2_rtsroute;
#[doc = "EUSART2_RXROUTE (rw) register accessor: RX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_rxroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_rxroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart2_rxroute`] module"]
#[doc(alias = "EUSART2_RXROUTE")]
pub type Eusart2Rxroute = crate::Reg<eusart2_rxroute::Eusart2RxrouteSpec>;
#[doc = "RX port/pin select"]
pub mod eusart2_rxroute;
#[doc = "EUSART2_SCLKROUTE (rw) register accessor: SCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_sclkroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_sclkroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart2_sclkroute`] module"]
#[doc(alias = "EUSART2_SCLKROUTE")]
pub type Eusart2Sclkroute = crate::Reg<eusart2_sclkroute::Eusart2SclkrouteSpec>;
#[doc = "SCLK port/pin select"]
pub mod eusart2_sclkroute;
#[doc = "EUSART2_TXROUTE (rw) register accessor: TX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_txroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_txroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart2_txroute`] module"]
#[doc(alias = "EUSART2_TXROUTE")]
pub type Eusart2Txroute = crate::Reg<eusart2_txroute::Eusart2TxrouteSpec>;
#[doc = "TX port/pin select"]
pub mod eusart2_txroute;
#[doc = "I2C0_ROUTEEN (rw) register accessor: I2C0 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_routeen`] module"]
#[doc(alias = "I2C0_ROUTEEN")]
pub type I2c0Routeen = crate::Reg<i2c0_routeen::I2c0RouteenSpec>;
#[doc = "I2C0 pin enable"]
pub mod i2c0_routeen;
#[doc = "I2C0_SCLROUTE (rw) register accessor: SCL port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_sclroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_sclroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_sclroute`] module"]
#[doc(alias = "I2C0_SCLROUTE")]
pub type I2c0Sclroute = crate::Reg<i2c0_sclroute::I2c0SclrouteSpec>;
#[doc = "SCL port/pin select"]
pub mod i2c0_sclroute;
#[doc = "I2C0_SDAROUTE (rw) register accessor: SDA port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0_sdaroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0_sdaroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0_sdaroute`] module"]
#[doc(alias = "I2C0_SDAROUTE")]
pub type I2c0Sdaroute = crate::Reg<i2c0_sdaroute::I2c0SdarouteSpec>;
#[doc = "SDA port/pin select"]
pub mod i2c0_sdaroute;
#[doc = "I2C1_ROUTEEN (rw) register accessor: I2C1 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_routeen`] module"]
#[doc(alias = "I2C1_ROUTEEN")]
pub type I2c1Routeen = crate::Reg<i2c1_routeen::I2c1RouteenSpec>;
#[doc = "I2C1 pin enable"]
pub mod i2c1_routeen;
#[doc = "I2C1_SCLROUTE (rw) register accessor: SCL port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_sclroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_sclroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_sclroute`] module"]
#[doc(alias = "I2C1_SCLROUTE")]
pub type I2c1Sclroute = crate::Reg<i2c1_sclroute::I2c1SclrouteSpec>;
#[doc = "SCL port/pin select"]
pub mod i2c1_sclroute;
#[doc = "I2C1_SDAROUTE (rw) register accessor: SDA port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_sdaroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_sdaroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1_sdaroute`] module"]
#[doc(alias = "I2C1_SDAROUTE")]
pub type I2c1Sdaroute = crate::Reg<i2c1_sdaroute::I2c1SdarouteSpec>;
#[doc = "SDA port/pin select"]
pub mod i2c1_sdaroute;
#[doc = "KEYSCAN_ROUTEEN (rw) register accessor: KEYSCAN pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_routeen`] module"]
#[doc(alias = "KEYSCAN_ROUTEEN")]
pub type KeyscanRouteen = crate::Reg<keyscan_routeen::KeyscanRouteenSpec>;
#[doc = "KEYSCAN pin enable"]
pub mod keyscan_routeen;
#[doc = "KEYSCAN_COLOUT0ROUTE (rw) register accessor: COLOUT0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_colout0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_colout0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_colout0route`] module"]
#[doc(alias = "KEYSCAN_COLOUT0ROUTE")]
pub type KeyscanColout0route = crate::Reg<keyscan_colout0route::KeyscanColout0routeSpec>;
#[doc = "COLOUT0 port/pin select"]
pub mod keyscan_colout0route;
#[doc = "KEYSCAN_COLOUT1ROUTE (rw) register accessor: COLOUT1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_colout1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_colout1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_colout1route`] module"]
#[doc(alias = "KEYSCAN_COLOUT1ROUTE")]
pub type KeyscanColout1route = crate::Reg<keyscan_colout1route::KeyscanColout1routeSpec>;
#[doc = "COLOUT1 port/pin select"]
pub mod keyscan_colout1route;
#[doc = "KEYSCAN_COLOUT2ROUTE (rw) register accessor: COLOUT2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_colout2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_colout2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_colout2route`] module"]
#[doc(alias = "KEYSCAN_COLOUT2ROUTE")]
pub type KeyscanColout2route = crate::Reg<keyscan_colout2route::KeyscanColout2routeSpec>;
#[doc = "COLOUT2 port/pin select"]
pub mod keyscan_colout2route;
#[doc = "KEYSCAN_COLOUT3ROUTE (rw) register accessor: COLOUT3 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_colout3route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_colout3route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_colout3route`] module"]
#[doc(alias = "KEYSCAN_COLOUT3ROUTE")]
pub type KeyscanColout3route = crate::Reg<keyscan_colout3route::KeyscanColout3routeSpec>;
#[doc = "COLOUT3 port/pin select"]
pub mod keyscan_colout3route;
#[doc = "KEYSCAN_COLOUT4ROUTE (rw) register accessor: COLOUT4 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_colout4route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_colout4route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_colout4route`] module"]
#[doc(alias = "KEYSCAN_COLOUT4ROUTE")]
pub type KeyscanColout4route = crate::Reg<keyscan_colout4route::KeyscanColout4routeSpec>;
#[doc = "COLOUT4 port/pin select"]
pub mod keyscan_colout4route;
#[doc = "KEYSCAN_COLOUT5ROUTE (rw) register accessor: COLOUT5 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_colout5route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_colout5route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_colout5route`] module"]
#[doc(alias = "KEYSCAN_COLOUT5ROUTE")]
pub type KeyscanColout5route = crate::Reg<keyscan_colout5route::KeyscanColout5routeSpec>;
#[doc = "COLOUT5 port/pin select"]
pub mod keyscan_colout5route;
#[doc = "KEYSCAN_COLOUT6ROUTE (rw) register accessor: COLOUT6 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_colout6route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_colout6route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_colout6route`] module"]
#[doc(alias = "KEYSCAN_COLOUT6ROUTE")]
pub type KeyscanColout6route = crate::Reg<keyscan_colout6route::KeyscanColout6routeSpec>;
#[doc = "COLOUT6 port/pin select"]
pub mod keyscan_colout6route;
#[doc = "KEYSCAN_COLOUT7ROUTE (rw) register accessor: COLOUT7 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_colout7route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_colout7route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_colout7route`] module"]
#[doc(alias = "KEYSCAN_COLOUT7ROUTE")]
pub type KeyscanColout7route = crate::Reg<keyscan_colout7route::KeyscanColout7routeSpec>;
#[doc = "COLOUT7 port/pin select"]
pub mod keyscan_colout7route;
#[doc = "KEYSCAN_ROWSENSE0ROUTE (rw) register accessor: ROWSENSE0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_rowsense0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_rowsense0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_rowsense0route`] module"]
#[doc(alias = "KEYSCAN_ROWSENSE0ROUTE")]
pub type KeyscanRowsense0route = crate::Reg<keyscan_rowsense0route::KeyscanRowsense0routeSpec>;
#[doc = "ROWSENSE0 port/pin select"]
pub mod keyscan_rowsense0route;
#[doc = "KEYSCAN_ROWSENSE1ROUTE (rw) register accessor: ROWSENSE1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_rowsense1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_rowsense1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_rowsense1route`] module"]
#[doc(alias = "KEYSCAN_ROWSENSE1ROUTE")]
pub type KeyscanRowsense1route = crate::Reg<keyscan_rowsense1route::KeyscanRowsense1routeSpec>;
#[doc = "ROWSENSE1 port/pin select"]
pub mod keyscan_rowsense1route;
#[doc = "KEYSCAN_ROWSENSE2ROUTE (rw) register accessor: ROWSENSE2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_rowsense2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_rowsense2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_rowsense2route`] module"]
#[doc(alias = "KEYSCAN_ROWSENSE2ROUTE")]
pub type KeyscanRowsense2route = crate::Reg<keyscan_rowsense2route::KeyscanRowsense2routeSpec>;
#[doc = "ROWSENSE2 port/pin select"]
pub mod keyscan_rowsense2route;
#[doc = "KEYSCAN_ROWSENSE3ROUTE (rw) register accessor: ROWSENSE3 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_rowsense3route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_rowsense3route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_rowsense3route`] module"]
#[doc(alias = "KEYSCAN_ROWSENSE3ROUTE")]
pub type KeyscanRowsense3route = crate::Reg<keyscan_rowsense3route::KeyscanRowsense3routeSpec>;
#[doc = "ROWSENSE3 port/pin select"]
pub mod keyscan_rowsense3route;
#[doc = "KEYSCAN_ROWSENSE4ROUTE (rw) register accessor: ROWSENSE4 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_rowsense4route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_rowsense4route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_rowsense4route`] module"]
#[doc(alias = "KEYSCAN_ROWSENSE4ROUTE")]
pub type KeyscanRowsense4route = crate::Reg<keyscan_rowsense4route::KeyscanRowsense4routeSpec>;
#[doc = "ROWSENSE4 port/pin select"]
pub mod keyscan_rowsense4route;
#[doc = "KEYSCAN_ROWSENSE5ROUTE (rw) register accessor: ROWSENSE5 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_rowsense5route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_rowsense5route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyscan_rowsense5route`] module"]
#[doc(alias = "KEYSCAN_ROWSENSE5ROUTE")]
pub type KeyscanRowsense5route = crate::Reg<keyscan_rowsense5route::KeyscanRowsense5routeSpec>;
#[doc = "ROWSENSE5 port/pin select"]
pub mod keyscan_rowsense5route;
#[doc = "LESENSE_ROUTEEN (rw) register accessor: LESENSE pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_routeen`] module"]
#[doc(alias = "LESENSE_ROUTEEN")]
pub type LesenseRouteen = crate::Reg<lesense_routeen::LesenseRouteenSpec>;
#[doc = "LESENSE pin enable"]
pub mod lesense_routeen;
#[doc = "LESENSE_CH0OUTROUTE (rw) register accessor: CH0OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch0outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch0outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch0outroute`] module"]
#[doc(alias = "LESENSE_CH0OUTROUTE")]
pub type LesenseCh0outroute = crate::Reg<lesense_ch0outroute::LesenseCh0outrouteSpec>;
#[doc = "CH0OUT port/pin select"]
pub mod lesense_ch0outroute;
#[doc = "LESENSE_CH1OUTROUTE (rw) register accessor: CH1OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch1outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch1outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch1outroute`] module"]
#[doc(alias = "LESENSE_CH1OUTROUTE")]
pub type LesenseCh1outroute = crate::Reg<lesense_ch1outroute::LesenseCh1outrouteSpec>;
#[doc = "CH1OUT port/pin select"]
pub mod lesense_ch1outroute;
#[doc = "LESENSE_CH2OUTROUTE (rw) register accessor: CH2OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch2outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch2outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch2outroute`] module"]
#[doc(alias = "LESENSE_CH2OUTROUTE")]
pub type LesenseCh2outroute = crate::Reg<lesense_ch2outroute::LesenseCh2outrouteSpec>;
#[doc = "CH2OUT port/pin select"]
pub mod lesense_ch2outroute;
#[doc = "LESENSE_CH3OUTROUTE (rw) register accessor: CH3OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch3outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch3outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch3outroute`] module"]
#[doc(alias = "LESENSE_CH3OUTROUTE")]
pub type LesenseCh3outroute = crate::Reg<lesense_ch3outroute::LesenseCh3outrouteSpec>;
#[doc = "CH3OUT port/pin select"]
pub mod lesense_ch3outroute;
#[doc = "LESENSE_CH4OUTROUTE (rw) register accessor: CH4OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch4outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch4outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch4outroute`] module"]
#[doc(alias = "LESENSE_CH4OUTROUTE")]
pub type LesenseCh4outroute = crate::Reg<lesense_ch4outroute::LesenseCh4outrouteSpec>;
#[doc = "CH4OUT port/pin select"]
pub mod lesense_ch4outroute;
#[doc = "LESENSE_CH5OUTROUTE (rw) register accessor: CH5OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch5outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch5outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch5outroute`] module"]
#[doc(alias = "LESENSE_CH5OUTROUTE")]
pub type LesenseCh5outroute = crate::Reg<lesense_ch5outroute::LesenseCh5outrouteSpec>;
#[doc = "CH5OUT port/pin select"]
pub mod lesense_ch5outroute;
#[doc = "LESENSE_CH6OUTROUTE (rw) register accessor: CH6OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch6outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch6outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch6outroute`] module"]
#[doc(alias = "LESENSE_CH6OUTROUTE")]
pub type LesenseCh6outroute = crate::Reg<lesense_ch6outroute::LesenseCh6outrouteSpec>;
#[doc = "CH6OUT port/pin select"]
pub mod lesense_ch6outroute;
#[doc = "LESENSE_CH7OUTROUTE (rw) register accessor: CH7OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch7outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch7outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch7outroute`] module"]
#[doc(alias = "LESENSE_CH7OUTROUTE")]
pub type LesenseCh7outroute = crate::Reg<lesense_ch7outroute::LesenseCh7outrouteSpec>;
#[doc = "CH7OUT port/pin select"]
pub mod lesense_ch7outroute;
#[doc = "LESENSE_CH8OUTROUTE (rw) register accessor: CH8OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch8outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch8outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch8outroute`] module"]
#[doc(alias = "LESENSE_CH8OUTROUTE")]
pub type LesenseCh8outroute = crate::Reg<lesense_ch8outroute::LesenseCh8outrouteSpec>;
#[doc = "CH8OUT port/pin select"]
pub mod lesense_ch8outroute;
#[doc = "LESENSE_CH9OUTROUTE (rw) register accessor: CH9OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch9outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch9outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch9outroute`] module"]
#[doc(alias = "LESENSE_CH9OUTROUTE")]
pub type LesenseCh9outroute = crate::Reg<lesense_ch9outroute::LesenseCh9outrouteSpec>;
#[doc = "CH9OUT port/pin select"]
pub mod lesense_ch9outroute;
#[doc = "LESENSE_CH10OUTROUTE (rw) register accessor: CH10OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch10outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch10outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch10outroute`] module"]
#[doc(alias = "LESENSE_CH10OUTROUTE")]
pub type LesenseCh10outroute = crate::Reg<lesense_ch10outroute::LesenseCh10outrouteSpec>;
#[doc = "CH10OUT port/pin select"]
pub mod lesense_ch10outroute;
#[doc = "LESENSE_CH11OUTROUTE (rw) register accessor: CH11OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch11outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch11outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch11outroute`] module"]
#[doc(alias = "LESENSE_CH11OUTROUTE")]
pub type LesenseCh11outroute = crate::Reg<lesense_ch11outroute::LesenseCh11outrouteSpec>;
#[doc = "CH11OUT port/pin select"]
pub mod lesense_ch11outroute;
#[doc = "LESENSE_CH12OUTROUTE (rw) register accessor: CH12OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch12outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch12outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch12outroute`] module"]
#[doc(alias = "LESENSE_CH12OUTROUTE")]
pub type LesenseCh12outroute = crate::Reg<lesense_ch12outroute::LesenseCh12outrouteSpec>;
#[doc = "CH12OUT port/pin select"]
pub mod lesense_ch12outroute;
#[doc = "LESENSE_CH13OUTROUTE (rw) register accessor: CH13OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch13outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch13outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch13outroute`] module"]
#[doc(alias = "LESENSE_CH13OUTROUTE")]
pub type LesenseCh13outroute = crate::Reg<lesense_ch13outroute::LesenseCh13outrouteSpec>;
#[doc = "CH13OUT port/pin select"]
pub mod lesense_ch13outroute;
#[doc = "LESENSE_CH14OUTROUTE (rw) register accessor: CH14OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch14outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch14outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch14outroute`] module"]
#[doc(alias = "LESENSE_CH14OUTROUTE")]
pub type LesenseCh14outroute = crate::Reg<lesense_ch14outroute::LesenseCh14outrouteSpec>;
#[doc = "CH14OUT port/pin select"]
pub mod lesense_ch14outroute;
#[doc = "LESENSE_CH15OUTROUTE (rw) register accessor: CH15OUT port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_ch15outroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_ch15outroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesense_ch15outroute`] module"]
#[doc(alias = "LESENSE_CH15OUTROUTE")]
pub type LesenseCh15outroute = crate::Reg<lesense_ch15outroute::LesenseCh15outrouteSpec>;
#[doc = "CH15OUT port/pin select"]
pub mod lesense_ch15outroute;
#[doc = "LETIMER_ROUTEEN (rw) register accessor: LETIMER pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`letimer_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`letimer_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@letimer_routeen`] module"]
#[doc(alias = "LETIMER_ROUTEEN")]
pub type LetimerRouteen = crate::Reg<letimer_routeen::LetimerRouteenSpec>;
#[doc = "LETIMER pin enable"]
pub mod letimer_routeen;
#[doc = "LETIMER_OUT0ROUTE (rw) register accessor: OUT0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`letimer_out0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`letimer_out0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@letimer_out0route`] module"]
#[doc(alias = "LETIMER_OUT0ROUTE")]
pub type LetimerOut0route = crate::Reg<letimer_out0route::LetimerOut0routeSpec>;
#[doc = "OUT0 port/pin select"]
pub mod letimer_out0route;
#[doc = "LETIMER_OUT1ROUTE (rw) register accessor: OUT1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`letimer_out1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`letimer_out1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@letimer_out1route`] module"]
#[doc(alias = "LETIMER_OUT1ROUTE")]
pub type LetimerOut1route = crate::Reg<letimer_out1route::LetimerOut1routeSpec>;
#[doc = "OUT1 port/pin select"]
pub mod letimer_out1route;
#[doc = "PCNT0_S0INROUTE (rw) register accessor: S0IN port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt0_s0inroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt0_s0inroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnt0_s0inroute`] module"]
#[doc(alias = "PCNT0_S0INROUTE")]
pub type Pcnt0S0inroute = crate::Reg<pcnt0_s0inroute::Pcnt0S0inrouteSpec>;
#[doc = "S0IN port/pin select"]
pub mod pcnt0_s0inroute;
#[doc = "PCNT0_S1INROUTE (rw) register accessor: S1IN port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt0_s1inroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt0_s1inroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnt0_s1inroute`] module"]
#[doc(alias = "PCNT0_S1INROUTE")]
pub type Pcnt0S1inroute = crate::Reg<pcnt0_s1inroute::Pcnt0S1inrouteSpec>;
#[doc = "S1IN port/pin select"]
pub mod pcnt0_s1inroute;
#[doc = "PRS0_ROUTEEN (rw) register accessor: PRS0 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_routeen`] module"]
#[doc(alias = "PRS0_ROUTEEN")]
pub type Prs0Routeen = crate::Reg<prs0_routeen::Prs0RouteenSpec>;
#[doc = "PRS0 pin enable"]
pub mod prs0_routeen;
#[doc = "PRS0_ASYNCH0ROUTE (rw) register accessor: ASYNCH0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch0route`] module"]
#[doc(alias = "PRS0_ASYNCH0ROUTE")]
pub type Prs0Asynch0route = crate::Reg<prs0_asynch0route::Prs0Asynch0routeSpec>;
#[doc = "ASYNCH0 port/pin select"]
pub mod prs0_asynch0route;
#[doc = "PRS0_ASYNCH1ROUTE (rw) register accessor: ASYNCH1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch1route`] module"]
#[doc(alias = "PRS0_ASYNCH1ROUTE")]
pub type Prs0Asynch1route = crate::Reg<prs0_asynch1route::Prs0Asynch1routeSpec>;
#[doc = "ASYNCH1 port/pin select"]
pub mod prs0_asynch1route;
#[doc = "PRS0_ASYNCH2ROUTE (rw) register accessor: ASYNCH2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch2route`] module"]
#[doc(alias = "PRS0_ASYNCH2ROUTE")]
pub type Prs0Asynch2route = crate::Reg<prs0_asynch2route::Prs0Asynch2routeSpec>;
#[doc = "ASYNCH2 port/pin select"]
pub mod prs0_asynch2route;
#[doc = "PRS0_ASYNCH3ROUTE (rw) register accessor: ASYNCH3 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch3route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch3route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch3route`] module"]
#[doc(alias = "PRS0_ASYNCH3ROUTE")]
pub type Prs0Asynch3route = crate::Reg<prs0_asynch3route::Prs0Asynch3routeSpec>;
#[doc = "ASYNCH3 port/pin select"]
pub mod prs0_asynch3route;
#[doc = "PRS0_ASYNCH4ROUTE (rw) register accessor: ASYNCH4 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch4route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch4route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch4route`] module"]
#[doc(alias = "PRS0_ASYNCH4ROUTE")]
pub type Prs0Asynch4route = crate::Reg<prs0_asynch4route::Prs0Asynch4routeSpec>;
#[doc = "ASYNCH4 port/pin select"]
pub mod prs0_asynch4route;
#[doc = "PRS0_ASYNCH5ROUTE (rw) register accessor: ASYNCH5 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch5route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch5route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch5route`] module"]
#[doc(alias = "PRS0_ASYNCH5ROUTE")]
pub type Prs0Asynch5route = crate::Reg<prs0_asynch5route::Prs0Asynch5routeSpec>;
#[doc = "ASYNCH5 port/pin select"]
pub mod prs0_asynch5route;
#[doc = "PRS0_ASYNCH6ROUTE (rw) register accessor: ASYNCH6 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch6route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch6route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch6route`] module"]
#[doc(alias = "PRS0_ASYNCH6ROUTE")]
pub type Prs0Asynch6route = crate::Reg<prs0_asynch6route::Prs0Asynch6routeSpec>;
#[doc = "ASYNCH6 port/pin select"]
pub mod prs0_asynch6route;
#[doc = "PRS0_ASYNCH7ROUTE (rw) register accessor: ASYNCH7 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch7route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch7route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch7route`] module"]
#[doc(alias = "PRS0_ASYNCH7ROUTE")]
pub type Prs0Asynch7route = crate::Reg<prs0_asynch7route::Prs0Asynch7routeSpec>;
#[doc = "ASYNCH7 port/pin select"]
pub mod prs0_asynch7route;
#[doc = "PRS0_ASYNCH8ROUTE (rw) register accessor: ASYNCH8 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch8route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch8route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch8route`] module"]
#[doc(alias = "PRS0_ASYNCH8ROUTE")]
pub type Prs0Asynch8route = crate::Reg<prs0_asynch8route::Prs0Asynch8routeSpec>;
#[doc = "ASYNCH8 port/pin select"]
pub mod prs0_asynch8route;
#[doc = "PRS0_ASYNCH9ROUTE (rw) register accessor: ASYNCH9 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch9route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch9route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch9route`] module"]
#[doc(alias = "PRS0_ASYNCH9ROUTE")]
pub type Prs0Asynch9route = crate::Reg<prs0_asynch9route::Prs0Asynch9routeSpec>;
#[doc = "ASYNCH9 port/pin select"]
pub mod prs0_asynch9route;
#[doc = "PRS0_ASYNCH10ROUTE (rw) register accessor: ASYNCH10 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch10route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch10route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch10route`] module"]
#[doc(alias = "PRS0_ASYNCH10ROUTE")]
pub type Prs0Asynch10route = crate::Reg<prs0_asynch10route::Prs0Asynch10routeSpec>;
#[doc = "ASYNCH10 port/pin select"]
pub mod prs0_asynch10route;
#[doc = "PRS0_ASYNCH11ROUTE (rw) register accessor: ASYNCH11 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_asynch11route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_asynch11route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_asynch11route`] module"]
#[doc(alias = "PRS0_ASYNCH11ROUTE")]
pub type Prs0Asynch11route = crate::Reg<prs0_asynch11route::Prs0Asynch11routeSpec>;
#[doc = "ASYNCH11 port/pin select"]
pub mod prs0_asynch11route;
#[doc = "PRS0_SYNCH0ROUTE (rw) register accessor: SYNCH0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_synch0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_synch0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_synch0route`] module"]
#[doc(alias = "PRS0_SYNCH0ROUTE")]
pub type Prs0Synch0route = crate::Reg<prs0_synch0route::Prs0Synch0routeSpec>;
#[doc = "SYNCH0 port/pin select"]
pub mod prs0_synch0route;
#[doc = "PRS0_SYNCH1ROUTE (rw) register accessor: SYNCH1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_synch1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_synch1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_synch1route`] module"]
#[doc(alias = "PRS0_SYNCH1ROUTE")]
pub type Prs0Synch1route = crate::Reg<prs0_synch1route::Prs0Synch1routeSpec>;
#[doc = "SYNCH1 port/pin select"]
pub mod prs0_synch1route;
#[doc = "PRS0_SYNCH2ROUTE (rw) register accessor: SYNCH2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_synch2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_synch2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_synch2route`] module"]
#[doc(alias = "PRS0_SYNCH2ROUTE")]
pub type Prs0Synch2route = crate::Reg<prs0_synch2route::Prs0Synch2routeSpec>;
#[doc = "SYNCH2 port/pin select"]
pub mod prs0_synch2route;
#[doc = "PRS0_SYNCH3ROUTE (rw) register accessor: SYNCH3 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`prs0_synch3route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prs0_synch3route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prs0_synch3route`] module"]
#[doc(alias = "PRS0_SYNCH3ROUTE")]
pub type Prs0Synch3route = crate::Reg<prs0_synch3route::Prs0Synch3routeSpec>;
#[doc = "SYNCH3 port/pin select"]
pub mod prs0_synch3route;
#[doc = "SYXO0_BUFOUTREQINASYNCROUTE (rw) register accessor: BUFOUTREQINASYNC port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`syxo0_bufoutreqinasyncroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syxo0_bufoutreqinasyncroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syxo0_bufoutreqinasyncroute`] module"]
#[doc(alias = "SYXO0_BUFOUTREQINASYNCROUTE")]
pub type Syxo0Bufoutreqinasyncroute =
    crate::Reg<syxo0_bufoutreqinasyncroute::Syxo0BufoutreqinasyncrouteSpec>;
#[doc = "BUFOUTREQINASYNC port/pin select"]
pub mod syxo0_bufoutreqinasyncroute;
#[doc = "TIMER0_ROUTEEN (rw) register accessor: TIMER0 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_routeen`] module"]
#[doc(alias = "TIMER0_ROUTEEN")]
pub type Timer0Routeen = crate::Reg<timer0_routeen::Timer0RouteenSpec>;
#[doc = "TIMER0 pin enable"]
pub mod timer0_routeen;
#[doc = "TIMER0_CC0ROUTE (rw) register accessor: CC0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_cc0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_cc0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cc0route`] module"]
#[doc(alias = "TIMER0_CC0ROUTE")]
pub type Timer0Cc0route = crate::Reg<timer0_cc0route::Timer0Cc0routeSpec>;
#[doc = "CC0 port/pin select"]
pub mod timer0_cc0route;
#[doc = "TIMER0_CC1ROUTE (rw) register accessor: CC1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_cc1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_cc1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cc1route`] module"]
#[doc(alias = "TIMER0_CC1ROUTE")]
pub type Timer0Cc1route = crate::Reg<timer0_cc1route::Timer0Cc1routeSpec>;
#[doc = "CC1 port/pin select"]
pub mod timer0_cc1route;
#[doc = "TIMER0_CC2ROUTE (rw) register accessor: CC2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_cc2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_cc2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cc2route`] module"]
#[doc(alias = "TIMER0_CC2ROUTE")]
pub type Timer0Cc2route = crate::Reg<timer0_cc2route::Timer0Cc2routeSpec>;
#[doc = "CC2 port/pin select"]
pub mod timer0_cc2route;
#[doc = "TIMER0_CDTI0ROUTE (rw) register accessor: CDTI0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_cdti0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_cdti0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cdti0route`] module"]
#[doc(alias = "TIMER0_CDTI0ROUTE")]
pub type Timer0Cdti0route = crate::Reg<timer0_cdti0route::Timer0Cdti0routeSpec>;
#[doc = "CDTI0 port/pin select"]
pub mod timer0_cdti0route;
#[doc = "TIMER0_CDTI1ROUTE (rw) register accessor: CDTI1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_cdti1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_cdti1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cdti1route`] module"]
#[doc(alias = "TIMER0_CDTI1ROUTE")]
pub type Timer0Cdti1route = crate::Reg<timer0_cdti1route::Timer0Cdti1routeSpec>;
#[doc = "CDTI1 port/pin select"]
pub mod timer0_cdti1route;
#[doc = "TIMER0_CDTI2ROUTE (rw) register accessor: CDTI2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_cdti2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_cdti2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cdti2route`] module"]
#[doc(alias = "TIMER0_CDTI2ROUTE")]
pub type Timer0Cdti2route = crate::Reg<timer0_cdti2route::Timer0Cdti2routeSpec>;
#[doc = "CDTI2 port/pin select"]
pub mod timer0_cdti2route;
#[doc = "TIMER1_ROUTEEN (rw) register accessor: TIMER1 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_routeen`] module"]
#[doc(alias = "TIMER1_ROUTEEN")]
pub type Timer1Routeen = crate::Reg<timer1_routeen::Timer1RouteenSpec>;
#[doc = "TIMER1 pin enable"]
pub mod timer1_routeen;
#[doc = "TIMER1_CC0ROUTE (rw) register accessor: CC0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_cc0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_cc0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cc0route`] module"]
#[doc(alias = "TIMER1_CC0ROUTE")]
pub type Timer1Cc0route = crate::Reg<timer1_cc0route::Timer1Cc0routeSpec>;
#[doc = "CC0 port/pin select"]
pub mod timer1_cc0route;
#[doc = "TIMER1_CC1ROUTE (rw) register accessor: CC1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_cc1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_cc1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cc1route`] module"]
#[doc(alias = "TIMER1_CC1ROUTE")]
pub type Timer1Cc1route = crate::Reg<timer1_cc1route::Timer1Cc1routeSpec>;
#[doc = "CC1 port/pin select"]
pub mod timer1_cc1route;
#[doc = "TIMER1_CC2ROUTE (rw) register accessor: CC2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_cc2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_cc2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cc2route`] module"]
#[doc(alias = "TIMER1_CC2ROUTE")]
pub type Timer1Cc2route = crate::Reg<timer1_cc2route::Timer1Cc2routeSpec>;
#[doc = "CC2 port/pin select"]
pub mod timer1_cc2route;
#[doc = "TIMER1_CDTI0ROUTE (rw) register accessor: CDTI0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_cdti0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_cdti0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cdti0route`] module"]
#[doc(alias = "TIMER1_CDTI0ROUTE")]
pub type Timer1Cdti0route = crate::Reg<timer1_cdti0route::Timer1Cdti0routeSpec>;
#[doc = "CDTI0 port/pin select"]
pub mod timer1_cdti0route;
#[doc = "TIMER1_CDTI1ROUTE (rw) register accessor: CDTI1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_cdti1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_cdti1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cdti1route`] module"]
#[doc(alias = "TIMER1_CDTI1ROUTE")]
pub type Timer1Cdti1route = crate::Reg<timer1_cdti1route::Timer1Cdti1routeSpec>;
#[doc = "CDTI1 port/pin select"]
pub mod timer1_cdti1route;
#[doc = "TIMER1_CDTI2ROUTE (rw) register accessor: CDTI2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_cdti2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_cdti2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cdti2route`] module"]
#[doc(alias = "TIMER1_CDTI2ROUTE")]
pub type Timer1Cdti2route = crate::Reg<timer1_cdti2route::Timer1Cdti2routeSpec>;
#[doc = "CDTI2 port/pin select"]
pub mod timer1_cdti2route;
#[doc = "TIMER2_ROUTEEN (rw) register accessor: TIMER2 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_routeen`] module"]
#[doc(alias = "TIMER2_ROUTEEN")]
pub type Timer2Routeen = crate::Reg<timer2_routeen::Timer2RouteenSpec>;
#[doc = "TIMER2 pin enable"]
pub mod timer2_routeen;
#[doc = "TIMER2_CC0ROUTE (rw) register accessor: CC0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_cc0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_cc0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cc0route`] module"]
#[doc(alias = "TIMER2_CC0ROUTE")]
pub type Timer2Cc0route = crate::Reg<timer2_cc0route::Timer2Cc0routeSpec>;
#[doc = "CC0 port/pin select"]
pub mod timer2_cc0route;
#[doc = "TIMER2_CC1ROUTE (rw) register accessor: CC1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_cc1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_cc1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cc1route`] module"]
#[doc(alias = "TIMER2_CC1ROUTE")]
pub type Timer2Cc1route = crate::Reg<timer2_cc1route::Timer2Cc1routeSpec>;
#[doc = "CC1 port/pin select"]
pub mod timer2_cc1route;
#[doc = "TIMER2_CC2ROUTE (rw) register accessor: CC2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_cc2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_cc2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cc2route`] module"]
#[doc(alias = "TIMER2_CC2ROUTE")]
pub type Timer2Cc2route = crate::Reg<timer2_cc2route::Timer2Cc2routeSpec>;
#[doc = "CC2 port/pin select"]
pub mod timer2_cc2route;
#[doc = "TIMER2_CDTI0ROUTE (rw) register accessor: CDTI0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_cdti0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_cdti0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cdti0route`] module"]
#[doc(alias = "TIMER2_CDTI0ROUTE")]
pub type Timer2Cdti0route = crate::Reg<timer2_cdti0route::Timer2Cdti0routeSpec>;
#[doc = "CDTI0 port/pin select"]
pub mod timer2_cdti0route;
#[doc = "TIMER2_CDTI1ROUTE (rw) register accessor: CDTI1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_cdti1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_cdti1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cdti1route`] module"]
#[doc(alias = "TIMER2_CDTI1ROUTE")]
pub type Timer2Cdti1route = crate::Reg<timer2_cdti1route::Timer2Cdti1routeSpec>;
#[doc = "CDTI1 port/pin select"]
pub mod timer2_cdti1route;
#[doc = "TIMER2_CDTI2ROUTE (rw) register accessor: CDTI2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2_cdti2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2_cdti2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cdti2route`] module"]
#[doc(alias = "TIMER2_CDTI2ROUTE")]
pub type Timer2Cdti2route = crate::Reg<timer2_cdti2route::Timer2Cdti2routeSpec>;
#[doc = "CDTI2 port/pin select"]
pub mod timer2_cdti2route;
#[doc = "TIMER3_ROUTEEN (rw) register accessor: TIMER3 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_routeen`] module"]
#[doc(alias = "TIMER3_ROUTEEN")]
pub type Timer3Routeen = crate::Reg<timer3_routeen::Timer3RouteenSpec>;
#[doc = "TIMER3 pin enable"]
pub mod timer3_routeen;
#[doc = "TIMER3_CC0ROUTE (rw) register accessor: CC0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_cc0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_cc0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_cc0route`] module"]
#[doc(alias = "TIMER3_CC0ROUTE")]
pub type Timer3Cc0route = crate::Reg<timer3_cc0route::Timer3Cc0routeSpec>;
#[doc = "CC0 port/pin select"]
pub mod timer3_cc0route;
#[doc = "TIMER3_CC1ROUTE (rw) register accessor: CC1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_cc1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_cc1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_cc1route`] module"]
#[doc(alias = "TIMER3_CC1ROUTE")]
pub type Timer3Cc1route = crate::Reg<timer3_cc1route::Timer3Cc1routeSpec>;
#[doc = "CC1 port/pin select"]
pub mod timer3_cc1route;
#[doc = "TIMER3_CC2ROUTE (rw) register accessor: CC2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_cc2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_cc2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_cc2route`] module"]
#[doc(alias = "TIMER3_CC2ROUTE")]
pub type Timer3Cc2route = crate::Reg<timer3_cc2route::Timer3Cc2routeSpec>;
#[doc = "CC2 port/pin select"]
pub mod timer3_cc2route;
#[doc = "TIMER3_CDTI0ROUTE (rw) register accessor: CDTI0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_cdti0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_cdti0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_cdti0route`] module"]
#[doc(alias = "TIMER3_CDTI0ROUTE")]
pub type Timer3Cdti0route = crate::Reg<timer3_cdti0route::Timer3Cdti0routeSpec>;
#[doc = "CDTI0 port/pin select"]
pub mod timer3_cdti0route;
#[doc = "TIMER3_CDTI1ROUTE (rw) register accessor: CDTI1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_cdti1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_cdti1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_cdti1route`] module"]
#[doc(alias = "TIMER3_CDTI1ROUTE")]
pub type Timer3Cdti1route = crate::Reg<timer3_cdti1route::Timer3Cdti1routeSpec>;
#[doc = "CDTI1 port/pin select"]
pub mod timer3_cdti1route;
#[doc = "TIMER3_CDTI2ROUTE (rw) register accessor: CDTI2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3_cdti2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3_cdti2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer3_cdti2route`] module"]
#[doc(alias = "TIMER3_CDTI2ROUTE")]
pub type Timer3Cdti2route = crate::Reg<timer3_cdti2route::Timer3Cdti2routeSpec>;
#[doc = "CDTI2 port/pin select"]
pub mod timer3_cdti2route;
#[doc = "TIMER4_ROUTEEN (rw) register accessor: TIMER4 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4_routeen`] module"]
#[doc(alias = "TIMER4_ROUTEEN")]
pub type Timer4Routeen = crate::Reg<timer4_routeen::Timer4RouteenSpec>;
#[doc = "TIMER4 pin enable"]
pub mod timer4_routeen;
#[doc = "TIMER4_CC0ROUTE (rw) register accessor: CC0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_cc0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_cc0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4_cc0route`] module"]
#[doc(alias = "TIMER4_CC0ROUTE")]
pub type Timer4Cc0route = crate::Reg<timer4_cc0route::Timer4Cc0routeSpec>;
#[doc = "CC0 port/pin select"]
pub mod timer4_cc0route;
#[doc = "TIMER4_CC1ROUTE (rw) register accessor: CC1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_cc1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_cc1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4_cc1route`] module"]
#[doc(alias = "TIMER4_CC1ROUTE")]
pub type Timer4Cc1route = crate::Reg<timer4_cc1route::Timer4Cc1routeSpec>;
#[doc = "CC1 port/pin select"]
pub mod timer4_cc1route;
#[doc = "TIMER4_CC2ROUTE (rw) register accessor: CC2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_cc2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_cc2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4_cc2route`] module"]
#[doc(alias = "TIMER4_CC2ROUTE")]
pub type Timer4Cc2route = crate::Reg<timer4_cc2route::Timer4Cc2routeSpec>;
#[doc = "CC2 port/pin select"]
pub mod timer4_cc2route;
#[doc = "TIMER4_CDTI0ROUTE (rw) register accessor: CDTI0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_cdti0route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_cdti0route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4_cdti0route`] module"]
#[doc(alias = "TIMER4_CDTI0ROUTE")]
pub type Timer4Cdti0route = crate::Reg<timer4_cdti0route::Timer4Cdti0routeSpec>;
#[doc = "CDTI0 port/pin select"]
pub mod timer4_cdti0route;
#[doc = "TIMER4_CDTI1ROUTE (rw) register accessor: CDTI1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_cdti1route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_cdti1route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4_cdti1route`] module"]
#[doc(alias = "TIMER4_CDTI1ROUTE")]
pub type Timer4Cdti1route = crate::Reg<timer4_cdti1route::Timer4Cdti1routeSpec>;
#[doc = "CDTI1 port/pin select"]
pub mod timer4_cdti1route;
#[doc = "TIMER4_CDTI2ROUTE (rw) register accessor: CDTI2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_cdti2route::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_cdti2route::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer4_cdti2route`] module"]
#[doc(alias = "TIMER4_CDTI2ROUTE")]
pub type Timer4Cdti2route = crate::Reg<timer4_cdti2route::Timer4Cdti2routeSpec>;
#[doc = "CDTI2 port/pin select"]
pub mod timer4_cdti2route;
#[doc = "USART0_ROUTEEN (rw) register accessor: USART0 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`usart0_routeen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart0_routeen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart0_routeen`] module"]
#[doc(alias = "USART0_ROUTEEN")]
pub type Usart0Routeen = crate::Reg<usart0_routeen::Usart0RouteenSpec>;
#[doc = "USART0 pin enable"]
pub mod usart0_routeen;
#[doc = "USART0_CSROUTE (rw) register accessor: CS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`usart0_csroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart0_csroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart0_csroute`] module"]
#[doc(alias = "USART0_CSROUTE")]
pub type Usart0Csroute = crate::Reg<usart0_csroute::Usart0CsrouteSpec>;
#[doc = "CS port/pin select"]
pub mod usart0_csroute;
#[doc = "USART0_CTSROUTE (rw) register accessor: CTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`usart0_ctsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart0_ctsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart0_ctsroute`] module"]
#[doc(alias = "USART0_CTSROUTE")]
pub type Usart0Ctsroute = crate::Reg<usart0_ctsroute::Usart0CtsrouteSpec>;
#[doc = "CTS port/pin select"]
pub mod usart0_ctsroute;
#[doc = "USART0_RTSROUTE (rw) register accessor: RTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`usart0_rtsroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart0_rtsroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart0_rtsroute`] module"]
#[doc(alias = "USART0_RTSROUTE")]
pub type Usart0Rtsroute = crate::Reg<usart0_rtsroute::Usart0RtsrouteSpec>;
#[doc = "RTS port/pin select"]
pub mod usart0_rtsroute;
#[doc = "USART0_RXROUTE (rw) register accessor: RX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`usart0_rxroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart0_rxroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart0_rxroute`] module"]
#[doc(alias = "USART0_RXROUTE")]
pub type Usart0Rxroute = crate::Reg<usart0_rxroute::Usart0RxrouteSpec>;
#[doc = "RX port/pin select"]
pub mod usart0_rxroute;
#[doc = "USART0_CLKROUTE (rw) register accessor: SCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`usart0_clkroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart0_clkroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart0_clkroute`] module"]
#[doc(alias = "USART0_CLKROUTE")]
pub type Usart0Clkroute = crate::Reg<usart0_clkroute::Usart0ClkrouteSpec>;
#[doc = "SCLK port/pin select"]
pub mod usart0_clkroute;
#[doc = "USART0_TXROUTE (rw) register accessor: TX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`usart0_txroute::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart0_txroute::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usart0_txroute`] module"]
#[doc(alias = "USART0_TXROUTE")]
pub type Usart0Txroute = crate::Reg<usart0_txroute::Usart0TxrouteSpec>;
#[doc = "TX port/pin select"]
pub mod usart0_txroute;
