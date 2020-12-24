#![allow(warnings)]
use ggez::graphics::Color;
use ggez::graphics::Rect;
use ggez::input::keyboard::is_key_pressed;
use ggez::input::keyboard::KeyCode;
use ggez::{
    audio::{self, SoundSource},
    graphics::present,
};
use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler},
};
use ggez::{event::KeyMods, graphics};
use ggez::{graphics::clear, mint::Point2, nalgebra::Point};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
type Vector = ggez::mint::Vector2<f32>;
use crate::function;

const DESIRED_FPS: u32 = 60;
const SCREEN_WIDTH: f32 = 600.;
const SCREEN_HEIGHT: f32 = 600.;

const SHIP_DIM: f32 = 25.;
const SPEED: f32 = 8.0;
const SHOT_DIMX: f32 = 20.;
const SHOT_DIMY: f32 = 40.;
const METE_DIM: f32 = 50.;

const SHOTS: f32 = 3.;
pub struct FireShot {
    pub Ball: Rect,
    pub life: bool,
}
pub struct Meteor {
    pub rock: Rect,
    pub life: bool,
}
pub struct Sound {
    pub shot_sound: audio::Source,
    pub collision: audio::Source,
    pub game_over: audio::Source,
    pub bg_loop: audio::Source,
}
impl Sound {
    pub fn default(ctx: &mut Context) -> Self {
        Sound {
            shot_sound: audio::Source::new(ctx, "/pew.wav").unwrap(),
            collision: audio::Source::new(ctx, "/collision.wav").unwrap(),
            game_over: audio::Source::new(ctx, "/game_over.wav").unwrap(),
            bg_loop: audio::Source::new(ctx, "/bg_loop.mp3").unwrap(),
        }
    }
}

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
    pub background: graphics::Image,
}

impl Meteor {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let rando: f32 = rng.gen_range(0.0 + METE_DIM, SCREEN_WIDTH - METE_DIM);
        Meteor {
            rock: Rect::new(rando, -SHIP_DIM, METE_DIM, METE_DIM),
            life: true,
        }
    }
}

impl GameScene {
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
            background: graphics::Image::new(ctx, "/bryan-goff1.jpg").unwrap(),
        }
    }

    pub fn new_shot(&mut self, x: f32, y: f32) {
        let pew = FireShot {
            Ball: Rect::new(x + SHIP_DIM / 2.0, y, SHOT_DIMX, SHOT_DIMY),
            life: true,
        };
        self.fire.push(pew);
        let _ = self.sound.shot_sound.play();
    }

    pub fn clear_dead_elem(&mut self) {
        self.fire.retain(|s| s.life == true);
        self.meteor.retain(|s| s.life == true);
    }

    pub fn destroy(&mut self) {
        for shot in self.fire.iter_mut() {
            for rock in self.meteor.iter_mut() {
                if shot.Ball.overlaps(&rock.rock) {
                    println!("destroy");
                    shot.life = false;
                    rock.life = false;
                    self.score += 1;
                    self.carole += 1;
                    let _ = self.sound.collision.play();
                }
            }
        }
    }
    pub fn game_over(&mut self) {
        if self.alive == false {
            let _ = self.sound.game_over.play();
            // event::quit(ctx);
            self.alive = true;
            self.remove_all();
            self.nb_rocks = 3;
            self.level = 1;
            self.score = 0;
            self.speed = SHOTS;     
        }
    }
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
    pub fn create_meteor(&mut self) {
        if self.meteor.len() < self.nb_rocks as usize {
            let met = Meteor::new();
            self.meteor.push(met);
        }
    }
    pub fn draw_elem(&mut self, ctx: &mut Context) {
        function::draw_e(self.ship, ctx, "/ship.png");
        for elem in self.fire.iter_mut() {
            function::draw_e(elem.Ball, ctx, "/laser_shot1.png");
        }
        for elem in self.meteor.iter_mut() {
            function::draw_e(elem.rock, ctx, "/meteor.png");
        }
        let score = graphics::Text::new(format!("Score : {}", self.score));
        let level = graphics::Text::new(format!("Level : {}", self.level));
        let coord = [0.0 + score.width(ctx) as f32, 20.0];
        let lvl_coord = [0.0 + score.width(ctx) as f32, 40.0];
        function::draw_text(ctx, score, coord[0], coord[1]);
        function::draw_text(ctx, level, lvl_coord[0], lvl_coord[1]);
    }

    pub fn level_up(&mut self) {
        if self.carole == self.nb_rocks{
            self.nb_rocks += 1;
            self.level += 1;
            self.speed += 0.5;
            self.carole = 0;        
        }
    }
    pub fn remove_all(&mut self){
         function::erase_vec(&mut self.fire);
         function::erase_vec(&mut self.meteor);
    }
}
