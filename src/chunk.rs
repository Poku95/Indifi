use crate::*;
use notan::random::rand::{ self, random };

const TEXTURE_SIZE: u32 = 512;

pub struct Chunk {
    coords: (usize, usize),
    floor_tiles: Vec<u32>,
    render_texture: RenderTexture,
    low_res: RenderTexture,
    level_of_detail: u32,
    lod: u32,
    in_bounds: bool,
}

fn index_to_pos(i: usize, level_of_detail: u32) -> (f32, f32) {
    (
        (((i % 16) * 32) / (level_of_detail as usize)) as f32,
        (((i / 16) * 32) / (level_of_detail as usize)) as f32,
    )
}

impl Chunk {
    pub fn new(gfx: &mut Graphics, x: usize, y: usize) -> Self {
        let mut floor_tiles = Vec::with_capacity(16 * 16);
        for i in 0..16 * 16 {
            floor_tiles.push((random::<f32>() * 6.0) as u32);
        }
        let mut render_texture = gfx.create_render_texture(64, 64).build().unwrap();
        Chunk {
            coords: (x, y),
            floor_tiles,
            low_res: render_texture.clone(),
            render_texture,
            level_of_detail: 3,
            lod: 4,
            in_bounds: false,
        }
    }

    pub fn set_lod(&mut self, lod: u32) {
        self.level_of_detail = lod.clamp(0, 4);
    }

    //lod = level of detail
    pub fn render_low_res(&mut self, gfx: &mut Graphics, textures: &Vec<Texture>) {
        let lod = self.lod;
        self.lod = 4;
        self.low_res = self.render_texture(gfx, textures);
        self.lod = lod;
    }

    fn render_texture(&mut self, gfx: &mut Graphics, textures: &Vec<Texture>) -> RenderTexture {
        let lod = (2_u32).pow(self.lod);
        let size = gfx.size();
        gfx.set_size(TEXTURE_SIZE / lod, TEXTURE_SIZE / lod);
        let mut texture = gfx
            .create_render_texture(TEXTURE_SIZE / lod, TEXTURE_SIZE / lod)
            .build()
            .unwrap();

        let mut draw = gfx.create_draw();
        self.floor_tiles
            .iter()
            .enumerate()
            .for_each(|(i, b)| {
                let (x, y) = index_to_pos(i, lod);
                draw.image(&textures[0])
                    .position(x, y)
                    .size(32.0 / (lod as f32), 32.0 / (lod as f32))
                    .crop((((*b % 16) as f32) * 32.0, (*b / 16) as f32), (32.0, 32.0));
            });
        gfx.render_to(&mut texture, &draw);
        gfx.set_size(size.0, size.1);
        texture
    }

    pub fn needs_redraw(&self) -> bool {
        self.lod != self.level_of_detail
    }

    pub fn redraw(&mut self, gfx: &mut Graphics, textures: &Vec<Texture>) {
        self.lod = self.level_of_detail;
        self.render_texture = self.render_texture(gfx, textures);
    }

    fn coords_to_position(coords: (usize, usize)) -> Vec2 {
        Vec2::new((coords.0 * Chunk::size()) as f32, (coords.1 * Chunk::size()) as f32)
    }

    pub fn pos_to_coords(pos: Vec2) -> (usize, usize) {
        ((pos.x as usize) / Chunk::size(), (pos.y as usize) / Chunk::size())
    }

    pub fn coords(&self) -> (usize, usize) {
        self.coords
    }

    pub fn size() -> usize {
        256
    }

    pub fn render(&self, draw: &mut Draw, debug: bool) {
        let (x, y) = Chunk::coords_to_position(self.coords).into();
        let (x2, y2) = Chunk::coords_to_position((self.coords.0, self.coords.1)).into();

        let screen_pos = draw.world_to_screen_position(x, y);
        let screen_pos2 = draw.world_to_screen_position(
            x + (Chunk::size() as f32),
            y + (Chunk::size() as f32)
        );
        if
            (screen_pos2.x < 0.0 && screen_pos2.y < 0.0) ||
            screen_pos.x > draw.size().0 ||
            screen_pos.y > draw.size().1
        {
            return;
        }

        if debug {
            draw.rect(
                ((self.coords.0 * Chunk::size()) as f32, (self.coords.1 * Chunk::size()) as f32),
                (Chunk::size() as f32, Chunk::size() as f32)
            ).color(
                Color::new(
                    1.0 / (5.0 - (self.lod as f32)),
                    1.0,
                    self.render_texture.width() / 1024.0,
                    1.0
                )
            );
        } else {
            if self.lod > 3 {
                draw.image(&self.low_res)
                    .position(
                        (self.coords.0 * Chunk::size()) as f32,
                        (self.coords.1 * Chunk::size()) as f32
                    )
                    .size(Chunk::size() as f32, Chunk::size() as f32);
            } else {
                draw.image(&self.render_texture)
                    .position(
                        (self.coords.0 * Chunk::size()) as f32,
                        (self.coords.1 * Chunk::size()) as f32
                    )
                    .size(Chunk::size() as f32, Chunk::size() as f32);
            }
        }
    }
}
