#![allow(warnings)]
use ggez::{audio::{self, SoundSource}, graphics::present};
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
const SHOT_DIMX: f32 = 20.;
const SHOT_DIMY: f32 = 40.;
const METE_DIM: f32 = 50.;

const SHOTS: f32 = 3.;
mod game;
mod main_menu;
mod function;
struct MainState{
    game_scene : game::GameScene,
    main_menu : main_menu::main_menu,
    switch_scene : bool,
}

impl MainState{
    fn new(ctx : &mut Context)->Self{
        MainState{
            game_scene : game::GameScene::new(ctx),
            main_menu : main_menu::main_menu::new(),
            switch_scene : false,
        }
    }
}
impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.game_scene.sound.bg_loop.playing() == false{
            let _ = self.game_scene.sound.bg_loop.play();
        }
        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            self.game_scene.create_meteor();
            self.game_scene.ship_event(ctx);
            for elem in self.game_scene.fire.iter_mut() {
                if elem.Ball.y > 0.0 {
                    elem.Ball.y -= SHOTS;
                } else if elem.Ball.y <= 0.0 {
                    elem.life = false;
                }
            }
            for rock in self.game_scene.meteor.iter_mut() {
                if rock.rock.y < SCREEN_HEIGHT {
                    rock.rock.y += self.game_scene.speed;
                } else if rock.rock.y >= SCREEN_HEIGHT {
                    rock.life = false;
                }
                if rock.rock.overlaps(&self.game_scene.ship) {
                    self.game_scene.carole = true;
                }
                // println!("FPS: {:?}", ggez::timer::fps(ctx));
            }
        }
        self.game_scene.destroy();
        self.game_scene.clear_dead_elem();
        self.game_scene.level_up();
        self.game_scene.game_over(&mut self.switch_scene,ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
        graphics::draw(
            ctx,
            &self.game_scene.background,
            graphics::DrawParam::default(),
        )
        .unwrap();
        if self.switch_scene == true{
        self.game_scene.draw_elem(ctx);

        let score = graphics::Text::new(format!("Score : {}", self.game_scene.score));
        let coord = [0.0 + score.width(ctx) as f32, 20.0];
        let params = graphics::DrawParam::default().dest(coord);
        graphics::draw(ctx, &score, params)?;

        let level = graphics::Text::new(format!("Level : {}", self.game_scene.level));
        let lvl_coord = [0.0 + score.width(ctx) as f32, 40.0];
        let params = graphics::DrawParam::default().dest(lvl_coord);
        graphics::draw(ctx, &level, params)?;}
        else if self.switch_scene == false{
            main_menu::main_menu::draw_welcome(ctx);
        }
        present(ctx)?;
        Ok(())
    }
    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Space => {
                self.game_scene.new_shot(self.game_scene.ship.x, self.game_scene.ship.y);
                println!("{}", self.game_scene.fire.len());
            }
            KeyCode::P => {
                self.switch_scene = true;
                println!("P {}",self.switch_scene);
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
        .add_resource_path("./src")
        .build()?;
    let main_state = &mut MainState::new(ctx);
    event::run(ctx, event_loop, main_state)
}
