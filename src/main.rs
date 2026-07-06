use std::env;

#[derive(Debug, Clone, Copy)]
struct CpuState {
    pc: u32,
    ra: u32,
}

impl CpuState {
    fn execute_jal(&mut self, target: u32) {
        let return_address = self.pc.wrapping_add(4);
        self.ra = return_address;
        self.pc = target;
    }
}

fn parse_address(input: Option<&String>, default_value: u32) -> u32 {
    let Some(raw_value) = input else {
        return default_value;
    };

    let trimmed = raw_value.trim();
    if let Some(hex_digits) = trimmed.strip_prefix("0x") {
        u32::from_str_radix(hex_digits, 16).unwrap_or(default_value)
    } else {
        trimmed.parse::<u32>().unwrap_or(default_value)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let initial_pc = parse_address(args.get(1), 0x0040_0000);
    let jal_target = parse_address(args.get(2), 0x0040_0020);

    let mut cpu = CpuState {
        pc: initial_pc,
        ra: 0,
    };

    println!("MIPS JAL visualizer");
    println!("Initial state: pc = {:#010x}, ra = {:#010x}", cpu.pc, cpu.ra);
    println!("Instruction: jal {:#010x}", jal_target);
    println!("Step 1: save return address -> ra = pc + 4 = {:#010x}", cpu.pc + 4);

    cpu.execute_jal(jal_target);

    println!("Step 2: jump to target      -> pc = {:#010x}", cpu.pc);
    println!("Final state:   pc = {:#010x}, ra = {:#010x}", cpu.pc, cpu.ra);
}
