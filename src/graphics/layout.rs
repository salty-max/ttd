use std::cmp::max;

use super::vec2::Vec2;

pub enum LayoutDir {
    Horizontal,
    Vertical,
}

pub struct Layout {
    pub dir: LayoutDir,
    pub pos: Vec2,
    pub size: Vec2,
}

impl Layout {
    pub fn new(dir: LayoutDir, pos: Vec2, size: Option<Vec2>) -> Self {
        Self {
            dir,
            pos,
            size: match size {
                Some(s) => s,
                None => Vec2::zero(),
            },
        }
    }

    pub fn available_pos(&self) -> Vec2 {
        use LayoutDir::*;
        match self.dir {
            Horizontal => self.pos + self.size * Vec2::new(1, 0),
            Vertical => self.pos + self.size * Vec2::new(0, 1),
        }
    }

    pub fn add_widget(&mut self, size: Vec2) {
        use LayoutDir::*;
        match self.dir {
            Horizontal => {
                self.size.x += size.x;
                self.size.y = max(self.size.y, size.y);
            }
            Vertical => {
                self.size.x = max(self.size.x, size.x);
                self.size.y += size.y;
            }
        }
    }
}
