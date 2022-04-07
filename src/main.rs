// TODO: add new items
// TODO: edit items
// TODO: delete items
// TODO: keep track of an item's done date
// TODO: undo system
// TODO: handle SIGINT

use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, ErrorKind, Write},
    process::{self},
};

use ncurses::*;
use ttd::{
    graphics::{layout::LayoutDir, ui::UI, vec2::Vec2},
    Status, HIGHLIGHT_PAIR, ID, REGULAR_PAIR,
};

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

fn drag_up(list: &mut Vec<String>, selected: &mut ID) {
    if *selected > 0 {
        list.swap(*selected, *selected - 1);
        *selected -= 1;
    }
}
fn drag_down(list: &mut Vec<String>, selected: &mut ID) {
    if *selected + 1 < list.len() {
        list.swap(*selected, *selected + 1);
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

fn parse_item(line: &str) -> Option<(Status, &str)> {
    let todo_item = line
        .strip_prefix("TODO: ")
        .map(|label| (Status::Todo, label));
    let done_item = line
        .strip_prefix("DONE: ")
        .map(|label| (Status::Done, label));

    todo_item.or(done_item)
}

fn load_state(todos: &mut Vec<String>, dones: &mut Vec<String>, file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;

    for (index, line) in BufReader::new(file).lines().enumerate() {
        match parse_item(&line?) {
            Some((Status::Todo, label)) => todos.push(String::from(label)),
            Some((Status::Done, label)) => dones.push(String::from(label)),
            None => {
                eprintln!("{}:{}: ERROR: ill-formed item line", file_path, index + 1);
                process::exit(1);
            }
        }
    }

    Ok(())
}

fn save_state(todos: &[String], dones: &[String], file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for todo in todos.iter() {
        writeln!(file, "TODO: {}", todo)?;
    }
    for done in dones.iter() {
        writeln!(file, "DONE: {}", done)?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    args.next().unwrap();

    let file_path = match args.next() {
        Some(file_path) => file_path,
        None => {
            eprintln!("Usage: ttd <file-path>");
            eprintln!("ERROR: file path is not provided");
            process::exit(1)
        }
    };

    let mut todos = Vec::<String>::new();
    let mut dones = Vec::<String>::new();
    let mut selected_todo: ID = 0;
    let mut selected_done: ID = 0;

    load_state(&mut todos, &mut dones, &file_path)?;
    let mut notification: String;

    match load_state(&mut todos, &mut dones, &file_path) {
        Ok(()) => notification = format!("Loaded file {}", file_path),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                notification = format!("New file {}", file_path);
            } else {
                panic!("Could not load state from file `{}`: {:?}", file_path, e);
            }
        }
    }

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;

    let mut focus = Status::Todo;

    let mut ui = UI::default();

    while !quit {
        erase();

        let mut window_w = 0;
        let mut window_h = 0;
        getmaxyx(stdscr(), &mut window_h, &mut window_w);

        ui.begin(Vec2::zero(), LayoutDir::Vertical);

        ui.label_fixed_width(&notification, REGULAR_PAIR, window_w);
        notification.clear();
        ui.label_fixed_width("", REGULAR_PAIR, window_w);

        ui.begin_layout(LayoutDir::Horizontal);

        {
            ui.begin_layout(LayoutDir::Vertical);

            let todo_header_color = if focus == Status::Todo {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            };

            ui.label_fixed_width(" TODO ", todo_header_color, window_w / 2);
            ui.label_fixed_width("------------", REGULAR_PAIR, window_w / 2);

            for (index, todo) in todos.iter().enumerate() {
                let pair = if index == selected_todo && focus == Status::Todo {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                };
                ui.label_fixed_width(&format!("- [ ] {}", todo), pair, window_w / 2);
            }

            ui.end_layout();
        }

        {
            ui.begin_layout(LayoutDir::Vertical);

            let done_header_color = if focus == Status::Done {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            };

            ui.label_fixed_width(" DONE ", done_header_color, window_w / 2);
            ui.label_fixed_width("------------", REGULAR_PAIR, window_w / 2);

            for (index, done) in dones.iter().enumerate() {
                let pair = if index == selected_done && focus == Status::Done {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                };
                ui.label_fixed_width(&format!("- [x] {}", done), pair, window_w / 2);
            }

            ui.end_layout();
        }

        ui.end_layout();

        ui.end();

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'w' | 'z' => match focus {
                Status::Todo => move_up(&mut selected_todo),
                Status::Done => move_up(&mut selected_done),
            },
            'W' | 'Z' => match focus {
                Status::Todo => drag_up(&mut todos, &mut selected_todo),
                Status::Done => drag_up(&mut dones, &mut selected_done),
            },
            's' => match focus {
                Status::Todo => move_down(&mut selected_todo, &todos),
                Status::Done => move_down(&mut selected_done, &dones),
            },
            'S' => match focus {
                Status::Todo => drag_down(&mut todos, &mut selected_todo),
                Status::Done => drag_down(&mut dones, &mut selected_done),
            },
            '\n' => match focus {
                Status::Todo => {
                    list_transfer(&mut todos, &mut dones, &mut selected_todo);
                    notification.push_str("DONE!");
                }
                Status::Done => {
                    list_transfer(&mut dones, &mut todos, &mut selected_done);
                    notification.push_str("Not done yet...");
                }
            },
            '\t' => {
                focus = focus.toggle();
            }
            _ => {}
        }
    }

    endwin();

    save_state(&todos, &dones, &file_path)?;
    println!("Saved state to {}", file_path);

    Ok(())
}
