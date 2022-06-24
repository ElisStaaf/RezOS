use core::mem::replace;
use x86::io;


const COMMON_COM1: u16 = 0x3F8;

static mut SERIAL: [Option<SerialHandle>; 8] = [None; 8];

pub fn init() {
    // bootboot setups serial debug on COM1
    unsafe { SERIAL[0] = Some(SerialHandle { port: COMMON_COM1 }) };
}

pub fn access(comport: usize) -> SerialHandle {
    let h = replace(&mut unsafe { *SERIAL.get(comport - 1).unwrap() }, None);
    h.unwrap()
}

#[derive(Clone, core::marker::Copy)]
pub struct SerialHandle {
    port: u16,
}

impl SerialHandle {
    pub fn wb(&self, byte: u8) {
        unsafe {
            io::outb(self.port, byte);
        }
    }
}