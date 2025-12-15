use std::env;

static FONTS: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0,
  0x20, 0x60, 0x20, 0x20, 0x70,
  0xF0, 0x10, 0xF0, 0x80, 0xF0,
  0xF0, 0x10, 0x90, 0x10, 0xF0,
  0x90, 0x90, 0xF0, 0x10, 0x10,
  0xF0, 0x80, 0xF0, 0x10, 0xF0,
  0xF0, 0x80, 0xF0, 0x90, 0xF0,
  0xF0, 0x10, 0x20, 0x40, 0x50,
  0xF0, 0x90, 0xF0, 0x90, 0xF0,
  0xF0, 0x90, 0xF0, 0x10, 0xF0,
  0x50, 0x90, 0xF0, 0x90, 0x90,
  0xE0, 0x90, 0xE0, 0x90, 0xE0,
  0xF0, 0x80, 0x80, 0x80, 0xF0,
  0xE0, 0x90, 0x90, 0x90, 0xE0,
  0xF0, 0x80, 0xF0, 0x80, 0xF0,
  0xF0, 0x80, 0xF0, 0x80, 0x80,
];

struct Chip8 {
    ram: [u8; 4096],
    v: [u8; 16],
    i:  u16,
    pc:  u16,
    sp:  u16,
    dt: u8,
    st: u8,
    display: [bool; 2048],
    stack: [u16; 16],
}

impl Chip8 {
    fn init() -> Self {
        // Initialize chip8 with respective zero values
        let mut chip = Chip8 {
            ram: [0; 4096],
            v: [0; 16],
            i: 0x0000,
            pc: 0x200, // 0x200(512) is where the program starts
            sp: 0x00,
            dt: 0x00,
            st: 0x00,
            display: [false; 2048], // set the display to be all zeros
            stack: [0x0000; 16],
        };
        // Insert FONTS into ram starting from 0x50(80)
        for (i, &byte) in FONTS.iter().enumerate() {
            chip.ram[0x50 + i] = byte;
        }
        // Return the initialized chip8 struct
        return chip
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: String = args[1].clone();

    let mut chip = Chip8::init();

    exec_instruction(&mut chip, 0xA602);
}

fn exec_instruction(chip: &mut Chip8, code: u16){
    let f = code >> 12; // Family identifier

    let nnn = code & 0x0FFF; // address
    let nn = (code & 0x00FF) as u8;
    let n = (code & 0x000F)  as u8; // nibble

    let x = (code & 0x0F00) as usize;
    let y = (code & 0x00F0) as usize;

    match f {
        0x0 => {
            match nn {
                0xE0 => {
                    clear_display(chip);
                    println!{"done!"}
                },
                0xEE => {
                    chip.pc = chip.sp;
                    chip.sp -= 1;
                },
                _ => {}
            }
        },
        0x1 => { chip.pc = nnn },
        0x2 => {
            chip.sp += 1;
            chip.stack[chip.sp as usize] = chip.pc;
            chip.pc = nnn;
        },
        0x3 => {
            if chip.v[x] == nn { chip.pc += 2; }
        },
        0x4 => {
            if chip.v[x] != nn { chip.pc += 2; }
        },
        0x5 => {
            if chip.v[x] == chip.v[y] {
                chip.pc += 2;
            }
        },
        0x6 => { chip.v[x] = nn; },
        0x7 => { chip.v[x] += nn; },
        0x8 => { 
            match n {
                0x0 => { chip.v[x] = chip.v[y]; },
                0x1 => { chip.v[x] = ( chip.v[x] | chip.v[y] ); },
                0x2 => { chip.v[x] = ( chip.v[x] & chip.v[y] ); },
                0x3 => { chip.v[x] = ( chip.v[x] ^ chip.v[y] ); },
                0x4 => {
                    chip.v[x] += chip.v[y];
                    if chip.v[x] > 0xFF {
                        chip.v[0xF] = 1;
                        chip.v[x] = (chip.v[x] & 0xFF)
                    }
                },
                0x5 => { 
                    chip.v[x] -= chip.v[y];
                    if chip.v[x] > chip.v[y] {
                        chip.v[0xF] = 1;
                        chip.v[x] = (chip.v[x] & 0xFF)
                    }
                },
                0x6 => { chip.v[x] >>= 1; },
                0x7 => {
                    chip.v[y] -= chip.v[x];
                    if chip.v[x] > chip.v[y] {
                        chip.v[0xF] = 1;
                        chip.v[x] = (chip.v[x] & 0xFF)
                    }
                },
                0xE => { chip.v[x] <<= 1; },
                _ => {}
            }},
        0x9 => {},
        0xa => {},
        0xb => {},
        0xc => {},
        0xd => {},
        0xe => {},
        0xf => {},
        _ => println!("Invalid instruction.")
    }
}

fn clear_display(chip: &mut Chip8){
    for ref mut i in chip.display{
        *i = false;
    }
}
