mod opcodes;

/// Register Enum for readable reference
/// 10 registers in total
/// 8 general purpose registers (R0 - R7)
///   - the general purpose registers can be addressed with 3 bits (log_2(8))
/// 1 program counter (PC)
/// 1 condition flag (COND)
enum Register {
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
enum Opcode {
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

const MEMORY_SIZE: usize = 1 << 16;
const REGISTER_COUNT: usize = 10;

struct VM {
    memory: [u16; MEMORY_SIZE],
    registers: [u16; REGISTER_COUNT],
}

impl VM {
    fn init() -> Self {
        VM {
            memory: [0; MEMORY_SIZE],
            registers: [0; REGISTER_COUNT],
        }
    }

    fn reg(&self, addr: u16) -> u16 {
        self.registers[addr as usize]
    }

    fn reg_mut(&mut self, addr: u16) -> &mut u16 {
        &mut self.registers[addr as usize]
    }

    fn mem(&self, addr: u16) -> u16 {
        self.memory[addr as usize]
    }

    fn mem_mut(&mut self, addr: u16) -> &mut u16 {
        &mut self.memory[addr as usize]
    }
}

/// Sign Extension
/// extends a binary value of a certain bit count to a larger bit count (u16 in this case)
fn sext(val: u16, bit_count: usize) -> u16 {
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
fn update_flags(vm: &mut VM, register_addr: u16) {
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
    use crate::{sext, Opcode, Register, VM};

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
