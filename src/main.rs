#![no_main]
#![no_std]
#![feature(asm)]
#![feature(global_asm)]

#[link_section = ".reset.boot"]
global_asm!(r#"
.global _start
_start:
    lui sp, %hi(stack_end)
    ori sp, sp, %lo(stack_end)
    j   __start_rust
stack_start:
    .skip   1024
stack_end:
"#);

#[no_mangle]
pub extern "C" fn __start_rust() -> ! {
    let uart = 0x1001_3000 as *mut u8;
    for c in b"Hello from Rust!".iter() {
        unsafe {            
            *uart = *c as u8;
        }
    }

    unsafe { asm!("wfi" :::: "volatile"); }
    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {}
}

