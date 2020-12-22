use ggez::{Context, audio::{self, SoundSource}, graphics::{Text, present}};
use ggez::graphics::Color;
use ggez::graphics::Rect;
use ggez::input::keyboard::is_key_pressed;
use ggez::input::keyboard::KeyCode;
use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler},
};
use ggez::{event::KeyMods, graphics};



pub(crate)  fn draw_text(ctx: &mut Context, label : Text,x : f32,y : f32){
    let coord = [x, y];
    let params = graphics::DrawParam::default().dest(coord);
    graphics::draw(ctx, &label, params).unwrap();
}