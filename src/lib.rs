mod ui;

pub mod prelude {
    pub use crate::ui::*;
    pub const REGULAR_PAIR: i16 = 0;
    pub const HIGHLIGHT_PAIR: i16 = 1;
    pub type ID = usize;
}
