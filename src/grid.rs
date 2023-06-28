use std::collections::VecDeque;

use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const BLOCK_SIZE: u32 = 25;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub struct Snake {
    direction: Direction,
    body: VecDeque<(usize, usize)>,
}

impl Snake {
    fn new(head: (usize, usize)) -> Self {
        Self {
            direction: Direction::Right,
            body: VecDeque::from([head]),
        }
    }
    pub fn up(&mut self) {
        if let Direction::Down = self.direction {
        } else {
            self.direction = Direction::Up;
        }
    }
    pub fn down(&mut self) {
        if let Direction::Up = self.direction {
        } else {
            self.direction = Direction::Down;
        }
    }
    pub fn right(&mut self) {
        if let Direction::Left = self.direction {
        } else {
            self.direction = Direction::Right;
        }
    }
    pub fn left(&mut self) {
        if let Direction::Right = self.direction {
        } else {
            self.direction = Direction::Left;
        }
    }
}

#[derive(Clone)]
enum Cell {
    Food(u8, u8, u8),
    Empty,
}
enum GameStatus {
    Running,
    Stoped,
}

pub struct Game {
    grid: Vec<Vec<Cell>>,
    status: GameStatus,
    pub snake: Option<Snake>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![vec![Cell::Empty; width]; height],
            status: GameStatus::Stoped,
            snake: None,
        }
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }
    fn height(&self) -> usize {
        self.grid.len()
    }
    pub fn pause(&mut self) {
        self.status = GameStatus::Stoped
    }
    pub fn resume(&mut self) {
        self.status = GameStatus::Running
    }

    pub fn toggle_game(&mut self) {
        match self.status {
            GameStatus::Running => self.pause(),
            GameStatus::Stoped => self.resume(),
        }
    }

    pub fn generate_food(&mut self) {
        let mut rng = rand::thread_rng();
        let x = (rng.gen::<f32>() * (self.width() - 1) as f32).ceil() as usize;
        let y = (rng.gen::<f32>() * (self.height() - 1) as f32).ceil() as usize;

        let r = (rng.gen::<f32>() * 255.0).ceil() as u8;
        let g = (rng.gen::<f32>() * 255.0).ceil() as u8;
        let b = (rng.gen::<f32>() * 255.0).ceil() as u8;
        self.grid[y][x] = Cell::Food(r, b, g)
    }
    pub fn start(&mut self) {
        match self.status {
            GameStatus::Running => panic!("Game is already running"),
            _ => (),
        }
        let height = self.grid.len();
        let width = self.grid[0].len();
        let center_w = width / 2;
        let center_h = height / 2;
        // self.grid[center_h][center_w] = Cell::Snake;
        self.snake = Some(Snake::new((center_h, center_w)));
        self.generate_food();
        self.status = GameStatus::Running
    }

    pub fn update(&mut self) {
        if let GameStatus::Stoped = self.status {
            return;
        }
        let len = self.grid.len();
        match &mut self.snake {
            Some(snake) => {
                let old_head = snake.body.back().unwrap().clone();
                // let old_head: (usize, usize) = (1, 2);
                match snake.direction {
                    Direction::Up => {
                        let mut yd = 0;
                        if old_head.0 < 1 {
                            yd = len - 1
                        } else {
                            yd = old_head.0 - 1;
                        }
                        snake.body.push_back((yd, old_head.1));
                    }
                    Direction::Down => {
                        snake.body.push_back(((old_head.0 + 1) % 20, old_head.1));
                    }
                    Direction::Left => {
                        let mut yd = 0;
                        if old_head.1 < 1 {
                            yd = len - 1
                        } else {
                            yd = old_head.1 - 1;
                        }
                        snake.body.push_back((old_head.0, yd));
                    }
                    Direction::Right => {
                        snake.body.push_back((old_head.0, (old_head.1 + 1) % 20));
                    }
                }

                if let Cell::Empty = self.grid[old_head.0][old_head.1] {
                    snake.body.pop_front();
                } else {
                    self.grid[old_head.0][old_head.1] = Cell::Empty;
                    self.generate_food();
                }
            }
            None => panic!(),
        }
    }
    pub fn render(&self, start_x: i32, start_y: i32, canvas: &mut Canvas<Window>) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if self.snake.as_ref().unwrap().body.contains(&(y, x)) {
                    canvas.set_draw_color(Color::RGB(100, 0, 0));
                    let rect = Rect::new(
                        (x as u32 * BLOCK_SIZE) as i32 + start_x,
                        (y as u32 * BLOCK_SIZE) as i32 + start_y,
                        BLOCK_SIZE,
                        BLOCK_SIZE,
                    );
                    canvas.fill_rect(rect).unwrap();
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.draw_rect(rect).unwrap();
                } else {
                    match cell {
                        Cell::Food(r, b, g) => {
                            let mut rng = rand::thread_rng();
                            canvas.set_draw_color(Color::RGB(*r, *g, *b));
                            let rect = Rect::new(
                                (x as u32 * BLOCK_SIZE) as i32 + start_x,
                                (y as u32 * BLOCK_SIZE) as i32 + start_y,
                                BLOCK_SIZE,
                                BLOCK_SIZE,
                            );
                            canvas.fill_rect(rect).unwrap();
                        }
                        Cell::Empty => {
                            canvas.set_draw_color(Color::RGB(100, 0, 0));
                            let rect = Rect::new(
                                (x as u32 * BLOCK_SIZE) as i32 + start_x,
                                (y as u32 * BLOCK_SIZE) as i32 + start_y,
                                BLOCK_SIZE,
                                BLOCK_SIZE,
                            );
                            canvas.draw_rect(rect).unwrap();
                        }
                    }
                }
            }
        }
    }
}
