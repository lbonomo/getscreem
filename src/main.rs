#![windows_subsystem = "windows"]

use screenshots::Screen;
use std::{fs, time::Instant};

fn main() {
  let _start = Instant::now();
  let screens = Screen::all().unwrap();

  for screen in screens {
    let image = screen.capture().unwrap();
    let buffer = image.buffer();
    fs::write(format!("./{}.png", screen.display_info.id), &buffer).unwrap();
  }
}
