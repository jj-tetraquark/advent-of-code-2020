use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;

struct Bag
{
    can_contain_ :HashMap<String, usize>
}

impl Bag
{
    fn can_contain(&self, bag_colour :&String) -> bool
    {
        self.can_contain_.keys().any(|k| k == bag_colour)
    }
}

fn can_be_contained_by(bag_types :&HashSet<String>, bag_rules :&HashMap<String, Bag>) -> HashSet<String>
{

    let contained_by = bag_types.iter().map(|bag_type|
                         {
                             bag_rules.iter().filter_map(|(k, v)| if v.can_contain(bag_type)
                                                         { Some(k.clone()) }
                                                         else
                                                         { None }
                                                         ).collect::<HashSet<String>>()
                         }).flatten().collect::<HashSet<String>>();

    if contained_by.is_empty()
    {
        return bag_types.clone();
    }
    else
    {
        let mut result = bag_types.clone();
        result.extend(can_be_contained_by(&contained_by, bag_rules));
        result
    }
}


fn calculate_child_bag_count(bag_colour :&String, bag_rules :&HashMap<String, Bag>) -> usize
{
    bag_rules.get(bag_colour).unwrap()
                             .can_contain_.iter().map(|(colour, count)| count * (1 + calculate_child_bag_count(colour, bag_rules))).sum()
}

fn parse_line(line :&str) -> Option<(String, Bag)>
{
    lazy_static!
    {
        static ref CONTAINER_REGEX : Regex = Regex::new(r"^([a-z ]+) bags contain").unwrap();
        static ref CONTAINING_REGEX : Regex = Regex::new(r"([0-9]) ([a-z ]+) bag").unwrap();
    }
    let container_bag_name = CONTAINER_REGEX.captures(line)
                                        .and_then(|cap| Some(cap.get(1)?.as_str()));
   
    let contains :HashMap<String, usize> = CONTAINING_REGEX.captures_iter(line)
                                                           .filter_map(|content| 
                                                            {
                                                                Some((content.get(2)?.as_str().to_owned(),
                                                                      content.get(1)?.as_str().parse::<usize>().ok()?))
                                                            }).collect();
    let bag = Bag                                                            
    {
        can_contain_ : contains,
    };

    Some((container_bag_name?.to_owned(), bag))
}

fn main() 
{
    let args :Vec<String> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]).expect("cannot open file");


    let bag_rules :HashMap<String, Bag> = std::io::BufReader::new(file)
                                                                .lines()
                                                                .filter_map(|line| parse_line(line.ok()?.as_str()))
                                                                .collect();
        
    let contained_by = can_be_contained_by(&vec!["shiny gold".to_string()].into_iter().collect::<HashSet<_>>(), &bag_rules);
    println!("Number of bags that can contain shiny gold: {}", contained_by.len() - 1);
    println!("Child bags: {}", calculate_child_bag_count(&"shiny gold".to_string(), &bag_rules));
}
