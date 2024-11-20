use crate::vm::{Register, VM};
use std::fs::File;
use std::io;
use std::io::ErrorKind::UnexpectedEof;
use std::io::{BufReader, Read};

mod decode_instruction;
pub mod opcodes;
pub mod vm;

fn main() {
    let path = "./src/programs/rogue.obj";
    let f = File::open(path).unwrap();
    let mut f = BufReader::new(f);

    let mut address = read_u16(&mut f).unwrap();

    let mut vm = VM::init();
    *vm.reg_mut(Register::PC.into()) = address;

    loop {
        match read_u16(&mut f) {
            Ok(instruction) => {
                dbg!(instruction);
                *vm.mem_mut(address) = instruction;
                address += 1;
            }
            Err(e) => {
                if e.kind() == UnexpectedEof {
                    println!("program loaded successfully!");
                }
                break;
            }
        }
    }

    vm.run();
}

fn read_u16(f: &mut BufReader<File>) -> io::Result<u16> {
    let mut buffer = [0_u8; 2];
    f.read_exact(&mut buffer)?;
    Ok(u16::from_be_bytes(buffer))
}
