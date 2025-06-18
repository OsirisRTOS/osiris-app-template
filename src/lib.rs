#![no_std]

pub fn hello() -> u32 {
    42
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
