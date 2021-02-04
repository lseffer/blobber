use super::circle::Circle;
use super::point::PointF32;
use super::rect::{Line, Rect};

pub type Velocity = PointF32;

pub fn collides(circle: &Circle, line: &Line) -> Option<f32> {
    // Solves
    // x = x_0 + t*dir
    // y = y_0 + t*dir
    // (x - x_1)^2 + (y - y_1)^2 = r^2
    // for t, which would be the "place" along 'line' where the collision
    // occurs, which boils down to solving a second order equation.

    // Note that because this is a mathematical line it has no start or end,
    // so the resulting 't' might be behind the start of line. However, because
    // we want to return only one solution we always return the closest one
    // (except if we are inside the circle, in which case it is arbitary).

    let diff = line.pos - circle.pos;
    let a = line.direction.dot(&line.direction);
    let b = 2f32 * diff.dot(&line.direction);
    let c = diff.dot(&diff) - circle.radius * circle.radius;

    let p = b / (2f32 * a);
    let d = p * p - c / a;

    // If the equation has no real solution then the line and the circle don't
    // collide.
    // TODO Instead of checking if 'd' is NaN maybe better to check if 'a' is really close to '0'
    if d < 0f32 || d.is_nan() {
        return None;
    }

    // Returns -p +/- d.sqrt(), depending on the solution is with
    // a positive or negative 't' (i.e. in 'front' or 'behind' the
    // starting point of 'line').
    let d_sqrt = d.sqrt();
    let t = -p + if -p >= d_sqrt { -d_sqrt } else { d_sqrt };

    return Some(t);
}

pub fn may_collide(
    circle1: (&Circle, &Velocity),
    circle2: (&Circle, &Velocity),
    time: f32,
) -> bool {
    let mut rect1 = Rect::new(circle1.0.pos, circle1.0.pos + circle1.1 * time);
    rect1.grow(circle1.0.radius, circle1.0.radius);
    let mut rect2 = Rect::new(circle2.0.pos, circle2.0.pos + circle2.1 * time);
    rect2.grow(circle2.0.radius, circle2.0.radius);

    return rect1.collides(&rect2);
}

pub fn collides_circle(
    circle1: (&Circle, &Velocity),
    circle2: (&Circle, &Velocity),
) -> Option<f32> {
    collides(
        &Circle {
            pos: circle1.0.pos,
            radius: circle1.0.radius + circle2.0.radius,
        },
        &Line {
            pos: circle2.0.pos,
            direction: circle2.1 - circle1.1,
        },
    )
}

pub fn collides_before(
    circle1: (&Circle, &Velocity),
    circle2: (&Circle, &Velocity),
    time: f32,
) -> Option<f32> {
    // TODO might be faster to first check 'may_collide'
    let collides_at = collides_circle(circle1, circle2);
    match collides_at {
        None => None,
        Some(t) if t < 0f32 || t > time => None,
        Some(t) => Some(t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!();
            }
        };
    }

    #[test]
    fn test_collides() {
        {
            let circle = Circle::new(3f32, -1f32, 3f32);
            let line1 = Line::through(PointF32::new(-2f32, 3f32), PointF32::new(3f32, 2.5f32));
            let line2 = Line::through(PointF32::new(-2f32, 3f32), PointF32::new(-0.5f32, -2f32));
            let line3 = Line::through(PointF32::new(5f32, -5f32), PointF32::new(6f32, -3f32));

            assert_eq!(collides(&circle, &line1), None);
            assert_eq!(collides(&circle, &line2), None);
            assert_eq!(collides(&circle, &line3), None);
        }

        {
            let circle = Circle::new(2f32, 1f32, 1.5f32);
            let line1 = Line::through(PointF32::new(-1f32, 2.5f32), PointF32::new(1f32, 2.5f32));
            let line2 = Line::through(PointF32::new(-1f32, 0f32), PointF32::new(0.5f32, 1f32));

            let d = 1.5f32 * 2f32.sqrt();
            let line3 = Line::through(
                PointF32::new(3f32, -3f32),
                PointF32::new(3f32, (1f32 - d) / 2f32),
            );

            assert_eq!(collides(&circle, &line1), Some(1.5f32));
            assert_delta!(collides(&circle, &line2).unwrap(), 1f32, 0.00001f32);
            assert_delta!(collides(&circle, &line3).unwrap(), 2f32, 0.00001f32);
        }

        {
            let circle = Circle::new(2f32, 1f32, 1.5f32);
            let line1 = Line::through(PointF32::new(1f32, 2.5f32), PointF32::new(-1f32, 2.5f32));
            let line2 = Line::through(PointF32::new(0.5f32, 1f32), PointF32::new(-1f32, 0f32));

            let d = 1.5f32 * 2f32.sqrt();
            let line3 = Line::through(
                PointF32::new(3f32, (1f32 - d) / 2f32),
                PointF32::new(3f32, -3f32),
            );

            assert_eq!(collides(&circle, &line1), Some(-0.5f32));
            assert_delta!(collides(&circle, &line2).unwrap(), 0f32, 0.00001f32);
            assert_delta!(collides(&circle, &line3).unwrap(), -1f32, 0.00001f32);
        }
    }

    #[test]
    fn test_collides_before() {
        // The circles are (0.5, 0) apart
        let circle1 = Circle::new(2f32, -2f32, 1.5f32);
        let circle2 = Circle::new(6.5f32, -2f32, 2.5f32);

        assert_eq!(
            collides_before(
                (&circle1, &PointF32::new(0f32, 0f32)),
                (&circle2, &PointF32::new(0f32, 0f32)),
                10f32
            ),
            None
        );

        assert_eq!(
            collides_before(
                (&circle1, &PointF32::new(-1f32, 0f32)),
                (&circle2, &PointF32::new(1f32, 0f32)),
                10f32
            ),
            None
        );

        assert_eq!(
            collides_before(
                (&circle1, &PointF32::new(-1f32, 2f32)),
                (&circle2, &PointF32::new(1f32, -2f32)),
                10f32
            ),
            None
        );

        assert_eq!(
            collides_before(
                (&circle1, &PointF32::new(1f32, 0f32)),
                (&circle2, &PointF32::new(1f32, 0f32)),
                10f32
            ),
            None
        );

        assert_eq!(
            collides_before(
                (&circle1, &PointF32::new(0.5f32, 0f32)),
                (&circle2, &PointF32::new(0f32, 0f32)),
                0.5f32
            ),
            None
        );

        assert_eq!(
            collides_before(
                (&circle1, &PointF32::new(1f32, 0f32)),
                (&circle2, &PointF32::new(0f32, 0f32)),
                1f32
            ),
            Some(0.5)
        );

        assert_eq!(
            collides_before(
                (&circle1, &PointF32::new(0.5f32, 0f32)),
                (&circle2, &PointF32::new(0f32, 0f32)),
                1f32
            ),
            Some(1f32)
        );

        assert_eq!(
            collides_before(
                (&circle1, &PointF32::new(0.5f32, 0f32)),
                (&circle2, &PointF32::new(-0.5f32, 0f32)),
                1f32
            ),
            Some(0.5)
        );
    }
}
