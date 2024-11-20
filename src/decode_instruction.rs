use crate::vm::{Opcode, Register};

struct DecodedInstruction {
    opcode: Opcode,
    // destination register
    dr: u16,
    // source register 1
    sr1: u16,
    // source register 2
    sr2: u16,
    // immediate value (5 bits + sign extended)
    imm5: u16,
    // nzp (branch conditional flag)
    nzp: u16,
    // base register
    base_r: u16,
    // sign extended offset
    offset: u16,
    // trap code
    trap_code: u16,
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
        }
    }
}

fn decode_instruction(instruction: u16) -> DecodedInstruction {
    let opcode = Opcode::try_from(instruction >> 12).expect("invalid instruction");
    let mut decoded_instruction = DecodedInstruction::init(opcode);
    decoded_instruction
}
