use crossterm::cursor::MoveToNextLine;
use rand::Rng;
use std::time::Instant;

use std::io::stdout;

use crossterm::{
    cursor::{MoveLeft, MoveRight},
    event::{read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
};

#[macro_use]
extern crate text_io;

//TODO Is better to implement Copy? Or to pass a reference?
#[derive(Clone, Copy)]
enum OpType {
    Add,
    Sub,
}

impl OpType {
    fn to_str(self) -> &'static str {
        match self {
            OpType::Add => "+",
            OpType::Sub => "-",
        }
    }

    fn do_op(self: Self, a: i32, b: i32) -> i32 {
        match self {
            OpType::Add => a + b,
            OpType::Sub => a - b,
        }
    }
}

fn old_do_input() -> i32 {
    read!()
}

fn do_input() -> i32 {
    let mut input = String::new();

    execute!(
        stdout(),
        //SetForegroundColor(Color::Blue),
        //SetBackgroundColor(Color::Red),
        MoveRight(3),
    )
    .ok();

    enable_raw_mode();

    loop {
        let event = read().unwrap();

        if event == Event::Key(KeyCode::Esc.into()) {
            disable_raw_mode();
            std::process::exit(0);
        }

        if event == Event::Key(KeyCode::Backspace.into()) {
            if input.len() > 0 {
                input.remove(0);
                execute!(stdout(), Print(" ")).ok();
            }
        }

        if event == Event::Key(KeyCode::Enter.into()) {
            execute!(stdout(), MoveToNextLine(1)).ok();
            println!();
            break;
        }

        match event {
            Event::Key(c) => {
                if c.code >= KeyCode::Char('0') && c.code <= KeyCode::Char('9') {
                    match c.code {
                        KeyCode::Char(ch) => {
                            input.insert(0, ch);
                            execute!(stdout(), Print(ch), MoveLeft(2),).ok();
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    disable_raw_mode();
    input.parse().unwrap()
}

fn main() {
    let mut good = 0;
    let mut bad = 0;
    let mut input: i32;
    let start = Instant::now();
    let mut rng = rand::thread_rng();
    let mut end = false;

    while !end {
        let a = rng.gen_range(0..99);
        let b = rng.gen_range(0..10);
        let op: OpType = OpType::Add;

        println!();
        println!("  {:>2}", a);
        println!("{} {:>2}", op.to_str(), b);
        println!("------");

        input = do_input();

        if input == op.do_op(a, b) {
            println!("OK");
            good = good + 1;
        } else {
            println!("KO");
            bad = bad + 1;
        }

        if start.elapsed().as_secs() > 60 * 1 {
            end = true;
            println!();
            println!("Respuestas:");
            println!("Totales:   {}", good + bad);
            println!("Correctas: {}", good);

            //TODO Guardar log en archivo
        }
    }
}
