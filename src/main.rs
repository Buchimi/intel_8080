mod cpu;
mod registers;

fn main() {
    let mut cpu = cpu::CPU::init_cpu();
    cpu.tick();

    let b = "H";
    let y = match b {
        "A" => 7,
        "H" => add(),
        _ => 0,
    };

    println!("{}", y);

    let mut x: u8 = 0b11001110;
    x &= !0b10000000;
    

    println!("{:b}", x);
}

fn add() -> u8 {
    return 5;
}
