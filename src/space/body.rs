use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
pub(super) struct Body {
    mass: f32,
    radius: f32,
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,

    updated: bool,
}

impl Body {
    pub fn builder() -> BodyBuilder {
        BodyBuilder::default()
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Entity {
        commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(self.radius).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(self.position.extend(0.0)),
                    ..default()
                },
                self,
            ))
            .id()
    }

    pub fn position(&mut self) -> Option<Vec2> {
        if self.updated {
            self.updated = false;
            return Some(self.position);
        }
        None
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.updated = true;
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }

    pub fn fixed_update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        self.acceleration = Vec2::ZERO;
    }

    pub fn force_between(&self, other: &Body) -> Vec2 {
        let __vec = other.position - self.position;
        __vec.normalize_or_zero() * (self.mass * other.mass / __vec.length())
    }
}

pub(super) struct BodyBuilder {
    mass: f32,
    radius: f32,
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,

    updated: bool,
}

impl BodyBuilder {
    pub fn mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self.radius = mass.sqrt();
        self
    }

    pub fn position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    pub fn velocity(mut self, velocity: Vec2) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn acceleration(mut self, acceleration: Vec2) -> Self {
        self.acceleration = acceleration;
        self
    }

    pub fn build(self) -> Body {
        Body {
            mass: self.mass,
            radius: self.radius,
            position: self.position,
            velocity: self.velocity,
            acceleration: self.acceleration,
            updated: false,
        }
    }
}

impl Default for BodyBuilder {
    fn default() -> Self {
        Self {
            mass: 1.0,
            radius: 1.0,
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            updated: false,
        }
    }
}
