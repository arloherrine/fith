use interpreter::Interpreter;

use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

pub fn run(mut interpreter: Interpreter) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
           "Welcome to fith. ^D to exit.\n{}Stack: \n{}> \n{}Output: {}{}",
           termion::cursor::Left(<u16>::max_value()),
           termion::cursor::Left(<u16>::max_value()),
           termion::cursor::Left(<u16>::max_value()),
           termion::cursor::Up(1),
           termion::cursor::Left(6),
    ).unwrap();
    stdout.flush().unwrap();

    loop {
        let result = run_line(&interpreter);
        match result {
            Some(new_interpreter) => {
                interpreter = new_interpreter;
            },
            None => break,
        }
    }
}

fn run_line(interpreter: &Interpreter) -> Option<Interpreter> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut current_line = Vec::new();
    let mut cursor_pos :usize = 0;
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => {
                write!(stdout, "{}{}", termion::cursor::Down(1), termion::cursor::Right(<u16>::max_value())).unwrap();
                return None
            },
            Key::Ctrl('d') => {
                write!(stdout, "{}{}", termion::cursor::Down(1), termion::cursor::Right(<u16>::max_value())).unwrap();
                return None
            },
            Key::Ctrl('a') => {
                if cursor_pos != 0 {
                    write!(stdout, "{}",
                           termion::cursor::Left(cursor_pos as u16),
                    ).unwrap();
                    cursor_pos = 0;
                }
            },
            Key::Ctrl('e') => {
                write!(stdout, "{}",
                       termion::cursor::Right((current_line.len() - cursor_pos) as u16),
                ).unwrap();
                cursor_pos = current_line.len();
            },
            Key::Ctrl('u') => {
                if cursor_pos != 0 {
                    current_line.drain(0..cursor_pos);
                    cursor_pos = 0;
                    let line_str: String = current_line.iter().cloned().collect();
                    let mut tmp_interpreter = interpreter.duplicate();
                    let result = tmp_interpreter.execute_line(&line_str);
                    write_stack_line(&mut stdout, &tmp_interpreter.stack_display());
                    write_output_line(&mut stdout, result);
                    write_prompt_line(&mut stdout, &line_str, cursor_pos);
                }
            },
            Key::Ctrl('k') => {
                if cursor_pos < current_line.len() {
                    current_line.truncate(cursor_pos);
                    cursor_pos = current_line.len();
                    let line_str: String = current_line.iter().cloned().collect();
                    let mut tmp_interpreter = interpreter.duplicate();
                    let result = tmp_interpreter.execute_line(&line_str);
                    write_stack_line(&mut stdout, &tmp_interpreter.stack_display());
                    write_output_line(&mut stdout, result);
                    write_prompt_line(&mut stdout, &line_str, cursor_pos);
                }
            },
            //Key::Ctrl('w') => ,
            Key::Char('\n') => {
                let line_str: String = current_line.iter().cloned().collect();
                let mut tmp_interpreter = interpreter.duplicate();
                let result = tmp_interpreter.execute_line(&line_str);
                write!(stdout, "{}{}",
                       termion::cursor::Up(1),
                       termion::clear::CurrentLine,
                ).unwrap();
                write_output_line(&mut stdout, result);
                write_prompt_line(&mut stdout, &line_str, cursor_pos);
                stdout.flush().unwrap();

                write!(stdout, "{}\n\n{}{}",
                    termion::cursor::Down(2),
                    termion::cursor::Up(1),
                    termion::cursor::Left(<u16>::max_value()),
                ).unwrap();
                stdout.flush().unwrap();

                write_stack_line(&mut stdout, &tmp_interpreter.stack_display());
                write_output_line(&mut stdout, Ok("".to_string()));
                write_prompt_line(&mut stdout, "", 0);
                stdout.flush().unwrap();
                return Some(tmp_interpreter);
            }
            Key::Char(c) => {
                if current_line.len() > cursor_pos {
                    current_line.insert(cursor_pos, c);
                } else {
                    current_line.push(c);
                }
                cursor_pos += 1;
                let line_str: String = current_line.iter().cloned().collect();
                let mut tmp_interpreter = interpreter.duplicate();
                let result = tmp_interpreter.execute_line(&line_str);
                write_stack_line(&mut stdout, &tmp_interpreter.stack_display());
                write_output_line(&mut stdout, result);
                write_prompt_line(&mut stdout, &line_str, cursor_pos);
            }
            Key::Left => {
                if cursor_pos != 0 {
                    cursor_pos -= 1;
                    write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
                }
            },
            Key::Right => {
                cursor_pos += 1;
                if cursor_pos > current_line.len() {
                    cursor_pos = current_line.len();
                } else {
                    write!(stdout, "{}", termion::cursor::Right(1)).unwrap();
                }
            },
            // TODO history back Key::Up =>
            // TODO history forward Key::Down =>
            Key::Backspace => {
                if cursor_pos != 0 {
                    cursor_pos -= 1;
                    current_line.remove(cursor_pos);
                    let line_str: String = current_line.iter().cloned().collect();
                    let mut tmp_interpreter = interpreter.duplicate();
                    let result = tmp_interpreter.execute_line(&line_str);
                    write_stack_line(&mut stdout, &tmp_interpreter.stack_display());
                    write_output_line(&mut stdout, result);
                    write_prompt_line(&mut stdout, &line_str, cursor_pos);
                }
            },
            _ => {}
        }
        stdout.flush().unwrap();
    }
    // I don't think should ever happen...
    None
}

fn write_stack_line<W: Write>(stdout: &mut termion::raw::RawTerminal<W>, line: &str) {
    write!(stdout, "{}{}{}Stack: {}{}{}{}{}",
        termion::cursor::Up(1),
        termion::clear::CurrentLine,
        termion::cursor::Left(<u16>::max_value()),
        termion::color::Fg(termion::color::Yellow),
        line,
        termion::style::Reset,
        termion::cursor::Down(1),
        termion::cursor::Left(<u16>::max_value()),
    ).unwrap();
    stdout.flush().unwrap();
}

fn write_output_line<W: Write>(stdout: &mut termion::raw::RawTerminal<W>, result: Result<String, String>) {
    let (error, line) = match result {
        Ok(s) => (false, s),
        Err(e) => (true, e),
    };
    if error {
        write!(stdout, "{}{}{}Output: {}{}{}{}{}",
               termion::cursor::Down(1),
               termion::clear::CurrentLine,
               termion::cursor::Left(<u16>::max_value()),
               termion::color::Fg(termion::color::Red),
               line,
               termion::style::Reset,
               termion::cursor::Up(1),
               termion::cursor::Left(<u16>::max_value()),
        ).unwrap();
    } else {
        write!(stdout, "{}{}{}Output: {}{}{}{}{}",
               termion::cursor::Down(1),
               termion::clear::CurrentLine,
               termion::cursor::Left(<u16>::max_value()),
               termion::color::Fg(termion::color::Blue),
               line,
               termion::style::Reset,
               termion::cursor::Up(1),
               termion::cursor::Left(<u16>::max_value()),
        ).unwrap();
    }
}

fn write_prompt_line<W: Write>(stdout: &mut termion::raw::RawTerminal<W>, line: &str, cursor_pos: usize) {
    if line.len() == cursor_pos {
        write!(stdout, "{}{}> {}",
               termion::clear::CurrentLine,
               termion::cursor::Left(cursor_pos as u16 + 2),
               line,
        ).unwrap();
    } else {
        write!(stdout, "{}{}> {}{}",
               termion::clear::CurrentLine,
               termion::cursor::Left(<u16>::max_value()),
               line,
               termion::cursor::Left((line.len() - cursor_pos) as u16),
        ).unwrap();
    }
}
