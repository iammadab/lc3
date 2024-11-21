use std::fmt::{Display, Formatter};
use crate::opcodes::mask;
use crate::vm::{sext, Opcode, Register};

pub(crate) struct DecodedInstruction {
    pub(crate) opcode: Opcode,
    // destination register
    pub(crate) dr: u16,
    // source register 1
    pub(crate) sr1: u16,
    // source register 2
    pub(crate) sr2: u16,
    // immediate value (5 bits + sign extended)
    pub(crate) imm5: u16,
    // nzp (branch conditional flag)
    pub(crate) nzp: u16,
    // base register
    pub(crate) base_r: u16,
    // sign extended offset
    pub(crate) offset: u16,
    // trap code
    pub(crate) trap_code: u16,
    // flag
    pub(crate) flag: u16,
}

impl DecodedInstruction {
    fn init(opcode: Opcode) -> Self {
        Self {
            opcode,
            dr: 0,
            sr1: 0,
            sr2: 0,
            imm5: 0,
            nzp: 0,
            base_r: 0,
            offset: 0,
            trap_code: 0,
            flag: 0,
        }
    }
}

pub fn decode_instruction(instruction: u16) -> DecodedInstruction {
    let opcode = Opcode::try_from(instruction >> 12).expect("invalid instruction");
    let mut decoded_instruction = DecodedInstruction::init(opcode.clone());

    decoded_instruction.dr = (instruction >> 9) & mask(3);
    decoded_instruction.sr1 = (instruction >> 6) & mask(3);
    decoded_instruction.sr2 = instruction & mask(3);
    decoded_instruction.imm5 = sext(instruction & mask(5), 5);
    decoded_instruction.nzp = (instruction >> 9) & mask(3);
    decoded_instruction.base_r = (instruction >> 6) & mask(3);
    decoded_instruction.trap_code = instruction & mask(8);
    decoded_instruction.offset = match opcode {
        // offset6
        Opcode::STR | Opcode::LDR => sext(instruction & mask(6), 6),
        // offset11
        Opcode::JSR => sext(instruction & mask(11), 11),
        // offset9
        _ => sext(instruction & mask(9), 9),
    };
    decoded_instruction.flag = match opcode {
        Opcode::ADD | Opcode::AND => (instruction >> 5) & mask(1),
        _ => (instruction >> 11) & mask(1),
    };

    decoded_instruction
}

#[cfg(test)]
mod tests {
    use crate::decode_instruction::decode_instruction;
    use crate::vm::{Opcode, Register};

    #[test]
    fn test_instruction_decoding() {
        let decoded_instruction = decode_instruction(0b0001_000_001_0_00_010);
        assert_eq!(decoded_instruction.opcode, Opcode::ADD);
        assert_eq!(decoded_instruction.dr, Register::R0.into());
        assert_eq!(decoded_instruction.sr1, Register::R1.into());
        assert_eq!(decoded_instruction.flag, 0); // register mode
        assert_eq!(decoded_instruction.sr2, Register::R2.into());
    }
}
