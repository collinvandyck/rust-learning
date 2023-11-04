use aoc_2020::prelude::*;

fn main() -> Result<()> {
    let s1 = Slope { rise: 1, run: 3 };
    let p1 = (
        trees_encountered("example.txt", s1),
        trees_encountered("input.txt", s1),
    );
    println!("p1={p1:?}");

    let s2s = [
        Slope { rise: 1, run: 1 },
        Slope { rise: 1, run: 3 },
        Slope { rise: 1, run: 5 },
        Slope { rise: 1, run: 7 },
        Slope { rise: 2, run: 1 },
    ];
    let p2 = ["example.txt", "input.txt"]
        .into_iter()
        .map(|f| {
            s2s.iter()
                .map(|slope| trees_encountered(f, *slope).unwrap())
                .product::<usize>()
        })
        .collect::<Vec<_>>();
    println!("p2={p2:?}");
    Ok(())
}

#[derive(Clone, Copy)]
struct Slope {
    run: usize,
    rise: usize,
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn trees_encountered(p: impl AsRef<Path>, slope: Slope) -> Result<usize> {
    let p = PathBuf::from(file!()).parent().unwrap().join(p.as_ref());
    let rows = file_to_lines(p)?
        .into_iter()
        .map(|s| s.parse::<Row>())
        .collect::<StdResult<Vec<_>, _>>()?;
    let map = Map(rows);
    let trees = map.trees_hit(slope);
    Ok(trees)
}

#[derive(Debug)]
struct Map(Vec<Row>);

impl Map {
    fn trees_hit(&self, slope: Slope) -> usize {
        let mut count = 0;
        let mut pos = Point { x: 0, y: 0 };
        loop {
            match self.tile_at(pos) {
                Some(Tile::Tree) => count += 1,
                None => return count,
                _ => {}
            }
            pos.x = (pos.x + slope.run) % self.width();
            pos.y += slope.rise;
        }
    }

    fn width(&self) -> usize {
        self.0.first().map(|row| row.len()).unwrap_or_default()
    }

    fn tile_at(&self, point: Point) -> Option<Tile> {
        self.0
            .get(point.y)
            .and_then(|row| row.get(point.x))
            .cloned()
    }
}

#[derive(Debug)]
struct Row(Vec<Tile>);

impl std::ops::Deref for Row {
    type Target = Vec<Tile>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Row {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(Row(s
            .chars()
            .map(|c| match c {
                '.' => Ok(Tile::Open),
                '#' => Ok(Tile::Tree),
                _ => bail!("unsupported char: {c}"),
            })
            .collect::<StdResult<Vec<_>, _>>()?))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Open,
    Tree,
}
