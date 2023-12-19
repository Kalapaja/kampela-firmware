use efm32pg23_fix::{Peripherals};
use crate::peripherals::usart::*;
use cortex_m::asm::delay;
use core::ops::{Shl, Shr};


#[repr(u8)]
enum FlashCommand {
    WriteEnable = 0x06, /* 06 xx xx xx xx sets the (WEL) write enable latch bit */
    WriteDisable = 0x04, /* 04 xx xx xx xx resets the (WEL) write enable latch bit*/
    ReadId = 0x9f, /* 9f xx xx xx xx outputs JEDEC ID: 1 byte manufacturer ID & 2 byte device ID */
    ReadDiscoverableParameters = 0x5A, /* 5A xx xx xx xx Serial Flash Discoverable Parameters */
    // ReadStatusRegister = 0x05, /* 05 xx xx xx xx to read out the values of the status register */
    ReadStatusRegisterAdressed = 0x65,
    // WriteStatusRegister = 0x01, /* 01 xx xx xx xx to write new values to the status register */
    WriteStatusRegisterAdressed = 0x71,
    Read = 0x03, /* 03 a1 a2 a3 xx n bytes read out until CS# goes high */
    // FastRead = 0x0b, /* 0b a1 a2 a3 dd n bytes read out until CS# goes high */
    // Read2 = 0xbb, /* bb 12 3d xx xx n bytes read out by 2 I/O until CS# goes high */
    // Read4 = 0xeb, /* eb 3a 3d xx xx n bytes read out by 4 x I/O until CS# goes high */
    ErasePage = 0x81, /* 20 a1 a2 a3 xx to erase the selected page 256 bytes */
    // EraseSector = 0x20, /* 20 a1 a2 a3 xx to erase the selected sector */
    // EraseBlock = 0xd8, /* d8 a1 a2 a3 xx to erase the selected block */
    // EraseChip = 0x60, /* 60 xx xx xx xx to erase whole chip (cmd or 0xc7) */
    WritePage = 0x02, /* 02 a1 a2 a3 xx to program the selected page */
    // WritePage4 = 0x38, /* 38 3a 3d xx xx quad input to program the selected page */
    // WriteContinously = 0xad, /* ad a1 a2 a3 xx continously program whole chip, the address is automaticlly increase */
    // DeepPowerDown = 0xb9, /* b9 xx xx xx xx enters deep power down mode */
    UltraDeepPowerDown = 0x79, /* 79 Ultra-Deep Power-Down mode */
    ResumeFromPowerDown = 0xab, /* ab xx xx xx xx release from deep power down mode */
    // ReadIdMfid = 0x90, /* 90 ?? ?? ?? xx output the manufacter ID & device ID */
    // EnterSecuredMode = 0xb1, /* b1 xx xx xx xx to enter the 512 bit secured OTP mode */
    // ExitSecuredMode = 0xc1, /* c1 xx xx xx xx to exit the 512 bit secured OTP mode */
    // ReadSecuredRegister = 0x2b, /* 2b xx xx xx xx to read value of secured register */
    // WriteSecuredRegister = 0x2f, /* 2f xx xx xx xx to set the lock down bit as "1" (once lock down, can not be updated) */
    // EnableStatusOutput = 0x70,/* 70 xx xx xx xx to enable SO to output RY/BY# during CP mode */
    // DisableStatusOutput = 0x80,/* 80 xx xx xx xx to disable SO to output RY/BY# during CP mode */
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
        res |= (write_to_usart(peripherals, 0) as u32).shl(24);
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
    delay(10000);
}

pub fn flash_sleep(peripherals: &mut Peripherals) {
    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::UltraDeepPowerDown);
    deselect_flash(&mut peripherals.GPIO_S);
}

pub fn flash_wakeup(peripherals: &mut Peripherals) {
    for _ in 0..2 {
        select_flash(&mut peripherals.GPIO_S);
        flash_cmd(peripherals, FlashCommand::ResumeFromPowerDown);
        deselect_flash(&mut peripherals.GPIO_S);
        // TODO: check if it's possible to determine readiness instead of using delay
        delay(10000);
    }
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
            flash_write_some($periph, &[($addr.shr(16) as u8)&0xff, ($addr.shr(8) as u8)&0xff, ($addr as u8)&0xff]);
        )*
    };
}

pub fn flash_read_sr(peripherals: &mut Peripherals) -> [u8;6] {

    let mut res = [0u8; 6];
    for i in 0..6 {
        select_flash(&mut peripherals.GPIO_S);
        flash_cmd(peripherals, FlashCommand::ReadStatusRegisterAdressed);
        flash_write_some(peripherals, &[i+1 as u8, 0u8]);
        let ind = i as usize;
        flash_read_some(peripherals, &mut res[ind..ind+1]);
        deselect_flash(&mut peripherals.GPIO_S);
    }

    res
}

pub fn flash_clear_sr(peripherals: &mut Peripherals) {
    for i in 0..6 {
        select_flash(&mut peripherals.GPIO_S);
        flash_cmd(peripherals, FlashCommand::WriteStatusRegisterAdressed);
        flash_write_some(peripherals, &[i+1 as u8, 0u8]);
        deselect_flash(&mut peripherals.GPIO_S);
    }
}


pub fn flash_wait_ready(peripherals: &mut Peripherals) {
    // while flash_read_status(peripherals, StatusRegister::SR1) & (StatusRegister1::BusyStatus as u8) != 0 {}
    select_flash(&mut peripherals.GPIO_S);
    flash_cmd(peripherals, FlashCommand::ActiveStatus);
    flash_write_some(peripherals, &[0_u8, 3]);
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
    let size_offset: usize = ((jdt_head[10] as usize) + 1).shl(3) + 4_usize;
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