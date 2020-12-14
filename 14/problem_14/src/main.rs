use std::collections::HashMap;
use std::io::prelude::*;
use regex::Regex;
use lazy_static::lazy_static;

enum Instruction
{
    Mask(String),
    MemoryWrite { idx :usize, value :u64 }
}

fn parse_line(line :&String) -> Instruction
{
    lazy_static!
    {
        static ref MASK_REGEX :Regex = Regex::new(r"mask = ([X01]+)").unwrap();
        static ref MEM_REGEX :Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    }

    if let Some(mask_captures) = MASK_REGEX.captures(line)
    {
        Instruction::Mask(mask_captures.get(1)
                                       .unwrap()
                                       .as_str()
                                       .to_owned())
    }
    else if let Some(mem_captures) = MEM_REGEX.captures(line)
    {
        Instruction::MemoryWrite
        {
            idx: mem_captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            value: mem_captures.get(2).unwrap().as_str().parse::<u64>().unwrap()
        }
    }
    else
    {
        panic!("Could not parse line! '{}'", line);
    }
}

fn apply_mask(mask :&String, value :u64) -> u64
{
    let maskp = u64::from_str_radix(mask.chars()
                                        .map(|c| if c == '1' {'1'} else {'0'})
                                        .collect::<String>().as_str(), 2).unwrap();

    let maskn = u64::from_str_radix(mask.chars()
                                        .map(|c| if c == '0' {'1'} else {'0'})
                                        .collect::<String>().as_str(), 2).unwrap();

    (value | maskp) & !maskn
}

fn convert_to_floating_value(value :u64, mask :&String) -> String
{
    format!("{:036b}", value).chars().zip(mask.chars())
            .map(|(v, m)| 
                 {
                    if m.is_digit(2)
                    {
                        std::char::from_digit(v.to_digit(2).unwrap() | 
                                              m.to_digit(2).unwrap(), 2)
                            .unwrap()
                    }
                    else
                    {
                        m
                    }
                 }).collect()
}


fn get_all_possible_values(mask :&String) -> Vec<u64>
{
    if !mask.contains('X')
    {
        vec![u64::from_str_radix(mask, 2).unwrap()]
    }
    else
    {
        let replaced1 = mask.replacen("X", "1", 1);
        let replaced0 = mask.replacen("X", "0", 1);

        vec![get_all_possible_values(&replaced0),
             get_all_possible_values(&replaced1)].iter()
                                                .cloned()
                                                .flatten()
                                                .collect()
    }
}


fn part1(file_contents :&Vec<String>)
{
    let mut current_mask = String::new();
    let memory :HashMap<usize, u64> = 
        file_contents
            .iter()
            .filter_map(|line|
                        {
                            match parse_line(&line)
                            {
                                Instruction::Mask(mask) => 
                                {
                                    current_mask = mask;
                                    None
                                },
                                Instruction::MemoryWrite { idx, value } =>
                                {
                                    Some((idx, apply_mask(&current_mask, value)))
                                }
                            }
                        }).collect();

    println!("Memory values sum: {}", memory.values().sum::<u64>());
}

fn part2(file_contents :&Vec<String>)
{
    let mut memory = HashMap::new();
    let mut current_mask = String::new();
    file_contents.iter()
            .for_each(|line|
                     {
                        match parse_line(&line)
                        {
                            Instruction::Mask(mask) => 
                            {
                                current_mask = mask;
                            },
                            Instruction::MemoryWrite { idx, value } =>
                            {
                                let floating_value = convert_to_floating_value(idx as u64, &current_mask);
                                for masked_idx in get_all_possible_values(&floating_value)
                                {
                                    memory.insert(masked_idx, value);
                                }
                            }
                        }
                    });
    
    //println!("Memory state:\n{:#?}", memory);
    println!("Memory values sum: {}", memory.values().sum::<u64>());
}

fn main() 
{
    let args :Vec<String> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]).expect("Cannot open input file");

    let file_contents :Vec<String> = std::io::BufReader::new(file)
                                                         .lines()
                                                         .filter_map(|line| line.ok())
                                                         .collect();

    println!("Part1");
    part1(&file_contents);
    
    println!("Part2");
    part2(&file_contents);

}
