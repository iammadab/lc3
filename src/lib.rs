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
    // unused
    RTI,
    // bitwise not
    NOT,
    // load indirect
    LDI,
    // store indirect
    STI,
    // jump
    JMP,
    // reserved (unused)
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

/// For opcode specification see: https://icourse.club/uploads/files/a9710bf2454961912f79d89b25ba33c4841f6c24.pdf

fn mask(n: u8) -> u16 {
    (1 << n) - 1
}

fn add(vm: &mut VM, instruction: u16) {
    let dr = (instruction >> 9) & mask(3);
    let sr1 = (instruction >> 6) & mask(3);
    let imm_flag = (instruction >> 5) & mask(1);

    if imm_flag == 1 {
        let imm5 = sext(instruction & mask(5), 5);
        *vm.reg_mut(dr) = vm.reg(sr1) + imm5;
    } else {
        let sr2 = instruction & mask(3);
        *vm.reg_mut(dr) = vm.reg(sr1) + vm.reg(sr2);
    }

    update_flags(vm, dr);
}

fn ldi(vm: &mut VM, instruction: u16) {
    let dr = (instruction >> 9) & mask(3);
    let pc_offset = sext(instruction & mask(9), 9);
    let mem_addr = pc_offset + vm.reg(Register::PC.into());
    *vm.reg_mut(dr) = vm.mem(mem_addr);
    update_flags(vm, dr);
}

#[cfg(test)]
mod tests {
    use crate::{add, mask, sext, Opcode, Register, VM};

    // (instr_value, instr_bit_count)
    type InstructionPart = (u16, u8);

    const OPCODE_BIT_COUNT: u8 = 4;
    const REGISTER_BIT_COUNT: u8 = 3;

    fn encode_instruction(instr_parts: Vec<InstructionPart>) -> u16 {
        let mut pad_count = 0;
        let mut instruction = 0;
        for (mut instr_part, instr_size) in instr_parts.into_iter().rev() {
            instr_part &= mask(instr_size);
            instruction += instr_part << pad_count;
            pad_count += instr_size;
        }
        instruction
    }

    fn encode_register(register: Register) -> InstructionPart {
        (register.into(), REGISTER_BIT_COUNT)
    }

    fn encode_opcode(opcode: Opcode) -> InstructionPart {
        (opcode.into(), OPCODE_BIT_COUNT)
    }

    fn encode_imm5(val: u16) -> InstructionPart {
        (val, 5)
    }

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

    #[test]
    fn test_encode_instruction() {
        assert_eq!(encode_instruction(vec![(5, 3)]), 0b0000000000000101);
        assert_eq!(encode_instruction(vec![(5, 3), (2, 2)]), 0b0000000000010110);
        assert_eq!(
            encode_instruction(vec![
                encode_opcode(Opcode::ADD),
                encode_register(Register::R0),
                encode_register(Register::R1),
                (0, 1),
                (0, 2),
                encode_register(Register::R2)
            ]),
            0b0001_000_001_0_00_010
        );
    }

    #[test]
    fn test_add_opcode() {
        // init vm
        let mut vm = VM::init();
        *vm.reg_mut(Register::R3.into()) = 4;
        *vm.reg_mut(Register::R4.into()) = 5;

        assert_eq!(vm.reg(Register::R2.into()), 0);

        // ADD R2, R3, R4 <- R2 = R3 + R4
        let instr_1 = encode_instruction(vec![
            encode_opcode(Opcode::ADD),
            encode_register(Register::R2),
            encode_register(Register::R3),
            (0, 1),
            (0, 2),
            encode_register(Register::R4),
        ]);

        // ADD R2, R3, 7  <- R2 = R3 + 7
        let instr_2 = encode_instruction(vec![
            encode_opcode(Opcode::ADD),
            encode_register(Register::R2),
            encode_register(Register::R3),
            (1, 1),
            encode_imm5(7),
        ]);

        add(&mut vm, instr_1);
        assert_eq!(vm.reg(Register::R2.into()), 9);
        add(&mut vm, instr_2);
        assert_eq!(vm.reg(Register::R2.into()), 11);
    }
}
