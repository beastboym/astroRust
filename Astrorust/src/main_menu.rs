use crate::function;
use ggez::graphics::Color;
use ggez::graphics::Rect;
use ggez::input::keyboard::is_key_pressed;
use ggez::input::keyboard::KeyCode;
use ggez::{
    audio::{self, SoundSource},
    graphics::present,
    Context,
};
use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler},
};
use ggez::{event::KeyMods, graphics};
    /// structure principal de la main menu
pub(crate) struct main_menu {
   pub welcome: String,
   pub  play: String,
   pub quit: String,
}

impl main_menu {
    /// defini une structure par default a main_menu
    pub fn default() -> Self {
        main_menu {
            welcome: "Bienvenu sur Astro Rust".to_string(),
            play: "pressez sur la touche P : PLAY".to_string(),
            quit: "pressez sur la touche echap : QUIT".to_string(),
        }
    }
/// dessine les elements texts de la scene.
    pub(crate) fn draw_welcome(&mut self,ctx: &mut Context) {
        let welcome_label = graphics::Text::new(main_menu::default().welcome);
        let size_x = 300.0 - welcome_label.dimensions(ctx).0 as f32 / 2.0;
        let size_y = 300.0 - welcome_label.dimensions(ctx).1 as f32 / 2.0;
        let play_label = graphics::Text::new(main_menu::default().play);
        let quit_label = graphics::Text::new(main_menu::default().quit);
        function::draw_text(ctx, welcome_label, size_x, size_x);
        function::draw_text(ctx, play_label, size_x, size_x + 50.0);
        function::draw_text(ctx, quit_label, size_x, size_x + 100.0);
    }
}
