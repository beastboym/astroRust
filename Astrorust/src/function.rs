use ggez::graphics::Color;
use ggez::graphics::Rect;
use ggez::input::keyboard::is_key_pressed;
use ggez::input::keyboard::KeyCode;
use ggez::{
    audio::{self, SoundSource},
    graphics::{present, Text},
    Context,
};
use ggez::{
    conf::WindowSetup,
    event::{self, EventHandler},
};
use ggez::{event::KeyMods, graphics};

/// Dessine le texte voulu a la position passer en parametre
pub(crate) fn draw_text(ctx: &mut Context, label: Text, x: f32, y: f32) {
    let coord = [x, y];
    let params = graphics::DrawParam::default().dest(coord);
    graphics::draw(ctx, &label, params).unwrap();
}

/// Dessine des elements de type Rect et leurs attribue la texture passer en argument par le biais de path(&str)
pub(crate) fn draw_e(elem: Rect, ctx: &mut Context, path: &str) {
    let ship_draw = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        elem,
        Color::new(1.0, 1.0, 1.0, 1.0),
    )
    .unwrap();
    let texture: graphics::Image = graphics::Image::new(ctx, path).unwrap();
    graphics::draw(
        ctx,
        &texture,
        graphics::DrawParam::default().dest(elem.point()),
    )
    .unwrap();
}

/// Efface tout le contenue d'un vecteur
pub(crate) fn erase_vec<T>(vec: &mut Vec<T>) {
    for i in 0..vec.len() {
        vec.pop();
    }
}
