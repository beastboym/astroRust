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

struct MainState {
   
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
    
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
       
       
        present(ctx).unwrap();
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
