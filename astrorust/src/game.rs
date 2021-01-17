use ggez::audio::SoundSource;
use ggez::graphics::{Rect, Text};
use ggez::input::keyboard::{is_key_pressed, KeyCode};
use ggez::Context;
use rand::Rng;

use crate::assets::*;
use crate::function;
/// Define the width of the game window
pub const SCREEN_WIDTH: f32 = 600.;
/// Define the height of the game window
pub const SCREEN_HEIGHT: f32 = 600.;

/// Dimensions of the ship's container, do not directly change the ship as it is represented (the image) but rather the rect on which it is drawn
pub const SHIP_DIM: f32 = 25.0;
/// Ship's movement speed
pub const SPEED: f32 = 8.0;
/// Width of the shots container, do not directly change the shots as they are represented (the image) but rather the rect on which they are drawn
pub const SHOT_DIMX: f32 = 20.0;
/// Height of the shots container, do not directly change the shots as they are represented (the image) but rather the rect on which they are drawn
pub const SHOT_DIMY: f32 = 40.0;
/// Dimensions of the meteorites' container, do not directly change the meteorites as they are represented (the image) but rather the rect on which they are drawn
pub const METE_DIM: f32 = 50.0;
/// Shots' movement speed
pub const SHOTS: f32 = 3.0;

/// FireShot and Meteor are visibly the same structures but a separation has been done to facilitate the implementation of differents elements (the shots and meteorites)
pub struct FireShot {
    pub ball: Rect,
    pub life: bool,
}

pub struct Meteor {
    pub rock: Rect,
    pub life: bool,
}

/// Main scene of the game, contain every entity that can be used to make the game run
pub(crate) struct GameScene {
    pub ship: Rect,
    pub fire: Vec<FireShot>,
    pub meteor: Vec<Meteor>,
    pub nb_rocks: u32,
    pub score: u32,
    pub level: u32,
    pub alive: bool,
    pub carole: u32,
    pub speed: f32,
    pub sound: Sound,
    pub images: Images,
}

impl Meteor {
    /// Define a default value for the structure Meteor
    pub fn default() -> Self {
        let mut rng = rand::thread_rng();
        let rando: f32 = rng.gen_range(0.0 + METE_DIM, SCREEN_WIDTH - METE_DIM);
        Meteor {
            rock: Rect::new(rando, -SHIP_DIM, METE_DIM, METE_DIM),
            life: true,
        }
    }
}

impl GameScene {
    /// Define a default value for the structure GameScene
    pub fn default(ctx: &mut Context) -> Self {
        GameScene {
            ship: Rect::new(
                SCREEN_WIDTH / 2. - SHIP_DIM / 2.0,
                SCREEN_HEIGHT - SHIP_DIM * 2.0,
                SHIP_DIM,
                SHIP_DIM,
            ),
            fire: Vec::new(),
            meteor: Vec::new(),
            nb_rocks: 3,
            score: 0,
            level: 1,
            alive: true,
            carole: 0,
            speed: SHOTS,
            sound: Sound::default(ctx),
            images: Images::default(),
        }
    }

    /// Create a new shot
    pub fn new_shot(&mut self, x: f32, y: f32) {
        let pew = FireShot {
            ball: Rect::new(x + SHIP_DIM / 2.0, y, SHOT_DIMX, SHOT_DIMY),
            life: true,
        };
        self.fire.push(pew);
        let _ = self.sound.shot_sound.play();
    }

    /// Erase every entity that goes out of the border of the window
    pub fn clear_dead_elem(&mut self) {
        self.fire.retain(|s| s.life == true);
        self.meteor.retain(|s| s.life == true);
    }

    /// Manage the collisions between meteorites and shots
    pub fn collision(&mut self) {
        for shot in self.fire.iter_mut() {
            for rock in self.meteor.iter_mut() {
                if shot.ball.overlaps(&rock.rock) {
                    shot.life = false;
                    rock.life = false;
                    self.score += 1;
                    self.carole += 1;
                    let _ = self.sound.collision.play();
                }
            }
        }
    }

    /// Define the defeat conditions
    pub fn game_over(&mut self) {
        if self.alive == false {
            let _ = self.sound.game_over.play();
            // event::quit(ctx);
            self.alive = true;
            self.remove_all();
            self.nb_rocks = 3;
            self.level = 1;
            self.score = 0;
            self.carole = 0;
            self.speed = SHOTS;
        }
    }

    /// Define the keys to make the ship move
    pub fn ship_event(&mut self, ctx: &Context) {
        if is_key_pressed(ctx, KeyCode::Right) {
            if self.ship.right() < SCREEN_WIDTH {
                self.ship.x += SPEED;
            }
        }
        if is_key_pressed(ctx, KeyCode::Left) {
            if self.ship.left() > 0.0 {
                self.ship.x -= SPEED;
            }
        }
    }

    /// Define the conditions to create a new meteorite
    pub fn create_meteor(&mut self) {
        if self.meteor.len() < self.nb_rocks as usize {
            let met = Meteor::default();
            self.meteor.push(met);
        }
    }

    /// Draw the scene's elements
    pub fn draw_elem(&mut self, ctx: &mut Context) {
        function::draw_e(self.ship, ctx, &self.images.ship);
        for elem in self.fire.iter_mut() {
            function::draw_e(elem.ball, ctx, &self.images.shot);
        }
        for elem in self.meteor.iter_mut() {
            function::draw_e(elem.rock, ctx, &self.images.meteor);
        }
        let score = Text::new(format!("Score : {}", self.score));
        let level = Text::new(format!("Level : {}", self.level));
        let coord = [0.0 + score.width(ctx) as f32, 20.0];
        let lvl_coord = [0.0 + score.width(ctx) as f32, 40.0];
        function::draw_text(ctx, score, coord[0], coord[1]);
        function::draw_text(ctx, level, lvl_coord[0], lvl_coord[1]);
    }

    /// Define the condition to level up and the new difficulties for this new level
    pub fn level_up(&mut self) {
        if self.carole == self.nb_rocks {
            self.nb_rocks += 1;
            self.level += 1;
            self.speed += 0.5;
            self.carole = 0;
            // self.remove_all();
        }
    }

    /// Erase all of the game's entities and reclaim memory.
    pub fn remove_all(&mut self) {
        // TODO: If shrink_to is stabilized use it
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.shrink_to
        // Clear vectors
        self.fire.clear();
        self.meteor.clear();
        // Reclaim memory
        self.meteor.shrink_to_fit();
        self.fire.shrink_to_fit();
    }
}
