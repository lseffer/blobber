use super::point::PointF32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    pub pos: PointF32,
    pub radius: f32,
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Circle {
            pos: PointF32 { x: x, y: y },
            radius: radius,
        }
    }
    pub fn collides(self, other: &Circle) -> bool {
        let delta = &self.pos - &other.pos;
        let d_sqr = delta.dot(&delta);
        let r_sum = &self.radius + &other.radius;
        return r_sum * r_sum <= d_sqr;
    }

    pub fn contains(self, other: &Circle) -> bool {
        let delta = &self.pos - &other.pos;
        let d = delta.dot(&delta).sqrt();
        return self.radius - other.radius > d;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inside() {
        {
            let c1 = Circle {
                pos: { PointF32 { x: 1.0, y: 2.0 } },
                radius: 3.0,
            };

            let c2 = Circle {
                pos: { PointF32 { x: 1.0, y: 2.0 } },
                radius: 2.0,
            };

            assert!(c1.contains(&c2));
            assert!(!c2.contains(&c1));
            assert!(!c1.contains(&c1));
        }

        {
            let c1 = Circle {
                pos: { PointF32 { x: 1.0, y: 2.0 } },
                radius: 3.0,
            };

            let c2 = Circle {
                pos: { PointF32 { x: -1.0, y: 2.0 } },
                radius: 2.0,
            };

            assert!(!c1.contains(&c2));
        }

        {
            let c1 = Circle {
                pos: { PointF32 { x: 1.0, y: 1.5 } },
                radius: 3.0,
            };
            let c2 = Circle {
                pos: { PointF32 { x: -1.0, y: -2.0 } },
                radius: 8.0,
            };
            let c3 = Circle {
                pos: { PointF32 { x: -1.0, y: -2.0 } },
                radius: 7.0,
            };
            assert!(c2.contains(&c1));
            assert!(!c3.contains(&c1));
        }
    }
}
