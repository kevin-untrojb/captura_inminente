
#[derive(Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}


impl Position {
    pub fn new(x:usize,y:usize) -> Position{
        Position{x,y}
    }

    pub fn get_dx_dy(&self, p: Position) ->(usize, usize){
        let dx = (self.x as isize - p.x as isize).abs();
        let dy = (self.y as isize - p.y as isize).abs();
        (dx.try_into().unwrap(), dy.try_into().unwrap())
    }
}