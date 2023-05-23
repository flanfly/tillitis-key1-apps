#![no_std]
#![no_main]
#![feature(start)]

use core::{
    panic::PanicInfo,
    arch::asm,
};

const TK1_MMIO_BASE: u32      = 0xc0000000;
const TK1_MMIO_TK1_BASE: u32         = TK1_MMIO_BASE | 0x3f000000;
const TK1_MMIO_UART_BASE: u32        = TK1_MMIO_BASE | 0x03000000;

#[repr(u32)]
enum Mmio {
    UartBitRate = TK1_MMIO_UART_BASE | 0x40,
    UartDataBits = TK1_MMIO_UART_BASE | 0x44,
    UartStopBits = TK1_MMIO_UART_BASE | 0x48,
    UartRxStatus = TK1_MMIO_UART_BASE | 0x80,
    UartRxData = TK1_MMIO_UART_BASE | 0x84,
    UartRxBytes  = TK1_MMIO_UART_BASE | 0x88,
    UartTxStatus = TK1_MMIO_UART_BASE | 0x100,
    UartTxData = TK1_MMIO_UART_BASE | 0x104,

    Led = TK1_MMIO_TK1_BASE | 0x24,
}

fn peek(addr: Mmio) -> u32 {
    unsafe {
        core::ptr::read_volatile(addr as u32 as *const u32)
    }
}

fn poke(addr: Mmio, data: u32) {
    unsafe {
        core::ptr::write_volatile(addr as u32 as *mut u32, data);
    }
}

fn tx(data: &[u8]) {
    for byte in data {
        while peek(Mmio::UartTxStatus) == 0 {}
        poke(Mmio::UartTxData, *byte as u32);
    }
}

fn sleep(cycles: usize) {
    for _ in 0..cycles {
        unsafe { asm!("nop"); }
    }
}

const TK1_MMIO_TK1_LED_R_BIT: u32    = 2;
const TK1_MMIO_TK1_LED_G_BIT: u32    = 1;
const TK1_MMIO_TK1_LED_B_BIT: u32    = 0;
   
#[no_mangle]
#[start]
pub extern "C" fn main() -> ! {
    let sleep_time = 100000;
    loop {
        tx(b"Hello, world!\n\r");
        //poke(Mmio::Led, 1 << TK1_MMIO_TK1_LED_R_BIT);
        //sleep(sleep_time);
        //poke(Mmio::Led, 1 << TK1_MMIO_TK1_LED_G_BIT);
        //sleep(sleep_time);
        //poke(Mmio::Led, 1 << TK1_MMIO_TK1_LED_B_BIT);
        //sleep(sleep_time);
    }
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
