use std::io::prelude::*;

fn main() 
{
    let args :Vec<String> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]).expect("Cannot open file");

    let program :Vec<String>= std::io::BufReader::new(file)
                                                .lines()
                                                .filter_map(|line| line.ok())
                                                .collect();

    let (_halted, history, accumulator) = run_program(&program);
    println!("Accumulator value before loop: {}", accumulator);

    let mut correct_accumulator = 0;
    for idx in history
    {
        match instruction_swapped_program(&program, idx)
        {
            Some(new_program) =>
            {
                let (halted, _, new_acc) = run_program(&new_program);
                if halted
                {
                    correct_accumulator = new_acc;
                    break
                }
            },
            None => continue
        }
    } 

    println!("Fixed program accumulated value: {}", correct_accumulator);
}

fn instruction_swapped_program(program : &Vec<String>, index :usize) -> Option<Vec<String>>
{
    let line = &program[index];
    if line.starts_with("acc")
    {
        return None;
    }

    let mut new_program = program.clone();
    let (instruction, value_str) = line.split_at(3);
    match instruction
    {
        "nop" => new_program[index] = "jmp".to_owned() + value_str,
        "jmp" => new_program[index] = "nop".to_owned() + value_str,
        _ => panic!("Unhandled instruction: {}", instruction)
    }
    Some(new_program)
}


fn parse_line(line :&String, acc :&mut i32 , pos :&mut usize) 
{
    let (instruction, value_str) = line.split_at(3);
    let value = 
    {
        let (sign, mag) = value_str.split_at(1);
        mag.parse::<i32>().unwrap() * if sign == "-" { -1 } else { 1 }
    };

    match instruction
    {
        "nop" => 
        {
            *pos += 1;
        }
        "acc" => 
        {
            *pos += 1; 
            *acc += value;
        }
        "jmp" =>
        {
            let new_position = *pos as i32 + value;
            *pos = if new_position.is_negative() { 0 } else { new_position as usize };
        }
        _ => panic!("Encountered unknown command: {}", instruction)
    }
}

fn run_program(program :&Vec<String>) -> (bool, Vec<usize>, i32)
{
    let mut accumulator= 0;
    let mut pos = 0;
    let mut history = vec![0];

    while pos < program.len()
    {
        let mut new_pos = pos;
        let mut new_acc = accumulator;

        parse_line(&program[pos], &mut new_acc, &mut new_pos);

        if history.contains(&new_pos)
        {
            return (false, history, accumulator);
        }

        history.push(new_pos);
        pos = new_pos;
        accumulator = new_acc;
    }
    
    return (true, history, accumulator);
}
