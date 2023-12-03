use crate::*;
use notan::random::rand;

pub struct Player {
    id: u16,
    display_name: String,
    desired_rotation: Rotation,
    color: Color,
    transform: TransformPR,
    last_coords: (usize, usize),
}

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Player{}({},{})",
            self.id,
            self.transform.position().x(),
            self.transform.position().y()
        )
    }
}

impl Player {
    pub fn speed_max_xy(&self) -> f32 {
        self.transform.position().speed_max_xy()
    }
    pub fn pos(&self) -> &Position {
        self.transform.position()
    }
    fn pos_touple(&self) -> (f32, f32) {
        self.transform.pos_touple()
    }
    pub fn last_coords(&self) -> (usize, usize) {
        self.last_coords
    }

    pub fn pos_mut(&mut self) -> &mut Position {
        self.transform.mut_position()
    }

    pub fn render(&self, draw: &mut Draw) {
        let (x, y) = self.pos_touple();
        draw.circle(5.0).position(x, y).color(Color::new(0.1, 0.1, 0.1, 1.0));
        draw.circle(4.0).position(x, y).color(self.color);
        draw.line((x, y), (x, y + 5.0))
            .rotate_degrees_from((x, y), self.transform.rotation().degrees_normalized() - 90.0)
            .color(Color::new(0.1, 0.1, 0.1, 0.7));
    }

    pub fn color_rgb(&self) -> [f32; 3] {
        self.color.rgb()
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = Color::new(r, g, b, 1.0);
    }

    pub fn rotation(&self) -> &Rotation {
        &self.transform.rotation()
    }
    pub fn desired_rotation(&self) -> &Rotation {
        &self.desired_rotation
    }
    pub fn set_desired_rotation(&mut self, degrees: f32) {
        self.desired_rotation.set_smooth(degrees);
    }

    pub fn update(&mut self, app: &mut App) -> bool {
        self.transform.update(
            app,
            Player::get_player_input(app),
            self.desired_rotation.smooth_degrees()
        );
        self.check_chunk_change()
    }

    fn check_chunk_change(&mut self) -> bool {
        let coords = Chunk::pos_to_coords(self.pos().vec());
        if self.last_coords != coords && self.speed_max_xy() < Chunk::size() as f32 / 4.0 {
            self.last_coords = coords;
            return true
        }
        false
    }

    fn get_player_input(app: &App) -> Vec2 {
        let mut pos: Vec2 = Vec2::new(0.0, 0.0);

        if app.keyboard.is_down(KeyCode::W) {
            pos.y -= 1.0;
        }

        if app.keyboard.is_down(KeyCode::A) {
            pos.x -= 0.75;
        }

        if app.keyboard.is_down(KeyCode::S) {
            pos.y += 0.5;
        }

        if app.keyboard.is_down(KeyCode::D) {
            pos.x += 0.75;
        }
        pos
    }
}

pub struct PlayerBuilder {
    id: u16,
    display_name: String,
    desired_rotation: Rotation,
    color: Color,
    transform: TransformPR,
}

impl Default for PlayerBuilder {
    fn default() -> Self {
        PlayerBuilder {
            id: 0,
            display_name: "Blank".to_string(),
            desired_rotation: Rotation::zero(),
            color: Color::BLUE,
            transform: TransformPR::new(),
        }
    }
}

impl PlayerBuilder {
    pub fn new() -> Self {
        PlayerBuilder::default()
    }

    pub fn color(mut self, color: Color) -> PlayerBuilder {
        self.color = color;
        self
    }

    pub fn color_random(mut self) -> PlayerBuilder {
        self.color = Color::new(rand::random(), rand::random(), rand::random(), 1.0);
        self
    }

    pub fn build(self) -> Player {
        Player {
            id: self.id,
            display_name: self.display_name,
            desired_rotation: self.desired_rotation,
            color: self.color,
            transform: self.transform,
            last_coords: (1, 1),
        }
    }
}
