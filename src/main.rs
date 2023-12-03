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

const AREA_SIZE: usize = 16;

#[derive(AppState)]
struct State {
    font: Font,
    fps: String,
    player: Player,
    mouse_pos: Vec2,
    chunks: Vec<Chunk>,
    chunk_i: usize,
    textures: Vec<Texture>,
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
    let mut textures = Vec::<Texture>::new();
    textures.push(
        gfx.create_texture().from_image(include_bytes!("assets/atlas_test.png")).build().unwrap()
    );

    let mut chunks = Vec::new();
    for y in 0..AREA_SIZE {
        for x in 0..AREA_SIZE {
            let mut chunk = Chunk::new(gfx, x, y);
            chunk.render_low_res(gfx, &textures);
            chunks.push(chunk);
        }
    }

    State {
        font,
        fps: "¯\\_(ツ)_/¯".to_string(),
        player: PlayerBuilder::new().color_random().build(),
        mouse_pos: Vec2::new(0.0, 0.0),
        chunks,
        textures,
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
    if app.keyboard.was_pressed(KeyCode::O) && state.render_size_pow >= 7 {
        state.render_size_pow -= 1;
    }
    if app.keyboard.was_pressed(KeyCode::P) && state.render_size_pow <= 14 {
        state.render_size_pow += 1;
    }
    state.debug = app.keyboard.is_down(KeyCode::L);

    state.mouse_pos = app.mouse.position().into();

    if state.player.update(app) {
        state.chunks.iter_mut().for_each(|c| {
            let (x, y) = Chunk::pos_to_coords(state.player.pos().vec());
            let (cx, cy) = c.coords();
            let lod = (
                i32::max(((x as i32) - (cx as i32)).abs() - 1, ((y as i32) - (cy as i32)).abs()) - 1
            ).clamp(0, 7);
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

fn render_chunks(
    gfx: &mut Graphics,
    state: &mut State,
    draw: &mut Draw,
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize)
) {
    'main: for y in y1..usize::min(y2, AREA_SIZE - 1) + 1 {
        for x in x1..usize::min(x2, AREA_SIZE - 1) + 1 {
            let index = x + y * AREA_SIZE;
            if index < 0 || (index as usize) >= state.chunks.len() {
                continue;
            }
            if state.chunks[index as usize].needs_redraw() {
                state.chunks[index as usize].redraw(gfx, &state.textures);
                break 'main;
            }
        }
    }
}

use notan::math::*;
fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let time = app.date_now();

    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.set_projection(Some(world_projection(gfx.size(), state.render_size_pow).0));
    draw.transform().set(
        Mat3::from_translation(Vec2::new(-state.player.pos().x(), -state.player.pos().y()))
    );

    let (x1, y1) = Chunk::pos_to_coords(draw.screen_to_world_position(0.0, 0.0));
    let (x2, y2) = Chunk::pos_to_coords(
        draw.screen_to_world_position(gfx.size().0 as f32, gfx.size().1 as f32)
    );

    render_chunks(gfx, state, &mut draw, (x1, y1), (x2, y2));

    state.player.set_desired_rotation(
        angle_between_points(
            &state.player.pos().vec2(),
            &draw.screen_to_world_position(state.mouse_pos.x, state.mouse_pos.y)
        )
    );

    'main: for y in y1..usize::min(y2, AREA_SIZE - 1) + 1 {
        for x in x1..usize::min(x2, AREA_SIZE - 1) + 1 {
            let index = x + y * AREA_SIZE;
            state.chunks[index as usize].render(&mut draw, state.debug);
        }
    }

    state.player.render(&mut draw);

    gfx.render(&draw);

    let mut draw_ui = gfx.create_draw();

    let draw_fps = format!("draw time: {:.2}ms", app.date_now() - time);
    draw_ui
        .text(
            &state.font,
            &format!(
                "x: {:.2}\ny: {:.2}\n{}\n{}\nResolution: {:?}\nScale: {}",
                state.player.pos().x(),
                state.player.pos().y(),
                &state.fps,
                &draw_fps,
                gfx.size(),
                state.render_size_pow
            )
        )
        .position(10.0, 10.0)
        .size(14.0);

    gfx.render(&draw_ui);
}
