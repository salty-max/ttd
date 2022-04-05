// TODO: persist app state
// TODO: add new items
// TODO: edit items
// TODO: keep track of an item's done date
// TODO: undo system

use ncurses::*;
use ttd::{ui::UI, Status, HIGHLIGHT_PAIR, ID, REGULAR_PAIR};

fn move_up(selected: &mut ID) {
    if *selected > 0 {
        *selected -= 1
    }
}
fn move_down(selected: &mut ID, list: &[String]) {
    if *selected + 1 < list.len() {
        *selected += 1;
    }
}

fn list_transfer(
    src_list: &mut Vec<String>,
    dest_list: &mut Vec<String>,
    src_selected_item: &mut ID,
) {
    if *src_selected_item < src_list.len() {
        dest_list.push(src_list.remove(*src_selected_item));
        if *src_selected_item >= src_list.len() && !src_list.is_empty() {
            *src_selected_item = src_list.len() - 1;
        }
    }
}

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;

    let mut todos = vec![
        String::from("Pet the dog"),
        String::from("Write a Todo app"),
        String::from("Eat junk food"),
    ];
    let mut dones = vec![
        String::from("Being pissed at react-native"),
        String::from("Eat lunch"),
    ];
    let mut selected_todo: usize = 0;
    let mut selected_done: usize = 0;

    let mut focus = Status::Todo;

    let mut ui = UI::default();

    while !quit {
        erase();
        ui.begin(0, 0);

        match focus {
            Status::Todo => {
                ui.label("[TODO] DONE ", REGULAR_PAIR);
                ui.label("------------", REGULAR_PAIR);
                ui.begin_list(selected_todo);
                for (index, todo) in todos.iter().enumerate() {
                    ui.list_element(&format!("- [ ] {}", todo), index);
                }
                ui.end_list();
            }
            Status::Done => {
                ui.label(" TODO [DONE]", REGULAR_PAIR);
                ui.label("------------", REGULAR_PAIR);
                ui.begin_list(selected_done);
                for (index, done) in dones.iter().enumerate() {
                    ui.list_element(&format!("- [X] {}", done), index);
                }
                ui.end_list();
            }
        }

        ui.label("------------------------------", REGULAR_PAIR);

        ui.end();

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'w' | 'z' => match focus {
                Status::Todo => move_up(&mut selected_todo),
                Status::Done => move_up(&mut selected_done),
            },
            's' => match focus {
                Status::Todo => move_down(&mut selected_todo, &todos),
                Status::Done => move_down(&mut selected_done, &dones),
            },
            '\n' => match focus {
                Status::Todo => list_transfer(&mut todos, &mut dones, &mut selected_todo),
                Status::Done => list_transfer(&mut dones, &mut todos, &mut selected_done),
            },
            '\t' => {
                focus = focus.toggle();
            }
            _ => {}
        }
    }

    endwin();
}
