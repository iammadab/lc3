use crate::cli::{Cli, Commands};
use crate::decode_instruction::decode_instruction;
use crate::vm::{Register, VM};
use clap::Parser;
use std::fs::File;
use std::io;
use std::io::ErrorKind::UnexpectedEof;
use std::io::{BufReader, Read};
use termios::*;

mod cli;
pub mod decode_instruction;
mod display;
pub mod opcodes;
pub mod vm;

fn main() {
    let cli = Cli::parse();

    let (path, execute) = match &cli.command {
        Commands::Execute { path } => (path, true),
        Commands::Disassemble { path } => (path, false),
    };

    // Some tricks to make the VM's terminal be interactive
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();

    // make a mutable copy of termios
    // that we will modify
    let mut new_termios = termios.clone();
    new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode

    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

    let f = File::open(path).unwrap();
    let mut f = BufReader::new(f);

    let mut address = read_u16(&mut f).unwrap();

    let mut vm = VM::init();
    *vm.reg_mut(Register::PC.into()) = address;

    loop {
        match read_u16(&mut f) {
            Ok(instruction) => {
                if !execute {
                    // disassemble
                    let decoded = decode_instruction(instruction);
                    println!("{}", decoded);
                }

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

    if execute {
        vm.run();
    }

    // reset the stdin to
    // original termios data
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}

fn read_u16(f: &mut BufReader<File>) -> io::Result<u16> {
    let mut buffer = [0_u8; 2];
    f.read_exact(&mut buffer)?;
    Ok(u16::from_be_bytes(buffer))
}
