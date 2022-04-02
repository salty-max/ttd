use ncurses::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let todos = vec!["Pet the dog", "Write a Todo app", "Eat junk food"];
    let mut selected_todo: usize = 0;

    while !quit {
        for (index, todo) in todos.iter().enumerate() {
            let pair = {
                if selected_todo == index {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                }
            };
            attron(COLOR_PAIR(pair));
            mv(index as i32, 0);
            addstr(*todo);
            attroff(COLOR_PAIR(pair));
        }

        refresh();

        let key = getch();
        match key {
            113 => quit = true,
            KEY_UP => {
                if selected_todo > 0 {
                    selected_todo -= 1;
                } else {
                    selected_todo = todos.len() - 1;
                }
            }
            KEY_DOWN => {
                if selected_todo < todos.len() - 1 {
                    selected_todo += 1;
                } else {
                    selected_todo = 0;
                }
            }
            _ => {}
        }
    }

    endwin();
}
