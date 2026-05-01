// Full game loop in the terminal with crossterm. State machines, event handling, ticks

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, terminal,
};
use rand::Rng;
use std::{
    io::{Write, stdout},
    time::Duration,
};

struct Game {
    snake: Vec<(u16, u16)>,
    score: u16,
    food: (u16, u16),
    width: u16,
    height: u16,
    dir: Direction,
    is_over: bool,
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    fn new(w: u16, h: u16) -> Self {
        Game {
            snake: vec![(w / 2, h / 2)],
            score: 0,
            food: (w / 4, h / 4),
            width: w,
            height: h,
            dir: Direction::Right,
            is_over: false,
        }
    }

    fn update(&mut self) -> Result<(), std::io::Error> {
        if self.is_over {
            return Ok(());
        }

        let mut head = *self.snake.first().unwrap();

        match self.dir {
            Direction::Up => head.1 = head.1.saturating_sub(1),
            Direction::Down => head.1 += 1,
            Direction::Right => head.0 += 1,
            Direction::Left => head.0 = head.0.saturating_sub(1),
        }

        if head.0 == 0 || head.0 >= self.width || head.1 == 0 || head.1 >= self.height {
            self.is_over = true;
            return Ok(());
        }

        if self.snake.contains(&head) {
            self.is_over = true;
            return Ok(());
        }

        if head == self.food {
            self.score += 1;
            self.food = (
                rand::thread_rng().gen_range(1..self.width - 1),
                rand::thread_rng().gen_range(1..self.height - 1),
            );
        } else {
            self.snake.pop();
        }

        self.snake.insert(0, head);
        Ok(())
    }

    fn update_dir(&mut self, dir: Direction) {
        self.dir = dir;
    }

    fn draw(&self) -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        self.draw_border()?;
        if self.is_over {
            execute!(stdout, cursor::MoveTo(self.width / 2 - 5, self.height / 2))?;
            print!("GAME OVER! Score: {}", self.score);
        } else {
            execute!(stdout, cursor::MoveTo(self.food.0, self.food.1))?;
            print!("*");

            for &(x, y) in &self.snake {
                execute!(stdout, cursor::MoveTo(x, y))?;
                print!("0");
            }
        }

        stdout.flush()?;
        Ok(())
    }

    fn draw_border(&self) -> Result<(), std::io::Error> {
        let mut stdout = stdout();

        for x in 0..self.width {
            execute!(stdout, cursor::MoveTo(x, 0))?;
            print!("#");
            execute!(stdout, cursor::MoveTo(x, self.height - 1))?;
            print!("#");
        }

        for y in 0..self.height {
            execute!(stdout, cursor::MoveTo(0, y))?;
            print!("#");
            execute!(stdout, cursor::MoveTo(self.width - 1, y))?;
            print!("#");
        }
        Ok(())
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    // let mut snake: Vec<(u16, u16)> = vec![(10, 10)];
    // let mut food: (u16, u16) = (12, 13);
    let _ = terminal::enable_raw_mode();
    execute!(stdout, cursor::Hide)?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    // let mut dir = Direction::Right;
    let mut game = Game::new(30, 30);

    loop {
        if event::poll(Duration::from_millis(300))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up if game.dir != Direction::Down => game.update_dir(Direction::Up),
                    KeyCode::Down if game.dir != Direction::Up => game.update_dir(Direction::Down),
                    KeyCode::Left if game.dir != Direction::Right => {
                        game.update_dir(Direction::Left)
                    }
                    KeyCode::Right if game.dir != Direction::Left => {
                        game.update_dir(Direction::Right)
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        game.update()?;
        game.draw()?;

        if game.is_over {
            std::thread::sleep(Duration::from_secs(2));
            break;
        }
    }

    let _ = terminal::disable_raw_mode();
    Ok(())
}
