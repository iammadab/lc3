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
    COUNT
}
struct VM {
    memory: [u16; 1 << 16],
    registers: [u16; 10]
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
