pub type Coord = (usize, usize);
#[derive(Debug)]
pub struct Rectangle {
    pub x: i64,
    pub y: i64,
    pub width: usize,
    pub height: usize
}

impl Rectangle {
    pub fn surround(coord: Coord, width: usize) -> Rectangle {
        let x = coord.0 as i64 - 1;
        let y = coord.1 as i64 - 1;
        Self::new(x, y, width + 2, 3)
    }

    pub fn new(x: i64, y: i64, width: usize, height: usize) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height
        }
    }
    pub fn is_inside(&self, coord: &Coord) -> bool {
        let x = coord.0 as i64;
        let y = coord.1 as i64;
        x >= self.x && y >= self.y && x < (self.x + self.width as i64) && y < (self.y + self.height as i64)
    }

}