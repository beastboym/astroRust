#![allow(warnings)]
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
use ggez::{graphics::clear, mint::Point2, nalgebra::Point};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
type Vector = ggez::mint::Vector2<f32>;
const DESIRED_FPS: u32 = 60;
const SCREEN_WIDTH: f32 = 600.;
const SCREEN_HEIGHT: f32 = 600.;

const SHIP_DIM: f32 = 25.;
const SPEED: f32 = 8.0;

const SHOTS: f32 = 3.;
struct FireShot {
    Ball: Rect,
    life: bool,
}
struct Meteor {
    rock: Rect,
    life: bool,
}
struct MainState {
    ship: Rect,
    fire: Vec<FireShot>,
    meteor: Vec<Meteor>,
    nb_rocks: u32,
    score: u32,
    level: u32,
    carole: bool,
    speed : f32,
}

impl Meteor {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let rando: f32 = rng.gen_range(0.0, SCREEN_WIDTH);
        Meteor {
            rock: Rect::new(rando, -SHIP_DIM, SHIP_DIM, SHIP_DIM),
            life: true,
        }
    }
}
fn draw_e(elem: Rect, ctx: &mut Context){
    let ship_draw = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        elem,
        Color::new(1.0, 1.0, 1.0, 1.0),
        
    )
    .unwrap();
    graphics::draw(ctx, &ship_draw, graphics::DrawParam::default()).unwrap();
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
            nb_rocks: 3,
            score: 0,
            level: 1,
            carole: false,
            speed : SHOTS,
        }
    }

    fn new_shot(&mut self, x: f32, y: f32) {
        let pew = FireShot {
            Ball: Rect::new(x, y, 10.0, 10.0),
            life: true,
        };
        self.fire.push(pew)
    }

    fn clear_dead_elem(&mut self) {
        self.fire.retain(|s| s.life == true);
        self.meteor.retain(|s| s.life == true);
    }

    fn destroy(&mut self) {
        for shot in self.fire.iter_mut() {
            for rock in self.meteor.iter_mut() {
                if shot.Ball.overlaps(&rock.rock) {
                    println!("destroy");
                    shot.life = false;
                    rock.life = false;
                    self.score += 1;
                }
            }
        }
    }
    fn game_over(&self, ctx: &mut Context) {
        if self.carole == true {
            event::quit(ctx);
            println!("dead");
        }
    }
    fn ship_event(&mut self, ctx: &Context) {
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
    fn create_meteor(&mut self) {
        if self.meteor.len() < self.nb_rocks as usize {
            let met = Meteor::new();
            self.meteor.push(met);
        }
    }
   
    fn draw_elem(&mut self, ctx: &mut Context) {
        draw_e(self.ship, ctx);
        for elem in self.fire.iter_mut() {
            draw_e(elem.Ball,ctx);
        }
        for elem in self.meteor.iter_mut() {
            draw_e(elem.rock,ctx);
            
        }

    }
    fn level_up(&mut self){
        if self.score == self.nb_rocks{
            self.nb_rocks += 1;
            self.level +=1;
            self.speed += 0.5;
        }
    }
}

impl FireShot {
    fn new() -> Self {
        FireShot {
            Ball: Rect::new(0.0, 0.0, 10.0, 10.0),
            life: true,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ggez::timer::check_update_time(ctx, DESIRED_FPS)
        {
        self.create_meteor();
        self.ship_event(ctx);
        for elem in self.fire.iter_mut() {
            if elem.Ball.y > 0.0 {
                elem.Ball.y -= SHOTS;

            } else if elem.Ball.y <= 0.0 {
                elem.life = false;
            }
        }
        for rock in self.meteor.iter_mut() {
            if rock.rock.y < SCREEN_HEIGHT {
                rock.rock.y += self.speed;
            }
            else if rock.rock.y >= SCREEN_HEIGHT {
                rock.life = false;
            }
            if rock.rock.overlaps(&self.ship) {
                self.carole = true;
            }
            println!("FPS: {:?}", ggez::timer::fps(ctx));
        }
    }
        self.destroy();
        self.clear_dead_elem();
        self.level_up();
        self.game_over(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
        
        self.draw_elem(ctx);

        let score = graphics::Text::new(format!("Score : {}", self.score));
        let coord = [0.0 + score.width(ctx) as f32, 20.0];
        let params = graphics::DrawParam::default().dest(coord);
        graphics::draw(ctx, &score, params)?;

        let level = graphics::Text::new(format!("Level : {}", self.level));
        let lvl_coord = [0.0 + score.width(ctx) as f32, 40.0];
        let params = graphics::DrawParam::default().dest(lvl_coord);
        graphics::draw(ctx, &level, params)?;
        present(ctx)?;
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
            KeyCode::Space => {
                self.new_shot(self.ship.x, self.ship.y);
                println!("{}", self.fire.len());
            }
            KeyCode::Escape => event::quit(ctx),
            _ => (), // Do nothing
        }
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("AstroRust", "Daouda, Claire")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .window_setup(WindowSetup::default().title("AstrooooRuuuust"))
        .build()?;
        
    let main_state = &mut MainState::new();
    event::run(ctx, event_loop, main_state)
}
