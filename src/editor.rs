use std::io::{self, stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
   should_quit: bool,
}

impl Editor {
 pub fn run(&mut self) {
   let _stdout = stdout().into_raw_mode().unwrap();
   
   loop {
      if let Err(error) = self.refresh_screen() {
         die(error);
      }
      if self.should_quit {
         break;
      }
      if let Err(error) = self.process_keypress() {
         die(error);
      }
   }
 }

 pub fn new() -> Self {
   Self { should_quit: false }
 }

 fn refresh_screen(&self) -> Result<(), std::io::Error> {
   print!("{}", termion::clear::All);
   io::stdout().flush()
 }

 fn process_keypress(&mut self) -> Result<(), std::io::Error> {
   let pressed_key = Self::read_key()?;
   match pressed_key {
      Key::Ctrl('x') => self.should_quit = true,
      _ => (),
   }
   Ok(())
 }

 fn read_key() -> Result<Key, std::io::Error> {
   loop {
      if let Some(key) = stdin().lock().keys().next() {
         return key;
      }
   }
 }

}

fn die(e: std::io::Error) {
   panic!("{}", e)
}