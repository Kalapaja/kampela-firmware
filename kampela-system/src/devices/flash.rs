use efm32pg23_fix::{Peripherals};
use crate::peripherals::usart::*;
use cortex_m::asm::delay;


#[allow(dead_code)]
enum FlashCommand {
    WriteEnable = 0x06, /* 06 xx xx xx xx sets the (WEL) write enable latch bit */
    WriteDisable = 0x04, /* 04 xx xx xx xx resets the (WEL) write enable latch bit*/
    ReadId = 0x9f, /* 9f xx xx xx xx outputs JEDEC ID: 1 byte manufacturer ID & 2 byte device ID */
    ReadDiscoverableParameters = 0x5A, /* 5A xx xx xx xx Serial Flash Discoverable Parameters */
    ReadStatusRegister = 0x05, /* 05 xx xx xx xx to read out the values of the status register */
    WriteStatusRegister = 0x01, /* 01 xx xx xx xx to write new values to the status register */
    Read = 0x03, /* 03 a1 a2 a3 xx n bytes read out until CS# goes high */
    FastRead = 0x0b, /* 0b a1 a2 a3 dd n bytes read out until CS# goes high */
    Read2 = 0xbb, /* bb 12 3d xx xx n bytes read out by 2 I/O until CS# goes high */
    Read4 = 0xeb, /* eb 3a 3d xx xx n bytes read out by 4 x I/O until CS# goes high */
    ErasePage = 0x81, /* 20 a1 a2 a3 xx to erase the selected page 256 bytes */
    EraseSector = 0x20, /* 20 a1 a2 a3 xx to erase the selected sector */
    EraseBlock = 0xd8, /* d8 a1 a2 a3 xx to erase the selected block */
    EraseChip = 0x60, /* 60 xx xx xx xx to erase whole chip (cmd or 0xc7) */
    WritePage = 0x02, /* 02 a1 a2 a3 xx to program the selected page */
    WritePage4 = 0x38, /* 38 3a 3d xx xx quad input to program the selected page */
    WriteContinously = 0xad, /* ad a1 a2 a3 xx continously program whole chip, the address is automaticlly increase */
    PowerDown = 0xb9, /* b9 xx xx xx xx enters deep power down mode */
    UltraDeepPowerDown = 0x79, /* 79 Ultra-Deep Power-Down mode */
    ResumeFromPowerDown = 0xab, /* ab xx xx xx xx release from deep power down mode */
    ReadIdMfid = 0x90, /* 90 ?? ?? ?? xx output the manufacter ID & device ID */
    EnterSecuredMode = 0xb1, /* b1 xx xx xx xx to enter the 512 bit secured OTP mode */
    ExitSecuredMode = 0xc1, /* c1 xx xx xx xx to exit the 512 bit secured OTP mode */
    ReadSecuredRegister = 0x2b, /* 2b xx xx xx xx to read value of secured register */
    WriteSecuredRegister = 0x2f, /* 2f xx xx xx xx to set the lock down bit as "1" (once lock down, can not be updated) */
    EnableStatusOutput = 0x70,/* 70 xx xx xx xx to enable SO to output RY/BY# during CP mode */
    DisableStatusOutput = 0x80,/* 80 xx xx xx xx to disable SO to output RY/BY# during CP mode */
    ActiveStatus = 0x25, /* Outputs ready/busy state to data output pin */
    EnableSoftReset = 0x66,
    SoftReset = 0x99,
}



fn flash_cmd(peripherals: &mut Peripherals, cmd: FlashCommand) {
    write_to_usart(peripherals, cmd as u8);
}


fn flash_write_some(peripherals: &mut Peripherals, command_set: &[u8]) {
    for command in command_set.iter() {
        write_to_usart(peripherals, *command);
    }
}



fn flash_read_some(peripherals: &mut Peripherals, data: &mut [u8]) {
    for v in data.iter_mut() {
        *v = write_to_usart(peripherals, 0);
    }
}

fn flash_read_u32(peripherals: &mut Peripherals) -> u32 {
    let mut res: u32 = 0;
    for _ in 0..4 {
        res >>= 8;
        res |= (write_to_usart(peripherals, 0) as u32) << 24;
    }
    res
}


pub fn flash_init(peripherals: &mut Peripherals) {
    deselect_flash(&mut peripherals.GPIO_S);
    write_to_usart(peripherals, 0); // for delay

    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::EnableSoftReset);
    deselect_flash(&mut peripherals.GPIO_S);    
    write_to_usart(peripherals, 0); // for delay

    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::SoftReset);
    deselect_flash(&mut peripherals.GPIO_S);
    // TODO: check if it's possible to determine readiness instead of using delay
    delay(1000);
}

pub fn flash_sleep(peripherals: &mut Peripherals) {
    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::UltraDeepPowerDown);
    deselect_flash(&mut peripherals.GPIO_S);
}

pub fn flash_wakeup(peripherals: &mut Peripherals) {
    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::ResumeFromPowerDown);
    deselect_flash(&mut peripherals.GPIO_S);
    // TODO: check if it's possible to determine readiness instead of using delay
    delay(10000);
}

pub fn flash_unlock(peripherals: &mut Peripherals) {
    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::WriteEnable);
    deselect_flash(&mut peripherals.GPIO_S);
}

pub fn flash_lock(peripherals: &mut Peripherals) {
    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::WriteDisable);
    deselect_flash(&mut peripherals.GPIO_S);
}

macro_rules! flash_write_addr {
    ( $( $periph: tt, $addr: tt ),* ) => {
        $(
            flash_write_some($periph, &[(($addr>>16)&0xff) as u8, (($addr>>8)&0xff) as u8, ($addr&0xff)as u8]);
        )*
    };
}

#[allow(dead_code)]
enum StatusRegister {
    SR1 = 0x01,
    SR2 = 0x02,
    SR3 = 0x03,
    SR4 = 0x04,
    SR5 = 0x05,
}

#[allow(dead_code)]
enum StatusRegister1 {
    /**
        * SRP0 works with the SRP1 bit in Status Register 1 and the WP
        * pin to control write protection. Types of protection include software
        * protection, hardware protection, one-time programmable (OTP)
        * protection, and power supply lock-down protectio
        */
    StatusRegisterProtect0 = 1<<7,
    /**
        * BPSIZE controls the size of the blocks protected by the Block
        * Protect Bits (BP2, BP1, BP0 in bits 4:2 of this register). Its
        * encoding is:
        * 0: 64 kB block size
        * 1: 4 kB block size
        * The blocks can be protected from the bottom up, or from the top
        * down, as described in the TB bit of this register
        */
    BlockProtectSize0 = 1<<6,
    /**
        * TB controls the direction of the blocks to be protected by the
        * Block Protect Bits (BP2, BP1, BP0 in bits 4:2 of this register). Its
        * encoding is:
        * 0: Protect from bottom up
        * 1: Protect from top down
        * The size of the protected blocks can also be selected, as
        * described in the BPSIZE bit of this register.
        */
    TopBottom = 1<<5,
    /**
        * The Block Protect field provides write protection control and
        * status. These bits are set using the Write Status Register 1 (01h)
        * command. This field can be programmed to protect all, none, or a
        * portion of the memory array. When that portion of the memory is
        * selected, it is protected from the Program and Erase commands
        * as described in the Memory Protection table.
        * The default is 3’b000 for this field, indicating that none of the
        * memory array is protected.
        */
    BlockProtect2 = 1<<4,
    BlockProtect1 = 1<<3,
    BlockProtect0 = 1<<2,
    /**
        * WEL gives the current status of the internal Write Enable Latch.
        * When WEL is logic 0, the device does not accept any program,
        * erase, memory protection, or Write Status Register commands.
        * WEL defaults to logic 0 after a device power-up or reset.
        * Its encoding is:
        * 0: Device is not write enabled (default).
        * 1: Device is write enabled.
        * If WEL is 1, it is not reset to a logic 0 if an operation aborts due to
        * an incomplete or unrecognized command being clocked into the
        * device before the CS pin is deasserted.
        * To reset the WEL bit when an operation aborts prematurely, the
        * entire command for a program, erase, memory protection, or Write
        * Status Register command must have been clocked into the
        * device.
        * When the Write Enable (06h) command is executed, the WEL bit
        * is set. Conversely, when the Volatile Status Register Write Enable
        * (50h) command is executed, the WEL bit is not set
        */
    WriteEnableLatchStatus = 1<<1,
    /**
        * RDY/BSY determines if an internal operation, such as a program
        * or erase, is in progress. To poll the RDY/BSY bit to detect the
        * completion of a program or erase cycle, new Status Register data
        * must be continually clocked out of the device until the state of the
        * RDY/BSY bit changes from a logic 1 to a logic 0.
        * Its encoding is:
        * 0: Device is ready.
        * 1: Device is busy with an internal operation.
        */
    BusyStatus = 1<<0,
}

fn flash_read_status(peripherals: &mut Peripherals, reg: StatusRegister) -> u8 {
    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::ReadStatusRegister);
    flash_write_some(peripherals, &[reg as u8]);    
    let res = write_to_usart(peripherals, 0);
    deselect_flash(&mut peripherals.GPIO_S);
    res
}

pub fn flash_wait_ready(peripherals: &mut Peripherals) {
    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::ActiveStatus);
    while write_to_usart(peripherals, 0) != 0 {}
    deselect_flash(&mut peripherals.GPIO_S);
}

pub fn flash_erase_page(peripherals: &mut Peripherals, addr: u32) {
    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::ErasePage);
    flash_write_addr!(peripherals, addr);
    deselect_flash(&mut peripherals.GPIO_S);
}


pub fn flash_write_page(peripherals: &mut Peripherals, addr: u32, data: &[u8]) {
    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::WritePage);
    flash_write_addr!(peripherals, addr);
    let xfer_len = if 256 < data.len() { 256 } else { data.len() };
    flash_write_some(peripherals, &data[0..xfer_len]);    
    deselect_flash(&mut peripherals.GPIO_S);
}

pub fn flash_get_size(peripherals: &mut Peripherals) -> u32 {
    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::ReadDiscoverableParameters);
    let mut jdt_head: [u8; 12] = [0; 12];
    flash_read_some(peripherals, &mut jdt_head);
    let size_offset = ((1 + jdt_head[11]) as usize) << 3 + 4;    
    for _ in 0..size_offset {
        write_to_usart(peripherals, 0);
    }
    let res = flash_read_u32(peripherals);
    deselect_flash(&mut peripherals.GPIO_S);
    (res + 1) >> 13
}

pub fn flash_get_id(peripherals: &mut Peripherals) -> u32 {
    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::ReadId);
    let res = flash_read_u32(peripherals);
    deselect_flash(&mut peripherals.GPIO_S);
    res
}

pub fn flash_read(peripherals: &mut Peripherals, addr: u32, data: &mut [u8]) {
    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::Read);
    flash_write_addr!(peripherals, addr);
    flash_read_some(peripherals, data);
    deselect_flash(&mut peripherals.GPIO_S);
}