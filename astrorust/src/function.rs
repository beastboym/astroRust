use ggez::graphics::Rect;

use ggez::{graphics::Text, Context};

use ggez::graphics::*;

/// Draw the text `label` at the given coordinates (`x`; `y`)
pub(crate) fn draw_text(ctx: &mut Context, label: Text, x: f32, y: f32) {
    let coord = [x, y];
    let params = DrawParam::default().dest(coord);
    draw(ctx, &label, params).unwrap();
}

/// Define the Rect type elements and draw an image at their coordinates, the image being given through `path`
pub(crate) fn draw_e(elem: Rect, ctx: &mut Context, path: &str) {
    let _ship_draw =
        Mesh::new_rectangle(ctx, DrawMode::fill(), elem, Color::new(1.0, 1.0, 1.0, 1.0)).unwrap();
    let texture: Image = Image::new(ctx, path).unwrap();
    draw(ctx, &texture, DrawParam::default().dest(elem.point())).unwrap();
}

/// Draw an image
pub(crate) fn draw_image(path: &str, ctx: &mut Context) {
    let image: Image = Image::new(ctx, path).unwrap();
    match draw(ctx, &image, DrawParam::default()) {
        Ok(ok) => ok,
        Err(_e) => (),
    }
}
