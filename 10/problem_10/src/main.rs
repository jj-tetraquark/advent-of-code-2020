use std::collections::HashMap;

fn main() 
{
// Part 1
    let args :Vec<String> = std::env::args().collect();

    let mut sequence :Vec<usize> = std::fs::read_to_string(&args[1])
                                                    .expect("could not read from file")
                                                    .split_whitespace()
                                                    .filter_map(|jolts| jolts.parse::<usize>().ok())
                                                    .collect();
    sequence.push(0); // add the wall socket
    sequence.sort_unstable();
    
    let joltage_diffs :Vec<usize> = sequence.iter()
                                            .zip(sequence.iter().skip(1))
                                            .map(|(a, b)| b - a).collect();

    let ones = joltage_diffs.iter().filter(|&j| j == &1).count();
    let threes = joltage_diffs.iter().filter(|&j| j == &3).count() + 1; //+1 for the device
    let twos = joltage_diffs.iter().filter(|&j| j == &2).count() + 1; //+1 for the device
    
    println!("ones: {}, twos: {}, threes: {}", ones, twos, threes);
    println!("answer : {}", ones * threes);


// Part 2
    let graph : HashMap<usize, Vec<usize>> = sequence.iter().map(|&node|
                                    {
                                        let min_candidate :usize = 
                                        { 
                                            let min = node as i32 - 3;
                                            if min < 0 { 0 } else { min as usize }
                                        };
                                        
                                        let edges :Vec<usize> = (min_candidate..node)
                                                                    .filter(|x| sequence.contains(&x))
                                                                    .collect();
                                        
                                        (node, edges)
                                    }).collect();

    let mut paths_to_zero = HashMap::<usize, usize>::new();
    for node in &sequence
    {
        paths_to_zero.insert(*node, 
                             graph[&node].iter()
                                          .fold(0, |paths, node|
                                                {
                                                    if node == &0
                                                    {
                                                        paths + 1
                                                    }
                                                    else
                                                    {
                                                        paths + paths_to_zero[node]
                                                    }
                                                })
                            );
    }

    println!("Number of paths: {}", paths_to_zero[sequence.iter().max().unwrap()]);
}
