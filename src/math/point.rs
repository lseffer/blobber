use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub type PointF32 = Point<f32>;
pub type PointU32 = Point<u32>;

impl Point<f32> {
    pub fn magnitude(&self) -> f32 {
        self.dot(&self).sqrt()
    }
}

impl<T> Point<T>
where
    T: Mul + Copy,
    T::Output: Add,
{
    pub fn dot(self, rhs: &Self) -> <T::Output as Add>::Output {
        return self.x * rhs.x + self.y * rhs.y;
    }
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x: x, y: y }
    }
}

impl<T: Add> Add<Point<T>> for Point<T> {
    type Output = Point<T::Output>;

    fn add(self, rhs: Self) -> Point<T::Output> {
        return Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl<T: AddAssign> AddAssign<Point<T>> for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: AddAssign + Copy> AddAssign<&Point<T>> for Point<T> {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: SubAssign> SubAssign<Point<T>> for Point<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: SubAssign + Copy> SubAssign<&Point<T>> for Point<T> {
    fn sub_assign(&mut self, rhs: &Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<'a, 'b, T: Copy + Add> Add<&'b Point<T>> for &'a Point<T> {
    type Output = Point<T::Output>;

    fn add(self, rhs: &'b Point<T>) -> Point<T::Output> {
        return Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl<T: Sub> Sub<Point<T>> for Point<T> {
    type Output = Point<T::Output>;

    fn sub(self, rhs: Self) -> Point<T::Output> {
        return Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl<'a, 'b, T: Copy + Sub> Sub<&'b Point<T>> for &'a Point<T> {
    type Output = Point<T::Output>;

    fn sub(self, rhs: &'b Point<T>) -> Point<T::Output> {
        return Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl<T: Mul<R>, R: Copy> Mul<R> for Point<T> {
    type Output = Point<<T as Mul<R>>::Output>;

    fn mul(self, rhs: R) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Mul<R> + Copy, R: Copy> Mul<R> for &Point<T> {
    type Output = Point<<T as Mul<R>>::Output>;

    fn mul(self, rhs: R) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Div<R>, R: Copy> Div<R> for Point<T> {
    type Output = Point<<T as Div<R>>::Output>;

    fn div(self, rhs: R) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Div<R> + Copy, R: Copy> Div<R> for &Point<T> {
    type Output = Point<<T as Div<R>>::Output>;

    fn div(self, rhs: R) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let p1 = Point { x: 1, y: 4 };
        let p2 = Point { x: 7, y: 2 };
        assert_eq!(p1 + p2, Point { x: 8, y: 6 });
        assert_eq!(&p1 + &p2, Point { x: 8, y: 6 });
    }

    #[test]
    fn test_sub() {
        let p1 = Point { x: 1, y: 3 };
        let p2 = Point { x: 7, y: 2 };
        assert_eq!(p1 - p2, Point { x: -6, y: 1 });
        assert_eq!(&p1 - &p2, Point { x: -6, y: 1 });
    }

    #[test]
    fn test_add_assign() {
        let mut p1 = Point { x: 1, y: 4 };
        let p2 = Point { x: 7, y: 2 };

        p1 += p2;
        assert_eq!(p1, Point { x: 8, y: 6 });

        let p3 = Point { x: -2, y: 4 };
        p1 += &p3;
        assert_eq!(p1, Point { x: 6, y: 10 });
    }

    #[test]
    fn test_sub_assign() {
        let mut p1 = Point { x: 1, y: 3 };
        let p2 = Point { x: 7, y: 2 };

        p1 -= p2;
        assert_eq!(p1, Point { x: -6, y: 1 });

        let p3 = Point { x: 5, y: -3 };
        p1 -= &p3;
        assert_eq!(p1, Point { x: -11, y: 4 });
    }
}
