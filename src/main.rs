mod cpu;
mod registers;

fn main() {
    let mut cpu = cpu::CPU::init_cpu();
    cpu.tick();
}
