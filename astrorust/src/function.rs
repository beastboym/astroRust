use ggez::graphics::Color;
use ggez::graphics::Rect;

use ggez::{graphics::Text, Context};

use ggez::graphics;

/// Dessine le texte voulu a la position passer en parametre
pub(crate) fn draw_text(ctx: &mut Context, label: Text, x: f32, y: f32) {
    let coord = [x, y];
    let params = graphics::DrawParam::default().dest(coord);
    graphics::draw(ctx, &label, params).unwrap();
}

/// Dessine des elements de type Rect et leurs attribue la texture passer en argument par le biais de path(&str)
pub(crate) fn draw_e(elem: Rect, ctx: &mut Context, path: &str) {
    let _ship_draw = graphics::Mesh::new_rectangle(
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
    for _i in 0..vec.len() {
        vec.pop();
    }
}
/// dessine une image
pub(crate) fn draw_image(path: &str, ctx: &mut Context) {
    let image: graphics::Image = graphics::Image::new(ctx, path).unwrap();
    match graphics::draw(ctx, &image, graphics::DrawParam::default()) {
        Ok(ok) => ok,
        Err(_e) => (),
    }
}
