use crate::math::circle::Circle;
use crate::math::point::PointF32;
use crate::math::rect::Rect;

pub type BlobId = u32;

#[derive(Debug)]
pub struct Blob {
    pub id: BlobId,
    pub circle: Circle,
    pub mass: f32,
    pub velocity: PointF32,
    pub force: PointF32,
    pub rotation: f32,
    pub angular_velocity: f32,
    pub angular_force: f32,
    pub collision_aabb: Rect,
}

impl Blob {
    pub fn new(x: f32, y: f32, radius: f32, id: BlobId) -> Self {
        Blob {
            id,
            circle: Circle::new(x, y, radius),
            mass: 13.0,
            velocity: PointF32::new(0.0, 0.0),
            force: PointF32::new(0.0, 0.0),
            rotation: 0.0,
            angular_velocity: 0.0,
            angular_force: 0.0,
            collision_aabb: Rect::new_empty(),
        }
    }

    fn clamp(value: f32, min: f32, max: f32) -> f32 {
        value.min(max).max(min)
    }

    pub fn update(&mut self, dt: f32) {
        if self.velocity.magnitude().abs() > f32::EPSILON {
            self.force -= self.velocity / self.velocity.magnitude()
                * self.velocity.dot(&self.velocity)
                * 0.2
                * self.mass
                * dt;
        }

        if self.angular_velocity > f32::EPSILON {
            self.angular_force -= self.angular_velocity.signum()
                * self.angular_velocity
                * self.angular_velocity
                * 1000.0
                * dt; // Or something
        }

        let acceleration = self.force * (1.0 / self.mass);
        let angular_acceleration = self.angular_force * (1.0 / self.mass); // Or something

        self.velocity += acceleration * dt;
        self.angular_velocity += angular_acceleration * dt;

        // We probably want to update the collision AABB before moving the actual object.
        self.collision_aabb = Rect::new(self.circle.pos, self.circle.pos + self.velocity * dt);
        self.collision_aabb
            .grow(self.circle.radius, self.circle.radius);

        self.circle.pos += self.velocity * dt;
        self.rotation += self.angular_velocity * dt;

        self.force = PointF32::new(0.0, 0.0);
        self.angular_force = 0.0;
    }
}
