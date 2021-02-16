use super::vec2d::{GridPos, Vec2d};
use crate::math::collision;
use crate::math::point::{PointF32, PointU32};
use crate::math::rect::Rect;
use std::vec::Vec;

pub trait AabbObject {
    fn aabb(&self) -> Rect;
}

struct Quad<T> {
    bounds: Rect,
    object_bounds: Rect,
    objects: Vec<T>,
}

pub struct AabbGrid<T> {
    grid_pos: GridPos,
    grid: Vec2d<Quad<T>>,
}

impl<T: AabbObject> AabbGrid<T> {
    pub fn new(rect: Rect, n_x: u32, n_y: u32) -> Self {
        let size = rect.size();
        let quad_size = PointF32 {
            x: size.x / n_x as f32,
            y: size.y / n_y as f32,
        };

        let grid = Vec2d::<Quad<T>>::new_with_creator(n_x, n_y, |x: u32, y: u32| -> Quad<T> {
            let x_f32 = x as f32;
            let y_f32 = y as f32;
            Quad {
                bounds: Rect::new(
                    PointF32::new(x_f32 * quad_size.x, y_f32 * quad_size.y),
                    PointF32::new(x_f32 * quad_size.x, y_f32 * quad_size.y) + quad_size,
                ),
                object_bounds: Rect::new_empty(),
                objects: Vec::<T>::new(),
            }
        });

        return AabbGrid {
            grid_pos: GridPos::new(rect, quad_size),
            grid,
        };
    }

    pub fn new_with_objects(
        rect: Rect,
        n_x: u32,
        n_y: u32,
        mut iter: impl Iterator<Item = T>,
    ) -> Self {
        let mut grid = AabbGrid::new(rect, n_x, n_y);
        while let Some(item) = iter.next() {
            let aabb = &item.aabb();
            let grid_pos = grid.grid_pos.grid_for(&aabb.center());
            let quad = grid.grid.value(grid_pos.x, grid_pos.y);
            quad.object_bounds.include(aabb);
            quad.objects.push(item);
        }
        grid
    }

    pub fn for_objects(&self, rect: &Rect, mut func: impl FnMut(&T)) {
        self.grid
            .iter()
            .filter(|quad| rect.collides(&quad.object_bounds))
            .flat_map(|quad| quad.objects.iter())
            .map(|object| (object, object.aabb()))
            .filter(|(_, object_rect)| object_rect.collides(&rect))
            .for_each(|(object, _)| func(object));
    }

    pub fn for_objects_mut(&mut self, rect: &Rect, mut func: impl FnMut(&mut T)) {
        self.grid
            .iter_mut()
            .filter(|quad| rect.collides(&quad.object_bounds))
            .flat_map(|quad| quad.objects.iter_mut())
            .map(|object| {
                let aabb = object.aabb();
                (object, aabb)
            })
            .filter(|(_, object_rect)| object_rect.collides(&rect))
            .for_each(|(object, _)| func(object));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct Object {
        rect: Rect,
    }

    impl AabbObject for &Object {
        fn aabb(&self) -> Rect {
            self.rect
        }
    }

    #[test]
    fn test_foo() {
        let objects = vec![
            Object {
                rect: Rect::new(PointF32::new(45.0, 80.0), PointF32::new(50.0, 85.0)),
            },
            Object {
                rect: Rect::new(PointF32::new(46.0, 75.0), PointF32::new(51.0, 80.0)),
            },
            Object {
                rect: Rect::new(PointF32::new(15.0, 90.0), PointF32::new(20.0, 95.0)),
            },
        ];

        let grid = AabbGrid::<&Object>::new_with_objects(
            Rect::new(PointF32::new(0.0, 0.0), PointF32::new(100.0, 100.0)),
            10,
            20,
            objects.iter().map(|item| item),
        );

        {
            let expected: Vec<&Object> = objects.iter().collect();
            let mut found = Vec::<&Object>::new();
            grid.for_objects(
                &Rect::new(PointF32::new(0.0, 0.0), PointF32::new(100.0, 100.0)),
                |item| found.push(item),
            );

            assert_vec_eq!(found, expected);
        }

        {
            let expected = vec![&objects[0], &objects[2]];
            let mut found = Vec::<&Object>::new();
            grid.for_objects(
                &Rect::new(PointF32::new(0.0, 0.0), PointF32::new(45.0, 100.0)),
                |item| found.push(item),
            );

            assert_vec_eq!(found, expected);
        }
    }
}
