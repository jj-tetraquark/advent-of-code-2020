use std::io::prelude::*;

fn parse_line(line :&String, acc :&mut i32 , pos :&mut usize) 
{
    let (command, value_str) = line.split_at(3);
    let value = 
    {
        let (sign, mag) = value_str.split_at(1);
        mag.parse::<i32>().unwrap() * if sign == "-" { -1 } else { 1 }
    };


    match command
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
            if new_position.is_negative()
            {
                *pos = 0;
            }
            else
            {
                *pos = new_position as usize;
            }
        }
        _ => panic!("Encountered unknown command: {}", command)
    }
}

fn main() 
{
    let args :Vec<String> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]).expect("Cannot open file");

    let program :Vec<String>= std::io::BufReader::new(file)
                                                .lines()
                                                .filter_map(|line| line.ok())
                                                .collect();

    let mut accumulator= 0;
    let mut pos = 0;
    let mut history = vec![0];

    loop
    {
        let mut new_pos = pos;
        let mut new_acc = accumulator;

        parse_line(&program[pos], &mut new_acc, &mut new_pos);

        if history.contains(&new_pos)
        {
            break;
        }

        history.push(new_pos);
        pos = new_pos;
        accumulator = new_acc;
    }

    println!("Accumulator value before loop: {}", accumulator);
}
