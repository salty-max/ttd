use crate::{HIGHLIGHT_PAIR, REGULAR_PAIR};

use super::{
    layout::{Layout, LayoutDir},
    vec2::Vec2,
};
use ncurses::*;

#[derive(Default)]
pub struct UI {
    layouts: Vec<Layout>,
    pub key: Option<i32>,
}

impl UI {
    pub fn begin(&mut self, pos: Vec2, dir: LayoutDir) {
        assert!(self.layouts.is_empty());

        self.layouts.push(Layout::new(dir, pos, None));
    }

    pub fn end(&mut self) {
        self.layouts.pop().expect("Layout stack underflow");
    }

    pub fn begin_layout(&mut self, dir: LayoutDir) {
        let layout = self
            .layouts
            .last()
            .expect("Cannot create a layout outside of UI");

        let pos = layout.available_pos();
        self.layouts.push(Layout::new(dir, pos, None));
    }

    pub fn end_layout(&mut self) {
        let layout = self.layouts.pop().expect("Layout stack underflow");
        self.layouts
            .last_mut()
            .expect("Layout stack underflow")
            .add_widget(layout.size);
    }

    #[allow(dead_code)]
    pub fn label(&mut self, s: &str, pair: i16) {
        self.label_fixed_width(s, pair, s.len() as i32)
    }

    pub fn label_fixed_width(&mut self, s: &str, pair: i16, width: i32) {
        let layout = self
            .layouts
            .last_mut()
            .expect("Trying to render label ouside of any layout");

        let pos = layout.available_pos();
        mv(pos.y, pos.x);
        attron(COLOR_PAIR(pair));
        addstr(s);
        attroff(COLOR_PAIR(pair));
        layout.add_widget(Vec2::new(width as i32, 1));
    }

    pub fn edit_field(&mut self, buffer: &mut String, cursor: &mut usize, width: i32) {
        let layout = self
            .layouts
            .last_mut()
            .expect("Trying to render edit field outside of a layout");

        let pos = layout.available_pos();

        if *cursor > buffer.len() {
            *cursor = buffer.len();
        }

        if let Some(key) = self.key.take() {
            match key {
                32..=126 => {
                    if *cursor >= buffer.len() {
                        buffer.push(key as u8 as char);
                    } else {
                        buffer.insert(*cursor, key as u8 as char);
                    }

                    *cursor += 1;
                }
                constants::KEY_LEFT => {
                    if *cursor > 0 {
                        *cursor -= 1;
                    }
                }
                constants::KEY_RIGHT => {
                    if *cursor < buffer.len() {
                        *cursor += 1;
                    }
                }
                constants::KEY_BACKSPACE | 127 => {
                    if *cursor > 0 {
                        *cursor -= 1;
                        if *cursor < buffer.len() {
                            buffer.remove(*cursor);
                        }
                    }
                }
                constants::KEY_DC => {
                    if *cursor < buffer.len() {
                        buffer.remove(*cursor);
                    }
                }
                _ => {
                    self.key = Some(key);
                }
            }
        }

        // Buffer
        {
            mv(pos.y, pos.x);
            attron(COLOR_PAIR(REGULAR_PAIR));
            addstr(buffer);
            attroff(COLOR_PAIR(REGULAR_PAIR));
            layout.add_widget(Vec2::new(width, 1));
        }

        // Cursor
        {
            mv(pos.y, pos.x + *cursor as i32);
            attron(COLOR_PAIR(HIGHLIGHT_PAIR));
            addstr(buffer.get(*cursor..=*cursor).unwrap_or(" "));
            attroff(COLOR_PAIR(HIGHLIGHT_PAIR));
        }
    }
}
