use super::CPU;

/// https://www.nesdev.org/obelisk-6502-guide/addressing.html
#[derive(Debug)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    Relative,
    IndirectX,
    IndirectY,
    Absolute,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Indirect,
    AbsoluteX,
    AbsoluteY,
}

impl AddressingMode {
    pub fn fetch_argument_address(&self, cpu: &mut CPU) -> u16 {
        let after_opcode = cpu.pc + 1;
        match self {
            // Dont actually have any arguments
            Self::Implied => 0,
            Self::Accumulator => 0,

            Self::Relative => {
                let after_param = cpu.pc + self.len();
                let offset = cpu.read_byte(after_opcode) as i8;
                after_param + offset as u16
            }

            Self::ZeroPage => cpu.read_byte(after_opcode) as u16,
            Self::ZeroPageX => cpu.read_byte(after_opcode) as u16 + cpu.register_x as u16,
            Self::ZeroPageY => cpu.read_byte(after_opcode) as u16 + cpu.register_y as u16,

            Self::Immediate => after_opcode,
            Self::Absolute => cpu.read_word(after_opcode),

            _ => todo!(),
        }
    }

    pub fn fetch_argument(&self, cpu: &mut CPU) -> u8 {
        let addr = self.fetch_argument_address(cpu);
        cpu.read_byte(addr)
    }

    /// The length of an instruction, counting the identifier and arguments
    pub const fn len(&self) -> u16 {
        match self {
            AddressingMode::Implied | AddressingMode::Accumulator => 1,

            AddressingMode::Immediate
            | AddressingMode::Relative
            | AddressingMode::IndirectX
            | AddressingMode::IndirectY
            | AddressingMode::ZeroPage
            | AddressingMode::ZeroPageX
            | AddressingMode::ZeroPageY => 2,

            AddressingMode::Indirect
            | AddressingMode::Absolute
            | AddressingMode::AbsoluteX
            | AddressingMode::AbsoluteY => 3,
        }
    }
}
