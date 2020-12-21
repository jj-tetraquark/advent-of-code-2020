pub trait Cube: std::hash::Hash + Eq + Clone + Copy
{
    fn new2d(x: i32, y :i32) -> Self;
    fn get_neighbours(&self) -> Vec<Self> where Self: std::marker::Sized;
}

#[derive(Clone, PartialEq, Debug, Eq, Hash, Copy)]
pub struct Cube3d
{
    x :i32,
    y :i32,
    z :i32,
}


impl Cube for Cube3d
{
    fn new2d(x: i32, y: i32) -> Cube3d
    {
        Cube3d
        {
            x: x,
            y: y,
            z: 0
        }
    }

    fn get_neighbours(&self) -> Vec<Cube3d>
    {
        let mut neighbours = Vec::new();
        for x in -1..2
        {
            for y in -1..2
            {
                for z in -1..2
                {
                    let neighbour = Cube3d
                    {
                        x: self.x + x as i32,
                        y: self.y + y as i32,
                        z: self.z + z as i32
                    };
                
                    if neighbour != *self
                    {
                        neighbours.push(neighbour);
                    }
                }
            }
        }
        return neighbours
    }
}


#[derive(Clone, PartialEq, Debug, Eq, Hash, Copy)]
pub struct HyperCube
{
    x :i32,
    y :i32,
    z :i32,
    w :i32
}

impl Cube for HyperCube
{
    fn new2d(x :i32, y :i32) -> HyperCube
    {
        HyperCube
        {
            x: x,
            y: y,
            z: 0,
            w: 0
        }
    }

    fn get_neighbours(&self) -> Vec<HyperCube>
    {
        let mut neighbours = Vec::new();
        for x in -1..2
        {
            for y in -1..2
            {
                for z in -1..2
                {
                    for w in -1..2
                    {
                        let neighbour = HyperCube
                        {
                            x: self.x + x as i32,
                            y: self.y + y as i32,
                            z: self.z + z as i32,
                            w: self.w + w as i32
                        };
                    
                        if neighbour != *self
                        {
                            neighbours.push(neighbour);
                        }
                    }
                }
            }
        }
        return neighbours
    }
}
