#![allow(warnings)]
use ggez::graphics::clear;
use ggez::graphics::present;
use ggez::graphics::Color;
use ggez::graphics::Rect;
use ggez::input::keyboard::is_key_pressed;
use ggez::input::keyboard::KeyCode;
use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler},
};
use ggez::{event::KeyMods, graphics};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
type Vector = ggez::mint::Vector2<f32>;

const SCREEN_WIDTH: f32 = 600.;
const SCREEN_HEIGHT: f32 = 600.;

const SHIP_DIM: f32 = 25.;
const SPEED: f32 = 8.0;

const SHOTS: f32 = 5.;
struct FireShot {
    Ball: Rect,
}
struct Meteor {
    rock: Rect,
    lifetime: bool,
}
struct MainState {
    ship: Rect,
    fire: Vec<FireShot>,
    meteor: Vec<Meteor>,
}

impl Meteor {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let rando: f32 = rng.gen_range(0.0, SCREEN_WIDTH);
        Meteor {
            rock: Rect::new(rando, -SHIP_DIM, SHIP_DIM, SHIP_DIM),
            lifetime: true,
        }
    }
}
impl MainState {
    fn new() -> Self {
        MainState {
            ship: Rect::new(
                SCREEN_WIDTH / 2. - SHIP_DIM / 2.0,
                SCREEN_HEIGHT - SHIP_DIM * 2.0,
                SHIP_DIM,
                SHIP_DIM,
            ),
            fire: Vec::new(),
            meteor: Vec::new(),
        }
    }

    fn new_shot(&mut self, x: f32, y: f32) {
        let pew = FireShot {
            Ball: Rect::new(x, y, 10.0, 10.0),
        };
        self.fire.push(pew)
        
    }

    // fn ship_event(&mut self, ctx: &Context) {
    //     if is_key_pressed(ctx, KeyCode::Right) {
    //         if self.ship.right() < SCREEN_WIDTH {
    //             self.ship.x += SPEED;
    //         }
    //     }
    //     if is_key_pressed(ctx, KeyCode::Left) {
    //         if self.ship.left() > 0.0 {
    //             self.ship.x -= SPEED;
    //         }
    //     }

    //     if is_key_pressed(ctx, KeyCode::Space) {
    //         self.new_shot(self.ship.x, self.ship.y);
    //         println!("{}", self.fire.len());
    //     }
    // }

    fn draw_elem(&mut self, ctx: &mut Context) {
        for elem in self.fire.iter_mut() {
            let ship_draw = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                elem.Ball,
                Color::new(1.0, 1.0, 1.0, 1.0),
            )
            .unwrap();
            graphics::draw(ctx, &ship_draw, graphics::DrawParam::default()).unwrap();
        }
        for elem in self.meteor.iter_mut() {
            let ship_draw = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                elem.rock,
                Color::new(1.0, 1.0, 1.0, 1.0),
            )
            .unwrap();
            graphics::draw(ctx, &ship_draw, graphics::DrawParam::default()).unwrap();
        }
    }
}

impl FireShot {
    fn new() -> Self {
        FireShot {
            Ball: Rect::new(0.0, 0.0, 10.0, 10.0),
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut rng = rand::thread_rng();
        let meteor: f32 = rng.gen_range(0., 100.0);
        if meteor < 2.0 {
            self.meteor.push(Meteor::new());
        };
        // self.ship_event(ctx);
        let mut  i : usize = 0;
        for elem in self.fire.iter_mut() {
            i += 1;
            if elem.Ball.y > -SHIP_DIM{
                elem.Ball.y -= SHOTS;
            }
           
        }
        for elem in self.meteor.iter_mut() {
            if elem.rock.y < SCREEN_HEIGHT {
                elem.rock.y += SHOTS;
            }
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
        let ship_draw = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.ship,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )
        .unwrap();
        graphics::draw(ctx, &ship_draw, graphics::DrawParam::default()).unwrap();
        self.draw_elem(ctx);
        present(ctx).unwrap();
        Ok(())
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Left => {
                if self.ship.left() > 0.0 {
                    self.ship.x -= SPEED;
                }
            }
            KeyCode::Right => {
                if self.ship.right() < SCREEN_WIDTH {
                    self.ship.x += SPEED;
                }
            }
            KeyCode::Space => {
                self.new_shot(self.ship.x, self.ship.y);
                println!("{}", self.fire.len());
            }
            KeyCode::Escape => event::quit(ctx),
            _ => (), // Do nothing
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Left | KeyCode::Right => {
                self.ship.x -= 0.0;
                self.ship.x += 0.0;
            }
            _ => (), // Do nothing
        }
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("AstroRust", "Daouda, Claire")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .window_setup(WindowSetup::default().title("AstrooooRuuuust"))
        .build()
        .unwrap();
    let main_state = &mut MainState::new();
    event::run(ctx, event_loop, main_state)
}
