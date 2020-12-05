use std::env;
use std::fs;
use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
use lazy_static::lazy_static;

fn main() 
{
    let args :Vec<String> = env::args().collect();
    let data :String = fs::read_to_string(&args[1]).expect("Cannot read file");

    let entries :Vec<HashMap<&str,&str>> = data.split("\n\n")
                                                 .map(|entry|
                                                  {
                                                      entry.trim()
                                                            .split_whitespace()
                                                            .filter_map(|field| 
                                                                        {
                                                                            field.split(':').collect_tuple()
                                                                        }).collect()
                                                  }).collect();
    
    let valid :Vec<HashMap<&str, &str>> = entries.into_iter().filter(validate_passport).collect();
    println!("There are {} valid passports", valid.len());
}

fn validate_field(key :&str, value :&str) -> bool
{
    match key
    {
        "byr"  => validate_year(value, 1920, 2002),
        "iyr"  => validate_year(value, 2010, 2020),
        "eyr" => validate_year(value, 2020, 2030),
        "hgt" => validate_height(value),
        "hcl" => validate_hair_colour(value),
        "ecl" => validate_eye_colour(value),
        "pid" => validate_passport_number(value),
        _ => panic!("invalid field somehow!")
    }
}

fn validate_passport(passport :&HashMap<&str, &str>) -> bool
{
    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for key in required_keys
    {
        match passport.get(key)
        {
            Some(value) => if !validate_field(key, value) { return false; },
            None => return false
        }
    }
    return true;
}

fn validate_height(value :&str) -> bool
{
    let (height_str, units) = value.split_at(value.len() - 2);
    match height_str.parse::<f32>()
    {
        Ok(height) => match units
        {
            "cm" => height >= 150.0 && height <= 193.0,
            "in" => height >= 59.0 && height <= 76.0,
            _ => false
        },
        Err(_) => false
    }
}

fn validate_year(value :&str, min :usize, max :usize) -> bool
{
    match value.parse::<usize>()
    {
        Ok(year) => year >= min && year <= max,
        Err(_) => false
    }
}

fn validate_hair_colour(value :&str) -> bool
{
    lazy_static!
    {
        static ref HEX_REGEX :Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    }
    HEX_REGEX.is_match(value)
}

fn validate_eye_colour(value :&str) -> bool
{
    let valid_eye_colours = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid_eye_colours.iter().any(|&colour| colour == value)
}

fn validate_passport_number(value :&str) -> bool
{
    value.len() == 9 && value.chars().all(char::is_numeric)
}
