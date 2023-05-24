#![no_std]
#![no_main]
#![feature(start)]

extern crate k256;

use core::{
    panic::PanicInfo,
    arch::asm,
};

use k256::{
    SecretKey,
    elliptic_curve::sec1::ToEncodedPoint,
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

struct MyRng();

impl rand_core::RngCore for MyRng {
    fn next_u32(&mut self) -> u32 {
        0xdeadbeef
    }

    fn next_u64(&mut self) -> u64 {
        0xdeadbeefdeadbeef
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for i in 0..dest.len() {
            dest[i] = 0x01;
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        for i in 0..dest.len() {
            dest[i] = 0x01;
        }
        Ok(())
    }
}

impl rand_core::CryptoRng for MyRng {}
   
#[no_mangle]
#[start]
pub extern "C" fn main() -> ! {
    tx(b"Tillitis Wallet App\n\r");
    
    let mut rng = MyRng();
    tx(b"Deriving secret key\n\r");
    let sec = SecretKey::random(&mut rng);

    tx(b"Computing public key\n\r");
    let key = sec.public_key();
    loop {
        tx(b"Public key\n\r");
        tx(key.to_encoded_point(false).as_bytes());
        tx(b"\n\r");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let sleep_time = 100000;

    loop {
        poke(Mmio::Led, 1 << TK1_MMIO_TK1_LED_R_BIT);
        sleep(sleep_time);
        poke(Mmio::Led, 0);
        sleep(sleep_time);
    }
}
