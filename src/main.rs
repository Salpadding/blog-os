#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(deprecated)]

use core::panic::PanicInfo;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    {
        use blog_os::{serial_println};
        serial_println!("{}", "hello main.rs");
        test_main();
    }
    loop {}
}


// our existing panic handler
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use blog_os::println;
    println!("{}", info);
    loop {}
}


// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}


#[cfg(test)]
mod test {
    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}
