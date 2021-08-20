use std::{convert::TryInto, time::Duration};

use termion;
use termion::raw::IntoRawMode;
use termion::terminal_size;

use std::io::{Read, Write};

use crate::{direction::Direction, point::Point2d};

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

    let mut snake: Vec<Point2d> = vec![
        Point2d::new(width / 2, height / 2),
        Point2d::new(width / 2 + 2, height / 2),
        Point2d::new(width / 2 + 4, height / 2),
        Point2d::new(width / 2 + 6, height / 2),
        Point2d::new(width / 2 + 8, height / 2),
    ];

    let mut exit = false;
    let mut lost = false;
    let mut direction: Direction = Direction::Left;
    let mut frute = Point2d::new_random_point(height, width);

    loop {
        write!(&mut stdout, "{}", termion::clear::All,).ok();
        write!(
            &mut stdout,
            "{}Higscore: {}",
            termion::cursor::Goto(1, 1),
            snake.len() * 100
        )
        .ok();

        let key = stdin.next();

        if let Some(Ok(key)) = key {
            match key {
                b'c' => {
                    write!(
                        &mut stdout,
                        "{}{}{}",
                        termion::cursor::Goto(1, 1),
                        termion::cursor::Show,
                        termion::clear::All
                    )
                    .ok();
                    exit = true;
                }
                x => {
                    let new_direction: Option<Direction> = x.try_into().ok();
                    if let Some(new_direction) = new_direction {
                        if !new_direction.isOpposite(&direction) {
                            direction = new_direction;
                        }
                    }
                }
            }
        }

        let last_tail = snake.last().unwrap().clone();

        if !lost {
            for index in (1..snake.len()).rev() {
                snake[index].x = snake[index - 1].x;
                snake[index].y = snake[index - 1].y;
            }

            if snake[0]
                .move_by_direction(height, width, &direction)
                .is_err()
            {
                lost = true;
            }

            lost = lost || snake.iter().skip(1).find(|x| &snake[0] == *x).is_some();

            if snake[0] == frute {
                snake.push(last_tail);
                frute = Point2d::new_random_point(height, width);
            }
        }

        for ele in snake.iter() {
            write!(
                &mut stdout,
                "{}\u{2B1B}",
                termion::cursor::Goto(ele.x, ele.y),
            )
            .ok();
        }

        write!(
            &mut stdout,
            "{}\u{1f34c}",
            termion::cursor::Goto(frute.x, frute.y),
        )
        .ok();

        if lost {
            write!(
                &mut stdout,
                "{}########################
                {}# You lose, Score: {} #
                {}########################",
                termion::cursor::Goto(width / 2 - 16, height / 2 - 1),
                termion::cursor::Goto(width / 2 - 16, height / 2),
                snake.len() * 100,
                termion::cursor::Goto(width / 2 - 16, height / 2 + 1),
            )
            .ok();
        }

        stdout.flush().unwrap();
        if exit {
            write!(
                &mut stdout,
                "{}{}{}",
                termion::cursor::Goto(1, 1),
                termion::cursor::Show,
                termion::clear::All
            )
            .ok();
            return;
        }
        std::thread::sleep(Duration::from_millis(33))
    }
}
