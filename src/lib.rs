
//use std::{fs::DirBuilder, arch::x86_64::_MM_FROUND_CUR_DIRECTION};

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

pub struct SnakeCell(usize);
struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake{
    fn new(spawn_index: usize, size: usize) -> Snake{

        let mut body = vec!();

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake { 
            body,
            direction: Direction::Down,
        
        }
    } 

}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
            World {
            width,
            size: width * width,
            snake: Snake::new(snake_idx, 3)
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self,  direction: Direction) {
        self.snake.direction = direction;

    }
    
    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn snake_cells(&self)-> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn oopsie(&mut self) {
        self.snake.body = vec![SnakeCell(2028)]
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let (row, col) = self.index_to_cell(snake_idx);
        let (row, col) = match self.snake.direction {
            Direction::Right => {
                (row, (col + 1) % self.width)    
            },
            Direction::Left => {
                (row, (col - 1) % self.width)    
            },
            Direction::Up => {
                ((row - 1) % self.width, col)    
            },
            Direction::Down => {
                ((row + 1) % self.width, col)    
            },
        };

        let next_idx = self.cell_to_index(row, col);
        self.set_snake_head(next_idx);
    }

    fn gen_next_snake_cell(&self) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;  
      
        
        return match self.snake.direction {
            Direction::Right => {
                SnakeCell(row * self.width + (snake_idx + 1) % self.width)

            },
            Direction::Left => {
                SnakeCell(row * self.width + (snake_idx + 1) % self.width)   
            },
            Direction::Up => {
                ((row - 1) % self.width, col)    
            },
            Direction::Down => {
                ((row + 1) % self.width, col)    
            },
        };
    }

    fn set_snake_head(&mut self, idx: usize){
        self.snake.body[0].0 = idx;
    }

    fn index_to_cell(&self, idx:usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }
}

// wasm-pack build --target web