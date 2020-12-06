use std::env;
use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn get_common_answer_count(answers :&Vec<&char>, group_size : usize) -> usize
{
    HashSet::<&char>::from_iter(answers.iter().cloned()).iter()
                                .filter(|answer| answers.iter()
                                        .filter(|a| a == answer).count() == group_size).count()
}


fn main() 
{
    let args :Vec<String> = env::args().collect();
    let data :String = fs::read_to_string(&args[1]).expect("Cannot read file");

    let entries :Vec<Vec<Vec<char>>> = data.split("\n\n")
                                            .map(|group| group.trim()
                                                              .split('\n')
                                                              .map(|responses| responses.trim().chars().collect())
                                                              .collect()).collect();
    
    //let group_positive_response_counts = entries.iter().map(|group| HashSet::<&char>::from_iter(group.into_iter().flatten()).len());
    //println!("positive sum count: {}", group_positive_response_counts.sum::<usize>());
    
    let group_positive_common_counts = entries.iter()
                                                .map(|group| get_common_answer_count(
                                                                &group.iter().flatten().collect(),
                                                                group.iter().count()));
    
    println!("Grop common positive answer count: {}", group_positive_common_counts.sum::<usize>());
    
}
