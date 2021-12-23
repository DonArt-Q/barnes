use std::rc::Rc;

use crate::cpu;
pub struct Bus {
    cpu: cpu::CPU,
    wram: [u8; 2048],
}

impl Bus {
    pub fn read(&self, address: u16) -> u8 {
        if address >= 0 && address < 0x1FFF {
            self.wram[(address & 0b0000011111111111) as usize]
        }
        else {0b00000000}
    }
    pub fn write(&mut self, address: u16, data: u8) {
        if address >= 0 && address < 0x1FFF {
            self.wram[(address & 0b0000011111111111) as usize] = data;
        }
    }
}

pub trait ConnectedToBus {
    fn read(address: u16, bus: Rc<Bus>) -> u8;
        
    fn write(address: u16, data: u8, bus: Rc<Bus>);
}