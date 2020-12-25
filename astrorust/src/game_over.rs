use crate::function;
use ggez::Context;

use ggez::graphics;
/// Structure de notre scene game over
pub(crate) struct GameOver {
    game_over: String,
    play: String,
    quit: String,
}

impl GameOver {
    /// valeur par defaut de notre structure GameOver
    pub fn default() -> Self {
        GameOver {
            game_over: format!("Dommage vous avez perdu!"),
            play: "pressez sur la touche P : PLAY".to_string(),
            quit: "pressez sur la touche echap : QUIT".to_string(),
        }
    }
    /// dessine les element de notre scene game over
    pub fn draw_game_over(&mut self, ctx: &mut Context) {
        let game_over_label = graphics::Text::new(GameOver::default().game_over);
        let size_x = 300.0 - game_over_label.dimensions(ctx).0 as f32 / 2.0;
        let _size_y = 300.0 - game_over_label.dimensions(ctx).1 as f32 / 2.0;
        let play_label = graphics::Text::new(GameOver::default().play);
        let quit_label = graphics::Text::new(GameOver::default().quit);
        function::draw_text(ctx, game_over_label, size_x, size_x);
        function::draw_text(ctx, play_label, size_x, size_x + 50.0);
        function::draw_text(ctx, quit_label, size_x, size_x + 100.0);
    }
}
