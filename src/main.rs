#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}


// Pour dire à rust de build directement dans le ARM et non pour windows/linux: rustup target add thumbv7em-none-eabihf
// ,ce qui permet donc de downloader localement une copie de core et std

// Pour build sans link C runtime, puisqu'on est sur un bare metal os: cargo build --target thumbv7em-none-eabihf

//In order to test the OS, use the following qemu command: qemu-system-x86_64 -drive format=raw,file={thePathToTheOs.binFolder}
//          or simply run cargo run