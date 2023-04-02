use std::fmt;
#[derive(Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}


impl Position {
    pub fn new(x:usize,y:usize) -> Position{
        Position{x,y}
    }

    pub fn get_dx_dy(&self, other: Self) ->(usize, usize){
        let dx = (self.x as isize - other.x as isize).abs();
        let dy = (self.y as isize - other.y as isize).abs();
        (dx.try_into().unwrap(), dy.try_into().unwrap())
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "posicion: x:{} y:{}",
               self.x,
               self.y,
        )
    }
}