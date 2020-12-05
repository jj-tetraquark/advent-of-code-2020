use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;


struct Map
{
    data : Vec<Vec<i32>>,
}

fn parse_row(row :&String) -> Vec<i32>
{
    row.chars().map(|el| match el
                    {
                        '.' => 0,
                        '#' => 1,
                        _ => panic!("this should not happen")
                    }).collect()
}

impl Map
{
    fn new(map_data : &Vec<String>) -> Map
    {
        let parsed_map = map_data.iter().map(|row| parse_row(row)).collect();
        Map 
        { 
            data : parsed_map,
        }
    }

    fn at(&self, x :usize, y :usize) -> Option<&i32>
    {
        let x_wrapped = x % self.width_wrap();
        self.data.get(y)?.get(x_wrapped)
    }

    fn height(&self) -> usize
    {
        self.data.len()
    }

    fn width_wrap(&self) -> usize
    {
        self.data[0].len()
    }
}

fn traverse_trees(x_step :usize, y_step :usize, map :&Map) -> i32
{
    let mut path = Vec::new();
    let mut x = 0;
    for y in (y_step..map.height()).step_by(y_step)
    {
        x += x_step;
        match map.at(x, y)
        {
            Some(el) => path.push(el),
            None => break
        }
    }
    
    path.into_iter().sum::<i32>()
}

fn main() 
{
    let args :Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("cannot open file");

    let file_lines: Vec<String> =  io::BufReader::new(file)
                                                    .lines()
                                                    .filter_map(|line| line.ok())
                                                    .collect();
    let map = Map::new(&file_lines);
    
    let trajectories = vec![[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let mut trajectory_results = Vec::new();

    for trajectory in trajectories
    {
        let x_step = trajectory[0];
        let y_step = trajectory[1];
        let tree_count = traverse_trees(x_step, y_step, &map);

        trajectory_results.push(tree_count);
        println!("Tree count: {}", tree_count);
    }
    
    println!("Tree product: {}", trajectory_results.into_iter().product::<i32>());
}
