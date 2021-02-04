use super::point::PointF32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pub bottom_left: PointF32,
    pub top_right: PointF32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
    pub pos: PointF32,
    pub direction: PointF32,
}

impl Rect {
    pub fn new(p1: PointF32, p2: PointF32) -> Self {
        let left = f32::min(p1.x, p2.x);
        let right = f32::max(p1.x, p2.y);
        let bottom = f32::min(p1.y, p2.y);
        let top = f32::max(p1.y, p2.y);

        Rect {
            bottom_left: PointF32 { x: left, y: bottom },
            top_right: PointF32 { x: right, y: top },
        }
    }

    pub fn grow(&mut self, x: f32, y: f32) {
        self.bottom_left -= PointF32 { x: x, y: y };
        self.top_right += PointF32 { x: x, y: y };
    }

    pub fn collides(&self, other: &Self) -> bool {
        return (self.bottom_left.x <= other.top_right.x)
            && (self.top_right.x >= other.bottom_left.x)
            && (self.bottom_left.y <= other.top_right.y)
            && (self.top_right.y >= other.bottom_left.y);
    }
}

impl Line {
    pub fn new(pos: PointF32, direction: PointF32) -> Self {
        Line {
            pos: pos,
            direction: direction,
        }
    }

    pub fn through(p1: PointF32, p2: PointF32) -> Self {
        let direction = p2 - p1;
        Line {
            pos: p1,
            direction: direction,
        }
    }
}
