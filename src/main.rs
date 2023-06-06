fn main() {
    let program = vec![
        Instruction::Push(1_000_000_000),
        Instruction::Store(0),
        Instruction::Push(1),
        Instruction::Load(0),
        Instruction::Sub,
        Instruction::Store(0),
        Instruction::Load(0),
        Instruction::JumpIf(2),
    ];
    let mut memory = vec![0; 10];
    let mut stack = Vec::new();
    let mut program_counter = 0;
    println!("start");
    while program_counter < program.len() {
        let instruction = unsafe { program.get_unchecked(program_counter) };
        program_counter += 1;
        match instruction {
            Instruction::Push(value) => {
                stack.push(*value);
            }
            Instruction::Add => {
                let x = unsafe { stack.pop().unwrap_unchecked() };
                let y = unsafe { stack.pop().unwrap_unchecked() };
                stack.push(x + y);
            }
            Instruction::Sub => {
                let x = unsafe { stack.pop().unwrap_unchecked() };
                let y = unsafe { stack.pop().unwrap_unchecked() };
                stack.push(x - y);
            }
            Instruction::Jump(dest) => {
                program_counter = *dest;
            }
            Instruction::JumpIf(dest) => {
                if unsafe { stack.pop().unwrap_unchecked() } != 0 {
                    program_counter = *dest;
                }
            }
            Instruction::Load(address) => {
                stack.push(*unsafe { memory.get_unchecked(*address) });
            }
            Instruction::Store(address) => {
                *unsafe { memory.get_unchecked_mut(*address) } =
                    unsafe { stack.pop().unwrap_unchecked() };
            }
        }
    }
    println!("end");
}

enum Instruction {
    Push(i64),
    Add,
    Sub,
    Jump(usize),
    JumpIf(usize),
    Load(usize),
    Store(usize),
}
