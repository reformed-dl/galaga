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
    pub fn new(interval: u64) -> Self {
        Self { current_timer: 0, interval, }
    }

    pub fn tick(&mut self) -> bool {
        self.current_timer += 1;//each time tick is called, we add 1 to current_timer
        if self.current_timer >= self.interval {//if current_timer is greater than or equal to interval...
            self.current_timer = 0;//set current_timer to 0
            true//then return true
        } else {
            false//anything else, return false
        }
    }
}