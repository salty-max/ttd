// TODO: add new items
// TODO: edit items
// TODO: keep track of an item's done date
// TODO: undo system
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, ErrorKind, Write},
    process::{self},
};

use ncurses::*;
use ttd::{
    ctrlc,
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

fn list_delete(list: &mut Vec<String>, selected: &mut ID) {
    if *selected < list.len() {
        list.remove(*selected);
        if *selected >= list.len() && !list.is_empty() {
            *selected = list.len() - 1;
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
    ctrlc::init();

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
    keypad(stdscr(), true);
    timeout(16);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;

    let mut focus = Status::Todo;

    let mut editing = false;
    let mut edit_cursor = 0;

    let mut ui = UI::default();
    let mut current_key = None;

    while !quit && !ctrlc::poll() {
        erase();

        let mut window_w = 0;
        let mut window_h = 0;
        getmaxyx(stdscr(), &mut window_h, &mut window_w);

        ui.begin(Vec2::zero(), LayoutDir::Vertical);

        {
            ui.label_fixed_width(&notification, REGULAR_PAIR, window_w);
            ui.label_fixed_width("", REGULAR_PAIR, window_w);

            ui.begin_layout(LayoutDir::Horizontal);

            {
                ui.begin_layout(LayoutDir::Vertical);

                {
                    if focus == Status::Todo {
                        ui.label_fixed_width(" TODO ", HIGHLIGHT_PAIR, window_w / 2);
                        ui.label_fixed_width("------------", REGULAR_PAIR, window_w / 2);

                        for (index, todo) in todos.iter_mut().enumerate() {
                            if index == selected_todo {
                                if editing {
                                    ui.edit_field(
                                        todo,
                                        &mut edit_cursor,
                                        &mut current_key,
                                        window_w / 2,
                                    );

                                    if let Some('\n') = current_key.take().map(|x| x as u8 as char)
                                    {
                                        notification.push_str("Exit EDIT mode");
                                        editing = false;
                                    }
                                } else {
                                    ui.label_fixed_width(
                                        &format!("- [ ] {}", todo),
                                        HIGHLIGHT_PAIR,
                                        window_w / 2,
                                    );

                                    if let Some('r') = current_key.map(|x| x as u8 as char) {
                                        notification.push_str("Enter EDIT mode");
                                        editing = true;
                                        edit_cursor = todo.len();
                                        current_key = None;
                                    }
                                }
                            } else {
                                ui.label_fixed_width(
                                    &format!("- [ ] {}", todo),
                                    REGULAR_PAIR,
                                    window_w / 2,
                                );
                            }
                        }

                        if let Some(key) = current_key.take() {
                            match key as u8 as char {
                                'W' | 'Z' => drag_up(&mut todos, &mut selected_todo),
                                'S' => drag_down(&mut todos, &mut selected_todo),
                                'w' | 'z' => move_up(&mut selected_todo),
                                's' => move_down(&mut selected_todo, &todos),
                                'i' => {
                                    todos.insert(selected_todo, String::new());
                                    edit_cursor = 0;
                                    editing = true;
                                    notification.push_str("What needs to be done?");
                                }
                                'd' => {
                                    notification.push_str(
                                        "Cannot remove items from TODO. Mark it as DONE first.",
                                    );
                                }
                                '\n' => {
                                    list_transfer(&mut todos, &mut dones, &mut selected_todo);
                                    notification.push_str("DONE!");
                                }
                                '\t' => {
                                    focus = focus.toggle();
                                }
                                _ => {}
                            }
                        }
                    } else {
                        ui.label_fixed_width(" TODO ", REGULAR_PAIR, window_w / 2);
                        ui.label_fixed_width("------------", REGULAR_PAIR, window_w / 2);

                        for todo in todos.iter() {
                            ui.label_fixed_width(
                                &format!("- [ ] {}", todo),
                                REGULAR_PAIR,
                                window_w / 2,
                            );
                        }
                    }
                }

                ui.end_layout();
            }

            {
                ui.begin_layout(LayoutDir::Vertical);

                {
                    if focus == Status::Done {
                        ui.label_fixed_width(" DONE ", HIGHLIGHT_PAIR, window_w / 2);
                        ui.label_fixed_width("------------", REGULAR_PAIR, window_w / 2);

                        for (index, done) in dones.iter_mut().enumerate() {
                            if index == selected_done {
                                if editing {
                                    ui.edit_field(
                                        done,
                                        &mut edit_cursor,
                                        &mut current_key,
                                        window_w / 2,
                                    );

                                    if let Some('\n') = current_key.take().map(|x| x as u8 as char)
                                    {
                                        editing = false;
                                    }
                                } else {
                                    ui.label_fixed_width(
                                        &format!("- [x] {}", done),
                                        HIGHLIGHT_PAIR,
                                        window_w / 2,
                                    );

                                    if let Some('r') = current_key.map(|x| x as u8 as char) {
                                        editing = true;
                                        edit_cursor = done.len();
                                        current_key = None;
                                    }
                                }
                            } else {
                                ui.label_fixed_width(
                                    &format!("- [x] {}", done),
                                    REGULAR_PAIR,
                                    window_w / 2,
                                );
                            }
                        }

                        if let Some(key) = current_key.take() {
                            match key as u8 as char {
                                'W' | 'Z' => drag_up(&mut dones, &mut selected_done),
                                'S' => drag_down(&mut dones, &mut selected_done),
                                'w' | 'z' => move_up(&mut selected_done),
                                's' => move_down(&mut selected_done, &dones),
                                'i' => notification.push_str(
                                    "Cannot insert new DONE items. Only TODO is allowed.",
                                ),
                                'd' => list_delete(&mut dones, &mut selected_done),
                                '\n' => {
                                    list_transfer(&mut dones, &mut todos, &mut selected_done);
                                    notification.push_str("Not done yet...");
                                }
                                '\t' => {
                                    focus = focus.toggle();
                                }
                                _ => {}
                            }
                        }
                    } else {
                        ui.label_fixed_width(" DONE ", REGULAR_PAIR, window_w / 2);
                        ui.label_fixed_width("------------", REGULAR_PAIR, window_w / 2);

                        for done in dones.iter() {
                            ui.label_fixed_width(
                                &format!("- [x] {}", done),
                                REGULAR_PAIR,
                                window_w / 2,
                            );
                        }
                    }
                }

                ui.end_layout();
            }

            ui.end_layout();
        }

        ui.end();

        if let Some('q') = current_key.take().map(|x| x as u8 as char) {
            quit = true;
        }

        refresh();

        let key = getch();
        if key != ERR {
            notification.clear();
            current_key = Some(key);
        }
    }

    endwin();

    save_state(&todos, &dones, &file_path)?;
    println!("Saved state to {} 👋", file_path);

    Ok(())
}
