//! https://www.nesdev.org/obelisk-6502-guide/reference.html

use super::{addressing_mode::AddressingMode, CPU};

/// Get the status of bit N in the given value.
pub const fn nth_bit(value: u8, n: u8) -> bool {
    value & (1 << n) != 0
}

impl CPU {
    // load the contents of register a into register y
    // and set the flags approprietly
    pub fn tay(&mut self, _____________________addr: &AddressingMode) {
        self.register_y = self.register_a;
        self.status.set_zero(self.register_y == 0);
        self.status.set_negative(nth_bit(self.register_y, 7));
    }

    // https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
    pub fn jmp(&mut self, addr: &AddressingMode) {
        self.pc = addr.fetch_argument_address(self);
    }

    //JSR - Jump to Subroutine
    pub fn jsr(&mut self, addr: &AddressingMode) {
        self.push_word(self.pc + addr.len() - 1);
        self.pc = addr.fetch_argument_address(self);
    }

    //LDX - Load X Register
    pub fn ldx(&mut self, addr: &AddressingMode) {
        self.register_x = addr.fetch_argument(self);
        self.status.set_zero(self.register_x == 0);
        self.status.set_negative(nth_bit(self.register_x, 7))
    }

    pub fn lda(&mut self, addr: &AddressingMode) {
        self.register_a = addr.fetch_argument(self);
        self.status.set_zero(self.register_a == 0);
        self.status.set_negative(nth_bit(self.register_x, 7))
    }

    //NOP - No Operation
    pub fn nop(&mut self, _____________addr: &AddressingMode) {}

    pub fn brk(&mut self, _____________addr: &AddressingMode) {}
}

pub struct Instruction {
    pub name: &'static str,
    pub function: fn(&mut CPU, &AddressingMode),
    pub mode: AddressingMode,
}

impl Instruction {
    pub fn new(
        name: &'static str,
        function: fn(&mut CPU, &AddressingMode),
        mode: AddressingMode,
    ) -> Self {
        Self {
            name,
            function,
            mode,
        }
    }
}

pub fn decode(opcode: u8) -> Instruction {
    match opcode {
        0xEA => Instruction::new("NOP", CPU::nop, AddressingMode::Implied),
        0x00 => Instruction::new("BRK", CPU::brk, AddressingMode::Implied),

        0x20 => Instruction::new("JSR", CPU::jsr, AddressingMode::Absolute),

        0x2A => Instruction::new("LDX", CPU::ldx, AddressingMode::Immediate),
        0xA6 => Instruction::new("LDX", CPU::ldx, AddressingMode::ZeroPage),
        0xB6 => Instruction::new("LDX", CPU::ldx, AddressingMode::ZeroPage),
        0xAE => Instruction::new("LDX", CPU::ldx, AddressingMode::Absolute),
        0xBE => Instruction::new("LDX", CPU::ldx, AddressingMode::AbsoluteY),

        0xA9 => Instruction::new("LDA", CPU::lda, AddressingMode::Immediate),
        0xA5 => Instruction::new("LDA", CPU::lda, AddressingMode::ZeroPage),
        0xB5 => Instruction::new("LDA", CPU::lda, AddressingMode::ZeroPageX),
        0xAD => Instruction::new("LDA", CPU::lda, AddressingMode::Absolute),
        0xDB => Instruction::new("LDA", CPU::lda, AddressingMode::AbsoluteX),
        0xD9 => Instruction::new("LDA", CPU::lda, AddressingMode::AbsoluteY),
        0xA1 => Instruction::new("LDA", CPU::lda, AddressingMode::IndirectX),
        0xB1 => Instruction::new("LDA", CPU::lda, AddressingMode::IndirectY),

        _ => unimplemented!("{opcode:2x}"),
    }
}
