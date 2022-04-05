// TODO: add new items
// TODO: edit items
// TODO: delete items
// TODO: keep track of an item's done date
// TODO: undo system
// TODO: handle SIGINT

use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
    process::{self},
};

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

fn parse_item(line: &str) -> Option<(Status, &str)> {
    let todo_prefix = "TODO: ";
    let done_prefix = "DONE: ";

    if let Some(label) = line.strip_prefix(todo_prefix) {
        return Some((Status::Todo, label));
    }

    if let Some(label) = line.strip_prefix(done_prefix) {
        return Some((Status::Done, label));
    }

    None
}

fn load_state(todos: &mut Vec<String>, dones: &mut Vec<String>, file_path: &str) -> io::Result<()> {
    let file = {
        if Path::new(file_path).exists() {
            File::open(file_path)?
        } else {
            File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(file_path)?
        }
    };

    let buffer = BufReader::new(file);

    for (index, line) in buffer.lines().enumerate() {
        match parse_item(&line.unwrap()) {
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
            // 'S' => {
            //     let mut file = File::create("TODO.txt")?;
            //     for todo in todos.iter() {
            //         writeln!(file, "TODO: {}", todo)?;
            //     }
            //     for done in dones.iter() {
            //         writeln!(file, "DONE: {}", done)?;
            //     }
            // }
            _ => {}
        }
    }

    save_state(&todos, &dones, &file_path)?;

    endwin();

    Ok(())
}
