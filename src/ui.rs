use ncurses::*;

use crate::{HIGHLIGHT_PAIR, ID, REGULAR_PAIR};

#[derive(Default)]
pub struct UI {
    current_list: Option<ID>,
    row: usize,
    col: usize,
}

impl UI {
    pub fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }
    pub fn begin_list(&mut self, id: ID) {
        assert!(self.current_list.is_none(), "Nested lists are not allowed!");
        self.current_list = Some(id);
    }

    pub fn list_element(&mut self, label: &str, id: ID) -> bool {
        let current_list_id = self
            .current_list
            .expect("Not allowed to create list elemets outside of a list");

        let pair = {
            if current_list_id == id {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            }
        };

        self.label(label, pair);

        false
    }

    pub fn end_list(&mut self) {
        self.current_list = None;
    }

    pub fn end(&mut self) {}

    pub fn label(&mut self, s: &str, pair: i16) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(s);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }
}
