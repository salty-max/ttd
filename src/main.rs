use std::cmp::min;

use ncurses::*;
use ttd::prelude::*;

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    // let mut items = vec![
    //     Todo::new(String::from("Pet the cat")),
    //     Todo::new(String::from("Write a Todo app")),
    //     Todo::new(String::from("Eat dinner")),
    // ];

    // items[0].set_state();
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

    let mut ui = UI::default();

    while !quit {
        ui.begin(0, 0);

        ui.label("TODO:", REGULAR_PAIR);
        ui.begin_list(selected_todo);
        for (index, todo) in todos.iter().enumerate() {
            ui.list_element(&format!("- [ ] {}", todo), index);
        }
        ui.end_list();

        ui.label("------------------------------", REGULAR_PAIR);

        ui.label("DONE:", REGULAR_PAIR);
        ui.begin_list(0);
        for (index, done) in dones.iter().enumerate() {
            ui.list_element(&format!("- [X] {}", done), index + 1);
        }
        ui.end_list();

        ui.end();

        refresh();

        let key = getch();
        match key {
            113 => quit = true,
            KEY_UP => {
                if selected_todo > 0 {
                    selected_todo -= 1
                }
            }
            KEY_DOWN => {
                if selected_todo + 1 < todos.len() {
                    selected_todo = min(selected_todo + 1, todos.len() - 1)
                }
            }
            10 => {
                if selected_todo < todos.len() {
                    dones.push(todos.remove(selected_todo));
                }
            }
            _ => {}
        }
    }

    endwin();
}
