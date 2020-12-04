#![allow(warnings)]
use ggez::{conf::WindowSetup, event::{self, EventHandler}};
use ggez::graphics;
use ggez::graphics::clear;
use ggez::graphics::present;
use ggez::graphics::Color;
use ggez::graphics::Rect;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::input::keyboard::is_key_pressed;
use ggez::input::keyboard::KeyCode;
type Vector = ggez::mint::Vector2<f32>;

const SCREEN_WIDTH: f32 = 600.;
const SCREEN_HEIGHT: f32 = 600.;

const SHIP_DIM: f32 = 50.;

struct MainState {
   Ship: Rect,
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
    
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
        let ship_draw = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.Ship,
            Color::new(1.0, 1.0, 1.0, 1.0)
        ).unwrap();
        graphics::draw(
            ctx,
            &ship_draw,
            graphics::DrawParam::default()
        ).unwrap();
        present(ctx).unwrap();
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("AstroRust", "Daouda, Claire")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH,SCREEN_HEIGHT))
        .window_setup(WindowSetup::default().title("AstrooooRuuuust"))
        .build().unwrap();
    let main_state = &mut MainState {
        Ship: Rect::new(SCREEN_WIDTH / 2. - SHIP_DIM / 2., SCREEN_HEIGHT - SHIP_DIM * 2., SHIP_DIM, SHIP_DIM),
    };
    event::run(ctx, event_loop, main_state)
}