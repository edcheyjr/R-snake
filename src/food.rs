use piston_window::{types::Color, Context, G2d};

use crate::draw::draw_block;

const FOOD_COLOR: Color = [0.90, 0.00, 0.02, 1.00];

#[derive(Debug, PartialEq)]
pub struct Food {
    food_exist: bool,
    x: i32,
    y: i32,
}

impl Food {
    pub fn new(x: i32, y: i32) -> Food {
        Food {
            food_exist: true,
            x,
            y,
        }
    }

    /// food draw function
    /// Take con:Context and mutable g: &mut G2d 2D Graphic
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        if self.food_exist {
            draw_block(FOOD_COLOR, self.x, self.y, con, g);
        }
    }

    // getters
    ///get if it exist
    pub fn is_exist(&self) -> bool {
        self.food_exist
    }

    ///gets X value of food
    pub fn get_x(&self) -> i32 {
        self.x
    }

    ///gets Y value of food
    pub fn get_y(&self) -> i32 {
        self.y
    }

    // setters
    ///sets X value of food
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    ///sets Y value of food
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    /// set food exist
    pub fn set_is_exist(&mut self, food_exist: bool) {
        self.food_exist = food_exist;
    }
}
