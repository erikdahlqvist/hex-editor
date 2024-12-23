use std::{env, fs};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{layout::{Constraint, Direction, Layout}, style::{Color, Stylize}, text::{Line, Span}, widgets::{Block, Paragraph, Wrap}, Frame};

fn main() {
    let file_path = get_file_path();    

    let mut terminal = ratatui::init();

    let raw_bytes = fs::read(&file_path).unwrap();
    let mut bytes = parse_bytes(raw_bytes);
    let mut selected_byte = 0;

    let mut input_buffer = String::with_capacity(2);

    loop {
        terminal.draw(|frame| draw(frame, &bytes, selected_byte, &input_buffer)).unwrap();
        match event::read().unwrap() {
            Event::Key(KeyEvent {code ,..}) => {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => selected_byte = if selected_byte == 0 {
                        selected_byte
                    } else {
                        selected_byte - 1
                    },
                    KeyCode::Right => selected_byte = (selected_byte + 1).min(bytes.len() - 1),
                    KeyCode::Delete | KeyCode::Backspace => if !input_buffer.is_empty() {
                        input_buffer.remove(input_buffer.len() - 1);
                    }
                    KeyCode::Char('w') => save(file_path.clone(), bytes.clone()),
                    KeyCode::Char(c) => if c.is_ascii_hexdigit() && input_buffer.len() < 2 {
                        input_buffer += &c.to_uppercase().collect::<String>();
                    },
                    KeyCode::Enter => if input_buffer.len() >= 2 {
                        bytes[selected_byte] = input_buffer.clone();
                        input_buffer.clear();
                    }
                    _ => (),
                }
            }
            _ => (),
        };
    }
    ratatui::restore();
}

fn get_file_path() -> String {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        panic!("No input file");
    }

    let file_path = &arguments[1];

    if fs::exists(&file_path).unwrap_or(false) == false {
        panic!("No such file");
    }

    return file_path.to_string();
}

fn save(file_path: String, bytes: Vec<String>) {
    let parsed: Vec<u8> = bytes.iter().map(|byte|
        u8::from_str_radix(&byte, 16).unwrap()
    ).collect();

    fs::write(file_path, parsed).unwrap();
}

fn parse_bytes(bytes: Vec<u8>) -> Vec<String> {
    return bytes.iter().map(|byte| format!("{byte:X}")).collect();
}

fn draw(frame: &mut Frame, bytes: &Vec<String>, selected_byte: usize, input_buffer: &str) {
    let editor_span = bytes.iter().enumerate().flat_map(|(i, byte)|
            [
                Span::from(byte).bg(if i == selected_byte {
                    Color::DarkGray
                } else {
                    Color::default()
                }),
                Span::raw(" ")
            ]
        ).collect::<Vec<Span>>();

    let editor_text: Line = editor_span.into();
    let editor = Paragraph::new(editor_text)
        .wrap(Wrap {trim: true})
        .block(Block::bordered());

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(Vec::from([Constraint::Percentage(100), Constraint::Min(1)]))
        .split(frame.area());

    let status_bar_layout= Layout::default()
        .direction(Direction::Horizontal)
        .constraints(Vec::from([Constraint::Length(20), Constraint::Length(20)]))
        .split(main_layout[1]);

    frame.render_widget(editor, main_layout[0]);

    frame.render_widget(String::from("Cursor: ") + &selected_byte.to_string(), status_bar_layout[0]);
    frame.render_widget(String::from("Buffer: ") + input_buffer, status_bar_layout[1]);
}