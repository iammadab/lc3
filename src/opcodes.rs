use crate::{sext, update_flags, Flags, Register, MEMORY_SIZE, VM};
use std::io::{Read, Write};

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
fn ldr_opcode(vm: &mut VM, instruction: u16) {
    let dr = (instruction >> 9) & mask(3);
    let base = (instruction >> 6) & mask(3);
    let base_offset = sext(instruction & mask(6), 6);
    let mem_addr = vm.reg(base) + base_offset;
    *vm.reg_mut(dr) = vm.mem(mem_addr);
    update_flags(vm, dr);
}

// TODO: add documentation
fn lea_opcode(vm: &mut VM, instruction: u16) {
    let dr = (instruction >> 9) & mask(3);
    let pc_offset = sext(instruction & mask(9), 9);
    *vm.reg_mut(dr) = vm.reg(Register::PC.into()) + pc_offset;
    update_flags(vm, dr);
}

// TODO: add documentation
fn st_opcode(vm: &mut VM, instruction: u16) {
    let sr = (instruction >> 9) & mask(3);
    let pc_offset = sext(instruction & mask(9), 9);
    *vm.mem_mut(vm.reg(Register::PC.into()) + pc_offset) = vm.reg(sr);
}

// TODO: add documentation
fn sti_opcode(vm: &mut VM, instruction: u16) {
    let sr = (instruction >> 9) & mask(3);
    let pc_offset = sext(instruction & mask(9), 9);
    let pointer_addr = pc_offset + vm.reg(Register::PC.into());
    *vm.mem_mut(vm.mem(pointer_addr)) = vm.reg(sr);
}

// TODO: add documentation
fn str_opcode(vm: &mut VM, instruction: u16) {
    let sr = (instruction >> 9) & mask(3);
    let base = (instruction >> 6) & mask(3);
    let base_offset = sext(instruction & mask(6), 6);
    let mem_addr = vm.reg(base) + base_offset;
    *vm.mem_mut(mem_addr) = vm.reg(sr);
}

// TODO: add documentation
fn and_opcode(vm: &mut VM, instruction: u16) {
    let dr = (instruction >> 9) & mask(3);
    let sr1 = (instruction >> 6) & mask(3);
    let imm_flag = (instruction >> 5) & mask(1);

    if imm_flag == 1 {
        let imm5 = sext(instruction & mask(5), 5);
        *vm.reg_mut(dr) = vm.reg(sr1) & imm5;
    } else {
        let sr2 = instruction & mask(3);
        *vm.reg_mut(dr) = vm.reg(sr1) & vm.reg(sr2);
    }

    update_flags(vm, dr);
}

// TODO: add documentation
fn not_opcode(vm: &mut VM, instruction: u16) {
    let dr = (instruction >> 9) & mask(3);
    let sr = (instruction >> 6) & mask(3);
    *vm.reg_mut(dr) = !sr;
    update_flags(vm, dr);
}

// TODO: add documentation
fn jmp_opcodee(vm: &mut VM, instruction: u16) {
    let base = (instruction >> 6) & mask(3);
    *vm.reg_mut(Register::PC.into()) = vm.reg(base);
}

// TODO: add documentation
fn jsr_opcode(vm: &mut VM, instruction: u16) {
    *vm.reg_mut(Register::R7.into()) = vm.reg(Register::PC.into());
    let mode = (instruction >> 11) & mask(1);
    if mode == 1 {
        // JSR
        let pc_offset = sext(instruction & mask(11), 11);
        *vm.reg_mut(Register::PC.into()) += pc_offset;
    } else {
        // JSSR
        let base = (instruction >> 6) & mask(3);
        *vm.reg_mut(Register::PC.into()) = vm.reg(base);
    }
}

// TODO: add documentation
fn trap_opcode(vm: &mut VM, instruction: u16) {
    let trap_code = instruction & mask(8);
    match trap_code {
        0x20 => trap_get_c(vm),
        0x21 => trap_out(vm),
        0x22 => trap_puts(vm),
        0x23 => trap_in(vm),
        0x24 => trap_putsp(vm),
        0x25 => trap_halt(),
        _ => unreachable!(),
    }
}

/// Get character from the keyboard and store into R0
fn trap_get_c(vm: &mut VM) {
    let mut buffer = [0, 1];
    std::io::stdin().read_exact(&mut buffer).unwrap();
    *vm.reg_mut(Register::R0.into()) = buffer[0] as u16;
    update_flags(vm, Register::R0.into());
}

/// Outputs a character
fn trap_out(vm: &mut VM) {
    println!("{}", vm.reg(Register::R0.into()) as u8 as char);
}

/// Starting from mem_addr = R0, print each cell as a character
/// until last memory cell is reached or 0 is encountered
fn trap_puts(vm: &mut VM) {
    let mut mem_addr = vm.reg(Register::R0.into());
    while mem_addr < MEMORY_SIZE as u16 {
        let data = vm.mem(mem_addr);
        if data == 0 {
            break;
        }

        print!("{}", data as u8 as char);
        mem_addr += 1;
    }
    std::io::stdout().flush().unwrap();
}

fn trap_in(vm: &mut VM) {
    print!("Enter a character: ");
    std::io::stdout().flush().unwrap();
    *vm.reg_mut(Register::R0.into()) = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u16)
        .unwrap();
}

/// Same as trap_puts but assumes two characters per word
fn trap_putsp(vm: &mut VM) {
    let mut mem_addr = vm.reg(Register::R0.into());
    while mem_addr < MEMORY_SIZE as u16 {
        let data = vm.mem(mem_addr);
        let first_half = data & mask(8);
        let second_half = data >> 8;

        print!("{}{}", first_half as u8 as char, second_half as u8 as char);
        mem_addr += 1;
    }
    std::io::stdout().flush().unwrap();
}

fn trap_halt() {
    std::process::exit(0);
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
