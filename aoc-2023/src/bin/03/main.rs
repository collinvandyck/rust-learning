fn main() {
    let example = include_str!("example.txt");
    let input = include_str!("input.txt");
    println!("p1ex={}", sum_of_part_numbers(example));
    println!("p1in={}", sum_of_part_numbers(input));
    println!("p2ex={}", sum_of_gear_ratios(example));
    println!("p2in={}", sum_of_gear_ratios(input));
}

fn sum_of_part_numbers(input: &str) -> u64 {
    Schema::new(input).parts().iter().map(|s| s.num).sum()
}

fn sum_of_gear_ratios(input: &str) -> u64 {
    Schema::new(input).gears().iter().map(|s| s.ratio).sum()
}

#[derive(Debug, Clone, Copy, strum_macros::EnumIs)]
enum Value {
    Space,
    Digit(u64),
    Symbol(char),
}

impl Value {
    fn from(ch: char) -> Self {
        match (ch, ch.to_digit(10)) {
            (_, Some(v)) => Value::Digit(v.into()),
            ('.', _) => Value::Space,
            _ => Value::Symbol(ch),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    num: u64,
    ul: Point,
    lr: Point,
}

impl Part {
    fn overlaps(&self, ul: Point, lr: Point) -> bool {
        self.points()
            .any(|p| p.0 >= ul.0 && p.0 <= lr.0 && p.1 >= ul.1 && p.1 <= lr.1)
    }
    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (self.ul.0..=(self.lr.0)).map(|x| Point(x, self.ul.1))
    }
}

#[derive(Debug)]
struct Gear {
    ratio: u64,
}

#[derive(Debug, Clone, Copy)]
struct Point(usize, usize);
struct Schema(Vec<Vec<Value>>);

impl Schema {
    fn new(s: &str) -> Self {
        Self(
            s.lines()
                .map(|l| l.chars().map(Value::from).collect())
                .collect(),
        )
    }
    fn height(&self) -> usize {
        self.0.len()
    }
    fn width(&self) -> usize {
        self.0.get(0).map(|l| l.len()).unwrap_or_default()
    }
    fn get(&self, x: usize, y: usize) -> Option<Value> {
        self.0.get(y).map(|row| row.get(x)).flatten().copied()
    }
    fn is_part(&self, part: &Part) -> bool {
        let Point(mut ulx, mut uly) = part.ul;
        let Point(mut lrx, mut lry) = part.lr;
        uly = uly.saturating_sub(1);
        ulx = ulx.saturating_sub(1);
        lrx = usize::min(self.width() - 1, lrx + 1);
        lry = usize::min(self.height() - 1, lry + 1);
        for y in uly..=lry {
            for x in ulx..=lrx {
                if let Some(v) = self.get(x, y) {
                    if v.is_symbol() {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn part_adjacent(part: &Part, p: Point) -> bool {
        let Point(x, y) = p;
        let ulx = x.saturating_sub(1);
        let uly = y.saturating_sub(1);
        let lrx = x + 1;
        let lry = y + 1;
        part.overlaps(Point(ulx, uly), Point(lrx, lry))
    }
    fn parts_adjacent(parts: &[Part], p: Point) -> Vec<Part> {
        parts
            .iter()
            .filter(|part| Self::part_adjacent(part, p))
            .take(3) // anything more than three is extra work
            .copied()
            .collect()
    }
    fn gears(&self) -> Vec<Gear> {
        let parts = self.parts();
        let mut gears = vec![];
        for (y, row) in self.0.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                if let Value::Symbol('*') = v {
                    let adj = Self::parts_adjacent(&parts, Point(x, y));
                    if adj.len() == 2 {
                        gears.push(Gear {
                            ratio: adj.iter().map(|a| a.num).product(),
                        })
                    }
                }
            }
        }
        gears
    }
    fn parts(&self) -> Vec<Part> {
        let mut parts = vec![];
        for (y, row) in self.0.iter().enumerate() {
            let mut part: Option<Part> = None;
            for (x, v) in row.iter().enumerate() {
                match part.as_mut() {
                    Some(p) => {
                        if let Value::Digit(v) = v {
                            p.num = p.num * 10 + v;
                            p.lr = Point(x, y);
                        } else {
                            parts.push(part.take().unwrap());
                        }
                    }
                    None => {
                        if let Value::Digit(v) = v {
                            part = Some(Part {
                                num: *v,
                                ul: Point(x, y),
                                lr: Point(x, y),
                            })
                        }
                    }
                }
            }
            if let Some(part) = part.take() {
                parts.push(part);
            }
        }
        parts.into_iter().filter(|p| self.is_part(p)).collect()
    }
}
