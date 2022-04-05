pub mod ui;

pub const REGULAR_PAIR: i16 = 0;
pub const HIGHLIGHT_PAIR: i16 = 1;
pub type ID = usize;

pub enum Focus {
    Todo,
    Done,
}

impl Focus {
    pub fn toggle(&self) -> Self {
        match self {
            Focus::Todo => Focus::Done,
            Focus::Done => Focus::Todo,
        }
    }
}
