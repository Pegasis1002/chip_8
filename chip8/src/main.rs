
struct Chip8 {
    ram: [u8; 4096],
    v: [u8; 16],
    i:  u16,
    pc:  u16,
    sp:  u16,
    dt: u8,
    st: u8,
    display: [bool, 2048],
    stack: [u16, 16],
}

impl chip8 {
    fn init() -> self{
        // Set special Registers to zero value
        self.pc = 0x200; // program starts at 0x220(512 bytes)
        self.sp = 0x00; // stack pointer is empty initialy
        self.i = 0x0000; // adderess reister stores nothing initialy

        // Clear all general purpose registers
        for i in self.v{
            self.v[i] = 0x000;
        }

        for i in self.stack {
            self.stack[i] = 0x0000
        }
    }
}

fn main() {
    println!("Hello, world!");
}
