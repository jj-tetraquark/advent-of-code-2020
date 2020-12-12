fn main() 
{
    let args :Vec<String> = std::env::args().collect();
    let seating_plan = 
    {
        let input = std::fs::read_to_string(&args[1]).expect("Could not open file");
        Seating::new(input.as_str())
    };
    
    let mut current_state = seating_plan.clone();

    let mut iterations = 0;
    let stable_occupancy = loop
    {
        iterations += 1;
        let next_state = Seating
        {
            width : current_state.width,
            height : current_state.height,
            spaces : current_state.iter().enumerate()
                                         .map(|(idx, space)|
                                              {
                                                  if space.is_seat()
                                                  {
                                                    //let adjacent_occupancy = current_state.get_adjacent_occupied_count(idx); // part1
                                                    let adjacent_occupancy = current_state.get_line_of_sight_occupied_count(idx);
                                                    //if space.is_occupied() && adjacent_occupancy >= 4 // part 1
                                                    if space.is_occupied() && adjacent_occupancy >= 5
                                                    {
                                                        return Space::empty();
                                                    }
                                                    else if !space.is_occupied() && adjacent_occupancy == 0
                                                    {
                                                        return Space::occupied();
                                                    }
                                                  }
                                                  return space.clone();
                                              }).collect()
        };

        //println!("Step {}: {:?}\n", iterations, next_state);

        if next_state == current_state
        {
            break current_state.iter().filter(|s| s.is_seat() && s.is_occupied()).count();
        }
        current_state = next_state;
    };

    println!("Stable occupancy: {} in {} iterations", stable_occupancy, iterations);
}

#[derive(Clone, PartialEq)]
struct Seating
{
    width  :usize,
    height :usize,
    spaces :Vec<Space>
}

#[derive(Clone, Debug, PartialEq, Copy)]
struct Space(char);

impl Space
{
    fn empty() -> Space
    {
        Space('L')
    }

    fn occupied() -> Space
    {
        Space('#')
    }

    fn is_occupied(&self) -> bool
    {
        self.0 == '#'
    }

    fn is_seat(&self) -> bool
    {
        self.0 == 'L' || self.0 == '#'
    } 

    fn to_string(&self) -> String
    {
        self.0.to_string()
    }
}

impl Seating
{
    fn new(string_layout :&str) -> Seating
    {
        Seating
        {
            width : string_layout.split_whitespace().next().unwrap().chars().count(),
            height : string_layout.split_whitespace().count(),
            spaces : string_layout.chars().filter(|x| !x.is_whitespace()).map(Space).collect()
        }
    }

    fn iter(&self) -> impl Iterator<Item=&Space>
    {
        self.spaces.iter() 
    }

    fn get_adjacent_occupied_count(&self, idx :usize) -> usize
    {
        let x = (idx % self.width) as i32;
        let y = (idx / self.width) as i32;

        let coords = [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                          (x - 1, y), (x + 1, y),
                          (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)];
        coords.iter().filter(|(i, j)| if let Some(space) = self.get(*i , *j)
                                      {
                                          space.is_occupied()
                                      }
                                      else
                                      {
                                          false
                                      }
                               ).count()
    }

    fn get_line_of_sight_occupied_count(&self, idx :usize) -> usize
    {
        let rules = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];
        let x = (idx % self.width) as i32;
        let y = (idx / self.width) as i32;

        rules.iter().fold(0, |acc, (dx, dy)|
                         {
                             let mut i = x + dx;
                             let mut j = y + dy;
                             while let Some(space) = self.get(i, j)
                             {
                                if !space.is_seat()
                                {
                                    i += dx;
                                    j += dy;
                                    continue
                                }
                                if space.is_occupied()
                                {
                                    return acc + 1
                                }
                                return acc
                             }
                             return acc
                         })
    }

    fn get(&self, x :i32, y :i32) -> Option<&Space>
    {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height 
        {
            return None;
        }
        self.spaces.get(y as usize * self.width + x as usize)    
    }
}

impl std::fmt::Debug for Seating
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let output = self.spaces.iter().enumerate().map(|(idx, el)| 
                                                  {
                                                      if idx % self.width == 0 
                                                      { 
                                                          "\n".to_string() + el.to_string().as_str()
                                                      }
                                                      else
                                                      {
                                                          el.to_string()
                                                      }
                                                  }).collect::<String>();
        formatter.write_str(output.as_str())
    }
}
