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
    let mut chip = Chip8::init();
    println!("Hello, world!");
    exec_instruction(&mut chip, 0xA602);
}


fn exec_instruction(chip: &mut Chip8, code: u16){
    let f = code >> 12; // Family identifier

    let nnn = code & 0x0FFF; // address
    let nn = code & 0x00FF;
    let n = code & 0x000F; // nibble

    let x = code & 0x0F00;
    let y = code & 0x00F0;

    match f {
        0x0 => {
            match nn {
                0xE0 => {
                    clearDisplay(chip);
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
        0x2 => {},
        0x3 => {},
        0x4 => {},
        0x5 => {},
        0x6 => {},
        0x7 => {},
        0x8 => {},
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

fn clearDisplay(chip: &mut Chip8){
    for ref mut i in chip.display{
        *i = false;
    }
}
