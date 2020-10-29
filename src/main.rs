/*
Disabling the builtin library
*/
#![no_std] // seperate this program from standard library
#![no_main] //disable all rust level entry points


use core::panic::PanicInfo;


#[panic_handler]  //called on panic
fn panic(_info: &PanicInfo)-> !{
    loop{}
}

//static field
static HELLO: &[u8] = b"Hello world !! A..B..C";
//name of our entry point
#[no_mangle] //dont mangle the name of this function
pub extern "C" fn _start() -> !{
    let vga_buffer = 0xb8000 as *mut u8;
    for(i,&byte) in HELLO.iter().enumerate(){
        unsafe{
            *vga_buffer.offset(i as isize * 2)=byte;
            *vga_buffer.offset(i as isize * 2+1)= 0xe;
        }
    }
    //implementation
    loop {}
}

