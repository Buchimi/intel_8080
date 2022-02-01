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
}

fn add() -> u8 {
    return 5;
}
