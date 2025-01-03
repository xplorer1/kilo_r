use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub struct Editor {

}

impl Editor {
    pub fn new() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();

        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{:?} \r", event);
                    match event.code {
                        Char(ch) => {
                            if ch == 'q' {
                                break;
                            }
                        },
                        _ => (),
                    }
                },
                Err(err) => println!("Error: {}", err),
                _ => ()
            }
        }

        disable_raw_mode().unwrap();
    }
}