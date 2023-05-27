#![no_std]
#![no_main]
#![feature(start)]
#![feature(global_allocator, allocator_api)]

extern crate k256;

use core::{
    panic::PanicInfo,
    arch::asm,
};

use k256::{
    SecretKey,
    elliptic_curve::sec1::ToEncodedPoint,
};
use bumpalo::{boxed::Box, Bump};

const TK1_MMIO_BASE: u32 = 0xc0000000;
const TK1_MMIO_TK1_BASE: u32 = TK1_MMIO_BASE | 0x3f000000;
const TK1_MMIO_UART_BASE: u32 = TK1_MMIO_BASE | 0x03000000;

core::arch::global_asm!(include_str!("../../../../tkey-libs/libcrt0/crt0.S"));
use bumpalo::AllocOrInitError::Alloc;

use core::alloc::GlobalAlloc;

struct Yolo {}

unsafe impl GlobalAlloc for Yolo {
    unsafe fn alloc(&self, l: core::alloc::Layout) -> *mut u8 {
        let y = 0x4000_1000 as *mut u8;
        y
    }
    unsafe fn dealloc(&self, p: *mut u8, l: core::alloc::Layout) {}
}

#[global_allocator]
static GLOBAL: Yolo = Yolo {};

#[repr(u32)]
enum Mmio {
    UartBitRate = TK1_MMIO_UART_BASE | 0x40,
    UartDataBits = TK1_MMIO_UART_BASE | 0x44,
    UartStopBits = TK1_MMIO_UART_BASE | 0x48,
    UartRxStatus = TK1_MMIO_UART_BASE | 0x80,
    UartRxData = TK1_MMIO_UART_BASE | 0x84,
    UartRxBytes = TK1_MMIO_UART_BASE | 0x88,
    UartTxStatus = TK1_MMIO_UART_BASE | 0x100,
    UartTxData = TK1_MMIO_UART_BASE | 0x104,

    Led = TK1_MMIO_TK1_BASE | 0x24,
}

fn peek(addr: Mmio) -> u32 {
    unsafe { core::ptr::read_volatile(addr as u32 as *const u32) }
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
        unsafe {
            asm!("nop");
        }
    }
}

fn read32(addr: u32) -> u32 {
    unsafe { core::ptr::read_volatile(addr as u32 as *const u32) }
}

fn write32(addr: u32, v: u32) {
    unsafe { core::ptr::write_volatile(addr as u32 as *mut u32, v) };
}

const TK1_MMIO_TK1_LED_R_BIT: u32 = 2;
const TK1_MMIO_TK1_LED_G_BIT: u32 = 1;
const TK1_MMIO_TK1_LED_B_BIT: u32 = 0;

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

fn print_nibble(byte: u8) {
    let b = if byte < 10 { byte + 0x30 } else { byte + 0x37 };
    tx(&[b]);
}

// TODO: implement print! macro for rich formatting
fn print_byte(byte: u8) {
    let nibble0 = byte >> 4;
    let nibble1 = byte & 0xf;
    print_nibble(nibble0);
    print_nibble(nibble1);
}

const SLEEP_TIME: u32 = 100000;

const TK1_MMIO_UDS_BASE: u32 = TK1_MMIO_BASE + 0x0200_0000;

const TK1_MMIO_TRNG_BASE: u32 = TK1_MMIO_BASE + 0x0000_0000;
const TK1_MMIO_TRNG_STATUS: u32 = TK1_MMIO_TRNG_BASE + 0x0024;
const TK1_MMIO_TRNG_ENTROPY: u32 = TK1_MMIO_TRNG_BASE + 0x0080;

fn gen_key() -> [u8; 32] {
    let mut key: [u8; 32] = [1; 32];

    for o in (0..32).step_by(4) {
        while read32(TK1_MMIO_TRNG_STATUS) & 0x1 == 0 {}
        let rand = read32(TK1_MMIO_TRNG_ENTROPY);
        let o = o as usize;
        key[0 + o] = (rand >> 24) as u8;
        key[1 + o] = (rand >> 16) as u8;
        key[2 + o] = (rand >> 8) as u8;
        key[3 + o] = rand as u8;
    }
    key
}

#[no_mangle]
#[start]
pub extern "C" fn _start() -> ! {
    tx(b"Secret....\n\r");
    let key = gen_key();
    let bump = Bump::new();

    match SecretKey::from_slice(&key) {
        Ok(key) => {
            let key = key.public_key().to_sec1_bytes();
            for k in key.into_iter() {
                print_byte(*k);
            }
            tx(b"\n\r");
        }
        Err(e) => {
            tx(b"Error\n");
        }
    }

    tx(b"Hello, world!\n\r");
    loop {
        //poke(Mmio::Led, 1 << TK1_MMIO_TK1_LED_R_BIT);
        //sleep(sleep_time);
        //poke(Mmio::Led, 1 << TK1_MMIO_TK1_LED_G_BIT);
        //sleep(sleep_time);
        //poke(Mmio::Led, 1 << TK1_MMIO_TK1_LED_B_BIT);
        //sleep(sleep_time);
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
