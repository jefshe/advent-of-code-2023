pub type Coord = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Status {
    Operational,
    Damaged,
    Unknown
}

impl Status {
    pub fn new(c: char) -> Status {
        match c {
            '#' => Status::Operational,
            '.' => Status::Damaged,
            _ => Status::Unknown
        }
    }
}