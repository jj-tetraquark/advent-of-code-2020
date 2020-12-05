use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use itertools::Itertools;

fn get_file_contents(filename : &String) -> Vec<i32>
{
    let file = fs::File::open(filename).expect("cannot open file");
    io::BufReader::new(file).lines()
                            .filter_map(|number| number.ok()?.parse::<i32>().ok())
                            .collect()
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "Need to provide an input file as a second argument. \
                               Number of arguments is not 2");
    
    let values = get_file_contents(&args[1]);

    for comb in values.into_iter().combinations(3)
    {
        if comb.iter().sum::<i32>() == 2020
        {
            println!("{} * {} * {} = {}", comb[0], comb[1], comb[2], comb.iter().product::<i32>())
        }
    }
}
