pub struct Iter {
    shapes: [Shape; 5],
    idx: usize,
}

impl Iterator for Iter {
    type Item = Shape;
    fn next(&mut self) -> Option<Self::Item> {
        let shape = self.shapes[self.idx];
        self.idx = (self.idx + 1) % self.shapes.len();
        Some(shape)
    }
}

impl IntoIterator for Shapes {
    type Item = Shape;
    type IntoIter = Iter;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            shapes: self.0,
            idx: 0,
        }
    }
}

pub struct Shapes([Shape; 5]);

pub fn shapes() -> Shapes {
    Shapes([
        Shape::Slab,
        Shape::Cross,
        Shape::L,
        Shape::Pipe,
        Shape::Square,
    ])
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Slab,
    Cross,
    L,
    Pipe,
    Square,
}

#[test]
fn test_shapes_iter() {
    let mut iter = shapes().into_iter();
    assert_eq!(iter.next(), Some(Shape::Slab));
    assert_eq!(iter.next(), Some(Shape::Cross));
    assert_eq!(iter.next(), Some(Shape::L));
    assert_eq!(iter.next(), Some(Shape::Pipe));
    assert_eq!(iter.next(), Some(Shape::Square));
    assert_eq!(iter.next(), Some(Shape::Slab));
    assert_eq!(iter.next(), Some(Shape::Cross));
    assert_eq!(iter.next(), Some(Shape::L));
    assert_eq!(iter.next(), Some(Shape::Pipe));
    assert_eq!(iter.next(), Some(Shape::Square));
}
