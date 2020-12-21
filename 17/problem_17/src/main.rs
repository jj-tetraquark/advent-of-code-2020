use std::collections::HashSet;

mod cubes;
use cubes::Cube;
use cubes::Cube3d;
use cubes::HyperCube;

fn state_from_string<T: Cube>(input :&String) -> HashSet<T>
{
    input.split_whitespace()
         .enumerate()
         .map(|(y, line)| 
                    line.chars()
                        .enumerate()
                        .filter_map(|(x, cube)|
                                        match cube
                                        {
                                            '#' => Some(T::new2d(x as i32, y as i32)),
                                             _ => None
                                        }).collect::<HashSet<_>>()
                ).flatten().collect::<HashSet<_>>()
}

fn do_step<T: Cube>(current_state :&HashSet<T>) -> HashSet<T>
{
    let mut currently_inactive_neighbours = HashSet::new();
    let mut next_state :HashSet<_> = 
        current_state.iter()
            .filter_map(|active_cube|
                        {
                            let active_neighbour_count = active_cube
                                    .get_neighbours().iter()
                                    .filter(|neighbour|
                                            {
                                                if !current_state.contains(neighbour)
                                                {
                                                    currently_inactive_neighbours
                                                        .insert(*neighbour.clone());
                                                    return false
                                                }
                                                true
                                            }).count();

                            if active_neighbour_count == 2 || active_neighbour_count == 3
                            {
                                Some(*active_cube)
                            }
                            else
                            {
                                None
                            }
                        }               
                 ).collect();

    currently_inactive_neighbours.iter()
        .for_each(|inactive_cube|
                  {
                      let active_neighbour_count = 
                          inactive_cube.get_neighbours()
                                   .iter()
                                   .filter(|neighbour| current_state.contains(neighbour))
                                   .count();

                      if active_neighbour_count == 3
                      {
                          next_state.insert(*inactive_cube);
                      }
                  });

    next_state
}

fn run_6_steps<T: Cube>(input :&String)
{
    let mut state = state_from_string::<T>(&input);
    for _ in 0..6
    {
        state = do_step(&state);
    }
    println!("{} cubes in active state", state.len());
}

fn main() 
{
    let args :Vec<String> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1]).expect("Could not open file");
    
    //part1
    run_6_steps::<Cube3d>(&input);

    //part2
    run_6_steps::<HyperCube>(&input);
}
