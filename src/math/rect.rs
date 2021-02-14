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
    pub fn new_valid(p1: PointF32, p2: PointF32) -> Self {
        let left = f32::min(p1.x, p2.x);
        let right = f32::max(p1.x, p2.x);
        let bottom = f32::min(p1.y, p2.y);
        let top = f32::max(p1.y, p2.y);

        Rect {
            bottom_left: PointF32 { x: left, y: bottom },
            top_right: PointF32 { x: right, y: top },
        }
    }

    pub fn new(bottom_left: PointF32, top_right: PointF32) -> Self {
        Rect {
            bottom_left,
            top_right,
        }
    }

    pub fn new_empty() -> Self {
        Rect::new(
            PointF32::new(f32::MAX, f32::MAX),
            PointF32::new(f32::MIN, f32::MIN),
        )
    }

    pub fn grow(&mut self, x: f32, y: f32) {
        self.bottom_left -= PointF32 { x: x, y: y };
        self.top_right += PointF32 { x: x, y: y };
    }

    pub fn include(&mut self, other: &Rect) {
        self.bottom_left.x = f32::min(self.bottom_left.x, other.bottom_left.x);
        self.bottom_left.y = f32::min(self.bottom_left.y, other.bottom_left.y);

        self.top_right.x = f32::max(self.top_right.x, other.top_right.x);
        self.top_right.y = f32::max(self.top_right.y, other.top_right.y);
    }

    pub fn collides(&self, other: &Self) -> bool {
        return (self.bottom_left.x <= other.top_right.x)
            && (self.top_right.x >= other.bottom_left.x)
            && (self.bottom_left.y <= other.top_right.y)
            && (self.top_right.y >= other.bottom_left.y);
    }

    pub fn size(&self) -> PointF32 {
        PointF32 {
            x: self.top_right.x - self.bottom_left.x,
            y: self.top_right.y - self.bottom_left.y,
        }
    }

    pub fn top_left(&self) -> PointF32 {
        PointF32 {
            x: self.bottom_left.x,
            y: self.top_right.y,
        }
    }

    pub fn bottom_right(&self) -> PointF32 {
        PointF32 {
            x: self.top_right.x,
            y: self.bottom_left.y,
        }
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
