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
pub(crate) struct GameOver {
    game_over: String,
    play: String,
    quit: String,
}

impl GameOver {
    pub fn default() -> Self {
        GameOver {
            game_over: format!("Dommage vous avez perdu aux niveaux"),
            play: "pressez sur la touche P : PLAY".to_string(),
            quit: "pressez sur la touche echap : QUIT".to_string(),
        }
    }

    pub fn draw_game_over(ctx: &mut Context, level: u32, score: u32) {
        let txt = format!(
            "{} {} avec {} points",
            GameOver::default().game_over,
            level,
            score
        );
        let game_over_label = graphics::Text::new(txt);
        let size_x = 300.0 - game_over_label.dimensions(ctx).0 as f32 / 2.0;
        let size_y = 300.0 - game_over_label.dimensions(ctx).1 as f32 / 2.0;
        let play_label = graphics::Text::new(GameOver::default().play);
        let quit_label = graphics::Text::new(GameOver::default().quit);
        function::draw_text(ctx, game_over_label, size_x, size_x);
        function::draw_text(ctx, play_label, size_x, size_x + 50.0);
        function::draw_text(ctx, quit_label, size_x, size_x + 100.0);
    }
}
