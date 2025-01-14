use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }

    fn repl(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;

        loop {
            if let Key(event) = read()? {
                println!("{event:?} \r");

                if let Char(c) = event.code {
                    if c == 'q' {
                        break;
                    }
                }
            }
        }

        disable_raw_mode()?;
        Ok(())
    }
}