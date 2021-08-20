use std::time::Duration;

use termion;
use termion::raw::IntoRawMode;
use termion::terminal_size;

use std::io::{Read, Write};

mod direction;
mod point;

fn main() {
    // Set terminal to raw mode to allow reading stdin one key at a time
    let mut stdout = std::io::stdout()
        .into_raw_mode()
        .expect("Can't enable terminal raw mode, exiting...");

    let mut stdin = termion::async_stdin().bytes();

    write!(
        &mut stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .ok();
    let (width, height) = terminal_size().expect("Can't get terminal size, exiting....");

    loop {
        write!(&mut stdout, "{}", termion::clear::All,).ok();

        let key = stdin.next();

        stdout.flush().unwrap();
        std::thread::sleep(Duration::from_millis(333))
    }
}
