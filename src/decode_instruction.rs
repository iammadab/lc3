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
    trap_code: u16
}