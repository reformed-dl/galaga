//Game Board dimensions that go inside struct.rs
pub const SIZE: usize = 10;
pub const ROWS: usize = SIZE;
pub const COLUMNS: usize = SIZE * 2;


//Cords Struct - Tuple
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cords (pub usize, pub usize);

//Time Struct - Named Field
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Timer {
    current_timer: u64,
    interval: u64,
}

impl Timer {
    fn new(interval: u64) -> Self {
        Self { current_timer: 0, interval, }
    }
}