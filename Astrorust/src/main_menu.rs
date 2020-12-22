use ggez::{Context, audio::{self, SoundSource}, graphics::present};
use ggez::graphics::Color;
use ggez::graphics::Rect;
use ggez::input::keyboard::is_key_pressed;
use ggez::input::keyboard::KeyCode;
use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler},
};
use ggez::{event::KeyMods, graphics};
use crate::function;
pub(crate) struct main_menu{
    welcome : String,
    play : String,
    quit : String,

}

impl main_menu{
   pub fn new()->Self{
        main_menu{
            welcome : "Bienvenu sur Astro Rust".to_string(),
            play : "pressez sur la touche P : PLAY".to_string(),
            quit : "pressez sur la touche echap : QUIT".to_string(),
        }
    }

  pub  fn draw_welcome(ctx: &mut Context){
        let welcome_label= graphics::Text::new(main_menu::new().welcome);
        let size_x = 300.0 - welcome_label.dimensions(ctx).0 as f32 /2.0;
        let size_y = 300.0 - welcome_label.dimensions(ctx).1 as f32 /2.0;
        function::draw_text(ctx, welcome_label, size_x, size_x);
        let play_label= graphics::Text::new(main_menu::new().play);
        function::draw_text(ctx, play_label, size_x , size_x + 50.0);
        let quit_label= graphics::Text::new(main_menu::new().quit);
        function::draw_text(ctx, quit_label, size_x , size_x + 100.0);

        
      
    }
}