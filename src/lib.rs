/// Register Enum for readable reference
/// 10 registers in total
/// 8 general purpose registers (R0 - R7)
/// 1 program counter (PC)
/// 1 condition flag (COND)
enum Registers {
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

/// Opcodes
enum Opcodes {
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

/// Conditional Flags
enum Flags {
    POSITIVE = 1 << 0,
    ZERO = 1 << 1,
    NEG = 1 << 2,
}

struct VM {
    memory: [u16; 1 << 16],
    registers: [u16; 10],
}

#[cfg(test)]
mod tests {
    use crate::Registers;

    #[test]
    fn test_register_implicit_ordering() {
        assert_eq!(Registers::R0 as usize, 0);
        assert_eq!(Registers::R3 as usize, 3);
        assert_eq!(Registers::PC as usize, 8);
    }
}
