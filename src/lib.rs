pub mod ctrlc;
pub mod graphics;

pub const REGULAR_PAIR: i16 = 0;
pub const HIGHLIGHT_PAIR: i16 = 1;
pub type ID = usize;

#[derive(PartialEq, Eq)]
pub enum Status {
    Todo,
    Done,
}

impl Status {
    pub fn toggle(&self) -> Self {
        match self {
            Status::Todo => Status::Done,
            Status::Done => Status::Todo,
        }
    }
}
