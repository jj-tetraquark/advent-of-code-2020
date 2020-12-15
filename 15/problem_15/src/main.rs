use std::collections::HashMap;

fn main() 
{
    let sequence :Vec<u32> = std::env::args().skip(1).filter_map(|x| x.parse::<u32>().ok()).collect();

    let mut last_spoken_map :HashMap<u32, u32> = sequence.iter()
                                                    .take(sequence.len() - 1)
                                                    .enumerate().
                                                    map(|(idx, v)| (*v, idx as u32 + 1)).collect();

    let mut current_number = *sequence.last().unwrap();
    let mut count = sequence.len() as u32;

    
    while count < 30000000
    {
        match last_spoken_map.insert(current_number, count)
        {
            Some(last_spoken) =>
            {
                current_number = count - last_spoken;
            },
            None =>
            {
                current_number = 0;
            }
        }
        count += 1;
    }
    println!("{}", current_number);
}
