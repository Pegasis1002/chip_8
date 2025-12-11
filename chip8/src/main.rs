
struct Chip8 {
    ram: [u8; 4096],
    v: [u8; 16],
    i:  u16,
    pc:  u16,
    sp:  u16,
    dt: u8,
    st: u8,
}

fn main() {
    println!("Hello, world!");
}
