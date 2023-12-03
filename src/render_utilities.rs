use notan::math::{ Mat4, Vec2 };
use notan::prelude::*;
use notan::draw::*;

pub fn render_bg(gfx: &mut Graphics, texture: &Texture) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    let (projection, screen_aspect_ratio) = bg_projection(gfx.size(), texture.size().0);
    let image_aspect_ratio = texture.size().0 / texture.size().1;
    draw.set_projection(Some(projection));

    let scale_factor = if screen_aspect_ratio > image_aspect_ratio {
        texture.size().1 / (gfx.size().1 as f32)
    } else {
        texture.size().0 / (gfx.size().0 as f32)
    };
    draw.image(texture)
        .position(-texture.size().0 / 2.0, -texture.size().1 / 2.0)
        .scale(scale_factor, scale_factor);
    gfx.render(&draw);
}

pub fn bg_projection(win_size: (u32, u32), img_bigger_side: f32) -> (Mat4, f32) {
    let win_size = Vec2::new(win_size.0 as f32, win_size.1 as f32);
    let work_size = Vec2::new(img_bigger_side, img_bigger_side);
    let aspect_ratio = win_size.x / win_size.y;
    let half_zoomed_work_size = work_size / 2.0;
    let left = -half_zoomed_work_size.x;
    let right = half_zoomed_work_size.x;
    let bottom = half_zoomed_work_size.y / aspect_ratio;
    let top = -half_zoomed_work_size.y / aspect_ratio;
    let near = -1.0;
    let far = 1.0;

    (Mat4::orthographic_rh_gl(left, right, bottom, top, near, far), aspect_ratio)
}

pub fn world_projection(win_size: (u32, u32), render_size: u8) -> (Mat4, f32) {
    let win_size = Vec2::new(win_size.0 as f32, win_size.1 as f32);
    let work_size = Vec2::new((2.0_f32).powi(render_size as i32), (2.0_f32).powi(render_size as i32));
    let aspect_ratio = win_size.x / win_size.y;
    let half_zoomed_work_size = work_size / 2.0;
    let left = -half_zoomed_work_size.x;
    let right = half_zoomed_work_size.x;
    let bottom = half_zoomed_work_size.y / aspect_ratio;
    let top = -half_zoomed_work_size.y / aspect_ratio;
    let near = -1.0;
    let far = 1.0;

    (Mat4::orthographic_rh_gl(left, right, bottom, top, near, far), aspect_ratio)
}
