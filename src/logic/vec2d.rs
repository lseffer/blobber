pub struct Vec2d<T> {
    n_x: u32,
    n_y: u32,
    data: Vec<T>,
}

impl<'a, T: Clone> Vec2d<T> {
    pub fn new(n_x: u32, n_y: u32, value: T) -> Self {
        let vec = vec![value; (n_x * n_y) as usize];
        Vec2d {
            n_x,
            n_y,
            data: vec,
        }
    }
}

impl<'a, T> Vec2d<T> {
    fn index(&self, x: u32, y: u32) -> usize {
        (x * self.n_y + y) as usize
    }

    pub fn new_with_creator(n_x: u32, n_y: u32, creator: impl Fn(u32, u32) -> T) -> Self {
        let mut vec = Vec::<T>::with_capacity((n_x * n_y) as usize);
        for x in 0..n_x {
            for y in 0..n_y {
                vec.push(creator(x, y));
            }
        }

        Vec2d {
            n_x,
            n_y,
            data: vec,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn shape(&self) -> (u32, u32) {
        (self.n_x, self.n_y)
    }

    pub fn get(&self, x: u32, y: u32) -> &T {
        let index = self.index(x, y);
        &self.data[index]
    }

    pub fn set(&mut self, x: u32, y: u32, value: T) {
        let index = self.index(x, y);
        self.data[index] = value
    }

    pub fn value(&mut self, x: u32, y: u32) -> &mut T {
        let index = self.index(x, y);
        &mut self.data[index]
    }

    pub fn for_each(&self, func: impl Fn(u32, u32, &T)) {
        for x in 0..self.n_x {
            for y in 0..self.n_y {
                func(x, y, self.get(x, y));
            }
        }
    }

    pub fn for_each_mut(&mut self, mut func: impl FnMut(u32, u32, &mut T)) {
        for x in 0..self.n_x {
            for y in 0..self.n_y {
                func(x, y, self.value(x, y));
            }
        }
    }

    pub fn iter_mut(&mut self) -> Vec2dIterMut<T> {
        Vec2dIterMut {
            iterator: self.data.iter_mut(),
        }
    }

    pub fn iter(&self) -> Vec2dIter<T> {
        Vec2dIter {
            index: 0,
            parent: self,
        }
    }
}

pub struct Vec2dIter<'a, T> {
    index: usize,
    parent: &'a Vec2d<T>,
}

impl<'a, T> Iterator for Vec2dIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index > self.parent.data.len() {
            None
        } else {
            Some(&self.parent.data[self.index - 1])
        }
    }
}

pub struct Vec2dIterMut<'a, T> {
    iterator: std::slice::IterMut<'a, T>,
}

impl<'a, T> Iterator for Vec2dIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec2d_default() {
        let mut vec = Vec2d::<i32>::new(2, 7, 33);
        assert_eq!(vec.len(), 14);
        assert_eq!(vec.shape(), (2, 7));

        vec.set(0, 3, 30);
        vec.set(1, 6, 42);
        *vec.value(1, 2) = 123;
        *vec.value(1, 2) += 3;

        for x in 0..2 {
            for y in 0..7 {
                match (x, y) {
                    (0, 3) => assert_eq!(vec.get(x, y), &30),
                    (1, 2) => assert_eq!(vec.get(x, y), &126),
                    (1, 6) => assert_eq!(vec.get(x, y), &42),
                    _ => assert_eq!(vec.get(x, y), &33),
                }
            }
        }
    }

    #[test]
    fn test_vec2d_creator() {
        let vec = Vec2d::<i32>::new_with_creator(3, 4, |x: u32, y: u32| -> i32 { (x + y) as i32 });
        assert_eq!(vec.len(), 12);
        assert_eq!(vec.shape(), (3, 4));

        for x in 0..3 {
            for y in 0..4 {
                assert_eq!(*vec.get(x, y), (x + y) as i32);
            }
        }
    }
}
