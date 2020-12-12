fn main() 
{
    let args :Vec<String> = std::env::args().collect();
    let lines :Vec<String>  = std::fs::read_to_string(&args[1]).expect("cannot read file")
                                            .split_whitespace().map(str::to_owned).collect();

    let mut bearing = 'E';
    let mut x = 0;
    let mut y = 0;

    lines.iter().for_each(|instruction|
                   {
                       let (command, magnitude) = parse_instruction(instruction);
                       match command
                       {
                           'F' => update_position(bearing, magnitude, &mut x, &mut y),
                           'R'|'L' => update_bearing(command, magnitude, &mut bearing),
                           'N'|'S'|'E'|'W' => update_position(command, magnitude, &mut x, &mut y),
                           _ => panic!("Unrecognised command: '{}'", command)
                       }
                       // println!("east {} north {}", x, y);
                   });
    println!("Position: ({}, {}), Manhatten Distance from start: {}", x, y, x.abs() + y.abs()); 

    // part 2
    
    x = 0;
    y = 0;
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;
    
    lines.iter().for_each(|instruction|
                          {
                              let (command, magnitude) = parse_instruction(instruction);
                              match command
                              {
                                  'F' => move_towards_waypoint(magnitude, waypoint_x, waypoint_y, &mut x, &mut y),
                                  'R'|'L' => rotate_waypoint(command, magnitude, &mut waypoint_x, &mut waypoint_y),
                                  'N'|'S'|'E'|'W' => update_position(command, magnitude, &mut waypoint_x, &mut waypoint_y),
                                  _ => panic!("Unrecognised command: '{}'", command)
                              }
                          });
    
    println!("Part 2: Position: ({}, {}), Manhatten Distance from start: {}", x, y, x.abs() + y.abs()); 
}

fn parse_instruction(instruction : &String) -> (char, u32)
{
    let (command_str, magnitude_str) = instruction.split_at(1);
    let command = command_str.chars().next().unwrap();
    let magnitude = magnitude_str.parse::<u32>().unwrap();
    (command, magnitude)
}

fn update_bearing(direction: char, degrees: u32, bearing :&mut char) 
{
    let bearings : &'static [char; 4] = &['N', 'E', 'S', 'W'];
    let steps = (degrees / 90) as usize;
    let cur_idx = bearings.iter().position(|d| d == bearing).unwrap();

    let change = match direction
    {
        'L' => 4 - steps,
        'R' => steps,
        _ => panic!("Unrecognised direction: {}", direction)
    };


    let idx = (cur_idx + change) % 4;
    *bearing = bearings[idx];
}


fn update_position(bearing :char, magnitude :u32, x :&mut i32, y :&mut i32)
{
    match bearing
    {
        'N' => *y += magnitude as i32,
        'S' => *y -= magnitude as i32,
        'E' => *x += magnitude as i32,
        'W' => *x -= magnitude as i32,
        _ => panic!("unrecognised bearing: {}", bearing)
    };
    // println!("{} units {}", magnitude, bearing);
}

fn move_towards_waypoint(magnitude :u32, waypoint_x :i32, waypoint_y :i32, x :&mut i32, y :&mut i32)
{
    *x += magnitude as i32 * waypoint_x;
    *y += magnitude as i32 * waypoint_y;
}

fn rotate_waypoint(direction :char, degrees :u32, waypoint_x :&mut i32, waypoint_y :&mut i32)
{
    let steps = (degrees / 90) as usize;
    let steps_cw = match direction
    {
        'L' => 4 - steps,
        'R' => steps,
        _ => panic!("Unrecognised direction: {}", direction)
    };

    match steps_cw
    {
        1 => 
        {
            std::mem::swap(waypoint_x, waypoint_y);
            *waypoint_y = -*waypoint_y
        }
        2 => 
        {
            *waypoint_x = -*waypoint_x;
            *waypoint_y = -*waypoint_y;
        }
        3 => 
        {
            std::mem::swap(waypoint_x, waypoint_y);
            *waypoint_x = -*waypoint_x;
        }
        _ => panic!("Unsupported number of steps {}", steps_cw)
    }
}
