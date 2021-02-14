use super::vec2d::Vec2d;
use crate::math::circle::Circle;
use crate::math::collision;
use crate::math::point::{Point, PointF32};
use crate::math::rect::Rect;
use std::vec::Vec;

type PointU32 = Point<u32>;

pub trait GridObject {
    fn pos(&self) -> PointF32;
    fn radius(&self) -> f32;
}

#[derive(Clone)]
struct Quad<T> {
    bounds: Rect,
    object_bounds: Rect,
    objects: Vec<T>,
}

struct GridPos {
    bounds: Rect,
    quad_size: PointF32,
}

impl GridPos {
    fn new(bounds: Rect, quad_size: PointF32) -> Self {
        GridPos { bounds, quad_size }
    }

    fn grid_for(&self, pos: &PointF32) -> PointU32 {
        let rel_pos = pos - &self.bounds.bottom_left;
        Point {
            x: (rel_pos.x / self.quad_size.x) as u32,
            y: (rel_pos.y / self.quad_size.y) as u32,
        }
    }
}

pub struct LooseGrid<T> {
    grid_pos: GridPos,
    grid: Vec2d<Quad<T>>,
}

impl<T: GridObject> LooseGrid<T> {
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
                    Point::new(x_f32 * quad_size.x, y_f32 * quad_size.y),
                    Point::new(x_f32 * quad_size.x, y_f32 * quad_size.y) + quad_size,
                ),
                object_bounds: Rect::new_empty(),
                objects: Vec::<T>::new(),
            }
        });

        return LooseGrid {
            grid_pos: GridPos::new(rect, quad_size),
            grid,
        };
    }

    pub fn moved_new_quads(&mut self) -> Vec<T> {
        let mut result = Vec::<T>::new();

        let grid = &mut self.grid;
        let grid_pos = &self.grid_pos;

        grid.for_each_mut(|x, y, quad| {
            let quad_pos = PointU32::new(x, y);
            let mut i: usize = 0;
            loop {
                if i >= quad.objects.len() {
                    break;
                }

                if grid_pos.grid_for(&quad.objects[i].pos()) != quad_pos {
                    let item = quad.objects.swap_remove(i);
                    // TODO Here we could just add the item to
                    // a new quad instead.
                    result.push(item);
                } else {
                    i += 1;
                }
            }
        });

        result
    }

    pub fn add(&mut self, item: T) {
        let grid_pos = self.grid_pos.grid_for(&item.pos());

        self.grid.value(grid_pos.x, grid_pos.y).objects.push(item);
    }

    pub fn update(&mut self) {
        self.moved_new_quads().into_iter().for_each(|object| {
            self.add(object);
        });

        self.grid.iter_mut().for_each(|quad| {
            quad.object_bounds = quad
                .objects
                .iter()
                .map(|item| {
                    let radius = PointF32::new(item.radius(), item.radius());
                    Rect::new(item.pos() - radius, item.pos() + radius)
                })
                .fold(Rect::new_empty(), |mut acc, rect| {
                    acc.include(&rect);
                    acc
                });
        });
    }

    pub fn remove_if(&mut self, predicate: impl Fn(&T) -> bool) {
        self.grid.iter_mut().for_each(|quad| {
            quad.objects.retain(|item| !predicate(&item));
        });
    }

    pub fn for_objects(&self, rect: &Rect, mut func: impl FnMut(&T)) {
        self.grid
            .iter()
            .filter(|quad| rect.collides(&quad.object_bounds))
            .flat_map(|quad| quad.objects.iter())
            .map(|object| (object, Circle::new_from_pos(object.pos(), object.radius())))
            .filter(|(_, circle)| collision::collides_rect(&circle, &rect))
            .for_each(|(object, _)| func(object));
    }

    pub fn for_objects_mut(&mut self, rect: &Rect, mut func: impl FnMut(&mut T)) {
        self.grid
            .iter_mut()
            .filter(|quad| rect.collides(&quad.object_bounds))
            .flat_map(|quad| quad.objects.iter_mut())
            .map(|object| {
                let circle = Circle::new_from_pos(object.pos(), object.radius());
                (object, circle)
            })
            .filter(|(_, circle)| collision::collides_rect(&circle, &rect))
            .for_each(|(object, _)| func(object));
    }

    pub fn for_objects2(&self, rect: &Rect, mut func: impl FnMut(&T)) {
        // TODO Here just for measuring performance difference with the iterator
        // based approach.
        self.grid.iter().for_each(|quad| {
            if rect.collides(&quad.object_bounds) {
                for item in &quad.objects {
                    let circle = Circle::new_from_pos(item.pos(), item.radius());
                    if collision::collides_rect(&circle, &rect) {
                        func(item);
                    }
                }
            }
        });
    }

    pub fn for_objects_mut2(&mut self, rect: &Rect, func: impl Fn(&mut T)) {
        // TODO Here just for measuring performance difference with the iterator
        // based approach.
        self.grid.iter_mut().for_each(|quad| {
            if rect.collides(&quad.object_bounds) {
                for item in &mut quad.objects {
                    let circle = Circle::new_from_pos(item.pos(), item.radius());
                    if collision::collides_rect(&circle, &rect) {
                        func(item);
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct Object {
        pos: PointF32,
        radius: f32,
        alive: bool,
    }

    impl GridObject for Object {
        fn pos(&self) -> PointF32 {
            self.pos
        }

        fn radius(&self) -> f32 {
            self.radius
        }
    }

    #[test]
    fn test_remove_if() {
        let mut grid = LooseGrid::<Object>::new(
            Rect::new(PointF32::new(0.0, 0.0), PointF32::new(100.0, 100.0)),
            10,
            20,
        );

        grid.remove_if(|object| !object.alive);

        grid.grid.value(1, 2).objects.push(Object {
            pos: PointF32::new(15.0, 20.0),
            radius: 5.0,
            alive: true,
        });

        grid.grid.value(1, 2).objects.push(Object {
            pos: PointF32::new(15.0, 21.0),
            radius: 5.0,
            alive: false,
        });

        grid.grid.value(3, 7).objects.push(Object {
            pos: PointF32::new(15.0, 20.0),
            radius: 5.0,
            alive: false,
        });

        grid.remove_if(|object| !object.alive);

        assert_eq!(grid.grid.value(1, 2).objects.len(), 1);
        assert_eq!(grid.grid.value(3, 7).objects.len(), 0);
        assert_eq!(
            grid.grid.value(1, 2).objects[0],
            Object {
                pos: PointF32::new(15.0, 20.0),
                radius: 5.0,
                alive: true,
            }
        );
    }

    macro_rules! assert_vec_eq {
        ($left:ident, $right:ident) => {
            if $left.len() != $right.len() {
                println!("{:?} but expected {:?}", $left, $right);
                panic!();
            }

            for x in &$left {
                let mut found = false;

                for y in &$right {
                    if x == y {
                        found = true;
                        break;
                    }
                }

                if !found {
                    println!("{:?} but expected {:?}", $left, $right);
                    panic!();
                }
            }

            for x in &$right {
                let mut found = false;

                for y in &$left {
                    if x == y {
                        found = true;
                        break;
                    }
                }

                if !found {
                    println!("{:?} but expected {:?}", $left, $right);
                    panic!();
                }
            }
        };
    }

    #[test]
    fn test_for_objects() {
        let mut grid = LooseGrid::<Object>::new(
            Rect::new(PointF32::new(0.0, 0.0), PointF32::new(100.0, 100.0)), //0-25-50-75-100|0-20-40-60-80-100
            4,
            5,
        );
        grid.update();

        grid.for_objects(
            &Rect::new(PointF32::new(0.0, 0.0), PointF32::new(100.0, 100.0)),
            |_| assert!(false),
        );

        let objects = vec![
            Object {
                pos: PointF32::new(45.0, 80.0),
                radius: 5.0,
                alive: true,
            },
            Object {
                pos: PointF32::new(46.0, 75.0),
                radius: 5.0,
                alive: false,
            },
            Object {
                pos: PointF32::new(15.0, 90.0),
                radius: 5.0,
                alive: false,
            },
        ];

        for obj in &objects {
            grid.add(*obj);
        }

        grid.update();

        {
            let mut found = Vec::<Object>::new();
            grid.for_objects(
                &Rect::new(PointF32::new(0.0, 0.0), PointF32::new(100.0, 100.0)),
                |item| found.push(*item),
            );
            assert_vec_eq!(found, objects);
        }

        {
            let expected = vec![objects[0]; 1];
            let mut found = Vec::<Object>::new();
            grid.for_objects(
                &Rect::new(PointF32::new(25.0, 70.0), PointF32::new(40.0, 90.0)),
                |item| found.push(*item),
            );
            assert_vec_eq!(found, expected);
        }

        {
            let expected = Vec::<Object>::new();
            let mut found = Vec::<Object>::new();
            grid.for_objects(
                &Rect::new(PointF32::new(0.0, 00.0), PointF32::new(20.0, 20.0)),
                |item| found.push(*item),
            );
            assert_vec_eq!(found, expected);
        }
    }
}
