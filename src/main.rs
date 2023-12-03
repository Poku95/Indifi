#![cfg_attr(debug_assertions, allow(warnings))]

use notan::draw::*;
use notan::prelude::*;
use std::collections::HashMap;

mod render_utilities;
mod transform;
mod player;
mod chunk;
use render_utilities::*;
use transform::*;
use player::*;
use chunk::*;

const MOVE_SPEED: f32 = 10.0;

#[derive(AppState)]
struct State {
    font: Font,
    fps: String,
    player: Player,
    mouse_pos: Vec2,
    chunks: Vec<Chunk>,
    chunk_i: usize,
    floor_textures: Vec<Texture>,
    render_size_pow: u8,
    debug: bool,
}

impl State {
    fn get_chunk(&mut self, index: usize) -> Option<&mut Chunk> {
        self.chunks.get_mut(index)
    }
}

#[notan_main]
fn main() {
    let window_config = WindowConfig::new().set_high_dpi(true);
    notan
        ::init_with(setup)
        .add_config(window_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
        .unwrap();
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx.create_font(include_bytes!("assets/Ubuntu-B.ttf")).unwrap();
    let mut floor_textures = Vec::<Texture>::new();
    floor_textures.push(
        gfx.create_texture().from_image(include_bytes!("assets/test0.png")).build().unwrap()
    );
    floor_textures.push(
        gfx.create_texture().from_image(include_bytes!("assets/test1.png")).build().unwrap()
    );
    floor_textures.push(
        gfx.create_texture().from_image(include_bytes!("assets/test2.png")).build().unwrap()
    );
    floor_textures.push(
        gfx.create_texture().from_image(include_bytes!("assets/test3.png")).build().unwrap()
    );
    floor_textures.push(
        gfx.create_texture().from_image(include_bytes!("assets/test4.png")).build().unwrap()
    );
    floor_textures.push(
        gfx.create_texture().from_image(include_bytes!("assets/test5.png")).build().unwrap()
    );
    floor_textures.push(
        gfx.create_texture().from_image(include_bytes!("assets/test6.png")).build().unwrap()
    );

    let mut chunks = Vec::new();
    for x in 0..16_i32 {
        for y in 0..16_i32 {
            let mut chunk = Chunk::new(gfx, x, y);
            chunk.render_texture(gfx, &floor_textures);
            chunks.push(chunk);
        }
    }

    State {
        font,
        fps: "¯\\_(ツ)_/¯".to_string(),
        player: PlayerBuilder::new().color_random().build(),
        mouse_pos: Vec2::new(0.0, 0.0),
        chunks,
        floor_textures,
        render_size_pow: 8,
        chunk_i: 0,
        debug: false,
    }
}

fn update(app: &mut App, state: &mut State) {
    let time = app.date_now();

    if app.keyboard.was_pressed(KeyCode::F11) {
        let full = !app.window().is_fullscreen();
        app.window().set_fullscreen(full);
    }
    if app.keyboard.was_pressed(KeyCode::O) {
        if state.render_size_pow >= 7 {
            state.render_size_pow -= 1;
        }
    }
    if app.keyboard.was_pressed(KeyCode::P) {
        if state.render_size_pow <= 10 {
            state.render_size_pow += 1;
        }
    }
    state.debug = app.keyboard.is_down(KeyCode::L);

    state.mouse_pos = app.mouse.position().into();

    if state.player.update(app) {
        state.chunks.iter_mut().for_each(|c| {
            let (x, y) = Chunk::pos_to_coords(state.player.pos().vec());
            let (cx, cy) = c.coords();
            let lod = (i32::max((x - cx).abs() - 1, (y - cy).abs()) - 1).clamp(0, 7);
            c.set_lod(lod as u32);
        });
    }
    state.fps = format!("{:.0} fps \nupdate time: {:.2}ms", app.timer.fps(), app.date_now() - time);
}

fn angle_between_points(point1: &Vec2, point2: &Vec2) -> f32 {
    let delta_x = point2.x - point1.x;
    let delta_y = point2.y - point1.y;

    delta_y.atan2(delta_x) * 57.2957795
}

use notan::math::*;
fn draw(gfx: &mut Graphics, state: &mut State) {
    for i in 0..64 {
        if state.chunks[state.chunk_i].update(gfx, &state.floor_textures) {
            break;
        }
        state.chunk_i += 1;
        if state.chunk_i >= state.chunks.len() {
            state.chunk_i = 0;
        }
    }

    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.set_projection(Some(world_projection(gfx.size(), state.render_size_pow).0));

    draw.transform().set(
        Mat3::from_translation(Vec2::new(-state.player.pos().x(), -state.player.pos().y()))
    );

    state.player.set_desired_rotation(
        angle_between_points(
            &state.player.pos().vec2(),
            &draw.screen_to_world_position(state.mouse_pos.x, state.mouse_pos.y)
        )
    );

    state.chunks.iter().for_each(|c| c.render(&mut draw, state.debug));

    state.player.render(&mut draw);

    gfx.render(&draw);

    let mut draw_ui = gfx.create_draw();

    draw_ui
        .text(
            &state.font,
            &format!(
                "x: {:.2}\ny: {:.2}\n{}\nResolution: {:?}\nScale: {}",
                state.player.pos().x(),
                state.player.pos().y(),
                &state.fps,
                gfx.size(),
                state.render_size_pow
            )
        )
        .position(10.0, 10.0)
        .size(14.0);

    gfx.render(&draw_ui);
}
