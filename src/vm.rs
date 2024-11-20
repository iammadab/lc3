use crate::decode_instruction::decode_instruction;
use crate::opcodes::{
    add_opcode, and_opcode, br_opcode, jmp_opcodee, jsr_opcode, ld_opcode, ldi_opcode, ldr_opcode,
    lea_opcode, not_opcode, st_opcode, sti_opcode, str_opcode, trap_opcode,
};

/// Register Enum for readable reference
/// 10 registers in total
/// 8 general purpose registers (R0 - R7)
///   - the general purpose registers can be addressed with 3 bits (log_2(8))
/// 1 program counter (PC)
/// 1 condition flag (COND)
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
}

impl From<Register> for u16 {
    fn from(value: Register) -> Self {
        value as u16
    }
}

/// Opcodes
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Opcode {
    // branch
    BR,
    // add
    ADD,
    // load
    LD,
    // store
    ST,
    // jump register
    JSR,
    // bitwise and
    AND,
    // load register
    LDR,
    // store register
    STR,

    // unused (add for proper instruction padding)
    RTI,

    // bitwise not
    NOT,
    // load indirect
    LDI,
    // store indirect
    STI,
    // jump
    JMP,

    // unused (add for proper instruction padding)
    RES,

    // load effective address
    LEA,
    // execute trap
    TRAP,
}

impl From<Opcode> for u16 {
    fn from(value: Opcode) -> Self {
        value as u16
    }
}

impl TryFrom<u16> for Opcode {
    type Error = &'static str;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value <= 15 {
            Ok(unsafe { std::mem::transmute(value) })
        } else {
            Err("invalid opcode")
        }
    }
}

/// Conditional Flags
enum Flags {
    POSITIVE = 1 << 0,
    ZERO = 1 << 1,
    NEG = 1 << 2,
}

impl From<Flags> for u16 {
    fn from(value: Flags) -> Self {
        value as u16
    }
}

pub const MEMORY_SIZE: usize = 1 << 16;
pub const REGISTER_COUNT: usize = 10;

// Memory Mapped Registers
const MR_KBSR: u16 = 0xFE00; // keyboard status
const MR_KBDR: u16 = 0xFE02; // keyboard status

pub struct VM {
    memory: [u16; MEMORY_SIZE],
    registers: [u16; REGISTER_COUNT],
    running: bool,
}

impl VM {
    pub fn init() -> Self {
        VM {
            memory: [0; MEMORY_SIZE],
            registers: [0; REGISTER_COUNT],
            running: false,
        }
    }

    pub fn reg(&self, addr: u16) -> u16 {
        self.registers[addr as usize]
    }

    pub fn reg_mut(&mut self, addr: u16) -> &mut u16 {
        &mut self.registers[addr as usize]
    }

    pub fn mem(&self, addr: u16) -> u16 {
        self.memory[addr as usize]
    }

    pub fn mem_mut(&mut self, addr: u16) -> &mut u16 {
        &mut self.memory[addr as usize]
    }

    pub fn run(&mut self) {
        self.running = true;

        while self.running {
            // fetch instruction
            let instruction = *self.mem_mut(self.reg(Register::PC.into()));

            // decode instruction
            let decoded_instruction = decode_instruction(instruction);

            // update pc
            *self.reg_mut(Register::PC.into()) += 1;

            // execute
            match decoded_instruction.opcode {
                Opcode::BR => br_opcode(self, decoded_instruction),
                Opcode::ADD => add_opcode(self, decoded_instruction),
                Opcode::LD => ld_opcode(self, decoded_instruction),
                Opcode::ST => st_opcode(self, decoded_instruction),
                Opcode::JSR => jsr_opcode(self, decoded_instruction),
                Opcode::AND => and_opcode(self, decoded_instruction),
                Opcode::LDR => ldr_opcode(self, decoded_instruction),
                Opcode::STR => str_opcode(self, decoded_instruction),
                Opcode::RTI => panic!("unused"),
                Opcode::NOT => not_opcode(self, decoded_instruction),
                Opcode::LDI => ldi_opcode(self, decoded_instruction),
                Opcode::STI => sti_opcode(self, decoded_instruction),
                Opcode::JMP => jmp_opcodee(self, decoded_instruction),
                Opcode::RES => panic!("unused"),
                Opcode::LEA => lea_opcode(self, decoded_instruction),
                Opcode::TRAP => trap_opcode(self, decoded_instruction),
            }
        }
    }
}

/// Sign Extension
/// extends a binary value of a certain bit count to a larger bit count (u16 in this case)
pub fn sext(val: u16, bit_count: usize) -> u16 {
    // if the sign bit is 1, add 1's to the most significant part of the number
    // NOTE: this does not change the 2's complement meaning

    // bit_count represent the original length of the sequence
    // right shift to erase all element other than first (bit_count - 1)
    let sign_bit = val >> (bit_count - 1);

    // if sign bit is a 1 (negative in 2's complement representation)
    // pad most significant side with 1's
    if sign_bit == 1 {
        // left shift by bit_count to prevent corruption of original bit values
        return val | (0xffff << bit_count);
    }

    // if not val already padded with 0's just return
    val
}

/// Update Registers::COND based on the value at some register address
pub fn update_flags(vm: &mut VM, register_addr: u16) {
    let register_value = vm.reg(register_addr);
    let cond_state = if register_value == 0 {
        Flags::ZERO
    } else if register_value >> 15 == 1 {
        Flags::NEG
    } else {
        Flags::POSITIVE
    };

    *vm.reg_mut(Register::COND.into()) = cond_state.into();
}

#[cfg(test)]
mod tests {
    use crate::vm::sext;
    use crate::{Register, VM};

    #[test]
    fn test_register_implicit_ordering() {
        assert_eq!(Register::R0 as usize, 0);
        assert_eq!(Register::R3 as usize, 3);
        assert_eq!(Register::PC as usize, 8);
    }

    #[test]
    fn test_vm_manipulation() {
        let mut vm = VM::init();
        assert_eq!(vm.mem(0), 0);
        assert_eq!(vm.reg(Register::PC.into()), 0);
        assert_eq!(vm.reg(Register::R0.into()), 0);

        *vm.reg_mut(Register::PC.into()) = 15;
        *vm.mem_mut(0) = 16;
        *vm.reg_mut(Register::PC.into()) = 30;

        assert_eq!(vm.mem(0), 16);
        assert_eq!(vm.reg(Register::PC.into()), 30);
        assert_eq!(vm.reg(Register::R0.into()), 0);
    }

    #[test]
    fn test_sign_extension() {
        assert_eq!(sext(0b11111, 5), 0b1111111111111111);
        assert_eq!(sext(0b01111, 5), 0b0000000000001111);
    }
}
