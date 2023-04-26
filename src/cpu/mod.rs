//! https://www.nesdev.org/wiki/CPU
//! https://www.nesdev.org/obelisk-6502-guide/reference.html
//! https://www.nesdev.org/wiki/CPU_addressing_modes
//! https://www.nesdev.org/wiki/CPU_registers
//! https://www.nesdev.org/wiki/CPU_memory_map
//! https://www.nesdev.org/wiki/CPU_power_up_state
//! https://github.com/lukexor/tetanes/tree/main/src

mod addressing_mode;
mod flags;
mod instructions;

const RAM_SIZE: usize = 0xFFFF;
const ROM_START: usize = 0x4020;
const NMI_VECTOR: usize = 0xFFFA;
const RESET_VECTOR: usize = 0xFFFC;
const IRQ_VECTOR: usize = 0xFFFE;

#[derive(Debug)]
pub struct CPU {
    pc: u16,
    sp: u16,
    ram: [u8; RAM_SIZE],
    status: flags::Status,
    register_x: u8,
    register_y: u8,
    register_a: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            ram: [0; RAM_SIZE],
            status: flags::Status::default()
                .with_interrupt_disable(true)
                .with_carry(true),
            register_x: 0,
            register_y: 0,
            register_a: 0,
        }
    }

    /*
    The CPU expects interrupt vectors in a fixed place at the end of the cartridge space:
    $FFFA–$FFFB: NMI vector
    $FFFC–$FFFD: Reset vector
    $FFFE–$FFFF: IRQ/BRK vector
    */
    pub fn reset(&mut self) {
        // TODO: this is not NES compliant
        self.pc = 0x600;
        // self.pc = self.read_word(RESET_VECTOR as u16);

        self.sp = 0xFD;
        self.register_x = 0;
        self.register_y = 0;
        self.register_a = 0;
    }

    pub fn load(&mut self, rom: &[u8]) {
        // TODO: this is not NES compliant
        // TODO: nothing here is NES compliant]
        //unimplemented
        self.ram[0x600..0x600 + rom.len()].copy_from_slice(rom);
        // self.ram[ROM_START..ROM_START + rom.len()].copy_from_slice(rom);
    }

    pub fn pop_byte(&mut self) -> u8 {
        self.sp += 1;
        let data = self.ram[self.sp as usize];
        log::trace!("pop: {data:02X}");
        data
    }

    pub fn pop_word(&mut self) -> u16 {
        u16::from_le_bytes([self.pop_byte(), self.pop_byte()])
    }

    pub fn push_byte(&mut self, val: u8) {
        self.ram[self.sp as usize] = val;
        self.sp -= 1;
    }

    pub fn push_word(&mut self, val: u16) {
        for byte in u16::to_be_bytes(val) {
            self.push_byte(byte);
        }
    }

    /// Read a u8 from the CPU's memory
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    /// Read a u16 from the CPU's memory
    pub fn read_word(&self, addr: u16) -> u16 {
        u16::from_le_bytes([self.read_byte(addr), self.read_byte(addr + 1)])
    }

    pub fn tick(&mut self) {
        let instr = instructions::decode(self.ram[self.pc as usize]);
        println!(
            "{:#04x}: A {:#x}, X {:#x}, Y {:#x}, {}",
            self.pc, self.register_a, self.register_x, self.register_y, instr.name
        );
        (instr.function)(self, &instr.mode);
        self.pc += instr.mode.len();
    }
}
