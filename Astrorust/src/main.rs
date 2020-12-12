#![allow(warnings)]
use ggez::graphics;
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
use ggez::{Context, ContextBuilder, GameResult};
type Vector = ggez::mint::Vector2<f32>;

const SCREEN_WIDTH: f32 = 600.;
const SCREEN_HEIGHT: f32 = 600.;

const SHIP_DIM: f32 = 25.;
const SPEED: f32 = 8.0;

struct FireShot {
    Ball: Rect,
    lifetime: f32,
}
struct MainState {
    ship: Rect,
    fire: Vec<FireShot>,
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
        }
    }

    fn new_shot(&mut self,x :f32, y : f32){
        let pew = FireShot {
            Ball: Rect::new(x,y,10.0,10.0,),
            lifetime: 1.0,
        };
        self.fire.push(pew)

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

        if is_key_pressed(ctx, KeyCode::Space) {
            self.new_shot(self.ship.x,self.ship.y);
            println!("{}",self.fire.len());
        }
    }

    fn draw_elem(&mut self,ctx: &mut Context){
        for elem in self.fire.iter_mut(){
            let ship_draw = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                elem.Ball,
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
            lifetime: 0.0,
        }
    }
}
impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.ship_event(ctx);
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
