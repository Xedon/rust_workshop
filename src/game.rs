use std::{convert::TryInto, io::Stdout, ops::Deref};

use termion::raw::RawTerminal;

use crate::{direction::Direction, point::Point2d};
use std::io::Write;
#[derive(Debug)]
pub struct Snake {
    body: Vec<Point2d>,
    direction: Direction,
    alive: bool,
}

impl Snake {
    fn new(max_y: u16, max_x: u16) -> Self {
        let mut body = vec![];
        body.push(Point2d::new((max_x / 2).into(), (max_y / 2).into()));

        Snake {
            body,
            alive: true,
            direction: Direction::Left,
        }
    }

    fn update(
        &mut self,
        new_direction: Option<Direction>,
        max_y: u16,
        max_x: u16,
        food_pos: &Point2d,
    ) {
        let current_tail = self.body[self.body.len() - 1].clone();
        let current_direction = &self.direction.clone();

        for i in (0..self.body.len()).rev() {
            if i != 0 {
                self.body[i].x = self.body[i - 1].x;
                self.body[i].y = self.body[i - 1].y;
            }
        }

        if self.body[0] == *food_pos {
            self.body.push(current_tail);
        }
        {
            let head = &mut self.body[0];

            if new_direction.is_some() {
                let new_direction = new_direction.unwrap();
                if new_direction.ne(current_direction)
                    && !new_direction.isOpposite(current_direction)
                {
                    self.alive =
                        self.alive && head.move_by_direction(max_y, max_x, &new_direction).is_ok();
                    self.direction = new_direction;
                } else {
                    self.alive = self.alive
                        && head
                            .move_by_direction(max_y, max_x, current_direction)
                            .is_ok();
                }
            } else {
                self.alive = self.alive
                    && head
                        .move_by_direction(max_y, max_x, current_direction)
                        .is_ok();
            }
        }

        let head = &self.body[0];
        self.alive = self.alive
            && self
                .body
                .iter()
                .skip(1)
                .find(|body_part| body_part.deref().eq(head))
                .is_none();
    }

    fn draw(&self, terminal: &mut RawTerminal<Stdout>) {
        for ele in self.body.iter() {
            write!(
                terminal,
                "{}{}",
                termion::cursor::Goto((ele.x).try_into().unwrap(), (ele.y).try_into().unwrap()),
                "\u{2B1B}",
            )
            .ok();
        }
    }
}

#[derive(Debug)]
pub struct Game {
    snake: Snake,
    max_y: u16,
    max_x: u16,
    score: u128,
    food_pos: Point2d,
    paused: bool,
    debug: bool,
}

impl Game {
    pub fn new(max_y: u16, max_x: u16) -> Self {
        Game {
            snake: Snake::new(max_y, max_x),
            max_x,
            max_y,
            score: 0,
            food_pos: Point2d::new_random_point(max_y, max_x),
            paused: true,
            debug: false,
        }
    }
}

impl Game {
    pub fn update(&mut self, pressed_key: char) {
        let new_direction: Option<Direction> = pressed_key.try_into().ok();

        match pressed_key {
            'p' => {
                self.paused = !self.paused;
            }
            '^' => {
                self.debug = !self.debug;
            }
            _ => {}
        }

        if !self.paused && self.snake.alive {
            let last_level = self.snake.body.len();
            self.snake
                .update(new_direction, self.max_y, self.max_x, &self.food_pos);
            let new_level = self.snake.body.len();

            if last_level != new_level {
                self.food_pos = Point2d::new_random_point(self.max_y, self.max_x);
            }
        }
    }

    pub fn draw(&mut self, terminal: &mut RawTerminal<Stdout>) {
        if self.debug {
            write!(
                terminal,
                "{} Snake: {:?} Food: {:?}",
                termion::cursor::Goto(1, 1),
                self.snake,
                self.food_pos,
            )
            .ok();
        } else {
            write!(
                terminal,
                "{}Level:{}",
                termion::cursor::Goto(1, 1),
                self.snake.body.len()
            )
            .ok();
        }

        self.snake.draw(terminal);
        write!(
            terminal,
            "{}{}",
            termion::cursor::Goto(
                (self.food_pos.x).try_into().unwrap(),
                (self.food_pos.y).try_into().unwrap()
            ),
            "\u{1F34E}",
        )
        .ok();

        if !self.snake.alive {
            write!(
                terminal,
                "{}{}",
                termion::cursor::Goto(
                    (self.max_x / 2 - 6).try_into().unwrap(),
                    (self.max_y / 2).try_into().unwrap()
                ),
                "############",
            )
            .ok();
            write!(
                terminal,
                "{}{}",
                termion::cursor::Goto(
                    (self.max_x / 2 - 6).try_into().unwrap(),
                    (self.max_y / 2 - 1).try_into().unwrap()
                ),
                "# You Lose #",
            )
            .ok();
            write!(
                terminal,
                "{}{}",
                termion::cursor::Goto(
                    (self.max_x / 2 - 6).try_into().unwrap(),
                    (self.max_y / 2 - 2).try_into().unwrap()
                ),
                "############",
            )
            .ok();
        }

        if self.paused {
            write!(
                terminal,
                "{}{}",
                termion::cursor::Goto(
                    (self.max_x / 2 - 12).try_into().unwrap(),
                    (self.max_y / 2).try_into().unwrap()
                ),
                "######################",
            )
            .ok();
            write!(
                terminal,
                "{}{}",
                termion::cursor::Goto(
                    (self.max_x / 2 - 12).try_into().unwrap(),
                    (self.max_y / 2 - 1).try_into().unwrap()
                ),
                "# Press 'p' to start #",
            )
            .ok();
            write!(
                terminal,
                "{}{}",
                termion::cursor::Goto(
                    (self.max_x / 2 - 12).try_into().unwrap(),
                    (self.max_y / 2 - 2).try_into().unwrap()
                ),
                "######################",
            )
            .ok();
        }
    }
}
