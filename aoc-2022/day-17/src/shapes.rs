struct ShapeIter {
    shapes: [Shape; 5],
    idx: usize,
}

impl Iterator for ShapeIter {
    type Item = Shape;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        let shape = self.shapes[idx];
        self.idx = idx % self.shapes.len();
        Some(shape)
    }
}

impl IntoIterator for Shapes {
    type Item = Shape;
    type IntoIter = ShapeIter;
    fn into_iter(self) -> Self::IntoIter {
        ShapeIter {
            shapes: self.0,
            idx: 0,
        }
    }
}

struct Shapes([Shape; 5]);

fn shapes() -> [Shape; 5] {
    [
        Shape::Slab,
        Shape::Cross,
        Shape::L,
        Shape::Pipe,
        Shape::Square,
    ]
}

#[derive(Clone, Copy)]
enum Shape {
    Slab,
    Cross,
    L,
    Pipe,
    Square,
}
