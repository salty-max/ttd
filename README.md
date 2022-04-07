# ttd

A interactive CLI Todo App in Rust

## Quick Start
```console
$ cargo run <file_path>
```

## Example
```console
$ cargo run SAMPLE.txt
```

## Controls

|Keys|Description|
|---|---|
|<kbd>w</kbd>, <kbd>z</kbd>|Move up|
|<kbd>s</kbd>|Move down|
|<kbd>Shift</kbd> + <kbd>w</kbd>, <kbd>Shift</kbd> + <kbd>z</kbd>|Drag item up|
|<kbd>Shift</kbd> + <kbd>s</kbd>|Drag item down|
|<kbd>Return</kbd>|Toggle selected element's state|
|<kbd>i</kbd>|Add a new TODO|
|<kbd>d</kbd>|Remove a DONE|
|<kbd>r</kbd>|Edit selected item (<kbd>Enter</kbd> to submit)|
|<kbd>Tab</kbd>|Switch between panels|
|<kbd>q</kbd>|Quit|