use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

use crate::draw::draw_rectangle;
use crate::food::Food;
use crate::snake::{Direction, Snake};

//#TODO: FOOD SHOULD NOT SPAWN INSIDE BORDERS
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 0.90];
const GAME0VER_COLOR: Color = [0.90, 0.00, 0.02, 0.5];
const GAMEPAUSED_COLOR: Color = [0.20, 0.00, 0.90, 0.5];

const MOVING_PERIOD: f64 = 0.5; //
const RESTART_TIME: f64 = 3.0; //TODO:remove this in favor of waiting of a keypress

#[derive(Debug)]
pub struct Game {
    snake: Snake,

    food: Food,

    pub width: i32,
    pub height: i32,

    game_over: bool,
    waiting_time: f64,
    game_paused: bool,
}
impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        let mut rng = thread_rng();

        Game {
            snake: Snake::new(2, 2), //TODO:Randomize snake starts at a fixed pos
            waiting_time: 0.0,
            width,
            height,
            food: Food::new(rng.gen_range(1..=width - 1), 4), //TODO:Randomize food also spawns the first time a fixed place
            game_over: false,                                 // game over
            game_paused: false,                               // game paused
        }
    }

    /// game draw function
    /// ------------------
    /// This the main function for drawing all the components in the gamed
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // draw snake
        self.snake.draw(con, g);
        // draw food
        self.food.draw(con, g);

        // draw borders
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        // game over screen
        // TODO: Add text large text in the middle saying 'game over'
        //TODO: and small ones below saying 'press anykey to restart'
        if self.game_over {
            draw_rectangle(GAME0VER_COLOR, 0, 0, self.width, self.height, con, g)
        }

        // paused screen
        // TODO: Add text large text in the middle saying 'game paused'
        //TODO: and small ones below saying 'press space to continue'
        if self.game_paused {
            draw_rectangle(GAMEPAUSED_COLOR, 0, 0, self.width, self.height, con, g)
        }
    }

    ///listens for key press on computer keyboard
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }
        // match keys to their use cases
        // i can potential add keys for othe function other than direction or WASD as alternative key for directions
        // for now anyway it is to much direction only :>)
        let dir = match key {
            Key::Up => Some(Direction::UP),
            Key::Down => Some(Direction::DOWN),
            Key::Right => Some(Direction::RIGHT),
            Key::Left => Some(Direction::LEFT),
            _ => None, //TODO: implement restart state on pressing any key
        };

        //TODO: add valid pasue functionality
        // This is check if space was pressed and pauses the game
        // let isPaused = match key {
        //     Key::Space => Some(!self.game_paused),
        //     _ => None,
        // };

        // self.game_paused = isPaused.unwrap();
        // check if the user is trying to move in the opposite direction in that case do nothing
        if dir.unwrap() == self.snake.head_dir().opposite() {
            return;
        }
        self.update_snake(dir);
    }

    /// Checks if the snake is still alive
    /// first check if the snake as "eaten itself" overlapped and if it as crossed
    /// Takes ref to self and dir
    pub fn is_snake_alive(&mut self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        // checks if the snake as hit the border
        //TODO: if i add levels this should be in the second level, the first level will have no  border
        next_x > 0 && next_y > 0 && next_x < &self.width - 1 && next_y < &self.height - 1
    }
    ///This function pause the game temp
    /// takes self returns a snapshot of the game states
    fn pause(&mut self) {}

    ///This function play the game
    /// takes the snapshot value of the game and resumes the game
    fn play(&mut self) {}

    /// This function restart the game
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food = Food::new(6, 4);
        self.game_over = false;
        self.game_paused = false;
    }

    /// Game update function
    /// --------------------
    /// runs all update for the game
    /// Takes a deltatime and mutable self
    ///
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        //restart after the waiting time passes
        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }
        // pause on game_paused
        if self.game_paused {
            self.pause();
        }

        if !self.game_paused {
            self.play();
        }
        if !self.food.is_exist() {
            self.spawn_food();
        }

        // update snake
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None)
        }
    }
    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.is_snake_alive(dir) {
            self.snake.update(dir, &mut self.food)
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn spawn_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width);
        let mut new_y = rng.gen_range(1..self.height);

        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width) - 1;
            new_y = rng.gen_range(1..self.height) - 1;
        }

        self.food.set_x(new_x);
        self.food.set_y(new_y);
        self.food.set_is_exist(true);
    }
}
