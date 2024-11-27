use crate::decode_instruction::DecodedInstruction;
use crate::vm::{Opcode, Register};
use std::fmt::{format, Display, Formatter, Write};

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::BR => f.write_str("BR"),
            Opcode::ADD => f.write_str("ADD"),
            Opcode::LD => f.write_str("LD"),
            Opcode::ST => f.write_str("ST"),
            Opcode::JSR => f.write_str("JS"),
            Opcode::AND => f.write_str("AND"),
            Opcode::LDR => f.write_str("LDR"),
            Opcode::STR => f.write_str("STR"),
            Opcode::RTI => f.write_str("RTI"),
            Opcode::NOT => f.write_str("NOT"),
            Opcode::LDI => f.write_str("LDI"),
            Opcode::STI => f.write_str("STI"),
            Opcode::JMP => f.write_str("JMP"),
            Opcode::RES => f.write_str("RES"),
            Opcode::LEA => f.write_str("LEA"),
            Opcode::TRAP => f.write_str("TRAP"),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::R0 => f.write_str("RO"),
            Register::R1 => f.write_str("R1"),
            Register::R2 => f.write_str("R2"),
            Register::R3 => f.write_str("R3"),
            Register::R4 => f.write_str("R4"),
            Register::R5 => f.write_str("R5"),
            Register::R6 => f.write_str("R6"),
            Register::R7 => f.write_str("R7"),
            Register::PC => f.write_str("PC"),
            Register::COND => f.write_str("COND"),
        }
    }
}

impl Display for DecodedInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = self.opcode.to_string();

        match self.opcode {
            Opcode::BR => result += format!(" {:b} {}", self.nzp, self.offset).as_str(),
            Opcode::ADD | Opcode::AND => {
                result += format!(" {} {}", r(self.dr), r(self.sr1)).as_str();
                if self.flag == 0 {
                    result += format!(" {}", r(self.sr2)).as_str();
                } else {
                    result += format!(" {}", self.imm5).as_str();
                }
            }
            Opcode::LD | Opcode::LDI | Opcode::ST | Opcode::STI | Opcode::LEA => {
                result += format!(" {} {}", r(self.dr), self.offset).as_str();
            }
            Opcode::JSR => {
                if self.flag == 1 {
                    result += format!("R {}", self.offset).as_str();
                } else {
                    result += format!("SR {}", r(self.base_r)).as_str();
                };
            }
            Opcode::LDR => {
                result += format!(" {} {} {}", r(self.dr), r(self.base_r), self.offset).as_str();
            }
            Opcode::STR => {
                result += format!(" {} {} {}", r(self.dr), r(self.base_r), self.offset).as_str();
            }
            Opcode::RTI => {
                result += "unused";
            }
            Opcode::NOT => {
                result += format!(" {} {}", r(self.dr), r(self.sr1)).as_str();
            }
            Opcode::JMP => {
                result += format!(" {}", r(self.base_r)).as_str();
            }
            Opcode::RES => result += "unused",
            Opcode::TRAP => match self.trap_code {
                0x20 => result += " GETC",
                0x21 => result += " OUT",
                0x22 => result += " PUTS",
                0x23 => result += " IN",
                0x24 => result += " PUTSp",
                0x25 => result += " HALT",
                _ => result += "unrecognized",
            },
        }

        f.write_str(result.as_str())
    }
}

fn r(reg: u16) -> Register {
    Register::try_from(reg).unwrap()
}
