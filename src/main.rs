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
                ui.label("[TODO] DONE", REGULAR_PAIR);
                ui.begin_list(selected_todo);
                for (index, todo) in todos.iter().enumerate() {
                    ui.list_element(&format!("- [ ] {}", todo), index);
                }
                ui.end_list();
            }
            Status::Done => {
                ui.label(" TODO [DONE]", REGULAR_PAIR);
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
                Status::Todo => {
                    if selected_todo < todos.len() {
                        dones.push(todos.remove(selected_todo))
                    }
                }
                Status::Done => {
                    if selected_done < dones.len() {
                        todos.push(dones.remove(selected_done))
                    }
                }
            },
            '\t' => {
                focus = focus.toggle();
            }
            _ => {}
        }
    }

    endwin();
}
