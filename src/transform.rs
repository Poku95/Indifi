use crate::*;

pub struct TransformPR {
    rotation: Rotation,
    position: Position,
    rotation_speed: f32,
    movement_speed: f32,
}

impl TransformPR {
    pub fn new() -> Self {
        TransformPR {
            rotation: Rotation::zero(),
            position: Position::new(2048.0, 2048.0),
            rotation_speed: 150.0,
            movement_speed: 50.0,
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
    pub fn mut_position(&mut self) -> &mut Position {
        &mut self.position
    }

    pub fn rotation(&self) -> &Rotation {
        &self.rotation
    }

    fn angle_to_speed(&self, angle: f32) -> f32 {
        (360.0 - angle.abs()) / 360.0
    }

    pub fn update(&mut self, app: &mut App, vel_vec: Vec2, desired_rotation: f32) {
        let dif = self.update_rotation(app, desired_rotation);
        if vel_vec.length() == 0.0 {
            return;
        }
        let dir =
            Vec2::from_angle(desired_rotation.to_radians()).rotate(vel_vec).perp() *
            self.angle_to_speed(dif) *
            self.movement_speed *
            app.timer.delta_f32();
        self.position.add_vec(&dir);
    }

    fn update_rotation(&mut self, app: &mut App, desired_rotation: f32) -> f32 {
        if
            self.rotation.smooth_degrees() > desired_rotation &&
            self.rotation().last_degrees() < desired_rotation
        {
            return 0.0;
        }
        let angle = match desired_rotation - (self.rotation.smooth_degrees() % 360.0) {
            x if x.abs() < 1.0 => {
                self.rotation.set_smooth(desired_rotation);
                return 0.0;
            }
            x => x,
        };
        let dir = ((angle + 180.0) % 360.0) - 180.0;
        self.rotation.add_smooth(
            (dir.clamp(-1.0, 1.0) as f32) * self.rotation_speed * app.timer.delta_f32()
        );
        dir
    }
}

pub struct Position {
    x: f32,
    y: f32,
    last_x: f32,
    last_y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position {
            x,
            y,
            last_x: x,
            last_y: y,
        }
    }
    pub fn set(&mut self, new_x: f32, new_y: f32) {
        self.last_x = self.x;
        self.x = new_x;
        self.last_y = self.y;
        self.y = new_y;
    }

    pub fn vec(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn zero() -> Self {
        Position {
            x: 0.0,
            y: 0.0,
            last_x: 0.0,
            last_y: 0.0,
        }
    }

    pub fn changed(&self) -> bool {
        self.x != self.last_x || self.y != self.last_y
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.set(self.x + x, self.y + y);
    }

    pub fn add_vec(&mut self, vec: &Vec2) {
        self.set(self.x + vec.x, self.y + vec.y);
    }

    pub fn vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

pub struct Rotation {
    degrees: f32,
    last_degrees: f32,
}

impl Rotation {
    pub fn zero() -> Self {
        Rotation {
            degrees: 0.0,
            last_degrees: 0.0,
        }
    }
    pub fn smooth_degrees(&self) -> f32 {
        self.degrees
    }
    pub fn degrees_normalized(&self) -> f32 {
        self.degrees % 360.0
    }

    pub fn set_smooth(&mut self, degrees: f32) {
        self.last_degrees = self.degrees;
        self.degrees = (degrees % 360.0) + 360.0;
    }
    pub fn set(&mut self, degrees: f32) {
        self.last_degrees = self.degrees;
        self.degrees = degrees % 360.0;
    }

    pub fn add_smooth(&mut self, degrees: f32) {
        self.set_smooth(self.degrees + degrees);
    }

    pub fn changed(&self) -> bool {
        self.last_degrees != self.degrees
    }

    fn last_degrees(&self) -> f32 {
        self.last_degrees
    }
}
