use super::{
    layout::{Layout, LayoutDir},
    vec2::Vec2,
};
use ncurses::*;

#[derive(Default)]
pub struct UI {
    layouts: Vec<Layout>,
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
}
