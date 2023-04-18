use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;
use crate::food::Food;

const SNAKE_COLOR: Color = [0.00, 0.90, 0.5, 1.00]; // [r,g,b,a]

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

/// Snake struct
/// takes:
/// {direction, body, tail}
///
#[derive(Debug)]
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    /// Defines the default snake when game starts
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x: x, y });
        // Default snake struct
        Snake {
            direction: Direction::RIGHT,
            body,
            tail: None,
        }
    }

    /// check if the food(apple) as being eaten by snake
    fn check_eating(&mut self, food: &mut Food) {
        let (head_x, head_y): (i32, i32) = self.head_pos();
        if food.is_exist() && food.get_x() == head_x && food.get_y() == head_y {
            food.set_is_exist(false);
            self.restore_tail();
        }
    }

    /// snake draw function
    /// Take con:Context and mutable g: &mut G2d 2D Graphic
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    /// find head position
    /// takes reference to self
    /// returns tuple:(i32,i32)
    ///
    pub fn head_pos(&self) -> (i32, i32) {
        let head_block = &self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    ///finds the current direction of the snake;
    /// take reference of snake;
    /// returns direction
    pub fn head_dir(&self) -> Direction {
        self.direction
    }

    /// find the next head
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_pos();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }
        match moving_dir {
            Direction::UP => (head_x, head_y - 1),
            Direction::DOWN => (head_x, head_y + 1),
            Direction::RIGHT => (head_x + 1, head_y),
            Direction::LEFT => (head_x - 1, head_y),
        }
    }

    /// private function for moving the snake forward
    fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }
        let (last_x, last_y): (i32, i32) = self.head_pos();

        // moving the snake "head" which is the first block in the linked list by changing it's y or x position
        //this achieved by adding new block to the front for the linked list the and making that the new head
        let new_block = match self.direction {
            Direction::UP => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::DOWN => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::RIGHT => Block {
                x: last_x + 1,
                y: last_y,
            },
            Direction::LEFT => Block {
                x: last_x - 1,
                y: last_y,
            },
        };
        self.body.push_front(new_block);
        //and removing the last block of the linked list to maintain the lenght of the snake
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    /// check if any snake overlap any part of it's body
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            // prevent ambuity when snake move in circle the block occupied by tail should vacant as the head occupies that block
            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }

    /// snake update function
    pub fn update(&mut self, dir: Option<Direction>, food: &mut Food) {
        self.move_forward(dir);
        self.check_eating(food);
    }

    /// restores tail block
    /// this will be helpful when we the snakes eat an apple we just restore the last deleted block which is stored in tail
    fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }
}
