use crate::*;
use notan::random::rand::{ self, random };

const TEXTURE_SIZE: u32 = 512;

pub struct Chunk {
    coords: (i32, i32),
    floor_tiles: Vec<u32>,
    render_texture: RenderTexture,
    level_of_detail: u32,
    lod: u32,
}

impl Chunk {
    pub fn new(gfx: &mut Graphics, x: i32, y: i32) -> Self {
        let mut floor_tiles = Vec::with_capacity(16 * 16);
        for i in 0..16 * 16 {
            floor_tiles.push((random::<f32>() * 6.0) as u32);
        }
        let mut render_texture = gfx.create_render_texture(64, 64).build().unwrap();
        Chunk {
            coords: (x, y),
            floor_tiles,
            render_texture,
            level_of_detail: 4,
            lod: 6,
        }
    }

    pub fn set_lod(&mut self, lod: u32) {
        self.level_of_detail = lod.clamp(0, 4);
    }

    //lod = level of detail
    pub fn render_texture(&mut self, gfx: &mut Graphics, floor_textures: &Vec<Texture>) {
        let lod = (2_u32).pow(self.lod);
        let size = gfx.size();
        gfx.set_size(TEXTURE_SIZE / lod, TEXTURE_SIZE / lod);
        self.render_texture = gfx
            .create_render_texture(TEXTURE_SIZE / lod, TEXTURE_SIZE / lod)
            .build()
            .unwrap();
        let mut draw = gfx.create_draw();
        self.floor_tiles
            .iter()
            .enumerate()
            .for_each(|(i, b)| {
                let x = (((i % 16) * 32) / (lod as usize)) as f32;
                let y = (((i / 16) * 32) / (lod as usize)) as f32;
                draw.image(&floor_textures[*b as usize])
                    .position(x, y)
                    .size(32.0 / (lod as f32), 32.0 / (lod as f32));
            });
        gfx.render_to(&mut self.render_texture, &draw);
        gfx.set_size(size.0, size.1);
    }

    pub fn update(&mut self, gfx: &mut Graphics, floor_textures: &Vec<Texture>) -> bool {
        let p = self.level_of_detail != self.lod;
        if p {
            self.lod = self.level_of_detail;
            self.render_texture(gfx, floor_textures);
        }
        p
    }

    fn coords_to_position(coords: (i32, i32)) -> Vec2 {
        Vec2::new((coords.0 * Chunk::size()) as f32, (coords.1 * Chunk::size()) as f32)
    }

    pub fn pos_to_coords(pos: Vec2) -> (i32, i32) {
        ((pos.x as i32) / Chunk::size(), (pos.y as i32) / Chunk::size())
    }

    pub fn coords(&self) -> (i32, i32) {
        self.coords
    }

    fn size() -> i32 {
        256
    }

    pub fn render(&self, draw: &mut Draw, debug: bool) {
        if self.lod > 3 {
            return;
        }
        let (x, y) = Chunk::coords_to_position(self.coords).into();

        if debug {
            draw.rect(
                ((self.coords.0 * Chunk::size()) as f32, (self.coords.1 * Chunk::size()) as f32),
                (Chunk::size() as f32, Chunk::size() as f32)
            ).color(
                Color::new(
                    1.0 / (5.0 - (self.lod as f32)),
                    1.0 / (5.0 - (self.lod as f32)),
                    1.0 / (5.0 - (self.lod as f32)),
                    1.0
                )
            );
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
