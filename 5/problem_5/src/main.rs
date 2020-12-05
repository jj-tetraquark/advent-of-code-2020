use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

fn get_seat_id(seat :&str) -> u32
{
    let (row_str, col_str) = seat.split_at(7); 
    let row_bin = row_str.chars().map(|c| match c
                                      {
                                          'F' => "0",
                                          'B' => "1",
                                          _ => panic!("Encountered {}", c)
                                      }).collect::<String>();

    let col_bin = col_str.chars().map(|c| match c
                                      {
                                          'L' => "0",
                                          'R' => "1",
                                          _ => panic!("Encountered {}", c)
                                      }).collect::<String>();

    let row = u32::from_str_radix(row_bin.as_str(), 2).unwrap();
    let col = u32::from_str_radix(col_bin.as_str(), 2).unwrap();

    row * 8 + col
}

fn main() 
{
    println!("BFFFBBFRRR -> {}", get_seat_id("BFFFBBFRRR"));
    println!("FFFBBBFRRR -> {}", get_seat_id("FFFBBBFRRR"));
    println!("BBFFBBFRLL -> {}", get_seat_id("BBFFBBFRLL"));

    let args :Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("Could not open file");
    let seat_ids :Vec<u32> =  io::BufReader::new(file).lines()
                                               .filter_map(|line|
                                                           {
                                                                let seat = line.ok()?;
                                                                Some(get_seat_id(seat.as_str()))
                                                           }).collect();
    let max_id = seat_ids.iter().max().unwrap();
    let min_id = seat_ids.iter().min().unwrap();
    println!("Max ID: {}", max_id);
    println!("Min ID: {}", min_id);

    let mut empty_seats = Vec::new();

    for seat_id in *min_id..*max_id
    {
        if !seat_ids.iter().any(|&id| id==seat_id)
        {
            empty_seats.push(seat_id);
        }
    }
    println!("Empty seats: {:?}", empty_seats)
}
