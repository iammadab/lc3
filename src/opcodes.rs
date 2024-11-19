use crate::{sext, update_flags, Flags, Register, VM};

/// For complete opcode specification
/// see: https://icourse.club/uploads/files/a9710bf2454961912f79d89b25ba33c4841f6c24.pdf

// The add instruction has two modes (the mode is determined by the 5th bit)
// if 0, register mode if 1 immediate mode
// Register Mode : ADD DR SR1 SR2 (DR = SR1 + SR2)
// Immediate Mode: ADD DR SR1 IMM5 (DR = SR1 + IMM5)
//  where imm5 is some constant from 0..=32
fn add_opcode(vm: &mut VM, instruction: u16) {
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

// Load Indirect
// loads via a pointer, reads dr and pc_offset
// pc_offset + pc point to mem that holds reference to actual data
fn ldi_opcode(vm: &mut VM, instruction: u16) {
    let dr = (instruction >> 9) & mask(3);
    let pc_offset = sext(instruction & mask(9), 9);
    let pointer_addr = pc_offset + vm.reg(Register::PC.into());
    *vm.reg_mut(dr) = vm.mem(vm.mem(pointer_addr));
    update_flags(vm, dr);
}

// TODO: add documentation
fn br_opcode(vm: &mut VM, instruction: u16) {
    let expected_cond = (instruction >> 9) & mask(3);
    let pc_offset = sext(instruction & mask(9), 9);

    let cond_state = vm.reg(Register::COND.into());

    if (expected_cond & cond_state) != 0 {
        *vm.reg_mut(Register::PC.into()) += pc_offset;
    }
}

// TODO: add documentation
fn ld_opcode(vm: &mut VM, instruction: u16) {
    let dr = (instruction >> 9) & mask(3);
    let pc_offset = sext(instruction & mask(9), 9);
    *vm.reg_mut(dr) = vm.mem(pc_offset + vm.reg(Register::PC.into()));
    update_flags(vm, dr);
}

// TODO: add documentation
fn st_opcode(vm: &mut VM, instruction: u16) {
    let sr = (instruction >> 9) & mask(3);
    let pc_offset = sext(instruction & mask(9), 9);
    *vm.mem_mut(vm.reg(Register::PC.into()) + pc_offset) = vm.reg(sr);
}

fn mask(n: u8) -> u16 {
    (1 << n) - 1
}

#[cfg(test)]
mod tests {
    use crate::opcodes::{add_opcode, ldi_opcode, mask};
    use crate::{Opcode, Register, VM};

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

        add_opcode(&mut vm, instr_1);
        assert_eq!(vm.reg(Register::R2.into()), 9);
        add_opcode(&mut vm, instr_2);
        assert_eq!(vm.reg(Register::R2.into()), 11);
    }

    #[test]
    fn test_ldi_opcode() {
        // goal is to read from mem_addr = 5
        // will put that address in mem_addr = 10
        // so immediate must be 10 - pc

        // init vm
        let mut vm = VM::init();
        *vm.mem_mut(5) = 42; // value that should be in dr
        *vm.mem_mut(10) = 5;
        // set PC
        *vm.reg_mut(Register::PC.into()) = 2;
        // since pc = 2 then imm5 = 8

        // LDI R2, 8
        let instr = encode_instruction(vec![
            encode_opcode(Opcode::LDI),
            encode_register(Register::R2),
            (8, 9),
        ]);

        assert_eq!(vm.reg(Register::R2.into()), 0);
        ldi_opcode(&mut vm, instr);
        assert_eq!(vm.reg(Register::R2.into()), 42);
    }
}
