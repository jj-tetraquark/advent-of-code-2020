use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

fn password_is_valid(min_bound : usize, max_bound : usize, policy_char : char, password : &str) -> bool
{
    let policy_char_count = password.matches(policy_char).count();
    policy_char_count >= min_bound && policy_char_count <= max_bound
}

fn new_password_is_valid(pos1 :usize, pos2 :usize, policy_char :char, password :&str) -> bool
{
    match_char_at_position(pos1, policy_char, password) 
        ^ match_char_at_position(pos2, policy_char, password)
}

fn match_char_at_position(pos :usize, test_char :char, test_str :&str) -> bool
{
    let char_at_pos = match test_str.chars().nth(pos - 1)
    {
        Some(c) => c,
        None => return false,
    };
    char_at_pos == test_char 
}


fn validate_line(entry : &str) -> Option<bool>
{
    let tokens :Vec<_> = entry.split_whitespace().collect();
    if tokens.len() != 3
    {
        return None;
    }

    let bounds :Vec<usize> = 
    {
        let parsed_bounds : Result<Vec<usize>, _> = tokens[0].split('-')
                                                            .map(|value| value.parse::<usize>())
                                                            .collect();
        match parsed_bounds
        {
            Ok(parsed_bounds) => if parsed_bounds.len() == 2 {parsed_bounds} else {return None},
            Err(_) => return None
        }
    };

    let policy_char : char = match tokens[1].strip_suffix(':')
    {
        Some(policy_str) => if policy_str.len() == 1 
                            { 
                                policy_str.chars().nth(0).unwrap()
                            }
                            else
                            {
                                return None;
                            },
        None => return None,
    };

    let password = tokens[2];

    return Some(new_password_is_valid(bounds[0], bounds[1], policy_char, password));
}

fn main() {
    let args:Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");


    let file = fs::File::open(&args[1]).expect("cannot open file");
    let password_validation_results: Vec<bool> =  io::BufReader::new(file)
                                                    .lines()
                                                    .filter_map(|line| match line 
                                                                {
                                                                    Ok(entry) => validate_line(entry.as_str()),
                                                                    Err(_) => None
                                                                }).collect();

    let valid_passwords = password_validation_results.iter().filter(|&valid| *valid).count();
    println!("Valid passwords: {}", valid_passwords); 

    let password1 = "1-3 a: abcde";
    let password2 = "1-3 b: cdefg";
    let password3 = "2-9 c: ccccccccc";

    println!("{}", password1);
    println!("password1 : {:?}", validate_line(password1));
    println!("{}", password2);
    println!("password2 : {:?}", validate_line(password2));
    println!("{}", password3);
    println!("password3 : {:?}", validate_line(password3));
}
